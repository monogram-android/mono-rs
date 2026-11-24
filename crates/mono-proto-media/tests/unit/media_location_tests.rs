use super::*;

#[test]
fn legacy_fileref_errors_trigger_reference_refresh() {
    for message in [
        "FILEREF_EXPIRED",
        "RPC 400: FILEREF_EXPIRED",
        "RPC 400: FILEREF_INVALID",
    ] {
        let err = MtprotoError::Message(message.into());
        assert!(is_file_reference_error(&err));
        assert!(crate::download::keep_partial(&err));
    }
    let unrelated = MtprotoError::Message("RPC 400: FILEREF_EXPIRED_OTHER".into());
    assert!(!is_file_reference_error(&unrelated));
    assert!(!crate::download::keep_partial(&unrelated));
}

#[test]
fn targeted_hash_repair_updates_only_requested_entry_and_both_locations() {
    use crate::peers::{CachedPeer, PeerKind};
    let mut peers = HashMap::default();
    peers.insert(
        7,
        CachedPeer {
            kind: PeerKind::User,
            id: 7,
            access_hash: 77,
            min_hash: false,
        },
    );
    let mut media = MediaIndex::default();
    let item = crate::media::media_ref_peer_photo(
        PeerKind::User,
        7,
        0,
        9,
        2,
        "avatar:7".into(),
        false,
        None,
    );
    media.insert((7, 0), item.clone());
    media.insert((7, 1), item.clone());
    fill_zero_peer_photo_hash(&mut media, &peers, 7, 0);
    assert!(matches!(
        media[&(7, 0)].location,
        MediaLocation::PeerPhoto {
            access_hash: 77,
            ..
        }
    ));
    assert!(matches!(
        media[&(7, 0)].display_location,
        Some(MediaLocation::PeerPhoto {
            access_hash: 77,
            ..
        })
    ));
    assert_eq!(media[&(7, 1)], item);
    // Nonzero hashes are not replaced, and min peers cannot supply a hash.
    peers.get_mut(&7).unwrap().access_hash = 88;
    fill_zero_peer_photo_hash(&mut media, &peers, 7, 0);
    assert!(matches!(
        media[&(7, 0)].location,
        MediaLocation::PeerPhoto {
            access_hash: 77,
            ..
        }
    ));
    peers.get_mut(&7).unwrap().min_hash = true;
    fill_zero_peer_photo_hash(&mut media, &peers, 7, 1);
    assert_eq!(media[&(7, 1)], item);
}

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
