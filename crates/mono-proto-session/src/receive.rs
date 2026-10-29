//! Authenticated receive-window policy and dispositions.

/// Result of registering an authenticated incoming message identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReceivedMessageResult {
    /// Identifier was accepted for the first time.
    Accepted,
    /// Identifier has already been processed.
    Duplicate,
    /// Identifier is below the retained replay window.
    TooOld,
    /// Identifier timestamp is outside the accepted clock window.
    InvalidTime,
}

/// Telegram Desktop retains this many recent message identifiers.
pub const RECEIVED_MESSAGE_WINDOW: usize = 400;
