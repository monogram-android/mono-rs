//! Scripted in-memory connection fake.

use mono_proto_transport::{Connection, Error as TransportError};
use std::collections::VecDeque;

/// In-memory connection with independently scripted reads and captured writes.
#[derive(Clone, Debug, Default)]
pub struct FakeConnection {
    reads: VecDeque<Result<Vec<u8>, TransportError>>,
    /// Complete buffers passed to [`Connection::send`].
    pub writes: Vec<Vec<u8>>,
    /// Whether close has been requested.
    pub closed: bool,
}
impl FakeConnection {
    /// Add bytes returned by future receive calls.
    pub fn push_read(&mut self, bytes: impl Into<Vec<u8>>) {
        self.reads.push_back(Ok(bytes.into()));
    }
    /// Add an error returned by a future receive call.
    pub fn push_error(&mut self, error: TransportError) {
        self.reads.push_back(Err(error));
    }
}
impl Connection for FakeConnection {
    fn send(&mut self, packet: &[u8]) -> Result<(), TransportError> {
        if self.closed {
            return Err(TransportError::Closed);
        }
        self.writes.push(packet.to_vec());
        Ok(())
    }
    fn receive(&mut self, output: &mut [u8]) -> Result<usize, TransportError> {
        if self.closed {
            return Ok(0);
        }
        let Some(next) = self.reads.pop_front() else {
            return Err(TransportError::Connection("no scripted read".into()));
        };
        let mut bytes = next?;
        let count = bytes.len().min(output.len());
        output[..count].copy_from_slice(&bytes[..count]);
        if count < bytes.len() {
            bytes.drain(..count);
            self.reads.push_front(Ok(bytes));
        }
        Ok(count)
    }
    fn close(&mut self) -> Result<(), TransportError> {
        self.closed = true;
        Ok(())
    }
}
