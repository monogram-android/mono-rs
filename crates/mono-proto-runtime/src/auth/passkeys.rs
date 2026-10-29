use mono_proto::latest::api::*;

use crate::auth_types::{PasskeyCredentialDto, PasskeyDto};

pub fn credential(
    value: PasskeyCredentialDto,
    registration: bool,
) -> Result<InputPasskeyCredential, &'static str> {
    let invalid = || "invalid passkey credential";
    if value.id.is_empty() || value.raw_id.is_empty() || value.client_data.len() > 65536 {
        return Err(invalid());
    }
    let data: serde_json::Value =
        serde_json::from_str(&value.client_data).map_err(|_| invalid())?;
    let expected = if registration {
        "webauthn.create"
    } else {
        "webauthn.get"
    };
    if data["type"].as_str() != Some(expected)
        || data["origin"].as_str() != Some("https://telegram.org")
        || data["challenge"].as_str().is_none_or(str::is_empty)
    {
        return Err(invalid());
    }
    let client_data = Box::new(DataJson::DataJson(DataJsonConstructor {
        data: value.client_data,
    }));
    let response = if registration {
        if value.attestation_data.is_empty() {
            return Err(invalid());
        }
        InputPasskeyResponse::InputPasskeyResponseRegister(
            InputPasskeyResponseRegisterConstructor {
                client_data,
                attestation_data: value.attestation_data,
            },
        )
    } else {
        login_identity(&value.user_handle)?;
        if value.authenticator_data.len() < 37 || value.signature.is_empty() {
            return Err(invalid());
        }
        InputPasskeyResponse::InputPasskeyResponseLogin(InputPasskeyResponseLoginConstructor {
            client_data,
            authenticator_data: value.authenticator_data,
            signature: value.signature,
            user_handle: value.user_handle,
        })
    };
    Ok(InputPasskeyCredential::InputPasskeyCredentialPublicKey(
        InputPasskeyCredentialPublicKeyConstructor {
            id: value.id,
            raw_id: value.raw_id,
            response: Box::new(response),
        },
    ))
}

pub fn login_identity(handle: &str) -> Result<(i32, i64), &'static str> {
    let invalid = || "invalid passkey user handle";
    let (dc, user) = handle.split_once(':').ok_or_else(invalid)?;
    let dc = dc.parse::<i32>().map_err(|_| invalid())?;
    let user = user.parse::<i64>().map_err(|_| invalid())?;
    if !(1..=5).contains(&dc) || user <= 0 {
        return Err(invalid());
    }
    Ok((dc, user))
}

pub fn to_dto(value: Passkey) -> PasskeyDto {
    let Passkey::Passkey(value) = value;
    PasskeyDto {
        id: value.id,
        name: value.name,
        date: value.date,
        software_emoji_id: value.software_emoji_id,
        last_usage_date: value.last_usage_date,
    }
}

#[cfg(test)]
#[path = "../../tests/unit/passkeys_tests.rs"]
mod tests;
