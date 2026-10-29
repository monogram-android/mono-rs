use crate::{access::with_client_mut, session::call_with_migrate};
use mono_proto::latest::api::*;
pub use mono_proto_messages::scheduled_mapping::{map_response, map_update};
use mono_proto_types::event_types::ScheduledMessageDto;
use mono_proto_types::{
    MtprotoError,
    peers::{input_peer_from_cached, require_usable_peer},
};
use std::collections::HashSet;

pub fn refresh_media(
    state: &mut crate::state::ClientState,
    chat_id: i64,
    id: i32,
) -> Result<(), MtprotoError> {
    let peer = input_peer_from_cached(require_usable_peer(&state.peers, chat_id)?);
    let response: MessagesMessages = mono_proto_runtime::api_invoke::invoke_api(
        &mut state.snapshot,
        state.api_id,
        MessagesGetScheduledMessagesRequest {
            peer: Box::new(peer),
            id: vector(vec![id]),
        },
    )?;
    state
        .media
        .shift_remove(&(chat_id, mono_proto_media::media::scheduled_media_id(id)?));
    map_response(response, &mut state.peers, &mut state.media)?;
    Ok(())
}

pub use mono_proto_runtime::send_options::SCHEDULE_WHEN_ONLINE;

pub fn get_scheduled_history(
    handle: u64,
    chat_id: i64,
) -> Result<Vec<ScheduledMessageDto>, MtprotoError> {
    with_client_mut(handle, |state| {
        call_with_migrate(state, |state| {
            let peer = input_peer_from_cached(require_usable_peer(&state.peers, chat_id)?);
            let response: MessagesMessages = mono_proto_runtime::api_invoke::invoke_api(
                &mut state.snapshot,
                state.api_id,
                MessagesGetScheduledHistoryRequest {
                    peer: Box::new(peer),
                    hash: 0,
                },
            )?;
            map_response(response, &mut state.peers, &mut state.media)
        })
    })
}

pub fn get_scheduled_messages(
    handle: u64,
    chat_id: i64,
    ids: Vec<i32>,
) -> Result<Vec<ScheduledMessageDto>, MtprotoError> {
    validate_ids(&ids)?;
    with_client_mut(handle, |state| {
        call_with_migrate(state, |state| {
            let peer = input_peer_from_cached(require_usable_peer(&state.peers, chat_id)?);
            let response: MessagesMessages = mono_proto_runtime::api_invoke::invoke_api(
                &mut state.snapshot,
                state.api_id,
                MessagesGetScheduledMessagesRequest {
                    peer: Box::new(peer),
                    id: vector(ids.clone()),
                },
            )?;
            map_response(response, &mut state.peers, &mut state.media)
        })
    })
}

pub fn send_scheduled_messages(
    handle: u64,
    chat_id: i64,
    ids: Vec<i32>,
) -> Result<(), MtprotoError> {
    change_queue(handle, chat_id, ids, true)
}

pub fn delete_scheduled_messages(
    handle: u64,
    chat_id: i64,
    ids: Vec<i32>,
) -> Result<(), MtprotoError> {
    change_queue(handle, chat_id, ids, false)
}

fn change_queue(handle: u64, chat_id: i64, ids: Vec<i32>, send: bool) -> Result<(), MtprotoError> {
    validate_ids(&ids)?;
    with_client_mut(handle, |state| {
        call_with_migrate(state, |state| {
            let peer = Box::new(input_peer_from_cached(require_usable_peer(
                &state.peers,
                chat_id,
            )?));
            let response: Updates = if send {
                mono_proto_runtime::api_invoke::invoke_api(
                    &mut state.snapshot,
                    state.api_id,
                    MessagesSendScheduledMessagesRequest {
                        peer,
                        id: vector(ids.clone()),
                    },
                )?
            } else {
                mono_proto_runtime::api_invoke::invoke_api(
                    &mut state.snapshot,
                    state.api_id,
                    MessagesDeleteScheduledMessagesRequest {
                        peer,
                        id: vector(ids.clone()),
                    },
                )?
            };
            crate::management::cache_response_peers(state, &response);
            crate::management::enqueue_response(state.snapshot.dc_id, &response)
        })
    })
}

pub fn edit_scheduled_message(
    handle: u64,
    chat_id: i64,
    message_id: i32,
    schedule_date: i32,
    text: Option<String>,
    entities_json: Option<String>,
    no_webpage: bool,
) -> Result<(), MtprotoError> {
    validate_ids(&[message_id])?;
    validate_date(schedule_date)?;
    if text.is_none() && (entities_json.is_some() || no_webpage) {
        return Err(MtprotoError::Message(
            "scheduled formatting requires replacement text".into(),
        ));
    }
    with_client_mut(handle, |state| {
        call_with_migrate(state, |state| {
            let peer = input_peer_from_cached(require_usable_peer(&state.peers, chat_id)?);
            if text.is_some() {
                let response: MessagesMessages = mono_proto_runtime::api_invoke::invoke_api(
                    &mut state.snapshot,
                    state.api_id,
                    MessagesGetScheduledMessagesRequest {
                        peer: Box::new(peer.clone()),
                        id: vector(vec![message_id]),
                    },
                )?;
                let current = map_response(response, &mut state.peers, &mut state.media)?;
                require_editable(&current, chat_id, message_id)?;
                if current[0].message.date != i64::from(schedule_date) {
                    return Err(MtprotoError::Message(
                        "scheduled date changed; refresh before editing".into(),
                    ));
                }
            }
            let request = edit_request(
                peer,
                message_id,
                schedule_date,
                text.clone(),
                entities_json.as_deref(),
                no_webpage,
                &state.peers,
            )?;
            let response: Updates = mono_proto_runtime::api_invoke::invoke_api(
                &mut state.snapshot,
                state.api_id,
                request,
            )?;
            crate::management::cache_response_peers(state, &response);
            crate::management::enqueue_response(state.snapshot.dc_id, &response)
        })
    })
}

fn require_editable(
    messages: &[ScheduledMessageDto],
    chat_id: i64,
    id: i32,
) -> Result<(), MtprotoError> {
    if matches!(messages, [value] if value.message.chat_id == chat_id && value.message.id == id && !value.has_unsupported_content
        && matches!(value.message.media_kind.as_deref(), None | Some("photo" | "video" | "gif" | "document" | "audio")))
    {
        Ok(())
    } else {
        Err(MtprotoError::Message(
            "scheduled content cannot be safely edited".into(),
        ))
    }
}

fn edit_request(
    peer: InputPeer,
    id: i32,
    date: i32,
    text: Option<String>,
    entities_json: Option<&str>,
    no_webpage: bool,
    peers: &mono_proto_types::collections::HashMap<i64, mono_proto_types::peers::CachedPeer>,
) -> Result<MessagesEditMessageRequest, MtprotoError> {
    let mut flags = MessagesEditMessageRequest::SCHEDULE_DATE_FLAG;
    let entities = if text.is_some() {
        flags |=
            MessagesEditMessageRequest::MESSAGE_FLAG | MessagesEditMessageRequest::ENTITIES_FLAG;
        Some(
            mono_proto_types::message_entities::input_entities_from_json(entities_json, peers)?
                .1
                .unwrap_or_else(|| vector(Vec::new())),
        )
    } else {
        None
    };
    if no_webpage {
        flags |= MessagesEditMessageRequest::NO_WEBPAGE_FLAG;
    }
    Ok(MessagesEditMessageRequest {
        flags,
        no_webpage: no_webpage.then(|| Box::new(True::True(TrueConstructor {}))),
        invert_media: None,
        peer: Box::new(peer),
        id,
        message: text,
        media: None,
        reply_markup: None,
        entities,
        schedule_date: Some(date),
        schedule_repeat_period: None,
        quick_reply_shortcut_id: None,
        rich_message: None,
    })
}

pub(crate) use mono_proto_runtime::send_options::validate_date;

fn validate_ids(ids: &[i32]) -> Result<(), MtprotoError> {
    if ids.is_empty()
        || ids.len() > 100
        || ids.iter().any(|id| *id <= 0)
        || ids.iter().collect::<HashSet<_>>().len() != ids.len()
    {
        return Err(MtprotoError::Message(
            "invalid scheduled message selection".into(),
        ));
    }
    Ok(())
}

fn vector<T>(items: Vec<T>) -> Box<Vector<T>> {
    Box::new(Vector::Vector(VectorConstructor {
        field_0: items.len() as u32,
        field_1: items,
    }))
}

#[cfg(test)]
#[path = "../tests/unit/scheduled_tests.rs"]
mod tests;
