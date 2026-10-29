use crate::state::ClientState;
use mono_proto_types::{MtprotoError, dto::JoinChatDto};

pub use mono_proto_messages::membership::updates_users_chats;

pub fn join_chat(state: &mut ClientState, chat_id: i64) -> Result<JoinChatDto, MtprotoError> {
    mono_proto_messages::membership::join_chat(&mut context(state), chat_id)
}

pub fn unblock_user(state: &mut ClientState, chat_id: i64) -> Result<(), MtprotoError> {
    mono_proto_messages::membership::unblock_user(&mut context(state), chat_id)
}

fn context(state: &mut ClientState) -> mono_proto_messages::membership::MembershipContext<'_> {
    mono_proto_messages::membership::MembershipContext {
        snapshot: &mut state.snapshot,
        api_id: state.api_id,
        peers: &mut state.peers,
        media: &mut state.media,
        user_id: state.user_id,
    }
}
