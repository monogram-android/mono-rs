//! Deterministic clocks and random sources.

use mono_proto_session::{Clock, Error as SessionError, RandomSource};
use std::cell::Cell;

/// Manually advanced microsecond clock.
#[derive(Debug)]
pub struct TestClock(Cell<i64>);
impl TestClock {
    /// Create a clock at a Unix timestamp.
    pub const fn new(unix_micros: i64) -> Self {
        Self(Cell::new(unix_micros))
    }
    /// Advance by a signed duration.
    pub fn advance(&self, micros: i64) {
        self.0.set(self.0.get().saturating_add(micros));
    }
    /// Set the timestamp directly.
    pub fn set(&self, unix_micros: i64) {
        self.0.set(unix_micros);
    }
}
impl Clock for TestClock {
    fn unix_micros(&self) -> i64 {
        self.0.get()
    }
}

/// Repeatable xorshift source for tests. It is intentionally not cryptographic.
#[derive(Clone, Debug)]
pub struct DeterministicRandom(u64);
impl DeterministicRandom {
    /// Seed the source; zero is replaced by a fixed nonzero seed.
    pub const fn new(seed: u64) -> Self {
        Self(if seed == 0 {
            0x9e37_79b9_7f4a_7c15
        } else {
            seed
        })
    }
}
impl RandomSource for DeterministicRandom {
    fn fill(&mut self, output: &mut [u8]) -> Result<(), SessionError> {
        for byte in output {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            *byte = self.0.to_le_bytes()[0];
        }
        Ok(())
    }
}
