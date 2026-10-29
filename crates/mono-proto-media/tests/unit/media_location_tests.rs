use super::*;

#[test]
fn scheduled_media_ids_exclude_history_avatars_and_instant_view() {
    for id in [1, 37, i32::MAX] {
        let encoded = scheduled_media_id(id).unwrap();
        assert!(encoded < -1);
        assert_eq!(scheduled_message_id(encoded), Some(id));
    }
    for id in [i32::MIN, -1, 0] {
        assert!(scheduled_media_id(id).is_err());
    }
    for id in [-1, 0, 1, i32::MAX] {
        assert_eq!(scheduled_message_id(id), None);
    }
}

#[test]
fn peer_photo_location_token_tracks_access_hash() {
    let peer = MediaLocation::PeerPhoto {
        peer_kind: crate::peers::PeerKind::User,
        peer_id: 1,
        access_hash: 0,
        photo_id: 1,
        big: true,
        dc_id: 2,
    };
    let mut updated = peer.clone();
    if let MediaLocation::PeerPhoto { access_hash, .. } = &mut updated {
        *access_hash = 9;
    }
    assert_ne!(location_token(&peer), location_token(&updated));
}
