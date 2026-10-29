//! Cryptographic operations required by MTProto 2.0.
//!
//! Hashing, symmetric primitives, message envelopes, RSA authorization, and
//! Diffie–Hellman validation are separated so each security boundary is explicit.

#![forbid(unsafe_code)]

mod dh;
mod error;
mod message;
mod rsa;
mod symmetric;

pub use dh::*;
pub use error::*;
pub use message::*;
pub use rsa::*;
pub use symmetric::*;

#[cfg(test)]
#[path = "../tests/unit/tests.rs"]
mod tests;
