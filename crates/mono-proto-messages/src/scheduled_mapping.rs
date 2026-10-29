use crate::MtprotoError;
use crate::event_types::ScheduledMessageDto;
use mono_proto::latest::api::*;

fn vector<T>(items: Vec<T>) -> Box<Vector<T>> {
    Box::new(Vector::Vector(VectorConstructor {
        field_0: items.len() as u32,
        field_1: items,
    }))
}

pub fn map_update(
    message: &Message,
    media: &mut crate::media::MediaIndex,
) -> Result<ScheduledMessageDto, MtprotoError> {
    if let Message::Message(value) = message {
        media.shift_remove(&(
            crate::peers::peer_chat_id(&value.peer_id),
            crate::media::scheduled_media_id(value.id)?,
        ));
    }
    let response = MessagesMessages::MessagesMessages(MessagesMessagesConstructor {
        messages: vector(vec![Box::new(message.clone())]),
        topics: vector(Vec::new()),
        users: vector(Vec::new()),
        chats: vector(Vec::new()),
    });
    map_response(response, &mut crate::HashMap::default(), media)?
        .into_iter()
        .next()
        .ok_or_else(|| MtprotoError::Message("scheduled update has no message".into()))
}

pub fn map_response(
    mut response: MessagesMessages,
    peers: &mut crate::HashMap<i64, crate::peers::CachedPeer>,
    media: &mut crate::media::MediaIndex,
) -> Result<Vec<ScheduledMessageDto>, MtprotoError> {
    let messages = match &mut response {
        MessagesMessages::MessagesMessages(value) => &mut value.messages,
        MessagesMessages::MessagesMessagesSlice(value) => &mut value.messages,
        MessagesMessages::MessagesChannelMessages(value) => &mut value.messages,
        _ => {
            return Err(MtprotoError::Message(
                "unexpected scheduled history response".into(),
            ));
        }
    };
    let mut entities = crate::peers::vector_boxed_items(messages)
        .filter_map(|message| match message {
            Message::Message(value) => {
                let json = crate::message_entities::draft_entities_json(value.entities.as_deref());
                let unsupported = (value
                    .entities
                    .as_deref()
                    .is_some_and(|items| !crate::peers::vector_items(items).is_empty())
                    && json.is_none())
                    || value.rich_message.is_some()
                    || value.invert_media.is_some()
                    || value.media.as_deref().is_some_and(|media| {
                        !matches!(
                            media,
                            MessageMedia::MessageMediaEmpty(_)
                                | MessageMedia::MessageMediaPhoto(_)
                                | MessageMedia::MessageMediaDocument(_)
                        )
                    });
                Some((value.id, (json, unsupported)))
            }
            _ => None,
        })
        .collect::<std::collections::HashMap<_, _>>();
    if let Vector::Vector(items) = messages.as_mut() {
        for message in &mut items.field_1 {
            let id = match message.as_mut() {
                Message::Message(value) => &mut value.id,
                Message::MessageService(value) => &mut value.id,
                Message::MessageEmpty(value) => &mut value.id,
                _ => return Err(MtprotoError::Message("unexpected scheduled message".into())),
            };
            *id = crate::media::scheduled_media_id(*id)?;
        }
    }
    Ok(crate::dialogs::dtos_from_messages(response, peers, media)?
        .into_iter()
        .map(|mut message| {
            message.id = crate::media::scheduled_message_id(message.id).expect("mapped queue ID");
            let has_unsupported_content =
                if let Some((value, unsupported)) = entities.remove(&message.id) {
                    message.entities_json = value;
                    unsupported
                } else {
                    true
                };
            ScheduledMessageDto {
                message,
                has_unsupported_content,
            }
        })
        .collect())
}
