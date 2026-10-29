//! Runtime-neutral byte connection boundary.

use crate::Error;

/// Runtime-neutral byte connection used by adapters and test fakes.
pub trait Connection {
    /// Write an entire framed packet.
    fn send(&mut self, packet: &[u8]) -> Result<(), Error>;
    /// Receive bytes, returning zero only on orderly close.
    fn receive(&mut self, output: &mut [u8]) -> Result<usize, Error>;
    /// Close the connection. Calling this more than once should be harmless.
    fn close(&mut self) -> Result<(), Error>;
}
