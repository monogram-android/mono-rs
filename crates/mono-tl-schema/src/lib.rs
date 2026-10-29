//! Validated, serializable representation of a TL schema.

mod builder;
mod identifier;
mod types;
mod validation;

pub use builder::{build, parse};
pub use types::*;
