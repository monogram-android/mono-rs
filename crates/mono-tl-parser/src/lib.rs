//! Parser for Telegram's [Type Language](https://core.telegram.org/mtproto/TL).
//!
//! The crate separates its syntax tree, lexer, diagnostics, and recursive
//! descent parser so each layer can evolve independently.

mod ast;
mod error;
mod lexer;
mod parser;

pub use ast::*;
pub use error::ParseError;

/// Parse a TL schema while optionally recording bounded source diagnostics.
///
/// # Errors
///
/// Returns the first syntax or declaration error encountered in `source`.
pub fn parse(source: &str) -> Result<Schema, ParseError> {
    #[cfg(all(feature = "tracing", feature = "dangerous-unredacted-logging"))]
    tracing::warn!(
        dangerous_unredacted_logging = true,
        source,
        "DANGEROUS: parsing unredacted TL schema"
    );
    #[cfg(all(feature = "tracing", not(feature = "dangerous-unredacted-logging")))]
    tracing::debug!(
        source = "[REDACTED]",
        source_length = source.len(),
        "Parsing TL schema"
    );
    parser::parse(source)
}
