#[cfg(test)]
#[path = "../tests/unit/drafts_tests.rs"]
mod tests;

use crate::{access::with_client_mut, session::call_with_migrate};
use mono_proto::latest::api::*;
use mono_proto_types::MtprotoError;
use mono_proto_types::peers::{input_peer_from_cached, require_usable_peer};

pub use mono_proto_types::event_types::CloudDraft;

pub fn get_cloud_drafts(handle: u64) -> Result<Vec<CloudDraft>, MtprotoError> {
    with_client_mut(handle, |state| {
        call_with_migrate(state, |state| {
            let response: Updates = mono_proto_runtime::api_invoke::invoke_api(
                &mut state.snapshot,
                state.api_id,
                MessagesGetAllDraftsRequest {},
            )?;
            crate::management::cache_response_peers(state, &response);
            Ok(drafts_from_updates(&response))
        })
    })
}

pub fn save_cloud_draft(handle: u64, draft: CloudDraft) -> Result<(), MtprotoError> {
    if draft.has_unsupported_content {
        return Err(MtprotoError::Message(
            "this draft contains content that cannot be changed by the text composer".into(),
        ));
    }
    if draft.thread_id < 0 || draft.reply_to_message_id < 0 {
        return Err(MtprotoError::Message("invalid draft reply context".into()));
    }
    with_client_mut(handle, |state| {
        call_with_migrate(state, |state| {
            let peer = input_peer_from_cached(require_usable_peer(&state.peers, draft.chat_id)?);
            let request = save_request(peer, &draft, &state.peers)?;
            let response: Bool = mono_proto_runtime::api_invoke::invoke_api(
                &mut state.snapshot,
                state.api_id,
                request,
            )?;
            match response {
                Bool::BoolTrue(_) => Ok(()),
                Bool::BoolFalse(_) => Err(MtprotoError::Message("draft was not saved".into())),
            }
        })
    })
}

pub use mono_proto_messages::drafts::{draft_from_update, drafts_from_updates, save_request};
