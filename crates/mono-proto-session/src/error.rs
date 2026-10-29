//! Inspectable session transition and persistence failures.

use thiserror::Error;

/// Session transition or persistence failure.
#[derive(Debug, Error, Clone, Eq, PartialEq)]
pub enum Error {
    /// An authorization key was not exactly 256 bytes.
    #[error("auth key length {0}; expected 256")]
    InvalidAuthKey(usize),
    /// A server-provided message id violates MTProto alignment.
    #[error("invalid server message id {0}")]
    InvalidServerMessageId(i64),
    /// A generated message id could not be made newer than the previous id.
    #[error("message id clock overflow")]
    MessageIdOverflow,
    /// Sequence state overflowed.
    #[error("sequence counter overflow")]
    SequenceOverflow,
    /// A bad-message notification has an unsupported error code.
    #[error("unsupported bad-message code {0}")]
    UnsupportedBadMessage(i32),
    /// Persistence backend error.
    #[error("session persistence error: {0}")]
    Persistence(String),
}
