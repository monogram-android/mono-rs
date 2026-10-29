use crate::peers::{CachedPeer, input_peer_from_cached, require_usable_peer};
use crate::{HashMap, MtprotoError};
use mono_proto::latest::api::*;

#[cfg(test)]
#[path = "../tests/unit/shared_folders_tests.rs"]
mod tests;

pub fn chatlist(filter_id: i32) -> Result<InputChatlist, MtprotoError> {
    if filter_id <= 1 {
        return Err(MtprotoError::Message(
            "a custom folder id is required".into(),
        ));
    }
    Ok(InputChatlist::InputChatlistDialogFilter(
        InputChatlistDialogFilterConstructor { filter_id },
    ))
}

pub fn validate_slug(slug: &str) -> Result<(), MtprotoError> {
    if slug.trim().is_empty() {
        return Err(MtprotoError::Message("an invite slug is required".into()));
    }
    Ok(())
}

pub fn peer_list(
    peers_by_id: &HashMap<i64, CachedPeer>,
    chat_ids: &[i64],
) -> Result<Box<Vector<Box<InputPeer>>>, MtprotoError> {
    let mut seen = std::collections::HashSet::new();
    let mut peers = Vec::with_capacity(chat_ids.len());
    for id in chat_ids {
        if !seen.insert(*id) {
            return Err(MtprotoError::Message("duplicate chat selection".into()));
        }
        peers.push(Box::new(input_peer_from_cached(require_usable_peer(
            peers_by_id,
            *id,
        )?)));
    }
    Ok(Box::new(Vector::Vector(VectorConstructor {
        field_0: peers
            .len()
            .try_into()
            .map_err(|_| MtprotoError::Message("too many chats".into()))?,
        field_1: peers,
    })))
}

pub fn confirm(response: Bool) -> Result<(), MtprotoError> {
    match response {
        Bool::BoolTrue(_) => Ok(()),
        Bool::BoolFalse(_) => Err(MtprotoError::Message(
            "shared folder change was not applied".into(),
        )),
    }
}
