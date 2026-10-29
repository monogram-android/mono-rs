use crate::{
    access::with_client_mut,
    auth::complete_sign_in,
    persist::persist,
    session::{
        call_with_migrate, drop_updates_transport, ensure_ready_after_migrate, reject_if_dead,
    },
};
use mono_proto::latest::api::*;
use mono_proto_runtime::auth_rpc;
use mono_proto_types::MtprotoError;
use mono_proto_types::auth::{
    AuthSignedIn, PasskeyCredentialDto, PasskeyDto, PasskeyLoginOptionsDto,
};

pub fn passkeys_enabled(handle: u64) -> Result<bool, MtprotoError> {
    with_client_mut(handle, |state| {
        call_with_migrate(state, |state| {
            let config: HelpAppConfig = mono_proto_runtime::api_invoke::invoke_api(
                &mut state.snapshot,
                state.api_id,
                HelpGetAppConfigRequest { hash: 0 },
            )?;
            let HelpAppConfig::HelpAppConfig(config) = config else {
                return Ok(false);
            };
            let JsonValue::JsonObject(object) = *config.config else {
                return Ok(false);
            };
            let Vector::Vector(entries) = *object.value;
            Ok(entries.field_1.iter().any(|entry| {
            let JsonObjectValue::JsonObjectValue(entry) = entry.as_ref();
            entry.key == "settings_display_passkeys" && matches!(entry.value.as_ref(), JsonValue::JsonBool(value) if matches!(value.value.as_ref(), Bool::BoolTrue(_)))
        }))
        })
    })
}

pub fn init_passkey_login(handle: u64) -> Result<PasskeyLoginOptionsDto, MtprotoError> {
    with_client_mut(handle, |state| {
        call_with_migrate(state, |state| {
            let AuthPasskeyLoginOptions::AuthPasskeyLoginOptions(options) =
                mono_proto_runtime::api_invoke::invoke_api(
                    &mut state.snapshot,
                    state.api_id,
                    AuthInitPasskeyLoginRequest {
                        api_id: state.api_id,
                        api_hash: state.api_hash.clone(),
                    },
                )?;
            let DataJson::DataJson(json) = *options.options;
            let snapshot = state.permanent.as_ref().unwrap_or(&state.snapshot);
            let key = snapshot
                .auth_key
                .as_deref()
                .ok_or_else(|| MtprotoError::Message("missing passkey login key".into()))?;
            Ok(PasskeyLoginOptionsDto {
                options: json.data,
                dc_id: snapshot.dc_id,
                auth_key_id: mono_proto_crypto::auth_key_id(key),
            })
        })
    })
}

pub fn finish_passkey_login(
    handle: u64,
    credential: PasskeyCredentialDto,
    dc_id: i32,
    auth_key_id: i64,
) -> Result<AuthSignedIn, MtprotoError> {
    let (destination, _) = crate::passkey_mapping::login_identity(&credential.user_handle)?;
    let credential = crate::passkey_mapping::credential(credential, false)?;
    let result = with_client_mut(handle, |state| {
        reject_if_dead(state)?;
        if state.user_id.is_some() || state.snapshot.dc_id != dc_id || !(1..=5).contains(&dc_id) {
            return Err(MtprotoError::Message("stale passkey login".into()));
        }
        let key = state
            .permanent
            .as_ref()
            .unwrap_or(&state.snapshot)
            .auth_key
            .as_deref()
            .ok_or_else(|| MtprotoError::Message("missing passkey login key".into()))?;
        if mono_proto_crypto::auth_key_id(key) != auth_key_id {
            return Err(MtprotoError::Message("stale passkey login".into()));
        }
        let migrated = destination != dc_id;
        if migrated {
            ensure_ready_after_migrate(
                state,
                MtprotoError::Message(format!("MIGRATE_{destination}")),
            )?;
        }
        let response = mono_proto_runtime::api_invoke::invoke_api(
            &mut state.snapshot,
            state.api_id,
            AuthFinishPasskeyLoginRequest {
                flags: if migrated { 1 } else { 0 },
                credential: Box::new(credential),
                from_dc_id: migrated.then_some(dc_id),
                from_auth_key_id: migrated.then_some(auth_key_id),
            },
        );
        match response {
            Ok(authorization) => {
                let signed =
                    auth_rpc::authorization_to_signed(authorization, state.snapshot.dc_id)?;
                complete_sign_in(state, signed.user_id)?;
                Ok(signed)
            }
            Err(MtprotoError::PasswordRequired) => {
                persist(state)?;
                Err(MtprotoError::PasswordRequired)
            }
            Err(error) => Err(error),
        }
    })?;
    drop_updates_transport(handle);
    Ok(result)
}

pub fn init_passkey_registration(handle: u64) -> Result<String, MtprotoError> {
    with_client_mut(handle, |state| {
        call_with_migrate(state, |state| {
            let AccountPasskeyRegistrationOptions::AccountPasskeyRegistrationOptions(options) =
                mono_proto_runtime::api_invoke::invoke_api(
                    &mut state.snapshot,
                    state.api_id,
                    AccountInitPasskeyRegistrationRequest {},
                )?;
            let DataJson::DataJson(json) = *options.options;
            Ok(json.data)
        })
    })
}

pub fn register_passkey(
    handle: u64,
    credential: PasskeyCredentialDto,
) -> Result<PasskeyDto, MtprotoError> {
    let credential = crate::passkey_mapping::credential(credential, true)?;
    with_client_mut(handle, |state| {
        call_with_migrate(state, |state| {
            mono_proto_runtime::api_invoke::invoke_api(
                &mut state.snapshot,
                state.api_id,
                AccountRegisterPasskeyRequest {
                    credential: Box::new(credential.clone()),
                },
            )
            .map(crate::passkey_mapping::to_dto)
        })
    })
}

pub fn get_passkeys(handle: u64) -> Result<Vec<PasskeyDto>, MtprotoError> {
    with_client_mut(handle, |state| {
        call_with_migrate(state, |state| {
            let AccountPasskeys::AccountPasskeys(keys) =
                mono_proto_runtime::api_invoke::invoke_api(
                    &mut state.snapshot,
                    state.api_id,
                    AccountGetPasskeysRequest {},
                )?;
            let Vector::Vector(keys) = *keys.passkeys;
            Ok(keys
                .field_1
                .into_iter()
                .map(|key| crate::passkey_mapping::to_dto(*key))
                .collect())
        })
    })
}

pub fn delete_passkey(handle: u64, id: String) -> Result<bool, MtprotoError> {
    with_client_mut(handle, |state| {
        call_with_migrate(state, |state| {
            let result: Bool = mono_proto_runtime::api_invoke::invoke_api(
                &mut state.snapshot,
                state.api_id,
                AccountDeletePasskeyRequest { id: id.clone() },
            )?;
            Ok(matches!(result, Bool::BoolTrue(_)))
        })
    })
}
