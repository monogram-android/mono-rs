//! Synthetic clean-read and map scaling: no RPC, auth negotiation or persistence.
//! Run explicitly with --release --ignored --nocapture --test-threads=1.
use super::*;
use mono_proto_storage as storage;
#[path = "../../../mono-proto-storage/tests/unit/native_workload_support.rs"]
mod support;
use mono_proto_media::{media::MediaIndex, persistence::media_and_order_from_index};
use support::{CLASSES, TempDir, fixture, measure};

struct Handle(u64);
impl Drop for Handle {
    fn drop(&mut self) {
        crate::registry::destroy_client(self.0);
    }
}

fn read_fixture(count: usize) -> (TempDir, Handle, Arc<Client>, usize) {
    let dir = TempDir::new();
    let path = dir.0.join("clean.session");
    assert!(!path.exists());
    let session = fixture(count);
    let bytes = serde_json::to_vec(&session).unwrap().len();
    let media: MediaIndex = (1..=count)
        .map(|n| ((n as i64, 1), session.media[&format!("{n}:1")].clone()))
        .collect();
    let handle = Handle(crate::construct::create_client(
        1,
        "synthetic-offline-workload".into(),
        path.to_string_lossy().into(),
    ));
    let client = get_client(handle.0).unwrap();
    let mut home = client.main.lock().snapshot.clone();
    home.auth_key = session.snapshot.auth_key.clone();
    client.main.lock().snapshot = home.clone();
    {
        let mut data = client.data.lock();
        data.user_id = session.user_id;
        data.home_auth_key = home.auth_key.clone();
        data.home_snapshot = home.clone();
        data.peers = session.peers;
        data.media = media;
    }
    // Every lane is already bound to the synthetic permanent identity. No live
    // transport exists and the timed closures only return local data.
    for lane in &client.rpc {
        let mut io = lane.io.lock();
        io.snapshot.auth_key = Some(vec![0x62; 256]);
        io.temp_expires_at = crate::bind_temp::unix_now() + 3_600;
        io.perm_key = home.auth_key.clone();
    }
    client
        .policy
        .main_session_allowance
        .store(3, Ordering::Relaxed);
    client
        .policy
        .bound_extra_sessions
        .store(2, Ordering::Relaxed);
    (dir, handle, client, bytes)
}
fn verify_clean(client: &Client, count: usize) {
    let data = client.data.lock();
    assert_eq!(data.peers.len(), count);
    assert_eq!(data.media.len(), count);
    assert_eq!(data.persist_epoch, 0);
    assert_eq!(data.persisted_epoch, 0);
    assert!(!client.persist_running.load(Ordering::Acquire));
    assert!(!client.persist_queued.load(Ordering::Acquire));
    assert!(!data.session_path.exists());
}

#[test]
fn native_workload_clean_lane_remains_clean() {
    let (_dir, handle, client, _) = read_fixture(8);
    let used_extra = with_read_lane(handle.0, |_| Ok(EXTRA_READ_LANE.with(Cell::get))).unwrap();
    assert!(used_extra, "clean benchmark must use the extra read lane");
    verify_clean(&client, 8);
    drop(handle);
    assert!(!client.data.lock().session_path.exists());
}

#[test]
#[ignore = "bounded clean-read scaling benchmark; run explicitly in release"]
fn native_workload_clean_read_scaling() {
    assert!(!cfg!(debug_assertions), "run with --release");
    for (class, count) in CLASSES {
        let (dir, handle, client, bytes) = read_fixture(count);
        // Baselines exercise the real collection Clone implementations. The
        // output lives past the timer, so destroying maps is not measured.
        let (peers, media) = {
            let data = client.data.lock();
            (data.peers.clone(), data.media.clone())
        };
        measure(
            "peer_map_clone",
            class,
            count,
            bytes,
            || (),
            |_| peers.clone(),
            |cloned| assert_eq!(cloned, &peers),
        );
        measure(
            "media_index_clone",
            class,
            count,
            bytes,
            || (),
            |_| media.clone(),
            |cloned| assert_eq!(cloned, &media),
        );
        measure(
            "session_capture_projection",
            class,
            count,
            bytes,
            || (),
            |_| {
                let (entries, order) = media_and_order_from_index(&media);
                (peers.clone(), entries, order)
            },
            |(cloned_peers, entries, order)| {
                assert_eq!(cloned_peers, &peers);
                assert_eq!(entries.len(), count);
                assert_eq!(order.len(), count);
                assert_eq!(order.first().unwrap(), "1:1");
                assert_eq!(order.last().unwrap(), &format!("{count}:1"));
            },
        );
        measure(
            "clean_extra_read_lane_total",
            class,
            count,
            bytes,
            || (),
            |_| with_read_lane(handle.0, |_| Ok(EXTRA_READ_LANE.with(Cell::get))),
            |result| {
                assert!(
                    *result.as_ref().unwrap(),
                    "must not measure auth repair/fallback"
                );
                verify_clean(&client, count);
            },
        );
        measure(
            "lane_local_metadata_read_total",
            class,
            count,
            bytes,
            || (),
            |_| {
                with_read_lane(handle.0, |state| {
                    state.snapshot.server_salt = state.snapshot.server_salt.wrapping_add(1);
                    Ok(EXTRA_READ_LANE.with(Cell::get))
                })
            },
            |result| {
                assert!(*result.as_ref().unwrap());
                verify_clean(&client, count);
            },
        );
        verify_clean(&client, count);
        drop(handle);
        assert_eq!(std::fs::read_dir(&dir.0).unwrap().count(), 0);
    }
}
