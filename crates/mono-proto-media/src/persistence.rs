use crate::collections::{HashMap, HashMapExt, HashSet, HashSetExt};
use crate::media::{MediaIndex, MediaRef};

pub fn media_key(chat_id: i64, message_id: i32) -> String {
    format!("{chat_id}:{message_id}")
}

pub fn media_from_index(index: &MediaIndex) -> HashMap<String, MediaRef> {
    let mut out = HashMap::with_capacity(index.len());
    for ((chat, message), media) in index {
        if let Some(stored) = crate::media::persisted_media_entry(*message, media) {
            out.insert(media_key(*chat, *message), stored);
        }
    }
    out
}

pub fn media_order_from_index(index: &MediaIndex) -> Vec<String> {
    index
        .iter()
        .filter(|((_, message), media)| crate::media::session_media_kept(*message, media))
        .map(|((chat, message), _)| media_key(*chat, *message))
        .collect()
}

pub fn media_to_index(map: &HashMap<String, MediaRef>, order: &[String]) -> MediaIndex {
    let mut index = MediaIndex::default();
    let listed: HashSet<&str> = order.iter().map(String::as_str).collect();
    for (key, media) in map {
        if !listed.contains(key.as_str()) {
            insert_parsed(&mut index, key, media);
        }
    }
    for key in order {
        if let Some(media) = map.get(key) {
            insert_parsed(&mut index, key, media);
        }
    }
    crate::media::bound_media_index(&mut index);
    index
}

fn insert_parsed(index: &mut MediaIndex, key: &str, media: &MediaRef) {
    let mut parts = key.splitn(2, ':');
    let chat = parts.next().and_then(|s| s.parse().ok());
    let message = parts.next().and_then(|s| s.parse().ok());
    if let (Some(chat), Some(message)) = (chat, message) {
        index.insert((chat, message), media.clone());
    }
}
