use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn upload_lanes_bind_on_first_use_and_remain_available_in_parallel() {
    let path = std::env::temp_dir().join(format!("mono-lazy-upload-{}", std::process::id()));
    let handle =
        crate::construct::create_client(1, "test".into(), path.to_string_lossy().into_owned());
    let client = crate::registry::get_client(handle).unwrap();
    {
        let mut data = client.data.lock();
        data.user_id = Some(42);
        data.home_auth_key = Some(vec![1; 256]);
        data.home_snapshot.auth_key = Some(vec![1; 256]);
    }
    let calls = AtomicUsize::new(0);
    let negotiate = |_: &[u8], dc: i32, _: i64| {
        calls.fetch_add(1, Ordering::SeqCst);
        let mut snapshot = Snapshot::new(dc, &mut mono_proto_session::OsRandom).unwrap();
        snapshot.auth_key = Some(vec![2; 256]);
        Ok(BoundTemp {
            snapshot,
            expires_at: unix_now() + 3600,
        })
    };
    prepare_extra_main_sessions(handle, negotiate).unwrap();
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(
        client
            .upload
            .iter()
            .all(|lane| lane.io.lock().perm_key.is_none())
    );

    let mut lanes: Vec<_> = (0..4)
        .map(|_| crate::lanes::lock_upload_lane(&client, true).unwrap())
        .collect();
    let failed = ensure_main_temp_with(&client, &mut lanes[0], true, &SystemClock, |_, _, _| {
        Err(MtprotoError::Message("bind failed".into()))
    });
    assert!(failed.is_err());
    assert!(lanes[0].perm_key.is_none());
    let invalid = ensure_main_temp_with(
        &client,
        &mut lanes[0],
        true,
        &SystemClock,
        |permanent, dc, _| {
            let mut snapshot = Snapshot::new(dc, &mut mono_proto_session::OsRandom).unwrap();
            snapshot.auth_key = Some(permanent.to_vec());
            Ok(BoundTemp {
                snapshot,
                expires_at: unix_now() + 3600,
            })
        },
    );
    assert!(invalid.is_err());
    assert!(lanes[0].perm_key.is_none());
    for lane in &mut lanes {
        ensure_main_temp_with(&client, lane, true, &SystemClock, negotiate).unwrap();
    }
    assert_eq!(calls.load(Ordering::SeqCst), 5);
    for lane in &mut lanes {
        assert!(!ensure_main_temp_with(&client, lane, true, &SystemClock, negotiate).unwrap());
    }
    assert_eq!(calls.load(Ordering::SeqCst), 5);
    drop(lanes);
    {
        let mut cold = client.upload[0].io.lock();
        cold.temp_expires_at = 0;
        cold.perm_key = None;
    }
    let preferred = crate::lanes::lock_upload_lane(&client, true).unwrap();
    assert!(preferred.perm_key.is_some());
    drop(preferred);
    drop(client);
    crate::registry::destroy_client(handle);
    let _ = std::fs::remove_file(path);
}

#[test]
fn upload_wait_uses_released_lane_and_observes_cancellation_and_shutdown() {
    use std::sync::mpsc;
    use std::time::Duration;

    let path = std::env::temp_dir().join(format!("mono-upload-wait-{}", std::process::id()));
    let handle =
        crate::construct::create_client(1, "test".into(), path.to_string_lossy().into_owned());
    let client = crate::registry::get_client(handle).unwrap();
    let mut held: Vec<_> = client
        .upload
        .iter()
        .map(|lane| Some(lane.io.lock()))
        .collect();
    let (tx, rx) = mpsc::channel();
    let worker = client.clone();
    let thread = std::thread::spawn(move || {
        tx.send(crate::lanes::lock_upload_lane(&worker, true).is_ok())
            .unwrap();
    });
    assert!(rx.recv_timeout(Duration::from_millis(50)).is_err());
    drop(held[2].take());
    assert_eq!(rx.recv_timeout(Duration::from_secs(2)).unwrap(), true);
    thread.join().unwrap();
    held[2] = Some(client.upload[2].io.lock());

    let id = mono_proto_runtime::request_control::create();
    let (tx, rx) = mpsc::channel();
    let worker = client.clone();
    let thread = std::thread::spawn(move || {
        mono_proto_runtime::request_control::bind(id);
        tx.send(crate::lanes::lock_upload_lane(&worker, true).is_err())
            .unwrap();
        mono_proto_runtime::request_control::bind(0);
    });
    assert!(rx.recv_timeout(Duration::from_millis(50)).is_err());
    mono_proto_runtime::request_control::cancel(id);
    assert_eq!(rx.recv_timeout(Duration::from_secs(2)).unwrap(), true);
    thread.join().unwrap();
    mono_proto_runtime::request_control::release(id);

    let (tx, rx) = mpsc::channel();
    let worker = client.clone();
    let thread = std::thread::spawn(move || {
        tx.send(crate::lanes::lock_upload_lane(&worker, true).is_err())
            .unwrap();
    });
    assert!(rx.recv_timeout(Duration::from_millis(50)).is_err());
    client.connections.close();
    assert_eq!(rx.recv_timeout(Duration::from_secs(2)).unwrap(), true);
    thread.join().unwrap();
    drop(held);
    drop(client);
    crate::registry::destroy_client(handle);
    let _ = std::fs::remove_file(path);
}
