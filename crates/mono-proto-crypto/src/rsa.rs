//! Telegram RSA key fingerprints and RSA_PAD authorization.

use crate::{Error, aes_ige_encrypt, sha1, sha256};
use rsa::{
    RsaPublicKey, pkcs1::DecodeRsaPublicKey, pkcs8::DecodePublicKey, traits::PublicKeyParts,
};

/// Parse an RSA public key in PKCS#1 or SubjectPublicKeyInfo PEM form.
pub fn parse_rsa_public_key(pem: &str) -> Result<RsaPublicKey, Error> {
    RsaPublicKey::from_pkcs1_pem(pem)
        .or_else(|_| RsaPublicKey::from_public_key_pem(pem))
        .map_err(|error| Error::InvalidRsaKey(error.to_string()))
}

fn append_tl_bytes(output: &mut Vec<u8>, value: &[u8]) {
    let prefix = if value.len() < 254 {
        output.push(u8::try_from(value.len()).expect("short TL byte length"));
        1
    } else {
        output.push(254);
        let length = u32::try_from(value.len())
            .expect("RSA component fits TL byte length")
            .to_le_bytes();
        output.extend_from_slice(&length[..3]);
        4
    };
    output.extend_from_slice(value);
    output.resize(output.len() + (4 - (prefix + value.len()) % 4) % 4, 0);
}

/// Compute Telegram's RSA public-key fingerprint from bare TL `n` and `e` bytes.
pub fn rsa_public_key_fingerprint(key: &RsaPublicKey) -> i64 {
    let mut serialized = Vec::new();
    append_tl_bytes(&mut serialized, &key.n().to_bytes_be());
    append_tl_bytes(&mut serialized, &key.e().to_bytes_be());
    i64::from_le_bytes(
        sha1(&serialized)[12..]
            .try_into()
            .expect("fixed hash suffix"),
    )
}

/// Apply the raw RSA public operation required by MTProto authorization padding.
///
/// Padding must be constructed and validated by the authorization orchestrator;
/// this function refuses values outside the modulus and always returns a full key-sized block.
pub fn rsa_encrypt_raw(key: &RsaPublicKey, padded_plaintext: &[u8]) -> Result<Vec<u8>, Error> {
    let value = rsa::BigUint::from_bytes_be(padded_plaintext);
    if &value >= key.n() {
        return Err(Error::RsaPlaintextTooLarge);
    }
    let encrypted = value.modpow(key.e(), key.n()).to_bytes_be();
    let mut output = vec![0_u8; key.size()];
    let start = output.len() - encrypted.len();
    output[start..].copy_from_slice(&encrypted);
    Ok(output)
}

/// Apply Telegram's `RSA_PAD` construction for authorization.
///
/// The callback must fill every requested buffer from a cryptographically secure
/// source. It is injectable so the official samples can be reproduced in tests.
pub fn rsa_pad_with_rng(
    key: &RsaPublicKey,
    data: &[u8],
    mut random: impl FnMut(&mut [u8]) -> Result<(), Error>,
) -> Result<Vec<u8>, Error> {
    if data.len() > 144 {
        return Err(Error::InvalidEnvelope("RSA_PAD data exceeds 144 bytes"));
    }
    if key.size() != 256 {
        return Err(Error::InvalidRsaKey(
            "RSA_PAD requires a 2048-bit key".into(),
        ));
    }
    let mut data_with_padding = [0_u8; 192];
    data_with_padding[..data.len()].copy_from_slice(data);
    random(&mut data_with_padding[data.len()..])?;

    for _ in 0..128 {
        let mut temp_key = [0_u8; 32];
        random(&mut temp_key)?;
        let mut reversed = data_with_padding;
        reversed.reverse();
        let mut hash_input = Vec::with_capacity(224);
        hash_input.extend_from_slice(&temp_key);
        hash_input.extend_from_slice(&data_with_padding);
        let hash = sha256(&hash_input);
        let mut data_with_hash = [0_u8; 224];
        data_with_hash[..192].copy_from_slice(&reversed);
        data_with_hash[192..].copy_from_slice(&hash);
        let aes_encrypted = aes_ige_encrypt(&data_with_hash, &temp_key, &[0; 32])?;
        let encrypted_hash = sha256(&aes_encrypted);
        let mut padded = [0_u8; 256];
        for index in 0..32 {
            padded[index] = temp_key[index] ^ encrypted_hash[index];
        }
        padded[32..].copy_from_slice(&aes_encrypted);
        if rsa::BigUint::from_bytes_be(&padded) < *key.n() {
            return rsa_encrypt_raw(key, &padded);
        }
    }
    Err(Error::Random("RSA_PAD rejection limit exceeded".into()))
}

/// Apply `RSA_PAD` using the operating-system CSPRNG.
pub fn rsa_pad(key: &RsaPublicKey, data: &[u8]) -> Result<Vec<u8>, Error> {
    rsa_pad_with_rng(key, data, fill_random)
}

/// Fill a caller-owned buffer from the operating system CSPRNG.
pub fn fill_random(output: &mut [u8]) -> Result<(), Error> {
    getrandom::fill(output).map_err(|error| Error::Random(error.to_string()))
}
