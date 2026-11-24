use super::*;

#[test]
fn chat_photo_display_prefers_bounded_x_even_when_it_is_full() {
    use crate::location::{MediaDownloadKind, media_for_download};
    use mono_proto::latest::api::{
        MessageMediaPhotoConstructor, PhotoConstructor, PhotoSizeConstructor, Vector,
        VectorConstructor,
    };

    for (renditions, expected_full, expected_display) in [
        (
            vec![("s", 90), ("m", 320), ("x", 800), ("y", 1280)],
            "y",
            Some("x"),
        ),
        (vec![("s", 90), ("m", 320), ("x", 800)], "x", Some("x")),
        (vec![("s", 90), ("m", 320), ("x", 801)], "x", Some("s")),
        (vec![("w", 2560)], "w", None),
    ] {
        let media = MessageMedia::MessageMediaPhoto(MessageMediaPhotoConstructor {
            flags: 0,
            spoiler: None,
            live_photo: None,
            photo: Some(Box::new(Photo::Photo(PhotoConstructor {
                flags: 0,
                has_stickers: None,
                id: 7,
                access_hash: 1,
                file_reference: vec![1],
                date: 0,
                sizes: Box::new(Vector::Vector(VectorConstructor {
                    field_0: renditions.len() as u32,
                    field_1: renditions
                        .into_iter()
                        .map(|(ty, w)| {
                            Box::new(PhotoSize::PhotoSize(PhotoSizeConstructor {
                                type_: ty.into(),
                                w,
                                h: w / 2,
                                size: 1000,
                            }))
                        })
                        .collect(),
                })),
                video_sizes: None,
                dc_id: 2,
            }))),
            ttl_seconds: None,
            video: None,
        });
        let (indexed, _) = extract_from_media(&media).unwrap();
        assert_eq!(indexed.cache_key, "photo:7");
        assert!(
            matches!(&indexed.location, MediaLocation::Photo { thumb_size, .. } if thumb_size == expected_full)
        );
        let display = media_for_download(&indexed, MediaDownloadKind::Display);
        if let Some(expected) = expected_display {
            let display = display.unwrap();
            assert_eq!(display.cache_key, "photo:7:display");
            assert!(
                matches!(&display.location, MediaLocation::Photo { thumb_size, .. } if thumb_size == expected)
            );
        } else {
            assert!(display.is_err());
        }
    }
}

#[test]
fn resolved_static_avatar_classification_requires_exact_peer_cache_identity() {
    let mut entry = media_ref_peer_photo(
        crate::peers::PeerKind::User,
        7,
        1,
        9,
        2,
        "avatar:7".into(),
        false,
        None,
    );
    entry.location = MediaLocation::Photo {
        id: 9,
        access_hash: 1,
        file_reference: vec![1],
        thumb_size: "x".into(),
        dc_id: 2,
    };
    assert!(matches!(
        persist_class(&(7, 0), &entry),
        PersistClass::Avatar
    ));
    assert!(matches!(
        persist_class(&(8, 0), &entry),
        PersistClass::ZeroDocument
    ));
    assert!(matches!(
        persist_class(&(7, 1), &entry),
        PersistClass::Message
    ));
    entry.cache_key = "avatar:7:unrelated".into();
    assert!(matches!(
        persist_class(&(7, 0), &entry),
        PersistClass::ZeroDocument
    ));
    entry.cache_key = "document:7".into();
    assert!(matches!(
        persist_class(&(7, 0), &entry),
        PersistClass::ZeroDocument
    ));
}

#[test]
fn bounding_mixed_classes_preserves_values_and_global_fifo_order() {
    let avatar = media_ref_peer_photo(
        crate::peers::PeerKind::User,
        7,
        1,
        9,
        2,
        "avatar:7".into(),
        false,
        None,
    );
    let mut document = avatar.clone();
    document.cache_key = "document:7".into();
    document.location = MediaLocation::Inline {
        bytes: vec![1, 2, 3],
        extension: "jpg".into(),
    };
    let mut catalog = document.clone();
    catalog.location = MediaLocation::Document {
        id: 7,
        access_hash: 1,
        file_reference: vec![1],
        thumb_size: String::new(),
        dc_id: 2,
        mime_type: "image/webp".into(),
    };
    catalog.sticker_set_id = Some(1);
    catalog.sticker_set_access_hash = Some(2);
    let mut gif = catalog.clone();
    gif.sticker_set_id = None;
    gif.sticker_set_access_hash = None;
    gif.refresh_source = Some(crate::media_location::MediaRefreshSource::SavedGifs);
    let mut mixed = MediaIndex::default();
    for n in 1..=1100 {
        mixed.insert((n, 0), document.clone());
        mixed.insert((n, 1), document.clone());
        mixed.insert((n, INSTANT_VIEW_MEDIA_MSG), document.clone());
        mixed.insert((n + 10000, 0), avatar.clone());
        insert_media(&mut mixed, (n + 20000, 0), catalog.clone());
        insert_media(&mut mixed, (n + 30000, 0), gif.clone());
    }
    // Replacing an existing entry must not promote its FIFO position.
    mixed.insert((1, 1), document);
    let mut reference = mixed.clone();
    for class in [
        PersistClass::Message,
        PersistClass::ZeroDocument,
        PersistClass::Instant,
    ] {
        let cap = class_cap(class).unwrap();
        let keys: Vec<_> = reference
            .iter()
            .filter(|(key, value)| persist_class(key, value) == class)
            .map(|(key, _)| *key)
            .collect();
        let overflow = keys.len().saturating_sub(cap);
        for key in keys.into_iter().take(overflow) {
            reference.shift_remove(&key);
        }
    }
    bound_media_index(&mut mixed);
    assert_eq!(
        mixed.len(),
        3300 + MESSAGE_MEDIA_CAP + ZERO_ID_DOCUMENT_CAP + INSTANT_VIEW_CAP
    );
    assert_eq!(
        mixed.iter().collect::<Vec<_>>(),
        reference.iter().collect::<Vec<_>>()
    );
    bound_media_index(&mut mixed);
    assert_eq!(
        mixed.iter().collect::<Vec<_>>(),
        reference.iter().collect::<Vec<_>>()
    );
}

#[test]
fn classify_document_kinds() {
    assert_eq!(
        classify_document("application/x-tgsticker", true, true, false),
        "sticker_animated"
    );
    assert_eq!(
        classify_document("image/webp", true, false, false),
        "sticker"
    );
    assert_eq!(classify_document("video/mp4", false, true, false), "gif");
    assert_eq!(classify_document("video/mp4", false, false, true), "video");
}

#[test]
fn round_video_attribute_is_video_note_not_voice() {
    use mono_proto::latest::api::{
        DocumentAttribute, DocumentAttributeVideoConstructor, DocumentConstructor, True,
        TrueConstructor, Vector, VectorConstructor,
    };
    let mut doc = DocumentConstructor {
        flags: 0,
        id: 1,
        access_hash: 1,
        file_reference: vec![1],
        date: 0,
        mime_type: "video/mp4".into(),
        size: 12,
        thumbs: None,
        video_thumbs: None,
        dc_id: 2,
        attributes: Box::new(Vector::Vector(VectorConstructor {
            field_0: 1,
            field_1: vec![Box::new(DocumentAttribute::DocumentAttributeVideo(
                DocumentAttributeVideoConstructor {
                    flags: DocumentAttributeVideoConstructor::ROUND_MESSAGE_FLAG,
                    round_message: Some(Box::new(True::True(TrueConstructor {}))),
                    supports_streaming: None,
                    nosound: None,
                    duration: 3.0,
                    w: 240,
                    h: 240,
                    preload_prefix_size: None,
                    video_start_ts: None,
                    video_codec: None,
                },
            ))],
        })),
    };
    assert_eq!(document_kind(&doc), "video_note");
    doc.attributes = Box::new(Vector::Vector(VectorConstructor {
        field_0: 2,
        field_1: vec![
            Box::new(DocumentAttribute::DocumentAttributeAudio(
                mono_proto::latest::api::DocumentAttributeAudioConstructor {
                    flags: 0,
                    voice: None,
                    duration: 2,
                    title: None,
                    performer: None,
                    waveform: None,
                },
            )),
            Box::new(DocumentAttribute::DocumentAttributeVideo(
                DocumentAttributeVideoConstructor {
                    flags: DocumentAttributeVideoConstructor::ROUND_MESSAGE_FLAG,
                    round_message: Some(Box::new(True::True(TrueConstructor {}))),
                    supports_streaming: None,
                    nosound: None,
                    duration: 3.0,
                    w: 240,
                    h: 240,
                    preload_prefix_size: None,
                    video_start_ts: None,
                    video_codec: None,
                },
            )),
        ],
    }));
    assert_eq!(
        document_kind(&doc),
        "video_note",
        "round video wins over a sidecar audio attribute",
    );
    assert_eq!(classify_document("image/gif", false, false, false), "gif");
    assert_eq!(
        classify_document("image/jpeg", false, false, false),
        "document"
    );
    assert_eq!(
        classify_document("image/png", false, false, false),
        "document"
    );
    assert_eq!(
        classify_document("image/webp", false, false, false),
        "document"
    );
    assert_eq!(
        classify_document("application/pdf", false, false, false),
        "document"
    );
}
