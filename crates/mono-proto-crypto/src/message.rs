//! MTProto authorization and encrypted-message key operations.

use crate::{Error, aes_ige_decrypt, aes_ige_encrypt, constant_time_eq, sha1, sha256};

/// Direction used by the MTProto 2.0 message-key KDF.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    /// Client-to-server traffic (`x = 0`).
    ClientToServer,
    /// Server-to-client traffic (`x = 8`).
    ServerToClient,
}

impl Direction {
    fn x(self) -> usize {
        if self == Self::ClientToServer { 0 } else { 8 }
    }
}

/// Derive the 128-bit MTProto 2.0 message key from padded plaintext.
pub fn message_key(
    auth_key: &[u8],
    plaintext_with_padding: &[u8],
    direction: Direction,
) -> Result<[u8; 16], Error> {
    if auth_key.len() < 128 {
        return Err(Error::InvalidLength {
            kind: "auth key",
            actual: auth_key.len(),
            expected: 128,
        });
    }
    let x = direction.x();
    let mut input = Vec::with_capacity(32 + plaintext_with_padding.len());
    input.extend_from_slice(&auth_key[88 + x..120 + x]);
    input.extend_from_slice(plaintext_with_padding);
    Ok(sha256(&input)[8..24].try_into().expect("fixed slice"))
}

/// Compute the transport quick-ack token for authenticated plaintext.
pub fn quick_ack_token(
    auth_key: &[u8],
    plaintext_with_padding: &[u8],
    direction: Direction,
) -> Result<u32, Error> {
    if auth_key.len() < 128 {
        return Err(Error::InvalidLength {
            kind: "auth key",
            actual: auth_key.len(),
            expected: 128,
        });
    }
    let x = direction.x();
    let mut input = Vec::with_capacity(32 + plaintext_with_padding.len());
    input.extend_from_slice(&auth_key[88 + x..120 + x]);
    input.extend_from_slice(plaintext_with_padding);
    let hash = sha256(&input);
    Ok(u32::from_le_bytes(hash[..4].try_into().expect("fixed hash prefix")) | 0x8000_0000)
}

/// Derive the AES key and IV for an MTProto 2.0 encrypted message.
pub fn message_aes_key_iv(
    auth_key: &[u8],
    msg_key: &[u8; 16],
    direction: Direction,
) -> Result<([u8; 32], [u8; 32]), Error> {
    if auth_key.len() < 128 {
        return Err(Error::InvalidLength {
            kind: "auth key",
            actual: auth_key.len(),
            expected: 128,
        });
    }
    let x = direction.x();
    let mut a_input = Vec::with_capacity(52);
    a_input.extend_from_slice(msg_key);
    a_input.extend_from_slice(&auth_key[x..x + 36]);
    let a = sha256(&a_input);
    let mut b_input = Vec::with_capacity(52);
    b_input.extend_from_slice(&auth_key[40 + x..76 + x]);
    b_input.extend_from_slice(msg_key);
    let b = sha256(&b_input);
    let mut key = [0_u8; 32];
    key[..8].copy_from_slice(&a[..8]);
    key[8..24].copy_from_slice(&b[8..24]);
    key[24..].copy_from_slice(&a[24..]);
    let mut iv = [0_u8; 32];
    iv[..8].copy_from_slice(&b[..8]);
    iv[8..24].copy_from_slice(&a[8..24]);
    iv[24..].copy_from_slice(&b[24..]);
    Ok((key, iv))
}

/// Compute the low 64 bits of SHA-1(auth_key), as transmitted by MTProto.
pub fn auth_key_id(auth_key: &[u8]) -> i64 {
    i64::from_le_bytes(sha1(auth_key)[12..20].try_into().expect("fixed slice"))
}

/// Compute the initial server salt as `new_nonce[0..8] XOR server_nonce[0..8]`.
pub fn initial_server_salt(new_nonce: &[u8; 32], server_nonce: &[u8; 16]) -> i64 {
    let mut salt = [0_u8; 8];
    for i in 0..8 {
        salt[i] = new_nonce[i] ^ server_nonce[i];
    }
    i64::from_le_bytes(salt)
}

/// Derive the temporary AES key and IV used during authorization.
pub fn temporary_aes_key_iv(new_nonce: &[u8; 32], server_nonce: &[u8; 16]) -> ([u8; 32], [u8; 32]) {
    let mut new_server = [0_u8; 48];
    new_server[..32].copy_from_slice(new_nonce);
    new_server[32..].copy_from_slice(server_nonce);
    let hash_new_server = sha1(&new_server);

    let mut server_new = [0_u8; 48];
    server_new[..16].copy_from_slice(server_nonce);
    server_new[16..].copy_from_slice(new_nonce);
    let hash_server_new = sha1(&server_new);

    let mut new_new = [0_u8; 64];
    new_new[..32].copy_from_slice(new_nonce);
    new_new[32..].copy_from_slice(new_nonce);
    let hash_new_new = sha1(&new_new);

    let mut key = [0_u8; 32];
    key[..20].copy_from_slice(&hash_new_server);
    key[20..].copy_from_slice(&hash_server_new[..12]);
    let mut iv = [0_u8; 32];
    iv[..8].copy_from_slice(&hash_server_new[12..]);
    iv[8..28].copy_from_slice(&hash_new_new);
    iv[28..].copy_from_slice(&new_nonce[..4]);
    (key, iv)
}

/// Compute `new_nonce_hash{1,2,3}` for a DH completion response.
pub fn new_nonce_hash(new_nonce: &[u8; 32], auth_key: &[u8], number: u8) -> [u8; 16] {
    let auth_hash = sha1(auth_key);
    let mut input = [0_u8; 41];
    input[..32].copy_from_slice(new_nonce);
    input[32] = number;
    input[33..].copy_from_slice(&auth_hash[..8]);
    sha1(&input)[4..20].try_into().expect("fixed slice")
}

/// Authenticated contents of an MTProto 2.0 encrypted message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecryptedMessage {
    /// Server salt carried by the envelope.
    pub server_salt: i64,
    /// Session identifier carried by the envelope.
    pub session_id: i64,
    /// MTProto message identifier.
    pub message_id: i64,
    /// Message sequence number.
    pub sequence: i32,
    /// Boxed TL message or container body.
    pub body: Vec<u8>,
    /// Authenticated random padding.
    pub padding: Vec<u8>,
}

/// Plaintext fields supplied when constructing an encrypted MTProto message.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MessageToEncrypt<'a> {
    /// Current server salt.
    pub server_salt: i64,
    /// Active session identifier.
    pub session_id: i64,
    /// MTProto message identifier.
    pub message_id: i64,
    /// Message sequence number.
    pub sequence: i32,
    /// Boxed TL message or container body.
    pub body: &'a [u8],
    /// Caller-provided authenticated random padding.
    pub padding: &'a [u8],
}

/// Encrypt a complete MTProto 2.0 message using caller-provided random padding.
///
/// Padding must contain 12..=1024 bytes and make the encrypted plaintext a
/// multiple of the AES block size. Accepting padding explicitly makes official
/// vectors and deterministic test sources reproducible.
pub fn encrypt_message(
    auth_key: &[u8],
    message: MessageToEncrypt<'_>,
    direction: Direction,
) -> Result<Vec<u8>, Error> {
    validate_message_parts(auth_key, message.body, message.padding)?;
    let mut plaintext = Vec::with_capacity(32 + message.body.len() + message.padding.len());
    plaintext.extend_from_slice(&message.server_salt.to_le_bytes());
    plaintext.extend_from_slice(&message.session_id.to_le_bytes());
    plaintext.extend_from_slice(&message.message_id.to_le_bytes());
    plaintext.extend_from_slice(&message.sequence.to_le_bytes());
    plaintext.extend_from_slice(
        &u32::try_from(message.body.len())
            .map_err(|_| Error::InvalidEnvelope("message body exceeds 32-bit length"))?
            .to_le_bytes(),
    );
    plaintext.extend_from_slice(message.body);
    plaintext.extend_from_slice(message.padding);

    let msg_key = message_key(auth_key, &plaintext, direction)?;
    let (aes_key, aes_iv) = message_aes_key_iv(auth_key, &msg_key, direction)?;
    let encrypted = aes_ige_encrypt(&plaintext, &aes_key, &aes_iv)?;
    let mut packet = Vec::with_capacity(24 + encrypted.len());
    packet.extend_from_slice(&auth_key_id(auth_key).to_le_bytes());
    packet.extend_from_slice(&msg_key);
    packet.extend_from_slice(&encrypted);
    Ok(packet)
}

fn validate_message_parts(auth_key: &[u8], body: &[u8], padding: &[u8]) -> Result<(), Error> {
    if auth_key.len() != 256 {
        return Err(Error::InvalidLength {
            kind: "auth key",
            actual: auth_key.len(),
            expected: 256,
        });
    }
    if body.is_empty() || body.len() > 16 * 1024 * 1024 || body.len() % 4 != 0 {
        return Err(Error::InvalidEnvelope(
            "message body must be nonempty, 4-byte aligned, and at most 16 MiB",
        ));
    }
    if !(12..=1024).contains(&padding.len()) {
        return Err(Error::InvalidEnvelope("padding is outside 12..=1024 bytes"));
    }
    if (32 + body.len() + padding.len()) % 16 != 0 {
        return Err(Error::InvalidEnvelope("plaintext is not AES-block aligned"));
    }
    Ok(())
}

/// Decrypt and authenticate a complete MTProto 2.0 message.
///
/// Authentication is checked before any parsed plaintext field is returned.
pub fn decrypt_message(
    auth_key: &[u8],
    packet: &[u8],
    direction: Direction,
    expected_session_id: i64,
    max_body_length: usize,
) -> Result<DecryptedMessage, Error> {
    if auth_key.len() != 256 {
        return Err(Error::InvalidLength {
            kind: "auth key",
            actual: auth_key.len(),
            expected: 256,
        });
    }
    if packet.len() < 24 + 48 || (packet.len() - 24) % 16 != 0 {
        return Err(Error::InvalidEnvelope("invalid ciphertext length"));
    }
    let encoded_key_id = i64::from_le_bytes(packet[..8].try_into().expect("fixed header"));
    let msg_key: [u8; 16] = packet[8..24].try_into().expect("fixed header");
    let (aes_key, aes_iv) = message_aes_key_iv(auth_key, &msg_key, direction)?;
    let plaintext = aes_ige_decrypt(&packet[24..], &aes_key, &aes_iv)?;
    let calculated = message_key(auth_key, &plaintext, direction)?;
    if !constant_time_eq(&msg_key, &calculated) {
        return Err(Error::MessageKeyMismatch);
    }
    if encoded_key_id != auth_key_id(auth_key) {
        return Err(Error::AuthKeyMismatch);
    }

    let server_salt = i64::from_le_bytes(plaintext[..8].try_into().expect("validated minimum"));
    let session_id = i64::from_le_bytes(plaintext[8..16].try_into().expect("validated minimum"));
    let message_id = i64::from_le_bytes(plaintext[16..24].try_into().expect("validated minimum"));
    let sequence = i32::from_le_bytes(plaintext[24..28].try_into().expect("validated minimum"));
    let body_length =
        u32::from_le_bytes(plaintext[28..32].try_into().expect("validated minimum")) as usize;
    if body_length == 0 || body_length > max_body_length || body_length % 4 != 0 {
        return Err(Error::InvalidEnvelope(
            "invalid authenticated message length",
        ));
    }
    let body_end = 32_usize
        .checked_add(body_length)
        .ok_or(Error::InvalidEnvelope("message length overflow"))?;
    if body_end > plaintext.len() {
        return Err(Error::InvalidEnvelope("message length exceeds plaintext"));
    }
    let padding_length = plaintext.len() - body_end;
    if !(12..=1024).contains(&padding_length) {
        return Err(Error::InvalidEnvelope(
            "invalid authenticated padding length",
        ));
    }
    if session_id != expected_session_id {
        return Err(Error::SessionMismatch {
            expected: expected_session_id,
            actual: session_id,
        });
    }
    let expected_parity = match direction {
        Direction::ClientToServer => 0,
        Direction::ServerToClient => 1,
    };
    if message_id & 1 != expected_parity {
        return Err(Error::InvalidEnvelope("invalid message-id parity"));
    }
    Ok(DecryptedMessage {
        server_salt,
        session_id,
        message_id,
        sequence,
        body: plaintext[32..body_end].to_vec(),
        padding: plaintext[body_end..].to_vec(),
    })
}
