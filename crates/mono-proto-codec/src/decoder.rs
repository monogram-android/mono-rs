//! Bounded cursor and canonical TL byte decoding.

use crate::{Error, Limits};
use std::fmt;

/// Cursor over untrusted TL input with allocation and recursion bounds.
pub struct Decoder<'a> {
    input: &'a [u8],
    offset: usize,
    depth: usize,
    limits: Limits,
}

impl fmt::Debug for Decoder<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Decoder")
            .field("offset", &self.offset)
            .field("remaining", &self.remaining())
            .field("depth", &self.depth)
            .field("limits", &self.limits)
            .finish()
    }
}

impl<'a> Decoder<'a> {
    /// Create a decoder, rejecting an input larger than the configured total bound.
    pub fn new(input: &'a [u8], limits: Limits) -> Result<Self, Error> {
        if input.len() > limits.max_total_bytes {
            return Err(Error::LimitExceeded {
                kind: "input",
                actual: input.len(),
                limit: limits.max_total_bytes,
            });
        }
        Ok(Self {
            input,
            offset: 0,
            depth: 0,
            limits,
        })
    }

    /// Current absolute byte offset.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Bytes not yet consumed.
    pub fn remaining(&self) -> usize {
        self.input.len() - self.offset
    }

    /// Require complete consumption of the input.
    pub fn finish(self) -> Result<(), Error> {
        if self.remaining() == 0 {
            Ok(())
        } else {
            Err(Error::TrailingBytes {
                remaining: self.remaining(),
            })
        }
    }

    /// Read exactly `length` uninterpreted bytes.
    pub fn read_raw(&mut self, length: usize) -> Result<&'a [u8], Error> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(Error::LimitExceeded {
                kind: "offset",
                actual: usize::MAX,
                limit: self.input.len(),
            })?;
        let result = self
            .input
            .get(self.offset..end)
            .ok_or(Error::UnexpectedEof {
                offset: self.offset,
                needed: length,
                remaining: self.remaining(),
            })?;
        self.offset = end;
        Ok(result)
    }

    /// Read a little-endian word.
    pub fn read_u32(&mut self) -> Result<u32, Error> {
        let bytes: [u8; 4] = self.read_raw(4)?.try_into().expect("exact length");
        Ok(u32::from_le_bytes(bytes))
    }

    /// Read canonical TL bytes and validate zero alignment padding.
    pub fn read_bytes(&mut self) -> Result<Vec<u8>, Error> {
        let marker_offset = self.offset;
        let marker = self.read_raw(1)?[0];
        let (length, prefix) = match marker {
            0..=253 => (usize::from(marker), 1),
            254 => {
                let raw = self.read_raw(3)?;
                let length =
                    usize::from(raw[0]) | (usize::from(raw[1]) << 8) | (usize::from(raw[2]) << 16);
                if length < 254 {
                    return Err(Error::NonCanonicalLength {
                        offset: marker_offset,
                        length,
                    });
                }
                (length, 4)
            }
            255 => {
                return Err(Error::InvalidLengthMarker {
                    offset: marker_offset,
                });
            }
        };
        if length > self.limits.max_bytes {
            return Err(Error::LimitExceeded {
                kind: "bytes",
                actual: length,
                limit: self.limits.max_bytes,
            });
        }
        let payload = self.read_raw(length)?.to_vec();
        let padding = (4 - ((prefix + length) % 4)) % 4;
        let padding_offset = self.offset;
        if let Some(index) = self.read_raw(padding)?.iter().position(|byte| *byte != 0) {
            return Err(Error::InvalidPadding {
                offset: padding_offset + index,
            });
        }
        Ok(payload)
    }

    /// Decode a nested value while enforcing the recursion limit.
    pub fn nested<T>(
        &mut self,
        decode: impl FnOnce(&mut Self) -> Result<T, Error>,
    ) -> Result<T, Error> {
        if self.depth >= self.limits.max_depth {
            return Err(Error::DepthExceeded {
                limit: self.limits.max_depth,
            });
        }
        self.depth += 1;
        let result = decode(self);
        self.depth -= 1;
        result
    }

    /// Read a vector element count and validate it before allocation.
    pub fn read_vector_len(&mut self) -> Result<usize, Error> {
        let actual = usize::try_from(self.read_u32()?).unwrap_or(usize::MAX);
        self.validate_vector_len(actual)?;
        Ok(actual)
    }

    /// Validate a schema-dependent repetition count before allocation.
    pub fn validate_vector_len(&self, actual: usize) -> Result<(), Error> {
        if actual > self.limits.max_vector_len {
            Err(Error::LimitExceeded {
                kind: "vector",
                actual,
                limit: self.limits.max_vector_len,
            })
        } else {
            Ok(())
        }
    }
}
