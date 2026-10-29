//! Low-level, runtime-neutral MTProto request engine.
//!
//! Wire/service handling, RPC correlation, error advice, and authorization
//! transitions are separate modules over generated declaration adapters.

#![forbid(unsafe_code)]

mod authorization;
mod error;
mod rpc;
mod rpc_error;
mod wire;

pub use authorization::*;
pub use error::*;
pub use rpc::*;
pub use rpc_error::*;
pub use wire::*;

#[cfg(test)]
#[path = "../tests/unit/tests.rs"]
mod tests;
