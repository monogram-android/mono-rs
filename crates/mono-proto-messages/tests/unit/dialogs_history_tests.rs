use super::{get_history, refresh_message_media};
use crate::{HashMap, HashMapExt};
use mono_proto_session::{OsRandom, Snapshot};

fn page_of_empty_messages(ids: &[i32]) -> super::HistoryPageDto {
    use mono_proto::latest::api::*;
    fn vector<T>(items: Vec<T>) -> Box<Vector<T>> {
        Box::new(Vector::Vector(VectorConstructor {
            field_0: items.len() as u32,
            field_1: items,
        }))
    }
    let response = MessagesMessages::MessagesMessages(MessagesMessagesConstructor {
        messages: vector(
            ids.iter()
                .map(|&id| {
                    Box::new(Message::MessageEmpty(MessageEmptyConstructor {
                        flags: 0,
                        id,
                        peer_id: None,
                    }))
                })
                .collect(),
        ),
        topics: vector(vec![]),
        chats: vector(vec![]),
        users: vector(vec![]),
    });
    super::history::history_page_from_response(
        response,
        &mut HashMap::new(),
        &mut Default::default(),
    )
    .unwrap()
}

#[test]
fn filtered_history_page_keeps_raw_count_and_oldest_id() {
    let page = page_of_empty_messages(&[11, 7, 10]);
    assert!(page.messages.is_empty());
    assert_eq!(page.raw_count, 3);
    assert_eq!(page.next_offset_id, Some(7));
}

#[test]
fn empty_history_page_has_no_cursor() {
    let page = page_of_empty_messages(&[]);
    assert!(page.messages.is_empty());
    assert_eq!(page.raw_count, 0);
    assert_eq!(page.next_offset_id, None);
}

#[test]
fn unknown_peer_fails_before_rpc() {
    let mut snapshot = Snapshot::new(2, &mut OsRandom).expect("snapshot");
    let mut peers = HashMap::new();
    let mut media = crate::media::MediaIndex::default();
    let err = get_history(
        &mut snapshot,
        1,
        &mut peers,
        &mut media,
        42,
        10,
        0,
        0,
        0,
        None,
    )
    .expect_err("unknown peer");
    match err {
        crate::MtprotoError::Message(message) => {
            assert!(message.contains("unknown peer 42"));
            assert!(message.contains("refresh chats first"));
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn get_dialogs_first_page_uses_empty_offset_peer() {
    use mono_proto::latest::api::InputPeerEmptyConstructor;
    let request = super::MessagesGetDialogsRequest {
        flags: 0,
        exclude_pinned: None,
        folder_id: None,
        offset_date: 0,
        offset_id: 0,
        offset_peer: Box::new(super::InputPeer::InputPeerEmpty(
            InputPeerEmptyConstructor {},
        )),
        limit: 40,
        hash: 0,
    };
    let bytes = crate::rpc::encode_boxed_bytes(&request).expect("encode");
    assert!(
        bytes
            .windows(4)
            .any(|w| w == &super::MessagesGetDialogsRequest::ID.to_le_bytes()),
        "messages.getDialogs ctor",
    );
    assert!(
        bytes
            .windows(4)
            .any(|w| w == &InputPeerEmptyConstructor::ID.to_le_bytes()),
        "first page offset_peer is inputPeerEmpty",
    );
}

#[test]
fn get_messages_request_encodes_input_message_id() {
    let req = super::MessagesGetMessagesRequest {
        id: super::input_message_ids(7),
    };
    let bytes = crate::rpc::encode_boxed_bytes(&req).expect("encode");
    assert!(
        bytes
            .windows(4)
            .any(|w| w == &super::MessagesGetMessagesRequest::ID.to_le_bytes()),
        "messages.getMessages ctor",
    );
    assert!(
        bytes
            .windows(4)
            .any(|w| w == &super::InputMessageIdConstructor::ID.to_le_bytes()),
        "inputMessageID ctor",
    );
    assert!(bytes.windows(4).any(|w| w == &7i32.to_le_bytes()));
}

#[test]
fn refresh_message_media_unknown_peer_fails_before_rpc() {
    let mut snapshot = Snapshot::new(2, &mut OsRandom).expect("snapshot");
    let peers = HashMap::new();
    let mut media = crate::media::MediaIndex::default();
    let err = refresh_message_media(&mut snapshot, 1, &peers, &mut media, 42, 7)
        .expect_err("unknown peer");
    match err {
        crate::MtprotoError::Message(message) => {
            assert!(message.contains("unknown peer 42"));
        }
        other => panic!("unexpected {other:?}"),
    }
}

use super::merge_channel_pts;

#[test]
fn dialog_pts_only_seeds_unknown_channels_with_positive_values() {
    let mut cursors = HashMap::from_iter([(42_i64, 100_i32)]);

    merge_channel_pts(&mut cursors, 42, Some(105));
    merge_channel_pts(&mut cursors, 42, Some(95));
    merge_channel_pts(&mut cursors, 43, None);
    merge_channel_pts(&mut cursors, 43, Some(0));
    merge_channel_pts(&mut cursors, 43, Some(-1));
    assert_eq!(cursors, HashMap::from_iter([(42, 100)]));

    merge_channel_pts(&mut cursors, 43, Some(105));
    merge_channel_pts(&mut cursors, 43, Some(110));
    assert_eq!(cursors, HashMap::from_iter([(42, 100), (43, 105)]));
}
