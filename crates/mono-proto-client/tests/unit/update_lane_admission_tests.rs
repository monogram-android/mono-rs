use std::fs;
use std::sync::atomic::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn background_update_admission_defers_for_interactive_waiters() {
    let path = std::env::temp_dir().join(format!(
        "monogram-update-lane-{}.session",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos(),
    ));
    let handle = crate::construct::create_client(1, "test-api-hash".into(), path.to_string_lossy().into());
    let client = crate::registry::get_client(handle).expect("client");

    assert!(super::lock_updates_lane(&client)
        .expect("background admission")
        .is_some());

    client
        .interactive_waiters
        .store(0, Ordering::Release);
    let raced = super::lock_updates_lane_with(&client, || {
        client
            .interactive_waiters
            .store(1, Ordering::Release);
    })
    .expect("race-window admission");
    assert!(raced.is_none());

    client
        .interactive_waiters
        .store(0, Ordering::Release);
    assert!(super::lock_updates_lane(&client)
        .expect("released background admission")
        .is_some());

    client
        .interactive_waiters
        .store(1, Ordering::Release);
    assert!(super::lock_updates_lane(&client)
        .expect("interactive deferral")
        .is_none());

    client
        .interactive_waiters
        .store(0, Ordering::Release);
    let held_gate = client
        .main_gate
        .try_acquire(mono_proto_runtime::scheduler::RequestClass::InteractiveRead)
        .expect("interactive gate");
    assert!(super::lock_updates_lane(&client)
        .expect("busy gate deferral")
        .is_none());
    drop(held_gate);

    crate::registry::destroy_client(handle);
    let _ = fs::remove_file(path);
}
