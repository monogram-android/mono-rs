//! Explicit, runtime-neutral MTProto session state.
//!
//! Persistence ports, receive-window policy, and mutable session transitions are
//! separated while the crate root preserves the original public API.

#![forbid(unsafe_code)]

mod error;
mod ports;
mod receive;
mod state;

pub use error::*;
pub use ports::*;
pub use receive::*;
pub use state::*;

#[cfg(test)]
#[path = "../tests/unit/tests.rs"]
mod tests;
