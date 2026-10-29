//! Actionable Telegram RPC error metadata.

use thiserror::Error;

/// Decoded `rpc_error`, including commonly actionable metadata.
#[derive(Clone, Debug, Error, Eq, PartialEq)]
#[error("RPC {code}: {message}")]
pub struct RpcError {
    /// Numeric Telegram error code.
    pub code: i32,
    /// Stable error text.
    pub message: String,
    /// Seconds to wait for `FLOOD_WAIT_n` and similar errors.
    pub flood_wait_seconds: Option<u32>,
    /// Target DC for `*_MIGRATE_n` errors.
    pub migrate_to_dc: Option<i32>,
}
impl RpcError {
    /// Construct and parse actionable suffixes without tying to an API layer.
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        let message = message.into();
        let flood_wait_seconds = message
            .strip_prefix("FLOOD_WAIT_")
            .or_else(|| message.strip_prefix("FLOOD_PREMIUM_WAIT_"))
            .and_then(|value| value.parse::<u32>().ok());
        let migrate_to_dc = [
            "PHONE_MIGRATE_",
            "NETWORK_MIGRATE_",
            "USER_MIGRATE_",
            "FILE_MIGRATE_",
            "STATS_MIGRATE_",
        ]
        .iter()
        .find_map(|prefix| {
            message
                .strip_prefix(prefix)
                .and_then(|value| value.parse::<i32>().ok())
        });
        Self {
            code,
            message,
            flood_wait_seconds,
            migrate_to_dc,
        }
    }
}
