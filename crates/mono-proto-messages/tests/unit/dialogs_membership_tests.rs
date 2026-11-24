use super::*;
use crate::{HashMap, HashMapExt};
use mono_proto::latest::api::{
    ChannelForbiddenConstructor, ChatConstructor, ChatForbiddenConstructor, ChatPhoto,
    ChatPhotoEmptyConstructor, Dialog, DialogConstructor, DialogFolderConstructor, Folder,
    FolderConstructor, Message, MessageAction, MessageActionEmptyConstructor,
    MessageServiceConstructor, MessagesDialogs, MessagesDialogsConstructor,
    MessagesDialogsSliceConstructor, Peer, PeerChatConstructor, PeerNotifySettings,
    PeerNotifySettingsConstructor, True, TrueConstructor, User, Vector, VectorConstructor,
};

fn flag(on: bool) -> Option<Box<True>> {
    on.then(|| Box::new(True::True(TrueConstructor {})))
}

fn basic_group(left: bool, deactivated: bool) -> TlChat {
    group(11, left, deactivated)
}

fn group(id: i64, left: bool, deactivated: bool) -> TlChat {
    TlChat::Chat(ChatConstructor {
        flags: 0,
        creator: None,
        left: flag(left),
        deactivated: flag(deactivated),
        call_active: None,
        call_not_empty: None,
        noforwards: None,
        id,
        title: "Comment group".into(),
        photo: Box::new(ChatPhoto::ChatPhotoEmpty(ChatPhotoEmptyConstructor {})),
        participants_count: 3,
        date: 20,
        version: 1,
        migrated_to: None,
        admin_rights: None,
        default_banned_rights: None,
    })
}

#[test]
fn joined_group_is_not_left() {
    let (id, meta) = chat_meta_from_tl(&basic_group(false, false)).expect("meta");
    assert_eq!(id, 11);
    assert!(meta.is_group);
    assert!(!meta.left);
}

#[test]
fn left_group_reports_membership_loss() {
    let (_, meta) = chat_meta_from_tl(&basic_group(true, false)).expect("meta");
    assert!(meta.left);
}

#[test]
fn migrated_group_reports_membership_loss() {
    let (_, meta) = chat_meta_from_tl(&basic_group(false, true)).expect("meta");
    assert!(meta.left);
}

#[test]
fn forbidden_peers_are_left() {
    let chat = TlChat::ChatForbidden(ChatForbiddenConstructor {
        id: 12,
        title: "Gone".into(),
    });
    let (_, meta) = chat_meta_from_tl(&chat).expect("meta");
    assert!(meta.left, "a forbidden group cannot be a membership");

    let channel = TlChat::ChannelForbidden(ChannelForbiddenConstructor {
        flags: 0,
        broadcast: None,
        megagroup: flag(true),
        monoforum: None,
        id: 13,
        access_hash: 99,
        title: "Gone too".into(),
        until_date: None,
    });
    let (_, meta) = chat_meta_from_tl(&channel).expect("meta");
    assert!(meta.left, "a forbidden channel cannot be a membership");
}

fn boxed_vec<T>(items: Vec<T>) -> Box<Vector<Box<T>>> {
    Box::new(Vector::Vector(VectorConstructor {
        field_0: items.len() as u32,
        field_1: items.into_iter().map(Box::new).collect(),
    }))
}

fn notify() -> PeerNotifySettings {
    PeerNotifySettings::PeerNotifySettings(PeerNotifySettingsConstructor {
        flags: 0,
        show_previews: None,
        silent: None,
        mute_until: None,
        ios_sound: None,
        android_sound: None,
        other_sound: None,
        stories_muted: None,
        stories_hide_sender: None,
        stories_ios_sound: None,
        stories_android_sound: None,
        stories_other_sound: None,
    })
}

fn chat_dialog(chat_id: i64) -> Dialog {
    Dialog::Dialog(DialogConstructor {
        flags: 0,
        pinned: None,
        unread_mark: None,
        view_forum_as_messages: None,
        peer: Box::new(Peer::PeerChat(PeerChatConstructor { chat_id })),
        top_message: 1,
        read_inbox_max_id: 1,
        read_outbox_max_id: 0,
        unread_count: 6,
        unread_mentions_count: 0,
        unread_reactions_count: 0,
        unread_poll_votes_count: 0,
        notify_settings: Box::new(notify()),
        pts: None,
        draft: None,
        folder_id: None,
        ttl_period: None,
    })
}

fn service_message(chat_id: i64, id: i32, date: i32) -> Message {
    Message::MessageService(MessageServiceConstructor {
        flags: 0,
        out: None,
        mentioned: None,
        media_unread: None,
        reactions_are_possible: None,
        silent: None,
        post: None,
        legacy: None,
        id,
        from_id: None,
        peer_id: Box::new(Peer::PeerChat(PeerChatConstructor { chat_id })),
        saved_peer_id: None,
        reply_to: None,
        date,
        action: Box::new(MessageAction::MessageActionEmpty(
            MessageActionEmptyConstructor {},
        )),
        reactions: None,
        ttl_period: None,
    })
}

#[test]
fn dialogs_page_continues_from_raw_slice_but_not_full_response() {
    let mut dialogs = (1..=39).map(chat_dialog).collect::<Vec<_>>();
    dialogs.push(Dialog::DialogFolder(DialogFolderConstructor {
        flags: 0,
        pinned: None,
        folder: Box::new(Folder::Folder(FolderConstructor {
            flags: 0,
            autofill_new_broadcasts: None,
            autofill_public_groups: None,
            autofill_new_correspondents: None,
            id: 1,
            title: "Archive".into(),
            photo: None,
        })),
        peer: Box::new(Peer::PeerChat(PeerChatConstructor { chat_id: 90 })),
        top_message: 1,
        unread_muted_peers_count: 0,
        unread_unmuted_peers_count: 0,
        unread_muted_messages_count: 0,
        unread_unmuted_messages_count: 0,
    }));
    let mut peers = HashMap::new();
    let mut media = crate::media::MediaIndex::default();
    let mut channel_pts = HashMap::new();
    let first = get_dialogs::map_dialogs_page(
        MessagesDialogs::MessagesDialogsSlice(MessagesDialogsSliceConstructor {
            count: 80,
            dialogs: boxed_vec(dialogs),
            messages: boxed_vec(vec![
                service_message(39, 1, 100),
                service_message(39, 2, 300),
                service_message(90, 1, 400),
            ]),
            chats: boxed_vec((1..=39).map(|id| group(id, false, false)).collect()),
            users: boxed_vec(vec![]),
        }),
        &mut peers,
        &mut media,
        &mut channel_pts,
    )
    .expect("slice");
    assert!(first.is_slice);
    assert_eq!(first.raw_count, 40);
    assert_eq!(first.chats.len(), 39);
    assert_eq!(
        first.next_cursor,
        Some(crate::DialogsCursorDto {
            offset_date: 100,
            offset_id: 1,
            offset_peer_id: -39,
        }),
    );

    let last = get_dialogs::map_dialogs_page(
        MessagesDialogs::MessagesDialogs(MessagesDialogsConstructor {
            dialogs: boxed_vec((40..80).map(chat_dialog).collect()),
            messages: boxed_vec(vec![service_message(79, 1, 50)]),
            chats: boxed_vec((40..80).map(|id| group(id, false, false)).collect()),
            users: boxed_vec(vec![]),
        }),
        &mut peers,
        &mut media,
        &mut channel_pts,
    )
    .expect("full response");
    assert!(!last.is_slice);
    assert_eq!(last.raw_count, 40);
    assert_eq!(last.chats.len(), 40);
    assert_eq!(last.next_cursor, None);
}

#[test]
fn first_dialogs_page_requires_pinned_dialogs_after_get_dialogs_succeeds() {
    let mut peers = HashMap::new();
    let mut media = crate::media::MediaIndex::default();
    let mut channel_pts = HashMap::new();
    let page = get_dialogs::map_dialogs_page(
        MessagesDialogs::MessagesDialogs(MessagesDialogsConstructor {
            dialogs: boxed_vec(vec![chat_dialog(11)]),
            messages: boxed_vec(vec![]),
            chats: boxed_vec(vec![basic_group(false, false)]),
            users: boxed_vec(vec![]),
        }),
        &mut peers,
        &mut media,
        &mut channel_pts,
    )
    .expect("getDialogs success");
    let first = crate::DialogsCursorDto {
        offset_date: 0,
        offset_id: 0,
        offset_peer_id: 0,
    };
    use get_dialogs::PinnedDialogs;
    let fallback =
        get_dialogs::finish_dialogs_page(page.clone(), first, PinnedDialogs::BestEffort, || {
            Err(crate::MtprotoError::Message("pins unavailable".into()))
        })
        .expect("legacy list keeps regular dialogs");
    assert_eq!(fallback.chats.len(), 1);
    assert_eq!(fallback.chats[0].id, -11);
    let error =
        get_dialogs::finish_dialogs_page(page.clone(), first, PinnedDialogs::Required, || {
            Err(crate::MtprotoError::Message(
                "RPC 420: FLOOD_WAIT_17".into(),
            ))
        })
        .expect_err("required pinned dialogs failed");
    assert_eq!(error.to_string(), "RPC 420: FLOOD_WAIT_17");

    let pinned = map_peer_dialogs(
        &mut peers,
        &mut media,
        &mut channel_pts,
        boxed_vec(vec![chat_dialog(12)]),
        boxed_vec(vec![]),
        boxed_vec(vec![group(12, false, false)]),
        boxed_vec(vec![]),
    );
    let complete =
        get_dialogs::finish_dialogs_page(page, first, PinnedDialogs::Required, || Ok(pinned))
            .expect("both RPCs succeeded");
    assert_eq!(complete.raw_count, 1);
    assert_eq!(complete.next_cursor, None);
    let rows: Vec<_> = complete
        .chats
        .iter()
        .map(|chat| (chat.id, chat.pinned))
        .collect();
    assert_eq!(rows, [(-12, true), (-11, false)]);
}

#[test]
fn left_dialogs_are_kept_so_room_can_hide_them() {
    let mut peers = HashMap::new();
    let mut media = crate::media::MediaIndex::default();
    let mut channel_pts = HashMap::new();
    let out = map_peer_dialogs(
        &mut peers,
        &mut media,
        &mut channel_pts,
        boxed_vec(vec![chat_dialog(11), chat_dialog(12)]),
        boxed_vec(Vec::<Message>::new()),
        boxed_vec(vec![group(11, true, false), group(12, false, false)]),
        boxed_vec(Vec::<User>::new()),
    );
    let left = out
        .iter()
        .find(|c| c.id == crate::peers::chat_id_for_chat(11));
    let joined = out
        .iter()
        .find(|c| c.id == crate::peers::chat_id_for_chat(12));
    assert!(left.expect("left dialog").left);
    assert!(!joined.expect("joined dialog").left);
}

#[test]
fn unknown_peer_dialogs_are_dropped() {
    let mut peers = HashMap::new();
    let mut media = crate::media::MediaIndex::default();
    let mut channel_pts = HashMap::new();
    let out = map_peer_dialogs(
        &mut peers,
        &mut media,
        &mut channel_pts,
        boxed_vec(vec![chat_dialog(99)]),
        boxed_vec(Vec::<Message>::new()),
        boxed_vec(Vec::<TlChat>::new()),
        boxed_vec(Vec::<User>::new()),
    );
    assert!(out.is_empty());
}
