//! Runtime-independent framing for MTProto transports.
//!
//! Framing, stream obfuscation, HTTP envelopes, controls, and connection traits
//! are independent modules; socket ownership remains in runtime adapters.

#![forbid(unsafe_code)]

mod common;
mod connection;
mod framing;
mod http;
mod obfuscated;

pub use common::*;
pub use connection::*;
pub use framing::*;
pub use http::*;
pub use obfuscated::*;

#[cfg(test)]
#[path = "../tests/unit/tests.rs"]
mod tests;
