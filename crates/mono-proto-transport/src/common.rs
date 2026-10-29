//! Shared framing values, controls, errors, and bounded packet helpers.

use thiserror::Error as ThisError;

/// Default maximum accepted packet size (16 MiB).
pub const DEFAULT_MAX_PACKET: usize = 16 * 1024 * 1024;

/// Transport framing or lifecycle failure.
#[derive(Debug, ThisError, Clone, Eq, PartialEq)]
pub enum Error {
    /// More input is required to decode a complete packet.
    #[error("incomplete packet: need {needed} bytes, have {available}")]
    Incomplete {
        /// Total bytes required for this packet or header.
        needed: usize,
        /// Bytes currently available.
        available: usize,
    },
    /// A packet length is malformed or outside configured bounds.
    #[error("invalid packet length {length} (maximum {maximum})")]
    InvalidLength {
        /// Observed or calculated length.
        length: usize,
        /// Largest accepted length.
        maximum: usize,
    },
    /// A word-oriented transport received an unaligned payload.
    #[error("payload length {0} is not divisible by four")]
    Unaligned(usize),
    /// Full transport sequence number did not match.
    #[error("full transport sequence mismatch: expected {expected}, got {actual}")]
    Sequence {
        /// Next receive sequence.
        expected: u32,
        /// Sequence encoded in the packet.
        actual: u32,
    },
    /// Full transport checksum did not match.
    #[error("full transport CRC mismatch: expected {expected:#010x}, got {actual:#010x}")]
    Checksum {
        /// CRC32 calculated from the packet.
        expected: u32,
        /// CRC32 encoded in the packet.
        actual: u32,
    },
    /// The HTTP response envelope is malformed.
    #[error("invalid HTTP transport response: {0}")]
    Http(&'static str),
    /// The underlying connection failed.
    #[error("connection error: {0}")]
    Connection(String),
    /// The connection has already been closed.
    #[error("connection is closed")]
    Closed,
    /// A caller-provided obfuscation nonce is distinguishable as another protocol.
    #[error("invalid transport obfuscation nonce")]
    InvalidObfuscationNonce,
    /// An MTProxy secret has an unsupported length.
    #[error("invalid MTProxy secret length {0}; expected 16 bytes")]
    InvalidProxySecret(usize),
}

/// Result of incrementally decoding a transport packet.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Packet {
    /// Decoded MTProto payload.
    pub payload: Vec<u8>,
    /// Number of input bytes consumed, allowing callers to retain a suffix.
    pub consumed: usize,
}

/// Control value carried outside normal MTProto payload processing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Control {
    /// Quick acknowledgment token associated with an encrypted payload.
    QuickAck(u32),
    /// Negative transport error code.
    TransportError(i32),
}

/// Decode a standalone abridged quick-ack packet when the high bit is set.
pub fn abridged_control(input: &[u8]) -> Result<Option<(Control, usize)>, Error> {
    let Some(first) = input.first() else {
        return Err(Error::Incomplete {
            needed: 1,
            available: 0,
        });
    };
    if first & 0x80 == 0 {
        return Ok(None);
    }
    if input.len() < 4 {
        return Err(Error::Incomplete {
            needed: 4,
            available: input.len(),
        });
    }
    Ok(Some((
        Control::QuickAck(u32::from_be_bytes(
            input[..4].try_into().expect("four bytes"),
        )),
        4,
    )))
}

/// Decode a standalone intermediate quick-ack packet when the high bit is set.
pub fn intermediate_control(input: &[u8]) -> Result<Option<(Control, usize)>, Error> {
    if input.len() < 4 {
        return Err(Error::Incomplete {
            needed: 4,
            available: input.len(),
        });
    }
    let value = u32::from_le_bytes(input[..4].try_into().expect("four bytes"));
    Ok((value & 0x8000_0000 != 0).then_some((Control::QuickAck(value), 4)))
}

/// Interpret a decoded payload as a transport error or padded quick ACK.
pub fn payload_control(payload: &[u8]) -> Option<Control> {
    if payload.len() == 4 {
        let code = i32::from_le_bytes(payload.try_into().ok()?);
        return (code < 0).then_some(Control::TransportError(code));
    }
    if (8..=16).contains(&payload.len()) && payload[..4] == [0xff; 4] {
        return Some(Control::QuickAck(u32::from_le_bytes(
            payload[4..8].try_into().ok()?,
        )));
    }
    None
}

/// Stateless packet framing implemented by each transport mode.
pub trait Framing {
    /// Initial marker bytes sent once when opening a connection.
    fn preamble(&self) -> &'static [u8];
    /// Encode a single MTProto payload.
    fn encode(&mut self, payload: &[u8]) -> Result<Vec<u8>, Error>;
    /// Decode a single packet from a potentially larger receive buffer.
    fn decode(&mut self, input: &[u8]) -> Result<Packet, Error>;
}

pub(crate) fn checked_length(length: usize, maximum: usize) -> Result<(), Error> {
    if length == 0 || length > maximum {
        Err(Error::InvalidLength { length, maximum })
    } else {
        Ok(())
    }
}

pub(crate) fn packet(
    input: &[u8],
    header: usize,
    length: usize,
    maximum: usize,
) -> Result<Packet, Error> {
    checked_length(length, maximum)?;
    let total = header
        .checked_add(length)
        .ok_or(Error::InvalidLength { length, maximum })?;
    if input.len() < total {
        return Err(Error::Incomplete {
            needed: total,
            available: input.len(),
        });
    }
    Ok(Packet {
        payload: input[header..total].to_vec(),
        consumed: total,
    })
}
