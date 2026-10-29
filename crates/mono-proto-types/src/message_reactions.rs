use crate::peers::vector_boxed_items;
use mono_proto::latest::api::{MessageReactions, MessageReplies, Reaction, ReactionCount};
use serde_json::json;

pub fn reactions_to_json(reactions: Option<&MessageReactions>) -> Option<String> {
    let Some(MessageReactions::MessageReactions(body)) = reactions else {
        return None;
    };
    let mut rows = Vec::new();
    for count in vector_boxed_items(&body.results) {
        let ReactionCount::ReactionCount(rc) = count else {
            continue;
        };
        let chosen = rc.chosen_order.is_some();
        match rc.reaction.as_ref() {
            Reaction::ReactionEmoji(e) => rows.push(json!({
                "e": e.emoticon,
                "c": rc.count,
                "me": chosen,
            })),
            Reaction::ReactionCustomEmoji(e) => rows.push(json!({
                "d": e.document_id,
                "c": rc.count,
                "me": chosen,
            })),
            _ => {}
        }
    }
    if rows.is_empty() {
        None
    } else {
        serde_json::to_string(&rows).ok()
    }
}

pub fn replies_meta(replies: Option<&MessageReplies>) -> (i32, Option<i64>) {
    match replies {
        Some(MessageReplies::MessageReplies(r)) if r.comments.is_some() => {
            let peer = r.channel_id.map(crate::peers::chat_id_for_channel);
            (r.replies, peer)
        }
        Some(MessageReplies::MessageReplies(r)) => (r.replies, None),
        _ => (0, None),
    }
}
