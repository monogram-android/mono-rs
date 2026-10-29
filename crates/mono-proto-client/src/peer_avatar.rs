use super::refresh_dialogs;
use crate::media_support as media;
use crate::{session::call_with_migrate, state::ClientState};

const BASIC_GROUP_MIN: i64 = -1_000_000_000_000;

/// How to load a missing peer avatar. Document ids that share `(id, 0)` are `Skip`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PeerAvatarFetch {
    Skip,
    Profile,
    Dialogs,
}

pub fn peer_avatar_fetch(
    peers: &mono_proto_types::collections::HashMap<i64, mono_proto_types::peers::CachedPeer>,
    user_id: Option<i64>,
    chat_id: i64,
) -> PeerAvatarFetch {
    if chat_id == 0 || user_id == Some(chat_id) {
        return PeerAvatarFetch::Profile;
    }
    if BASIC_GROUP_MIN < chat_id && chat_id < 0 {
        return PeerAvatarFetch::Profile;
    }
    let Some(peer) = peers.get(&chat_id) else {
        return PeerAvatarFetch::Skip;
    };
    if !matches!(
        peer.kind,
        mono_proto_types::peers::PeerKind::User | mono_proto_types::peers::PeerKind::Channel
    ) {
        return PeerAvatarFetch::Skip;
    }
    if mono_proto_types::peers::has_usable_access_hash(peer) {
        PeerAvatarFetch::Profile
    } else {
        PeerAvatarFetch::Dialogs
    }
}

fn load_avatar_profile(state: &mut ClientState, chat_id: i64) {
    let _ = call_with_migrate(state, |state| {
        mono_proto_messages::profile::get_profile(
            &mut state.snapshot,
            state.api_id,
            &state.peers,
            &mut state.media,
            state.user_id,
            chat_id,
        )
    });
}

pub fn reindex_peer_avatar(state: &mut ClientState, chat_id: i64) {
    media::fill_zero_peer_photo_hashes(&mut state.media, &state.peers);
    if state.media.contains_key(&(chat_id, 0)) {
        return;
    }
    match peer_avatar_fetch(&state.peers, state.user_id, chat_id) {
        PeerAvatarFetch::Skip => {}
        PeerAvatarFetch::Profile => load_avatar_profile(state, chat_id),
        PeerAvatarFetch::Dialogs => {
            let _ = call_with_migrate(state, refresh_dialogs);
            media::fill_zero_peer_photo_hashes(&mut state.media, &state.peers);
            if state.media.contains_key(&(chat_id, 0)) {
                return;
            }
            if peer_avatar_fetch(&state.peers, state.user_id, chat_id) == PeerAvatarFetch::Profile {
                load_avatar_profile(state, chat_id);
            }
        }
    }
    media::fill_zero_peer_photo_hashes(&mut state.media, &state.peers);
}

#[cfg(test)]
#[path = "../tests/unit/peer_avatar_tests.rs"]
mod tests;
