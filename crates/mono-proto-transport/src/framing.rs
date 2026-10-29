//! Abridged, intermediate, padded-intermediate, and full framing.

use crate::common::{checked_length, packet};
use crate::{DEFAULT_MAX_PACKET, Error, Framing, Packet};

/// Abridged transport (`0xef` preamble and lengths measured in 32-bit words).
#[derive(Clone, Debug)]
pub struct Abridged {
    max_packet: usize,
}

impl Abridged {
    /// Create abridged framing with a packet byte limit.
    pub const fn new(max_packet: usize) -> Self {
        Self { max_packet }
    }

    /// Encode a packet with the quick-ack request bit set.
    pub fn encode_quick_ack(&mut self, payload: &[u8]) -> Result<Vec<u8>, Error> {
        let mut packet = self.encode(payload)?;
        packet[0] |= 0x80;
        Ok(packet)
    }
}

impl Default for Abridged {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_PACKET)
    }
}

impl Framing for Abridged {
    fn preamble(&self) -> &'static [u8] {
        &[0xef]
    }
    fn encode(&mut self, payload: &[u8]) -> Result<Vec<u8>, Error> {
        if payload.len() % 4 != 0 {
            return Err(Error::Unaligned(payload.len()));
        }
        checked_length(payload.len(), self.max_packet)?;
        let words = payload.len() / 4;
        let mut out = Vec::with_capacity(payload.len() + 4);
        if words < 127 {
            out.push(u8::try_from(words).expect("word count below 127"));
        } else {
            if words > 0x00ff_ffff {
                return Err(Error::InvalidLength {
                    length: payload.len(),
                    maximum: self.max_packet,
                });
            }
            out.push(0x7f);
            let encoded = u32::try_from(words)
                .expect("24-bit word count")
                .to_le_bytes();
            out.extend_from_slice(&encoded[..3]);
        }
        out.extend_from_slice(payload);
        Ok(out)
    }
    fn decode(&mut self, input: &[u8]) -> Result<Packet, Error> {
        let first = *input.first().ok_or(Error::Incomplete {
            needed: 1,
            available: 0,
        })?;
        let (words, header) = if first < 0x7f {
            (usize::from(first), 1)
        } else if first == 0x7f {
            if input.len() < 4 {
                return Err(Error::Incomplete {
                    needed: 4,
                    available: input.len(),
                });
            }
            (
                usize::from(input[1]) | usize::from(input[2]) << 8 | usize::from(input[3]) << 16,
                4,
            )
        } else {
            return Err(Error::InvalidLength {
                length: 0,
                maximum: self.max_packet,
            });
        };
        let length = words.checked_mul(4).ok_or(Error::InvalidLength {
            length: usize::MAX,
            maximum: self.max_packet,
        })?;
        packet(input, header, length, self.max_packet)
    }
}

/// Intermediate transport (`0xeeeeeeee` preamble and a byte length prefix).
#[derive(Clone, Debug)]
pub struct Intermediate {
    max_packet: usize,
}

/// Padded intermediate transport (`0xdddddddd`) with 0..15 random bytes.
///
/// The decoder returns the complete payload-plus-padding region because this
/// framing layer does not carry the unpadded payload length. The authenticated
/// MTProto envelope determines the body and padding boundary after decryption.
#[derive(Clone, Debug)]
pub struct PaddedIntermediate {
    max_packet: usize,
}

impl PaddedIntermediate {
    /// Create padded intermediate framing with a packet byte limit.
    pub const fn new(max_packet: usize) -> Self {
        Self { max_packet }
    }

    /// Encode with explicit padding for deterministic and official fixtures.
    pub fn encode_with_padding(&self, payload: &[u8], padding: &[u8]) -> Result<Vec<u8>, Error> {
        if padding.len() > 15 {
            return Err(Error::InvalidLength {
                length: padding.len(),
                maximum: 15,
            });
        }
        checked_length(payload.len(), self.max_packet)?;
        let length = payload
            .len()
            .checked_add(padding.len())
            .ok_or(Error::InvalidLength {
                length: usize::MAX,
                maximum: self.max_packet + 15,
            })?;
        if length > self.max_packet + 15 {
            return Err(Error::InvalidLength {
                length,
                maximum: self.max_packet + 15,
            });
        }
        let mut output = Vec::with_capacity(4 + length);
        output.extend_from_slice(
            &u32::try_from(length)
                .map_err(|_| Error::InvalidLength {
                    length,
                    maximum: self.max_packet + 15,
                })?
                .to_le_bytes(),
        );
        output.extend_from_slice(payload);
        output.extend_from_slice(padding);
        Ok(output)
    }

    /// Encode explicit padding with the quick-ack request bit set.
    pub fn encode_quick_ack_with_padding(
        &self,
        payload: &[u8],
        padding: &[u8],
    ) -> Result<Vec<u8>, Error> {
        let mut packet = self.encode_with_padding(payload, padding)?;
        let length = u32::from_le_bytes(packet[..4].try_into().expect("encoded header"));
        packet[..4].copy_from_slice(&(length | 0x8000_0000).to_le_bytes());
        Ok(packet)
    }
}

impl Default for PaddedIntermediate {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_PACKET)
    }
}

impl Framing for PaddedIntermediate {
    fn preamble(&self) -> &'static [u8] {
        &[0xdd; 4]
    }

    fn encode(&mut self, payload: &[u8]) -> Result<Vec<u8>, Error> {
        let mut random = [0_u8; 16];
        getrandom::fill(&mut random).map_err(|error| Error::Connection(error.to_string()))?;
        let padding_length = usize::from(random[0] & 0x0f);
        let padding_end = 1 + padding_length;
        self.encode_with_padding(payload, &random[1..padding_end])
    }

    fn decode(&mut self, input: &[u8]) -> Result<Packet, Error> {
        if input.len() < 4 {
            return Err(Error::Incomplete {
                needed: 4,
                available: input.len(),
            });
        }
        let length = u32::from_le_bytes(input[..4].try_into().expect("four bytes")) as usize;
        packet(input, 4, length, self.max_packet + 15)
    }
}
impl Intermediate {
    /// Create intermediate framing.
    pub const fn new(max_packet: usize) -> Self {
        Self { max_packet }
    }

    /// Encode a packet with the quick-ack request bit set.
    pub fn encode_quick_ack(&mut self, payload: &[u8]) -> Result<Vec<u8>, Error> {
        let mut packet = self.encode(payload)?;
        let length = u32::from_le_bytes(packet[..4].try_into().expect("encoded header"));
        packet[..4].copy_from_slice(&(length | 0x8000_0000).to_le_bytes());
        Ok(packet)
    }
}
impl Default for Intermediate {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_PACKET)
    }
}
impl Framing for Intermediate {
    fn preamble(&self) -> &'static [u8] {
        &[0xee; 4]
    }
    fn encode(&mut self, payload: &[u8]) -> Result<Vec<u8>, Error> {
        checked_length(payload.len(), self.max_packet)?;
        let length = u32::try_from(payload.len()).map_err(|_| Error::InvalidLength {
            length: payload.len(),
            maximum: self.max_packet,
        })?;
        let mut out = Vec::with_capacity(payload.len() + 4);
        out.extend_from_slice(&length.to_le_bytes());
        out.extend_from_slice(payload);
        Ok(out)
    }
    fn decode(&mut self, input: &[u8]) -> Result<Packet, Error> {
        if input.len() < 4 {
            return Err(Error::Incomplete {
                needed: 4,
                available: input.len(),
            });
        }
        let length = u32::from_le_bytes(input[..4].try_into().expect("four bytes")) as usize;
        packet(input, 4, length, self.max_packet)
    }
}

/// Full transport with sequence numbers and CRC32 checksums.
#[derive(Clone, Debug)]
pub struct Full {
    send_sequence: u32,
    receive_sequence: u32,
    max_packet: usize,
}
impl Full {
    /// Create full framing with sequences starting at zero.
    pub const fn new(max_packet: usize) -> Self {
        Self {
            send_sequence: 0,
            receive_sequence: 0,
            max_packet,
        }
    }
}
impl Default for Full {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_PACKET)
    }
}
impl Framing for Full {
    fn preamble(&self) -> &'static [u8] {
        &[]
    }
    fn encode(&mut self, payload: &[u8]) -> Result<Vec<u8>, Error> {
        let total = payload.len().checked_add(12).ok_or(Error::InvalidLength {
            length: usize::MAX,
            maximum: self.max_packet,
        })?;
        checked_length(payload.len(), self.max_packet)?;
        let mut out = Vec::with_capacity(total);
        out.extend_from_slice(
            &u32::try_from(total)
                .map_err(|_| Error::InvalidLength {
                    length: total,
                    maximum: self.max_packet,
                })?
                .to_le_bytes(),
        );
        out.extend_from_slice(&self.send_sequence.to_le_bytes());
        out.extend_from_slice(payload);
        out.extend_from_slice(&crc32fast::hash(&out).to_le_bytes());
        self.send_sequence = self.send_sequence.wrapping_add(1);
        Ok(out)
    }
    fn decode(&mut self, input: &[u8]) -> Result<Packet, Error> {
        if input.len() < 4 {
            return Err(Error::Incomplete {
                needed: 4,
                available: input.len(),
            });
        }
        let total = u32::from_le_bytes(input[..4].try_into().expect("four bytes")) as usize;
        if total < 12 || total - 12 > self.max_packet {
            return Err(Error::InvalidLength {
                length: total,
                maximum: self.max_packet + 12,
            });
        }
        if input.len() < total {
            return Err(Error::Incomplete {
                needed: total,
                available: input.len(),
            });
        }
        let sequence = u32::from_le_bytes(input[4..8].try_into().expect("four bytes"));
        if sequence != self.receive_sequence {
            return Err(Error::Sequence {
                expected: self.receive_sequence,
                actual: sequence,
            });
        }
        let actual = u32::from_le_bytes(input[total - 4..total].try_into().expect("four bytes"));
        let expected = crc32fast::hash(&input[..total - 4]);
        if actual != expected {
            return Err(Error::Checksum { expected, actual });
        }
        self.receive_sequence = self.receive_sequence.wrapping_add(1);
        Ok(Packet {
            payload: input[8..total - 4].to_vec(),
            consumed: total,
        })
    }
}
