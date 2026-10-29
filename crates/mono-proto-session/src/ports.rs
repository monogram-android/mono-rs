//! Clock, randomness, and durable storage ports.

use crate::{Error, Snapshot};

/// Time source supplied by an application or deterministic test.
pub trait Clock {
    /// Current Unix time in microseconds.
    fn unix_micros(&self) -> i64;
}

/// Source for session identifiers.
pub trait RandomSource {
    /// Fill `output` with unpredictable or deterministic bytes.
    fn fill(&mut self, output: &mut [u8]) -> Result<(), Error>;
}

/// Operating-system random source.
#[derive(Clone, Copy, Debug, Default)]
pub struct OsRandom;
impl RandomSource for OsRandom {
    fn fill(&mut self, output: &mut [u8]) -> Result<(), Error> {
        getrandom::fill(output).map_err(|error| Error::Persistence(error.to_string()))
    }
}

/// Durable storage boundary. Backends decide serialization, locking and encryption.
pub trait SessionStore {
    /// Load a snapshot, or `None` if this account/DC has no session.
    fn load(&mut self) -> Result<Option<Snapshot>, Error>;
    /// Atomically replace the stored snapshot.
    fn save(&mut self, snapshot: &Snapshot) -> Result<(), Error>;
    /// Remove persisted state.
    fn clear(&mut self) -> Result<(), Error>;
}
