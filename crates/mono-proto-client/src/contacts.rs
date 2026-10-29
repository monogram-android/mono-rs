use crate::{access::with_client_mut, session::call_with_migrate};
use mono_proto::latest::api::ContactsImportedContacts;
use mono_proto_messages::contacts::{
    ContactImportResult, ContactImportSpec, import_request, import_result,
};
use mono_proto_types::MtprotoError;

pub fn import_contacts(
    handle: u64,
    contacts: Vec<ContactImportSpec>,
) -> Result<ContactImportResult, MtprotoError> {
    let request = import_request(&contacts)?;
    with_client_mut(handle, |state| {
        call_with_migrate(state, |state| {
            let response: ContactsImportedContacts = mono_proto_runtime::api_invoke::invoke_api(
                &mut state.snapshot,
                state.api_id,
                request.clone(),
            )?;
            let ContactsImportedContacts::ContactsImportedContacts(body) = response;
            let result = import_result(&contacts, &body)?;
            mono_proto_messages::dialogs::cache_from_users_chats(
                &mut state.peers,
                &mut state.media,
                mono_proto_types::peers::vector_boxed_items(&body.users).cloned(),
                std::iter::empty(),
            );
            Ok(result)
        })
    })
}
