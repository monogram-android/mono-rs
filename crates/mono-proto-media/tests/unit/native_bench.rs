//! Run: cargo test --release -p mono-proto-media --lib native_bench -- --ignored --nocapture --test-threads=1
use super::*;
use crate::media::MediaLocation;
#[path = "native_bench_support.rs"]
mod support;
use support::{bench, bench_reset, fixture};

fn owned_snapshot(index: &MediaIndex) -> MediaIndex {
    index
        .iter()
        .map(|(key, value)| (*key, value.clone()))
        .collect()
}

#[test]
#[ignore = "large native microbenchmark; run explicitly in release"]
fn native_bench_media_projection_lookup_and_prune() {
    assert!(!cfg!(debug_assertions), "run with --release");
    let (peers, media, bytes) = fixture();
    let media = MediaIndex::from(media);
    let separate = (media_from_index(&media), media_order_from_index(&media));
    let combined = media_and_order_from_index(&media);
    assert_eq!(separate, combined);
    assert_eq!(combined.0.len(), support::COUNT);
    assert_eq!(combined.1.len(), support::COUNT);
    bench("media_projection_separate_pass_reference", bytes, || {
        (media_from_index(&media), media_order_from_index(&media))
    });
    bench("media_projection_combined_current", bytes, || {
        media_and_order_from_index(&media)
    });
    bench("media_snapshot_clone", bytes, || media.clone());
    bench("peers_snapshot_clone", bytes, || peers.clone());
    // The previous lookup repair scanned everything, while a lookup only needs
    // its selected entry. Verify selected-entry equivalence, not global equality.
    let target = 8701;
    let mut whole = media.clone();
    let mut selected = media.clone();
    crate::media::fill_zero_peer_photo_hashes(&mut whole, &peers);
    crate::media::fill_zero_peer_photo_hash(&mut selected, &peers, target, 0);
    assert_eq!(whole.get(&(target, 0)), selected.get(&(target, 0)));
    bench_reset(
        "avatar_hash_full_scan_reference",
        bytes,
        || owned_snapshot(&media),
        |mut m| {
            crate::media::fill_zero_peer_photo_hashes(&mut m, &peers);
            // Keep destruction out of the timed region for this mutation-only pair.
            m
        },
    );
    bench_reset(
        "avatar_hash_selected_current",
        bytes,
        || owned_snapshot(&media),
        |mut m| {
            crate::media::fill_zero_peer_photo_hash(&mut m, &peers, target, 0);
            m
        },
    );
    // Already bounded avatar-only state: measures repeated classification cost.
    bench_reset(
        "media_bound_avatar_only",
        bytes,
        || media.clone(),
        |mut m| {
            crate::media::bound_media_index(&mut m);
            m
        },
    );
    // Overflow reference faithfully uses repeated stable removals; compare final
    // content AND insertion order with the production algorithm.
    let mut overflow = media.clone();
    for n in 1..=1000 {
        let mut entry = media.get(&(n, 0)).unwrap().clone();
        entry.cache_key = format!("photo:{n}");
        entry.location = MediaLocation::Inline {
            bytes: vec![7; 256],
            extension: "jpg".into(),
        };
        overflow.insert((n, n as i32), entry);
    }
    let reference = |mut m: MediaIndex| {
        for n in 1..=500 {
            m.shift_remove(&(n, n as i32));
        }
        m
    };
    let mut bounded = overflow.clone();
    crate::media::bound_media_index(&mut bounded);
    assert_eq!(
        bounded.iter().collect::<Vec<_>>(),
        reference(overflow.clone()).iter().collect::<Vec<_>>()
    );
    bench_reset(
        "media_prune_shift_remove_reference",
        bytes + 256_000,
        || owned_snapshot(&overflow),
        reference,
    );
    bench_reset(
        "media_prune_current",
        bytes + 256_000,
        || owned_snapshot(&overflow),
        |mut m| {
            crate::media::bound_media_index(&mut m);
            m
        },
    );
    bench_reset(
        "avatar_hash_selected_shared_snapshot",
        bytes,
        || media.clone(),
        |mut m| {
            crate::media::fill_zero_peer_photo_hash(&mut m, &peers, target, 0);
            m
        },
    );
    bench_reset(
        "media_prune_shared_snapshot",
        bytes + 256_000,
        || overflow.clone(),
        |mut m| {
            crate::media::bound_media_index(&mut m);
            m
        },
    );
}
