use super::*;

#[test]
fn paired_projection_matches_separate_projections_and_roundtrips_order() {
    let mut index = MediaIndex::default();
    for (key, source_url) in [
        ((7, 3), None),
        ((9, 0), None),
        ((10, -1), None),
        ((11, -1), Some("https://example.org".into())),
    ] {
        let mut media = crate::media::media_ref_peer_photo(
            crate::peers::PeerKind::User,
            key.0,
            5,
            6,
            2,
            format!("avatar:{}", key.0),
            false,
            None,
        );
        media.source_url = source_url;
        if key.1 == -1 {
            media.location = crate::media::MediaLocation::Inline {
                bytes: vec![1; 100_000],
                extension: "jpg".into(),
            };
        }
        index.insert(key, media);
    }
    let (entries, order) = media_and_order_from_index(&index);
    assert_eq!(entries, media_from_index(&index));
    assert_eq!(order, media_order_from_index(&index));
    assert!(!entries.contains_key("10:-1"));
    assert_eq!(order, vec!["7:3", "9:0", "11:-1"]);
    let restored = media_to_index(&entries, &order);
    assert_eq!(
        restored.keys().copied().collect::<Vec<_>>(),
        vec![(7, 3), (9, 0), (11, -1)]
    );
}
