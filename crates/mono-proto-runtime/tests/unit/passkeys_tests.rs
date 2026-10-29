use super::*;

fn fixture() -> PasskeyCredentialDto {
    PasskeyCredentialDto {
        id: "credential".into(),
        raw_id: "credential".into(),
        client_data:
            r#"{"type":"webauthn.get","origin":"https://telegram.org","challenge":"challenge"}"#
                .into(),
        authenticator_data: vec![0; 37],
        signature: vec![1],
        user_handle: "2:123".into(),
        attestation_data: vec![],
    }
}

#[test]
fn login_credential_preserves_signed_client_data() {
    let input = fixture();
    let original = input.client_data.clone();
    let InputPasskeyCredential::InputPasskeyCredentialPublicKey(result) =
        credential(input, false).unwrap()
    else {
        panic!("wrong credential");
    };
    let InputPasskeyResponse::InputPasskeyResponseLogin(response) = *result.response else {
        panic!("wrong response");
    };
    let DataJson::DataJson(data) = *response.client_data;
    assert_eq!(data.data, original);
    assert_eq!(response.user_handle, "2:123");
}

#[test]
fn rejects_wrong_origin_operation_and_incomplete_assertions() {
    for data in [
        r#"{"type":"webauthn.get","origin":"https://example.org","challenge":"x"}"#,
        r#"{"type":"webauthn.create","origin":"https://telegram.org","challenge":"x"}"#,
        r#"{"type":"webauthn.get","origin":"https://telegram.org","challenge":""}"#,
        "invalid json",
    ] {
        let mut input = fixture();
        input.client_data = data.into();
        assert!(credential(input, false).is_err());
    }
    let mut input = fixture();
    input.authenticator_data.truncate(36);
    assert!(credential(input, false).is_err());
    let mut input = fixture();
    input.signature.clear();
    assert!(credential(input, false).is_err());
}

#[test]
fn registration_requires_attestation() {
    let mut input = fixture();
    input.client_data = input.client_data.replace("webauthn.get", "webauthn.create");
    assert!(credential(input.clone(), true).is_err());
    input.attestation_data = vec![1];
    assert!(credential(input, true).is_ok());
}

#[test]
fn login_identity_rejects_invalid_routes() {
    assert_eq!(
        login_identity("2:1234567890123").unwrap(),
        (2, 1234567890123)
    );
    for handle in [
        "",
        "0:1",
        "6:1",
        "2:0",
        "2:-1",
        "2:1:3",
        "2:9223372036854775808",
    ] {
        assert!(login_identity(handle).is_err());
    }
}
