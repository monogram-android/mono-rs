//! Parser diagnostics.

use thiserror::Error;

/// A parser diagnostic with exact source location.
#[derive(Clone, Debug, Error, Eq, PartialEq)]
#[error("Parse error: {message} at byte {offset}")]
pub struct ParseError {
    message: String,
    offset: usize,
}

impl ParseError {
    /// Human-readable explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
    /// Byte offset at which parsing failed.
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }
    pub(crate) fn new(message: impl Into<String>, offset: usize) -> Self {
        Self {
            message: message.into(),
            offset,
        }
    }
}
