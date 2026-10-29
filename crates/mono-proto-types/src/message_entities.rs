//! https://core.telegram.org/type/MessageEntity

use mono_proto::latest::api::{
    MessageEntity, MessageEntityBlockquoteConstructor, MessageEntityBoldConstructor,
    MessageEntityCodeConstructor, MessageEntityCustomEmojiConstructor,
    MessageEntityItalicConstructor, MessageEntityPreConstructor, MessageEntitySpoilerConstructor,
    MessageEntityStrikeConstructor, MessageEntityTextUrlConstructor,
    MessageEntityUnderlineConstructor, MessagesSendMessageRequest, True, TrueConstructor, Vector,
    VectorConstructor,
};

use crate::MtprotoError;
use crate::peers::vector_boxed_items;
use mono_proto::latest::api as tl;

#[derive(serde::Deserialize)]
struct WireEntity {
    kind: String,
    offset: i32,
    length: i32,
    url: Option<String>,
}

pub fn input_entities_from_json(
    json: Option<&str>,
    peers: &crate::collections::HashMap<i64, crate::peers::CachedPeer>,
) -> Result<(u32, Option<Box<Vector<Box<MessageEntity>>>>), MtprotoError> {
    let (flag, mut entities) = entities_from_json(json)?;
    if let Some(vector) = entities.as_deref_mut() {
        if let Vector::Vector(items) = vector {
            for entity in &mut items.field_1 {
                if let MessageEntity::MessageEntityMentionName(mention) = entity.as_ref() {
                    let peer = crate::peers::require_usable_peer(peers, mention.user_id)?;
                    if peer.kind != crate::peers::PeerKind::User
                        || peer.id != mention.user_id
                        || peer.id <= 0
                    {
                        return Err(MtprotoError::Message(
                            "named mention requires a full user peer".into(),
                        ));
                    }
                    **entity = MessageEntity::InputMessageEntityMentionName(
                        tl::InputMessageEntityMentionNameConstructor {
                            offset: mention.offset,
                            length: mention.length,
                            user_id: Box::new(tl::InputUser::InputUser(tl::InputUserConstructor {
                                user_id: peer.id,
                                access_hash: peer.access_hash,
                            })),
                        },
                    );
                }
            }
        }
    }
    Ok((flag, entities))
}

pub fn entities_from_json(
    json: Option<&str>,
) -> Result<(u32, Option<Box<Vector<Box<MessageEntity>>>>), MtprotoError> {
    let Some(raw) = json.map(str::trim).filter(|s| !s.is_empty()) else {
        return Ok((0, None));
    };
    let parsed: Vec<WireEntity> = serde_json::from_str(raw)
        .map_err(|e| MtprotoError::Message(format!("entities json: {e}")))?;
    let items: Vec<Box<MessageEntity>> = parsed
        .into_iter()
        .filter_map(wire_entity)
        .map(Box::new)
        .collect();
    if items.is_empty() {
        return Ok((0, None));
    }
    Ok((
        MessagesSendMessageRequest::ENTITIES_FLAG,
        Some(Box::new(Vector::Vector(VectorConstructor {
            field_0: items.len() as u32,
            field_1: items,
        }))),
    ))
}

fn wire_entity(entity: WireEntity) -> Option<MessageEntity> {
    if entity.length <= 0 || entity.offset < 0 {
        return None;
    }
    let offset = entity.offset;
    let length = entity.length;
    Some(match entity.kind.as_str() {
        "bold" | "heading" => {
            MessageEntity::MessageEntityBold(MessageEntityBoldConstructor { offset, length })
        }
        "italic" => {
            MessageEntity::MessageEntityItalic(MessageEntityItalicConstructor { offset, length })
        }
        "underline" => MessageEntity::MessageEntityUnderline(MessageEntityUnderlineConstructor {
            offset,
            length,
        }),
        "strike" => {
            MessageEntity::MessageEntityStrike(MessageEntityStrikeConstructor { offset, length })
        }
        "code" => MessageEntity::MessageEntityCode(MessageEntityCodeConstructor { offset, length }),
        "pre" => MessageEntity::MessageEntityPre(MessageEntityPreConstructor {
            offset,
            length,
            language: entity.url.unwrap_or_default(),
        }),
        "spoiler" => {
            MessageEntity::MessageEntitySpoiler(MessageEntitySpoilerConstructor { offset, length })
        }
        "blockquote" => {
            let collapsed = entity.url.as_deref() == Some("collapsed");
            MessageEntity::MessageEntityBlockquote(MessageEntityBlockquoteConstructor {
                flags: if collapsed {
                    MessageEntityBlockquoteConstructor::COLLAPSED_FLAG
                } else {
                    0
                },
                collapsed: collapsed.then(|| Box::new(True::True(TrueConstructor {}))),
                offset,
                length,
            })
        }
        "url" => {
            MessageEntity::MessageEntityUrl(tl::MessageEntityUrlConstructor { offset, length })
        }
        "mention" => MessageEntity::MessageEntityMention(tl::MessageEntityMentionConstructor {
            offset,
            length,
        }),
        "hashtag" => MessageEntity::MessageEntityHashtag(tl::MessageEntityHashtagConstructor {
            offset,
            length,
        }),
        "bot_command" => {
            MessageEntity::MessageEntityBotCommand(tl::MessageEntityBotCommandConstructor {
                offset,
                length,
            })
        }
        "email" => {
            MessageEntity::MessageEntityEmail(tl::MessageEntityEmailConstructor { offset, length })
        }
        "phone" => {
            MessageEntity::MessageEntityPhone(tl::MessageEntityPhoneConstructor { offset, length })
        }
        "cashtag" => MessageEntity::MessageEntityCashtag(tl::MessageEntityCashtagConstructor {
            offset,
            length,
        }),
        "bank_card" => MessageEntity::MessageEntityBankCard(tl::MessageEntityBankCardConstructor {
            offset,
            length,
        }),
        "text_url" => {
            let url = entity.url.filter(|s| !s.is_empty())?;
            MessageEntity::MessageEntityTextUrl(MessageEntityTextUrlConstructor {
                offset,
                length,
                url,
            })
        }
        "custom_emoji" => {
            let document_id = entity.url.as_deref()?.parse().ok()?;
            MessageEntity::MessageEntityCustomEmoji(MessageEntityCustomEmojiConstructor {
                offset,
                length,
                document_id,
            })
        }
        "mention_name" => {
            let user_id = entity.url.as_deref()?.parse().ok()?;
            MessageEntity::MessageEntityMentionName(
                mono_proto::latest::api::MessageEntityMentionNameConstructor {
                    offset,
                    length,
                    user_id,
                },
            )
        }
        _ => return None,
    })
}

#[cfg(test)]
#[path = "../tests/unit/messages_entities_tests.rs"]
mod tests;

#[derive(serde::Serialize)]
pub struct EntityJson {
    kind: String,
    offset: i32,
    length: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

pub fn entity_json(kind: &str, offset: i32, length: i32, url: Option<String>) -> EntityJson {
    EntityJson {
        kind: kind.to_string(),
        offset,
        length,
        url,
    }
}

pub fn entities_to_json(entities: Option<&Vector<Box<MessageEntity>>>) -> Option<String> {
    let items = entities?;
    let mapped: Vec<EntityJson> = vector_boxed_items(items)
        .filter_map(|entity| match entity {
            MessageEntity::MessageEntityBold(e) => {
                Some(entity_json("bold", e.offset, e.length, None))
            }
            MessageEntity::MessageEntityItalic(e) => {
                Some(entity_json("italic", e.offset, e.length, None))
            }
            MessageEntity::MessageEntityUnderline(e) => {
                Some(entity_json("underline", e.offset, e.length, None))
            }
            MessageEntity::MessageEntityStrike(e) => {
                Some(entity_json("strike", e.offset, e.length, None))
            }
            MessageEntity::MessageEntityCode(e) => {
                Some(entity_json("code", e.offset, e.length, None))
            }
            MessageEntity::MessageEntityPre(e) => Some(entity_json(
                "pre",
                e.offset,
                e.length,
                Some(e.language.clone()).filter(|s| !s.is_empty()),
            )),
            MessageEntity::MessageEntitySpoiler(e) => {
                Some(entity_json("spoiler", e.offset, e.length, None))
            }
            MessageEntity::MessageEntityBlockquote(e) => {
                Some(entity_json("blockquote", e.offset, e.length, None))
            }
            MessageEntity::MessageEntityUrl(e) => {
                Some(entity_json("url", e.offset, e.length, None))
            }
            MessageEntity::MessageEntityTextUrl(e) => Some(entity_json(
                "text_url",
                e.offset,
                e.length,
                Some(e.url.clone()),
            )),
            MessageEntity::MessageEntityMention(e) => {
                Some(entity_json("mention", e.offset, e.length, None))
            }
            MessageEntity::MessageEntityMentionName(e) => {
                Some(entity_json("mention", e.offset, e.length, None))
            }
            MessageEntity::MessageEntityHashtag(e) => {
                Some(entity_json("hashtag", e.offset, e.length, None))
            }
            MessageEntity::MessageEntityBotCommand(e) => {
                Some(entity_json("bot_command", e.offset, e.length, None))
            }
            MessageEntity::MessageEntityEmail(e) => {
                Some(entity_json("email", e.offset, e.length, None))
            }
            MessageEntity::MessageEntityPhone(e) => {
                Some(entity_json("phone", e.offset, e.length, None))
            }
            MessageEntity::MessageEntityCashtag(e) => {
                Some(entity_json("cashtag", e.offset, e.length, None))
            }
            MessageEntity::MessageEntityBankCard(e) => {
                Some(entity_json("bank_card", e.offset, e.length, None))
            }
            MessageEntity::MessageEntityCustomEmoji(e) => Some(entity_json(
                "custom_emoji",
                e.offset,
                e.length,
                Some(e.document_id.to_string()),
            )),
            MessageEntity::MessageEntityFormattedDate(e) => {
                Some(entity_json("date", e.offset, e.length, None))
            }
            _ => None,
        })
        .collect();
    if mapped.is_empty() {
        None
    } else {
        serde_json::to_string(&mapped).ok()
    }
}

pub fn draft_entities_json(entities: Option<&Vector<Box<MessageEntity>>>) -> Option<String> {
    let entities = entities?;
    let raw = entities_to_json(Some(entities))?;
    let mut mapped: Vec<serde_json::Value> = serde_json::from_str(&raw).ok()?;
    if mapped.len() != crate::peers::vector_items(entities).len() {
        return None;
    }
    for (entity, json) in vector_boxed_items(entities).zip(&mut mapped) {
        match entity {
            MessageEntity::MessageEntityMentionName(mention) => {
                json["kind"] = "mention_name".into();
                json["url"] = mention.user_id.to_string().into();
            }
            MessageEntity::MessageEntityBlockquote(quote) if quote.collapsed.is_some() => {
                json["url"] = "collapsed".into();
            }
            _ => {}
        }
    }
    serde_json::to_string(&mapped).ok()
}
