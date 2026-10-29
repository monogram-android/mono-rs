use mono_proto::latest::api::{InputPasskeyCredential, Passkey};
use mono_proto_types::{
    MtprotoError,
    auth::{PasskeyCredentialDto, PasskeyDto},
};

pub fn credential(
    value: PasskeyCredentialDto,
    registration: bool,
) -> Result<InputPasskeyCredential, MtprotoError> {
    mono_proto_runtime::passkeys::credential(value, registration)
        .map_err(|message| MtprotoError::Message(message.into()))
}

pub fn login_identity(handle: &str) -> Result<(i32, i64), MtprotoError> {
    mono_proto_runtime::passkeys::login_identity(handle)
        .map_err(|message| MtprotoError::Message(message.into()))
}

pub fn to_dto(value: Passkey) -> PasskeyDto {
    mono_proto_runtime::passkeys::to_dto(value)
}
