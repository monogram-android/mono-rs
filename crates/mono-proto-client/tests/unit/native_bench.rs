//! Run: cargo test --release -p mono-proto-client --lib native_bench -- --ignored --nocapture --test-threads=1
use super::*;
#[path = "../../../mono-proto-media/tests/unit/native_bench_support.rs"]
mod support;
use mono_proto_media::persistence::{media_and_order_from_index, media_from_index, media_order_from_index};
use support::{bench, bench_reset, fixture};

#[test]
#[ignore = "large native microbenchmark; run explicitly in release"]
fn native_bench_read_lane_and_snapshot_projection() {
    assert!(!cfg!(debug_assertions), "run with --release");
    // Synthetic bound-key identities prevent authorization repair/network access.
    // No request is sent and no durable mutation is made by these closures.
    let path = std::env::temp_dir().join(format!("mono-native-bench-{}", std::process::id()));
    assert!(!path.exists(), "benchmark requires an unused session path");
    let handle = crate::construct::create_client(1, "synthetic-bench".into(), path.to_string_lossy().into());
    let client = get_client(handle).unwrap();
    let mut home = client.main.lock().snapshot.clone();
    home.auth_key = Some(vec![1; 256]);
    client.main.lock().snapshot = home.clone();
    let (peers, media, bytes) = fixture();
    let media = mono_proto_media::media::MediaIndex::from(media);
    {
        let mut d = client.data.lock();
        d.user_id = Some(42);
        d.home_auth_key = home.auth_key.clone();
        d.home_snapshot = home.clone();
        d.peers = peers.clone();
        d.media = media.clone();
    }
    for lane in &client.rpc {
        let mut io = lane.io.lock();
        io.snapshot.auth_key = Some(vec![2; 256]);
        io.temp_expires_at = crate::bind_temp::unix_now() + 3600;
        io.perm_key = home.auth_key.clone();
    }
    client.policy.main_session_allowance.store(3, Ordering::Relaxed);
    client.policy.bound_extra_sessions.store(2, Ordering::Relaxed);
    bench("read_lane_noop_current", bytes, || {
        with_read_lane(handle, |_| {
            assert!(EXTRA_READ_LANE.with(Cell::get), "must not measure fallback");
            Ok(())
        }).unwrap();
    });
    bench("read_lane_lane_local_metadata_current", bytes, || {
        with_read_lane(handle, |state| {
            assert!(EXTRA_READ_LANE.with(Cell::get));
            state.snapshot.server_salt = state.snapshot.server_salt.wrapping_add(1);
            Ok(())
        }).unwrap();
    });
    assert_eq!(client.data.lock().persist_epoch, 0);
    assert!(!client.persist_running.load(Ordering::Acquire));
    assert!(!client.persist_queued.load(Ordering::Acquire));
    // Isolate a metadata-only peer change in the actual three-way merge, with
    // fixture clones outside timing. No persistence scheduling/disk is involved.
    bench_reset("peer_metadata_three_way_merge_current", bytes,
        || { let mut after = peers.clone(); after.get_mut(&8701).unwrap().access_hash += 1; (peers.clone(), after) },
        |(mut live, after)| {
            assert!(crate::merge::merge_changed_entries(&mut live, &peers, after));
            assert_eq!(live.get(&8701).unwrap().access_hash, 8701 * 31 + 1);
            live
        });
    // Faithful map/projection portion of native session capture only: not JSON,
    // encryption, fsync, SQLite, or a controlled old-binary comparison.
    let reference = (peers.clone(), media_from_index(&media), media_order_from_index(&media));
    let (entries, order) = media_and_order_from_index(&media);
    assert_eq!(reference, (peers.clone(), entries, order));
    bench("native_capture_separate_projection_reference", bytes, ||
        (peers.clone(), media_from_index(&media), media_order_from_index(&media)));
    bench("native_capture_combined_projection_current", bytes, || {
        let (entries, order) = media_and_order_from_index(&media);
        (peers.clone(), entries, order)
    });
    crate::registry::destroy_client(handle);
    drop(client);
    assert!(!path.exists(), "clean microbench must not persist a session");
}
