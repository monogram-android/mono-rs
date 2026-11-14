use super::index_full_chat_photo;
use crate::media::{self, MediaIndex, MediaLocation};
use mono_proto::codec::{Encoder, Limits, TlEncode, from_bytes};
use mono_proto::latest::api::*;

fn full_chat(channel: bool, photo: Option<Photo>) -> ChatFull {
    // Encode only required fields to keep these fixtures independent of dozens
    // of unrelated optional profile fields.
    let mut e = Encoder::new();
    e.write_u32(if channel {
        ChannelFullConstructor::ID
    } else {
        ChatFullConstructor::ID
    });
    e.write_u32(if !channel && photo.is_some() {
        ChatFullConstructor::CHAT_PHOTO_FLAG
    } else {
        0
    });
    if channel {
        e.write_u32(0);
    } // flags2
    7_i64.encode(&mut e).unwrap();
    String::new().encode(&mut e).unwrap(); // about
    if channel {
        for _ in 0..3 {
            0_i32.encode(&mut e).unwrap();
        } // inbox/outbox/unread
        photo
            .unwrap_or(Photo::PhotoEmpty(PhotoEmptyConstructor { id: 0 }))
            .encode(&mut e)
            .unwrap();
    } else {
        e.write_u32(ChatParticipantsForbiddenConstructor::ID);
        e.write_u32(0);
        7_i64.encode(&mut e).unwrap();
        if let Some(photo) = photo {
            photo.encode(&mut e).unwrap();
        }
    }
    e.write_u32(PeerNotifySettingsConstructor::ID);
    e.write_u32(0);
    if channel {
        e.write_u32(VectorConstructor::<Box<BotInfo>>::ID);
        e.write_u32(0); // bot_info
        0_i32.encode(&mut e).unwrap(); // pts
    }
    from_bytes(e.as_slice(), Limits::default()).unwrap()
}

fn photo(video: bool) -> Photo {
    Photo::Photo(PhotoConstructor {
        flags: if video {
            PhotoConstructor::VIDEO_SIZES_FLAG
        } else {
            0
        },
        has_stickers: None,
        id: 9,
        access_hash: 99,
        file_reference: vec![1, 2],
        date: 0,
        sizes: Box::new(Vector::Vector(VectorConstructor {
            field_0: 1,
            field_1: vec![Box::new(PhotoSize::PhotoSize(PhotoSizeConstructor {
                type_: "x".into(),
                w: 640,
                h: 640,
                size: 123,
            }))],
        })),
        video_sizes: video.then(|| {
            Box::new(Vector::Vector(VectorConstructor {
                field_0: 1,
                field_1: vec![Box::new(VideoSize::VideoSize(VideoSizeConstructor {
                    flags: 0,
                    type_: "v".into(),
                    w: 640,
                    h: 640,
                    size: 456,
                    video_start_ts: None,
                }))],
            }))
        }),
        dc_id: 2,
    })
}

#[test]
fn full_channel_and_basic_chat_photos_replace_lightweight_and_keep_video_display() {
    for channel in [false, true] {
        for video in [false, true] {
            let id = if channel { -1_000_000_000_007 } else { -7 };
            let mut index = MediaIndex::default();
            let lightweight = media::media_ref_peer_photo(
                crate::peers::PeerKind::Chat,
                7,
                0,
                9,
                2,
                format!("avatar:{id}"),
                video,
                None,
            );
            media::index_avatar(&mut index, id, lightweight);
            let key =
                index_full_chat_photo(&mut index, id, &full_chat(channel, Some(photo(video))))
                    .unwrap();
            let indexed = &index[&(id, 0)];
            assert_eq!(key, indexed.cache_key);
            assert!(matches!(
                indexed.location,
                MediaLocation::Photo {
                    id: 9,
                    access_hash: 99,
                    ..
                }
            ));
            for kind in [
                media::MediaDownloadKind::Display,
                media::MediaDownloadKind::Thumb,
            ] {
                assert!(
                    media::media_for_download(indexed, kind).is_ok(),
                    "channel={channel} video={video} kind={kind:?}"
                );
            }
            if video {
                assert_eq!(indexed.kind, "video_avatar");
                assert!(matches!(
                    indexed.display_location,
                    Some(MediaLocation::Photo { .. })
                ));
            }
        }
    }
}

#[test]
fn full_video_without_still_sizes_keeps_matching_lightweight_projections() {
    for channel in [false, true] {
        let id = if channel { -1_000_000_000_007 } else { -7 };
        let mut index = MediaIndex::default();
        let mut lightweight = media::media_ref_peer_photo(
            crate::peers::PeerKind::Chat,
            7,
            0,
            9,
            2,
            format!("avatar:{id}:video"),
            true,
            None,
        );
        lightweight.thumb_cache_key = Some(format!("avatar:{id}:thumb"));
        lightweight.thumb_location = Some(MediaLocation::Inline {
            bytes: vec![1, 2, 3],
            extension: "jpg".into(),
        });
        media::index_avatar(&mut index, id, lightweight);
        let Photo::Photo(mut full_photo) = photo(true) else {
            unreachable!()
        };
        full_photo.sizes = Box::new(Vector::Vector(VectorConstructor {
            field_0: 0,
            field_1: vec![],
        }));
        let full = full_chat(channel, Some(Photo::Photo(full_photo)));
        // Repeat the refresh to cover matching resolved, not just lightweight,
        // cache entries as the fallback projection source.
        for _ in 0..2 {
            index_full_chat_photo(&mut index, id, &full).unwrap();
        }
        let indexed = &index[&(id, 0)];
        assert!(matches!(
            media::media_for_download(indexed, media::MediaDownloadKind::Thumb)
                .unwrap()
                .location,
            MediaLocation::Inline { .. }
        ));
        assert!(matches!(
            media::media_for_download(indexed, media::MediaDownloadKind::Display)
                .unwrap()
                .location,
            MediaLocation::PeerPhoto {
                photo_id: 9,
                big: false,
                ..
            }
        ));
        assert!(media::is_resolved_video_avatar(indexed));
    }
}

#[test]
fn full_static_avatar_survives_saturated_document_pool_and_persistence() {
    for channel in [false, true] {
        let id = if channel { -1_000_000_000_007 } else { -7 };
        let mut index = MediaIndex::default();
        media::index_avatar(
            &mut index,
            id,
            media::media_ref_peer_photo(
                crate::peers::PeerKind::Chat,
                7,
                0,
                9,
                2,
                format!("avatar:{id}"),
                false,
                None,
            ),
        );
        // The avatar is older than every document, so reclassification would
        // evict the replacement immediately rather than just a later document.
        for document in 1..=media::ZERO_ID_DOCUMENT_CAP as i64 {
            let mut entry = media::media_ref_peer_photo(
                crate::peers::PeerKind::User,
                document,
                1,
                9,
                2,
                format!("document:{document}"),
                false,
                None,
            );
            entry.location = MediaLocation::Photo {
                id: document,
                access_hash: 1,
                file_reference: vec![1],
                thumb_size: "x".into(),
                dc_id: 2,
            };
            media::insert_media(&mut index, (document, 0), entry);
        }
        index_full_chat_photo(&mut index, id, &full_chat(channel, Some(photo(false)))).unwrap();
        assert!(index.contains_key(&(id, 0)));
        assert_eq!(index.len(), media::ZERO_ID_DOCUMENT_CAP + 1);
        media::bound_media_index(&mut index);
        let (entries, order) = mono_proto_media::persistence::media_and_order_from_index(&index);
        let mut restored = mono_proto_media::persistence::media_to_index(&entries, &order);
        media::bound_media_index(&mut restored);
        assert_eq!(restored[&(id, 0)], index[&(id, 0)]);
        assert_eq!(restored.len(), media::ZERO_ID_DOCUMENT_CAP + 1);
        let mut next_document = restored[&(1, 0)].clone();
        next_document.cache_key = "document:999".into();
        media::insert_media(&mut restored, (999, 0), next_document);
        assert!(restored.contains_key(&(id, 0)));
        assert!(!restored.contains_key(&(1, 0)));
        assert_eq!(restored.len(), media::ZERO_ID_DOCUMENT_CAP + 1);
    }
}

#[test]
fn empty_or_absent_full_photo_preserves_lightweight_fallback() {
    for full in [full_chat(false, None), full_chat(true, None)] {
        let mut index = MediaIndex::default();
        let lightweight = media::media_ref_peer_photo(
            crate::peers::PeerKind::Chat,
            7,
            0,
            9,
            2,
            "avatar:-7".into(),
            false,
            None,
        );
        media::index_avatar(&mut index, -7, lightweight);
        let before = index.clone();
        assert!(index_full_chat_photo(&mut index, -7, &full).is_none());
        assert_eq!(index, before);
    }
}
