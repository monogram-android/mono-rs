use super::*;
use mono_proto_media::media::{MediaLocation, MediaRef};

fn entry(value: u8) -> MediaRef {
    MediaRef {
        file_size: None,
        kind: "photo".into(),
        cache_key: format!("snapshot:{value}"),
        location: MediaLocation::Inline { bytes: vec![value; 4096], extension: "jpg".into() },
        thumb_cache_key: None,
        thumb_location: None,
        display_cache_key: None,
        display_location: None,
        sticker_set_id: None,
        sticker_set_access_hash: None,
        source_url: None, refresh_source: None, webpage_id: None,
    }
}

fn fixture() -> MediaIndex {
    (1..=5).map(|id| ((id, id as i32), entry(id as u8))).collect()
}

#[test]
fn cloned_media_snapshots_share_storage_until_mutated() {
    let original = fixture();
    let mut copy = original.clone();
    assert!(original.shares_storage_with(&copy));
    assert_eq!(copy.get(&(1, 1)), original.get(&(1, 1)));
    assert_eq!(copy.iter().count(), 5);
    assert!(original.shares_storage_with(&copy));

    copy.get_mut(&(1, 1)).unwrap().cache_key = "changed".into();
    assert!(!original.shares_storage_with(&copy));
    assert_eq!(original.get(&(1, 1)).unwrap().cache_key, "snapshot:1");
    assert_eq!(copy.get(&(1, 1)).unwrap().cache_key, "changed");
    for (_, value) in &mut copy {
        value.file_size = Some(12);
    }
    assert!(original.values().all(|value| value.file_size.is_none()));
    assert!(copy.values().all(|value| value.file_size == Some(12)));
    assert_eq!(copy.keys().copied().collect::<Vec<_>>(), original.keys().copied().collect::<Vec<_>>());
}

#[test]
fn unchanged_media_lane_keeps_concurrent_updates_without_detaching_live_storage() {
    let before = fixture();
    let incoming = before.clone();
    let mut current = before.clone();
    assert!(!merge_changed_entries(&mut current, &before, incoming));
    assert!(current.shares_storage_with(&before));

    current.insert((1, 1), entry(91));
    current.shift_remove(&(2, 2));
    current.insert((6, 6), entry(96));
    let live_snapshot = current.clone();
    assert!(!merge_changed_entries(&mut current, &before, before.clone()));
    assert!(current.shares_storage_with(&live_snapshot));
    assert_eq!(current, live_snapshot);
}

#[test]
fn detached_media_lane_matches_three_way_map_merge_and_global_order() {
    let before = fixture();
    let mut current = before.clone();
    current.insert((1, 1), entry(91));
    current.insert((2, 2), entry(92));
    current.insert((6, 6), entry(96));
    let mut incoming = before.clone();
    incoming.insert((1, 1), entry(81));
    incoming.insert((3, 3), entry(83));
    incoming.shift_remove(&(2, 2));
    incoming.shift_remove(&(5, 5));
    incoming.insert((7, 7), entry(87));

    let plain = |index: &MediaIndex| index.iter().map(|(key, value)| (*key, value.clone())).collect::<IndexMap<_, _>>();
    let mut expected = plain(&current);
    assert!(merge_changed_entries(&mut expected, &plain(&before), plain(&incoming)));
    assert!(merge_changed_entries(&mut current, &before, incoming));
    assert_eq!(current.iter().collect::<Vec<_>>(), expected.iter().collect::<Vec<_>>());
    assert_eq!(current.get(&(1, 1)).unwrap(), &entry(91));
    assert_eq!(current.get(&(2, 2)).unwrap(), &entry(92));
    assert_eq!(current.get(&(3, 3)).unwrap(), &entry(83));
    assert!(!current.contains_key(&(5, 5)));
    assert!(current.contains_key(&(6, 6)));
    assert!(current.contains_key(&(7, 7)));
    assert_eq!(before, fixture());
}

#[test]
fn shared_media_snapshot_persists_with_identical_payload_and_order() {
    use mono_proto_media::persistence::{media_and_order_from_index, media_to_index};
    let original = fixture();
    let snapshot = original.clone();
    let (stored, order) = media_and_order_from_index(&snapshot);
    assert!(original.shares_storage_with(&snapshot));
    let restored = media_to_index(&stored, &order);
    assert_eq!(original.iter().collect::<Vec<_>>(), restored.iter().collect::<Vec<_>>());
    let expected: Vec<_> = original.iter().map(|(key, value)| (*key, value.clone())).collect();
    assert_eq!(snapshot.into_iter().collect::<Vec<_>>(), expected);
    assert_eq!(original.len(), 5);
    let owned: IndexMap<_, _> = original.clone().into();
    assert_eq!(original.iter().collect::<Vec<_>>(), owned.iter().collect::<Vec<_>>());
}

#[test]
fn mutable_iteration_detaches_a_shared_snapshot_before_its_first_write() {
    let original = fixture();
    let mut copy = original.clone();
    assert!(copy.shares_storage_with(&original));
    for (_, value) in &mut copy {
        value.cache_key.push_str(":changed");
    }
    assert!(!copy.shares_storage_with(&original));
    assert_eq!(original, fixture());
    assert_eq!(copy.keys().collect::<Vec<_>>(), original.keys().collect::<Vec<_>>());
    assert!(copy.values().all(|value| value.cache_key.ends_with(":changed")));
}

#[test]
fn semantically_unchanged_detached_lane_preserves_current_values_and_order() {
    let before = fixture();
    let mut incoming = before.clone();
    assert!(incoming.get_mut(&(99, 99)).is_none());
    assert!(!incoming.shares_storage_with(&before));
    let mut current = before.clone();
    current.shift_remove(&(1, 1));
    current.insert((2, 2), entry(92));
    current.insert((6, 6), entry(96));
    let expected = current.clone();
    assert!(!merge_changed_entries(&mut current, &before, incoming));
    assert_eq!(current.iter().collect::<Vec<_>>(), expected.iter().collect::<Vec<_>>());
}

#[test]
fn dirty_lane_cannot_restore_a_live_deletion_or_overwrite_a_concurrent_insert() {
    let before = fixture();
    let mut incoming = before.clone();
    incoming.insert((1, 1), entry(81));
    incoming.insert((6, 6), entry(86));
    let mut current = before.clone();
    current.shift_remove(&(1, 1));
    current.insert((6, 6), entry(96));
    let expected = current.clone();
    assert!(!merge_changed_entries(&mut current, &before, incoming));
    assert!(!current.contains_key(&(1, 1)));
    assert_eq!(current.get(&(6, 6)), Some(&entry(96)));
    assert_eq!(current.iter().collect::<Vec<_>>(), expected.iter().collect::<Vec<_>>());
}
