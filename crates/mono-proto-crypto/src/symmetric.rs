//! Hash, constant-time comparison, AES-IGE, and stateful AES-CTR primitives.

use crate::Error;
use aes::Aes256;
use cipher::{Block, BlockCipherDecrypt, BlockCipherEncrypt, KeyInit, KeyIvInit, StreamCipher};
use sha1::{Digest as _, Sha1};
use sha2::Sha256;
use subtle::ConstantTimeEq;

/// SHA-1 digest used by MTProto authorization and key identifiers.
pub fn sha1(data: &[u8]) -> [u8; 20] {
    Sha1::digest(data).into()
}

/// SHA-256 digest used by MTProto 2.0 message encryption.
pub fn sha256(data: &[u8]) -> [u8; 32] {
    Sha256::digest(data).into()
}

/// Compare secret byte strings without early exit. Length remains observable.
pub fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    left.len() == right.len() && bool::from(left.ct_eq(right))
}

fn aes(key: &[u8]) -> Result<Aes256, Error> {
    if key.len() != 32 {
        return Err(Error::InvalidLength {
            kind: "AES-256 key",
            actual: key.len(),
            expected: 32,
        });
    }
    Aes256::new_from_slice(key).map_err(|_| Error::InvalidLength {
        kind: "AES-256 key",
        actual: key.len(),
        expected: 32,
    })
}

fn validate_ige(payload: &[u8], iv: &[u8]) -> Result<(), Error> {
    if iv.len() != 32 {
        return Err(Error::InvalidLength {
            kind: "AES-IGE IV",
            actual: iv.len(),
            expected: 32,
        });
    }
    if payload.len() % 16 != 0 {
        return Err(Error::UnalignedPayload(payload.len()));
    }
    Ok(())
}

/// Encrypt complete blocks using AES-256-IGE.
pub fn aes_ige_encrypt(payload: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, Error> {
    validate_ige(payload, iv)?;
    let cipher = aes(key)?;
    let mut previous_ciphertext = <[u8; 16]>::try_from(&iv[..16]).expect("validated IV");
    let mut previous_plaintext = <[u8; 16]>::try_from(&iv[16..]).expect("validated IV");
    let mut output = Vec::with_capacity(payload.len());
    for plaintext in payload.chunks_exact(16) {
        let mut block = [0_u8; 16];
        for i in 0..16 {
            block[i] = plaintext[i] ^ previous_ciphertext[i];
        }
        let mut aes_block = Block::<Aes256>::from(block);
        cipher.encrypt_block(&mut aes_block);
        block = aes_block.into();
        for i in 0..16 {
            block[i] ^= previous_plaintext[i];
        }
        output.extend_from_slice(&block);
        previous_ciphertext = block;
        previous_plaintext.copy_from_slice(plaintext);
    }
    Ok(output)
}

/// Decrypt complete blocks using AES-256-IGE.
pub fn aes_ige_decrypt(payload: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, Error> {
    validate_ige(payload, iv)?;
    let cipher = aes(key)?;
    let mut previous_ciphertext = <[u8; 16]>::try_from(&iv[..16]).expect("validated IV");
    let mut previous_plaintext = <[u8; 16]>::try_from(&iv[16..]).expect("validated IV");
    let mut output = Vec::with_capacity(payload.len());
    for ciphertext in payload.chunks_exact(16) {
        let mut block = [0_u8; 16];
        for i in 0..16 {
            block[i] = ciphertext[i] ^ previous_plaintext[i];
        }
        let mut aes_block = Block::<Aes256>::from(block);
        cipher.decrypt_block(&mut aes_block);
        block = aes_block.into();
        for i in 0..16 {
            block[i] ^= previous_ciphertext[i];
        }
        output.extend_from_slice(&block);
        previous_plaintext = block;
        previous_ciphertext.copy_from_slice(ciphertext);
    }
    Ok(output)
}

/// Apply AES-256-CTR in place with the MTProto-compatible 128-bit counter.
pub fn aes_ctr_apply(data: &mut [u8], key: &[u8], iv: &[u8]) -> Result<(), Error> {
    AesCtr::new(key, iv)?.apply(data);
    Ok(())
}

/// Stateful AES-256-CTR stream used by transport obfuscation.
pub struct AesCtr {
    cipher: ctr::Ctr128BE<Aes256>,
}

impl std::fmt::Debug for AesCtr {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("AesCtr").finish_non_exhaustive()
    }
}

impl AesCtr {
    /// Initialize a stream. Subsequent calls continue from the previous counter.
    pub fn new(key: &[u8], iv: &[u8]) -> Result<Self, Error> {
        if key.len() != 32 {
            return Err(Error::InvalidLength {
                kind: "AES-256 key",
                actual: key.len(),
                expected: 32,
            });
        }
        if iv.len() != 16 {
            return Err(Error::InvalidLength {
                kind: "AES-CTR IV",
                actual: iv.len(),
                expected: 16,
            });
        }
        Ok(Self {
            cipher: ctr::Ctr128BE::<Aes256>::new_from_slices(key, iv)
                .expect("validated key and IV"),
        })
    }

    /// Encrypt or decrypt the next portion of the stream in place.
    pub fn apply(&mut self, data: &mut [u8]) {
        self.cipher.apply_keystream(data);
    }
}
