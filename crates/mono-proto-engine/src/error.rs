//! Low-level engine failures with protocol context.

use crate::RpcError;
use thiserror::Error as ThisError;

/// Engine failure with inspectable protocol context.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Session transition failed.
    #[error(transparent)]
    Session(#[from] mono_proto_session::Error),
    /// Generated declaration codec failed.
    #[error(transparent)]
    Codec(#[from] mono_proto_codec::Error),
    /// Transport failed.
    #[error(transparent)]
    Transport(#[from] mono_proto_transport::Error),
    /// Cryptographic authentication or envelope processing failed.
    #[error(transparent)]
    Crypto(#[from] mono_proto_crypto::Error),
    /// A response referenced no outstanding request.
    #[error("response for unknown request message {0}")]
    UnknownRequest(i64),
    /// A response did not arrive before its deadline.
    #[error("request message {message_id} timed out after {attempts} attempts")]
    Timeout {
        /// Request message identifier.
        message_id: i64,
        /// Number of sends.
        attempts: u32,
    },
    /// Retry limit was exhausted.
    #[error("request message {message_id} exhausted {attempts} attempts")]
    RetryExhausted {
        /// Request message identifier.
        message_id: i64,
        /// Number of sends.
        attempts: u32,
    },
    /// An RPC returned a Telegram error.
    #[error(transparent)]
    Rpc(#[from] RpcError),
    /// Authorization state machine rejected an event.
    #[error("authorization failed: {0}")]
    Authorization(String),
    /// A low-level MTProto envelope or service value is malformed.
    #[error("malformed MTProto message: {0}")]
    MalformedMessage(&'static str),
    /// A `gzip_packed` payload could not be safely expanded.
    #[error("invalid gzip_packed payload: {0}")]
    Compression(String),
}
