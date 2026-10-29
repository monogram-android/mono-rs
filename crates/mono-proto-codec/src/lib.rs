//! Runtime-neutral, bounded encoding and decoding for Telegram's TL wire format.
//!
//! Generated declarations supply schema-specific constructor dispatch; the modules
//! here provide defensive wire primitives without a global Telegram API table.

#![forbid(unsafe_code)]

mod decoder;
mod encoder;
mod error;
mod traits;
mod value;

pub use decoder::*;
pub use encoder::*;
pub use error::*;
pub use traits::*;
pub use value::*;

#[cfg(test)]
#[path = "../tests/unit/tests.rs"]
mod tests;
