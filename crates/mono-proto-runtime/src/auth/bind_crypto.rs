use crate::MtprotoError;
use mono_proto::codec::{Boxed, Encoder};
use mono_proto::transport::BindAuthKeyInnerConstructor;
#[cfg(test)]
use mono_proto_crypto::aes_ige_decrypt;
use mono_proto_crypto::{aes_ige_encrypt, auth_key_id, sha1};

/// MTProto v1 `encrypted_message` for `auth.bindTempAuthKey`.
/// `msg_key` covers the envelope before alignment padding.
pub fn encrypt_bind_message(
    perm_key: &[u8],
    msg_id: i64,
    inner: &BindAuthKeyInnerConstructor,
    random_prefix: &[u8; 16],
    padding: &[u8],
) -> Result<Vec<u8>, MtprotoError> {
    if perm_key.len() != 256 {
        return Err(MtprotoError::Message("permanent key required".into()));
    }
    let body = encode_inner(inner)?;
    if body.len() != 40 {
        return Err(MtprotoError::Message("bind inner length".into()));
    }
    let mut plain = Vec::with_capacity(72 + padding.len());
    plain.extend_from_slice(random_prefix);
    plain.extend_from_slice(&msg_id.to_le_bytes());
    plain.extend_from_slice(&0_i32.to_le_bytes());
    plain.extend_from_slice(&40_i32.to_le_bytes());
    plain.extend_from_slice(&body);
    let msg_key = v1_msg_key(&plain);
    if padding.len() > 15 || (plain.len() + padding.len()) % 16 != 0 {
        return Err(MtprotoError::Message("bind padding".into()));
    }
    plain.extend_from_slice(padding);
    let (aes_key, aes_iv) = v1_aes(&msg_key, perm_key);
    let encrypted = aes_ige_encrypt(&plain, &aes_key, &aes_iv)
        .map_err(|e| MtprotoError::Message(e.to_string()))?;
    let mut out = Vec::with_capacity(24 + encrypted.len());
    out.extend_from_slice(&auth_key_id(perm_key).to_le_bytes());
    out.extend_from_slice(&msg_key);
    out.extend_from_slice(&encrypted);
    Ok(out)
}

fn v1_msg_key(unpadded: &[u8]) -> [u8; 16] {
    sha1(unpadded)[4..20].try_into().expect("sha1 slice")
}

fn v1_aes(msg_key: &[u8; 16], auth_key: &[u8]) -> ([u8; 32], [u8; 32]) {
    let sha_a = sha1_parts(&[msg_key, &auth_key[0..32]]);
    let sha_b = sha1_parts(&[&auth_key[32..48], msg_key, &auth_key[48..64]]);
    let sha_c = sha1_parts(&[&auth_key[64..96], msg_key]);
    let sha_d = sha1_parts(&[msg_key, &auth_key[96..128]]);
    let mut key = [0_u8; 32];
    key[..8].copy_from_slice(&sha_a[..8]);
    key[8..20].copy_from_slice(&sha_b[8..20]);
    key[20..].copy_from_slice(&sha_c[4..16]);
    let mut iv = [0_u8; 32];
    iv[..12].copy_from_slice(&sha_a[8..20]);
    iv[12..20].copy_from_slice(&sha_b[..8]);
    iv[20..24].copy_from_slice(&sha_c[16..20]);
    iv[24..].copy_from_slice(&sha_d[..8]);
    (key, iv)
}

fn sha1_parts(parts: &[&[u8]]) -> [u8; 20] {
    let mut buf = Vec::new();
    for part in parts {
        buf.extend_from_slice(part);
    }
    sha1(&buf)
}

fn encode_inner(inner: &BindAuthKeyInnerConstructor) -> Result<Vec<u8>, MtprotoError> {
    let mut encoder = Encoder::new();
    inner
        .encode_boxed(&mut encoder)
        .map_err(|e| MtprotoError::Message(e.to_string()))?;
    Ok(encoder.into_bytes())
}

#[cfg(test)]
#[path = "../../tests/unit/bind_crypto_tests.rs"]
mod tests;
