use crate::dialogs::message_to_dto;
use crate::media::MediaIndex;
use crate::peers::vector_boxed_items;
use crate::{HashMap, HashMapExt, MessageDto};
use mono_proto::latest::api::*;
use mono_proto_crypto::fill_random;

pub fn message_from_updates(
    updates: Updates,
    chat_id: i64,
    text: &str,
    media_index: &mut MediaIndex,
) -> MessageDto {
    match updates {
        Updates::Updates(u) => first_new_message(&u.updates, media_index),
        Updates::UpdatesCombined(u) => first_new_message(&u.updates, media_index),
        Updates::UpdateShortSentMessage(s) => Some(MessageDto {
            media_preview_restricted: None,
            chat_id,
            id: s.id,
            sender_id: None,
            text: Some(text.to_string()),
            date: i64::from(s.date),
            edit_date: None,
            outgoing: true,
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
        }),
        Updates::UpdateShortMessage(m) => {
            let (reply_to_msg_id, reply_to_top_id, reply_quote, reply_quote_entities_json, _) =
                crate::dialogs::reply_meta(m.reply_to.as_deref());
            Some(MessageDto {
                media_preview_restricted: None,
                chat_id,
                id: m.id,
                sender_id: Some(m.user_id),
                text: Some(m.message.clone()),
                date: i64::from(m.date),
                edit_date: None,
                outgoing: m.out.is_some(),
                media_kind: None,
                media_cache_key: None,
                thumb_cache_key: None,
                media_duration: None,
                media_width: None,
                media_height: None,
                reply_quote,
                reply_quote_entities_json,
                entities_json: None,
                noforwards: false,
                reply_to_msg_id,
                reply_to_top_id,
                fwd_from: crate::dialogs::fwd_from_label(
                    m.fwd_from.as_deref(),
                    &crate::HashMap::new(),
                ),
                fwd_from_id: crate::dialogs::fwd_origin(m.fwd_from.as_deref()).0,
                fwd_date: crate::dialogs::fwd_origin(m.fwd_from.as_deref()).1,
                via_bot: crate::dialogs::via_bot_label(
                    m.via_bot_id,
                    &crate::HashMap::new(),
                    &crate::HashMap::new(),
                ),
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
            })
        }
        Updates::UpdateShortChatMessage(m) => {
            let (reply_to_msg_id, reply_to_top_id, reply_quote, reply_quote_entities_json, _) =
                crate::dialogs::reply_meta(m.reply_to.as_deref());
            Some(MessageDto {
                media_preview_restricted: None,
                chat_id,
                id: m.id,
                sender_id: Some(m.from_id),
                text: Some(m.message.clone()),
                date: i64::from(m.date),
                edit_date: None,
                outgoing: m.out.is_some(),
                media_kind: None,
                media_cache_key: None,
                thumb_cache_key: None,
                media_duration: None,
                media_width: None,
                media_height: None,
                reply_quote,
                reply_quote_entities_json,
                entities_json: None,
                noforwards: false,
                reply_to_msg_id,
                reply_to_top_id,
                fwd_from: crate::dialogs::fwd_from_label(
                    m.fwd_from.as_deref(),
                    &crate::HashMap::new(),
                ),
                fwd_from_id: crate::dialogs::fwd_origin(m.fwd_from.as_deref()).0,
                fwd_date: crate::dialogs::fwd_origin(m.fwd_from.as_deref()).1,
                via_bot: crate::dialogs::via_bot_label(
                    m.via_bot_id,
                    &crate::HashMap::new(),
                    &crate::HashMap::new(),
                ),
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
            })
        }
        _ => None,
    }
    .unwrap_or(MessageDto {
        media_preview_restricted: None,
        chat_id,
        id: 0,
        sender_id: None,
        text: Some(text.to_string()),
        date: 0,
        edit_date: None,
        outgoing: true,
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
    })
}

pub fn first_new_message(
    updates: &mono_proto::latest::api::Vector<Box<mono_proto::latest::api::Update>>,
    media_index: &mut MediaIndex,
) -> Option<MessageDto> {
    use mono_proto::latest::api::Update;
    for update in vector_boxed_items(updates) {
        match update {
            Update::UpdateNewMessage(u) => {
                if let Some(dto) = message_to_dto(&u.message, media_index) {
                    return Some(dto);
                }
            }
            Update::UpdateNewChannelMessage(u) => {
                if let Some(dto) = message_to_dto(&u.message, media_index) {
                    return Some(dto);
                }
            }
            Update::UpdateEditMessage(u) => {
                if let Some(dto) = message_to_dto(&u.message, media_index) {
                    return Some(dto);
                }
            }
            Update::UpdateEditChannelMessage(u) => {
                if let Some(dto) = message_to_dto(&u.message, media_index) {
                    return Some(dto);
                }
            }
            _ => {}
        }
    }
    None
}

pub fn all_new_messages(
    updates: &mono_proto::latest::api::Vector<Box<mono_proto::latest::api::Update>>,
    media_index: &mut MediaIndex,
) -> Vec<MessageDto> {
    use mono_proto::latest::api::Update;
    let mut out = Vec::new();
    for update in vector_boxed_items(updates) {
        match update {
            Update::UpdateNewMessage(u) => {
                if let Some(dto) = message_to_dto(&u.message, media_index) {
                    out.push(dto);
                }
            }
            Update::UpdateNewChannelMessage(u) => {
                if let Some(dto) = message_to_dto(&u.message, media_index) {
                    out.push(dto);
                }
            }
            _ => {}
        }
    }
    out
}

pub fn input_reply_to(reply_to_msg_id: i32) -> (u32, Option<Box<InputReplyTo>>) {
    input_reply_to_thread(reply_to_msg_id, 0)
}

pub fn input_reply_to_thread(
    reply_to_msg_id: i32,
    top_msg_id: i32,
) -> (u32, Option<Box<InputReplyTo>>) {
    if reply_to_msg_id <= 0 && top_msg_id <= 0 {
        return (0, None);
    }
    let reply_id = if reply_to_msg_id > 0 {
        reply_to_msg_id
    } else {
        top_msg_id
    };
    let (top_flag, top) = if top_msg_id > 0 {
        (
            InputReplyToMessageConstructor::TOP_MSG_ID_FLAG,
            Some(top_msg_id),
        )
    } else {
        (0, None)
    };
    (
        MessagesSendMessageRequest::REPLY_TO_FLAG,
        Some(Box::new(InputReplyTo::InputReplyToMessage(
            InputReplyToMessageConstructor {
                flags: top_flag,
                reply_to_msg_id: reply_id,
                top_msg_id: top,
                reply_to_peer_id: None,
                quote_text: None,
                quote_entities: None,
                quote_offset: None,
                monoforum_peer_id: None,
                todo_item_id: None,
                poll_option: None,
                quote_rich: None,
            },
        ))),
    )
}

pub fn random_id() -> i64 {
    let mut bytes = [0_u8; 8];
    let _ = fill_random(&mut bytes);
    let value = i64::from_le_bytes(bytes);
    if value == 0 { 1 } else { value }
}

/// `0` means the caller has no id yet. A non-zero id is the pending UI id.
pub fn outgoing_random_id(requested: i64) -> i64 {
    if requested == 0 {
        random_id()
    } else {
        requested
    }
}

#[cfg(test)]
#[path = "../../tests/unit/messages_send_random_id_tests.rs"]
mod random_id_tests;

pub fn send_message_request(
    flags: u32,
    peer: Box<InputPeer>,
    reply_to: Option<Box<InputReplyTo>>,
    message: String,
    client_random_id: i64,
    entities: Option<Box<Vector<Box<MessageEntity>>>>,
) -> MessagesSendMessageRequest {
    MessagesSendMessageRequest {
        flags,
        no_webpage: None,
        silent: None,
        background: None,
        clear_draft: None,
        noforwards: None,
        update_stickersets_order: None,
        invert_media: None,
        allow_paid_floodskip: None,
        peer,
        reply_to,
        message,
        random_id: outgoing_random_id(client_random_id),
        reply_markup: None,
        entities,
        schedule_date: None,
        schedule_repeat_period: None,
        send_as: None,
        quick_reply_shortcut: None,
        effect: None,
        allow_paid_stars: None,
        suggested_post: None,
        rich_message: None,
    }
}
