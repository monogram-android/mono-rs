//! Cryptographic primitive and envelope regressions.

use super::*;

#[test]
fn ige_round_trip_and_chains() {
    let key = [7_u8; 32];
    let iv = [9_u8; 32];
    let plaintext = [3_u8; 64];
    let ciphertext = aes_ige_encrypt(&plaintext, &key, &iv).unwrap();
    assert_ne!(ciphertext, plaintext);
    assert_eq!(aes_ige_decrypt(&ciphertext, &key, &iv).unwrap(), plaintext);
}

#[test]
fn ctr_round_trip() {
    let mut data = b"obfuscated transport header".to_vec();
    let original = data.clone();
    aes_ctr_apply(&mut data, &[1; 32], &[2; 16]).unwrap();
    aes_ctr_apply(&mut data, &[1; 32], &[2; 16]).unwrap();
    assert_eq!(data, original);
}

#[test]
fn known_hashes() {
    assert_eq!(
        hex(&sha1(b"abc")),
        "a9993e364706816aba3e25717850c26c9cd0d89d"
    );
    assert_eq!(
        hex(&sha256(b"abc")),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn encrypted_message_round_trip_and_authentication() {
    let auth_key: Vec<u8> = (0_u8..=255).collect();
    let body = [0x78, 0x56, 0x34, 0x12];
    let padding = [0x5a; 12];
    let packet = encrypt_message(
        &auth_key,
        MessageToEncrypt {
            server_salt: 11,
            session_id: 22,
            message_id: 0x1122_3344_5566_7788,
            sequence: 1,
            body: &body,
            padding: &padding,
        },
        Direction::ClientToServer,
    )
    .unwrap();
    let decoded = decrypt_message(&auth_key, &packet, Direction::ClientToServer, 22, 1024).unwrap();
    assert_eq!(decoded.server_salt, 11);
    assert_eq!(decoded.body, body);
    assert_eq!(decoded.padding, padding);

    let mut corrupted = packet;
    corrupted[30] ^= 1;
    assert!(matches!(
        decrypt_message(&auth_key, &corrupted, Direction::ClientToServer, 22, 1024),
        Err(Error::MessageKeyMismatch)
    ));
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
