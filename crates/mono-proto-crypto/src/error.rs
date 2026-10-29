//! Cryptographic and authenticated-envelope failures.

use thiserror::Error;

/// Failures which prevent a cryptographic operation from being safe.
#[derive(Debug, Error)]
pub enum Error {
    /// AES key or IV has the wrong length.
    #[error("invalid {kind} length {actual}; expected {expected}")]
    InvalidLength {
        /// Parameter name.
        kind: &'static str,
        /// Actual byte count.
        actual: usize,
        /// Required byte count.
        expected: usize,
    },
    /// IGE requires complete AES blocks.
    #[error("AES-IGE payload length {0} is not a multiple of 16")]
    UnalignedPayload(usize),
    /// An RSA key could not be parsed.
    #[error("invalid RSA public key: {0}")]
    InvalidRsaKey(String),
    /// RSA plaintext is outside the key's mathematical domain.
    #[error("RSA plaintext does not fit the modulus")]
    RsaPlaintextTooLarge,
    /// Diffie-Hellman parameters fail MTProto validation.
    #[error("invalid Diffie-Hellman parameters: {0}")]
    InvalidDh(&'static str),
    /// The operating system random source failed.
    #[error("secure random source failed: {0}")]
    Random(String),
    /// An encrypted MTProto envelope failed structural or authentication checks.
    #[error("invalid encrypted message: {0}")]
    InvalidEnvelope(&'static str),
    /// The encrypted message belongs to another authorization key.
    #[error("encrypted message auth_key_id does not match")]
    AuthKeyMismatch,
    /// The authenticated message key does not match the decrypted plaintext.
    #[error("encrypted message key authentication failed")]
    MessageKeyMismatch,
    /// The encrypted message belongs to another session.
    #[error("encrypted message session mismatch: expected {expected}, got {actual}")]
    SessionMismatch {
        /// Active session identifier.
        expected: i64,
        /// Identifier in the authenticated envelope.
        actual: i64,
    },
    /// The authorization proof-of-work value could not be safely factored.
    #[error("invalid pq value: {0}")]
    InvalidPq(&'static str),
}
