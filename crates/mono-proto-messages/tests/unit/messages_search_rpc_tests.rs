use super::*;
use mono_proto::latest::api::{PeerChannelConstructor, PeerChatConstructor, PeerUserConstructor};

#[test]
fn global_filters_accept_supported_names_and_reject_invalid_ones() {
    assert!(matches!(
        global_search_filter("").unwrap(),
        MessagesFilter::InputMessagesFilterEmpty(_)
    ));
    for name in [
        "photo_video",
        "document",
        "url",
        "gif",
        "voice",
        "music",
        "photos",
        "videos",
        "chat_photos",
    ] {
        let filter = global_search_filter(name).unwrap();
        assert_eq!(
            crate::messages_search::messages_filter_key(&filter),
            Some(name)
        );
    }
    for name in ["pinned", "unknown"] {
        assert!(global_search_filter(name).is_err());
    }
}

#[test]
fn global_search_request_preserves_fields_and_encodes_filter() {
    use mono_proto::latest::api::{
        InputMessagesFilterPhotosConstructor, InputPeerEmptyConstructor,
    };
    let peers = HashMap::new();
    let request = global_search_request(&peers, "query", "photos", 42, 0, 77, 500, 1).unwrap();
    assert_eq!(request.flags, MessagesSearchGlobalRequest::FOLDER_ID_FLAG);
    assert_eq!(request.folder_id, Some(1));
    assert_eq!(request.q, "query");
    assert_eq!((request.min_date, request.max_date), (0, 0));
    assert_eq!(
        (request.offset_rate, request.offset_id, request.limit),
        (42, 77, 100)
    );
    assert!(matches!(
        *request.filter,
        MessagesFilter::InputMessagesFilterPhotos(_)
    ));
    let bytes = crate::rpc::encode_boxed_bytes(&request).unwrap();
    for id in [
        MessagesSearchGlobalRequest::ID,
        InputMessagesFilterPhotosConstructor::ID,
        InputPeerEmptyConstructor::ID,
    ] {
        assert!(bytes.windows(4).any(|window| window == id.to_le_bytes()));
    }
    let empty = global_search_request(&peers, "", "", 0, 0, 0, 0, 0).unwrap();
    assert_eq!(empty.limit, 1);
    assert!(matches!(
        *empty.filter,
        MessagesFilter::InputMessagesFilterEmpty(_)
    ));
    assert!(global_search_request(&peers, "", "photos", 0, 123, 0, 10, 0).is_err());
    assert!(global_search_request(&peers, "", "pinned", 0, 0, 0, 10, 0).is_err());
}

#[test]
fn people_are_user_peers() {
    let user = Peer::PeerUser(PeerUserConstructor { user_id: 7 });
    let chat = Peer::PeerChat(PeerChatConstructor { chat_id: 3 });
    let channel = Peer::PeerChannel(PeerChannelConstructor { channel_id: 9 });
    assert!(is_people_peer(&user));
    assert!(!is_people_peer(&chat));
    assert!(!is_people_peer(&channel));
}

#[test]
fn empty_page_clears_offsets() {
    let (rate, peer, id) = next_global_page(&[], 44);
    assert_eq!((rate, peer, id), (0, 0, 0));
}

#[test]
fn next_page_uses_last_message_and_rate() {
    let last = MessageDto {
        media_preview_restricted: None,
        chat_id: -1_000_000_000_042,
        id: 88,
        sender_id: None,
        text: None,
        date: 1,
        edit_date: None,
        outgoing: false,
        media_kind: None,
        media_cache_key: None,
        thumb_cache_key: None,
        media_duration: None,
        media_width: None,
        media_height: None,
        reply_quote: None,
        reply_quote_entities_json: None,
        entities_json: None,
        noforwards: false,
        reply_to_msg_id: None,
        reply_to_top_id: None,
        fwd_from: None,
        fwd_from_id: None,
        fwd_date: None,
        via_bot: None,
        sender_name: None,
        sender_emoji_status_document_id: None,
        grouped_id: None,
        file_name: None,
        file_size: None,
        supports_streaming: false,
        reactions_json: None,
        replies_count: 0,
        discussion_peer_id: None,
        reply_markup_json: None,
        forum_topic: false,
        sender_photo_cache_key: None,

        summary_from_language: None,
    };
    let (rate, peer, id) = next_global_page(&[last], 17);
    assert_eq!(rate, 17);
    assert_eq!(peer, -1_000_000_000_042);
    assert_eq!(id, 88);
}

#[test]
fn first_page_uses_empty_offset_peer() {
    let peers = HashMap::new();
    let peer = offset_input_peer(&peers, 0).expect("empty");
    assert!(matches!(peer, InputPeer::InputPeerEmpty(_)));
}

#[test]
fn empty_contacts_query_is_rejected() {
    let err = contacts_search_query("   ").expect_err("blank");
    assert!(matches!(err, MtprotoError::Message(message) if message == "empty search query"));
    assert_eq!(contacts_search_query(" ada ").expect("trim"), "ada");
}

#[test]
fn search_global_always_sends_folder_id() {
    let (flags, folder) = search_global_folder(0);
    assert_eq!(flags, MessagesSearchGlobalRequest::FOLDER_ID_FLAG);
    assert_eq!(folder, Some(0));
    let (archive_flags, archive) = search_global_folder(1);
    assert_eq!(archive_flags, MessagesSearchGlobalRequest::FOLDER_ID_FLAG);
    assert_eq!(archive, Some(1));
}
