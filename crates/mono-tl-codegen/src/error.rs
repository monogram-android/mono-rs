//! Generator errors.

use thiserror::Error;

/// Failure while translating a language-neutral TL model to Rust.
#[derive(Debug, Error)]
pub enum GenerateError {
    /// Generated syntax failed validation.
    #[error("Generated invalid Rust syntax: {0}")]
    Syntax(#[from] syn::Error),
    /// A manually constructed model contains an expression that cannot denote a Rust type.
    #[error("Invalid type expression: {0}")]
    InvalidExpression(String),
}
