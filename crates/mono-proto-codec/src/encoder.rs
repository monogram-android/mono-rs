//! Canonical TL byte encoder.

use crate::Error;

/// Accumulates a TL value.
#[derive(Clone, Debug, Default)]
pub struct Encoder {
    bytes: Vec<u8>,
}

impl Encoder {
    /// Create an empty encoder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Finish encoding.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Borrow bytes encoded so far.
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    /// Append raw bytes. This is intended for already-validated protocol envelopes.
    pub fn write_raw(&mut self, bytes: &[u8]) {
        trace_bytes("codec.write_raw", bytes);
        self.bytes.extend_from_slice(bytes);
    }

    /// Append a little-endian 32-bit word.
    pub fn write_u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    /// Append a TL byte string, including canonical length prefix and zero padding.
    pub fn write_bytes(&mut self, value: &[u8]) -> Result<(), Error> {
        trace_bytes("codec.write_bytes", value);
        let prefix = if value.len() < 254 {
            self.bytes
                .push(u8::try_from(value.len()).map_err(|_| Error::LimitExceeded {
                    kind: "bytes",
                    actual: value.len(),
                    limit: 253,
                })?);
            1
        } else {
            if value.len() > 0x00ff_ffff {
                return Err(Error::LimitExceeded {
                    kind: "TL bytes",
                    actual: value.len(),
                    limit: 0x00ff_ffff,
                });
            }
            self.bytes.push(254);
            let len = u32::try_from(value.len())
                .expect("24-bit length fits u32")
                .to_le_bytes();
            self.bytes.extend_from_slice(&len[..3]);
            4
        };
        self.bytes.extend_from_slice(value);
        let padding = (4 - ((prefix + value.len()) % 4)) % 4;
        self.bytes.resize(self.bytes.len() + padding, 0);
        Ok(())
    }
}

#[cfg(all(feature = "tracing", feature = "dangerous-unredacted-logging"))]
fn trace_bytes(event: &'static str, value: &[u8]) {
    tracing::warn!(event, dangerous_unredacted_logging = true, value = ?value, "DANGEROUS: encoding unredacted TL bytes");
}

#[cfg(all(feature = "tracing", not(feature = "dangerous-unredacted-logging")))]
fn trace_bytes(event: &'static str, value: &[u8]) {
    tracing::debug!(
        event,
        value = "[REDACTED]",
        length = value.len(),
        "Encoding redacted TL bytes"
    );
}

#[cfg(not(feature = "tracing"))]
fn trace_bytes(_event: &'static str, _value: &[u8]) {}
