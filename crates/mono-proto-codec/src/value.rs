//! Primitive, string, vector, and dependent-value codecs.

use crate::{
    BOOL_FALSE, BOOL_TRUE, Decoder, Encoder, Error, Limits, TlDecode, TlEncode, VECTOR_CONSTRUCTOR,
};

macro_rules! fixed {
    ($type:ty, $write:ident, $read:ident, $size:literal) => {
        impl TlEncode for $type {
            fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
                encoder.write_raw(&self.to_le_bytes());
                Ok(())
            }
        }
        impl TlDecode for $type {
            fn decode(decoder: &mut Decoder<'_>) -> Result<Self, Error> {
                let bytes: [u8; $size] = decoder.read_raw($size)?.try_into().expect("exact length");
                Ok(<$type>::from_le_bytes(bytes))
            }
        }
    };
}

fixed!(i32, write_i32, read_i32, 4);
fixed!(u32, write_u32, read_u32, 4);
fixed!(i64, write_i64, read_i64, 8);
fixed!(u64, write_u64, read_u64, 8);
fixed!(f64, write_f64, read_f64, 8);

macro_rules! fixed_bigint {
    ($type:ty, $size:literal) => {
        impl TlEncode for $type {
            fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
                // bnum 0.14 stores little-endian bytes; the 0.13 `to_bits().digits()` API is gone.
                encoder.write_raw(&self.to_le_bytes());
                Ok(())
            }
        }
        impl TlDecode for $type {
            fn decode(decoder: &mut Decoder<'_>) -> Result<Self, Error> {
                Ok(<$type>::from_le_slice(decoder.read_raw($size)?)
                    .expect("Exact fixed-width integer length"))
            }
        }
    };
}
fixed_bigint!(bnum::types::I128, 16);
fixed_bigint!(bnum::types::I256, 32);
fixed_bigint!(bnum::types::I512, 64);

impl TlEncode for bool {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        encoder.write_u32(if *self { BOOL_TRUE } else { BOOL_FALSE });
        Ok(())
    }
}

impl TlDecode for bool {
    fn decode(decoder: &mut Decoder<'_>) -> Result<Self, Error> {
        let offset = decoder.offset();
        match decoder.read_u32()? {
            BOOL_TRUE => Ok(true),
            BOOL_FALSE => Ok(false),
            constructor => Err(Error::UnknownConstructor {
                constructor,
                offset,
            }),
        }
    }
}

impl TlEncode for [u8; 16] {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        encoder.write_raw(self);
        Ok(())
    }
}
impl TlDecode for [u8; 16] {
    fn decode(decoder: &mut Decoder<'_>) -> Result<Self, Error> {
        Ok(decoder.read_raw(16)?.try_into().expect("exact length"))
    }
}
impl TlEncode for [u8; 32] {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        encoder.write_raw(self);
        Ok(())
    }
}
impl TlDecode for [u8; 32] {
    fn decode(decoder: &mut Decoder<'_>) -> Result<Self, Error> {
        Ok(decoder.read_raw(32)?.try_into().expect("exact length"))
    }
}

impl TlEncode for str {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        encoder.write_bytes(self.as_bytes())
    }
}
impl TlEncode for String {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        self.as_str().encode(encoder)
    }
}
impl TlDecode for String {
    fn decode(decoder: &mut Decoder<'_>) -> Result<Self, Error> {
        let offset = decoder.offset();
        String::from_utf8(decoder.read_bytes()?).map_err(|_| Error::InvalidUtf8 { offset })
    }
}

/// Encode a TL `bytes` value (distinct from `Vector<int>`).
pub fn encode_bytes(value: &[u8], encoder: &mut Encoder) -> Result<(), Error> {
    encoder.write_bytes(value)
}

/// Decode a TL `bytes` value.
pub fn decode_bytes(decoder: &mut Decoder<'_>) -> Result<Vec<u8>, Error> {
    decoder.read_bytes()
}

/// Encode a generic vector, optionally including the boxed vector constructor.
pub fn encode_vector<T: TlEncode>(
    values: &[T],
    encoder: &mut Encoder,
    boxed: bool,
) -> Result<(), Error> {
    if boxed {
        encoder.write_u32(VECTOR_CONSTRUCTOR);
    }
    encoder.write_u32(
        u32::try_from(values.len()).map_err(|_| Error::LimitExceeded {
            kind: "vector",
            actual: values.len(),
            limit: u32::MAX as usize,
        })?,
    );
    for value in values {
        value.encode(encoder)?;
    }
    Ok(())
}

/// Decode a generic vector, optionally requiring the boxed vector constructor.
pub fn decode_vector<T: TlDecode>(decoder: &mut Decoder<'_>, boxed: bool) -> Result<Vec<T>, Error> {
    if boxed {
        let actual = decoder.read_u32()?;
        if actual != VECTOR_CONSTRUCTOR {
            return Err(Error::ConstructorMismatch {
                expected: VECTOR_CONSTRUCTOR,
                actual,
            });
        }
    }
    let count = decoder.read_vector_len()?;
    decoder.nested(|decoder| (0..count).map(|_| T::decode(decoder)).collect())
}

impl TlEncode for Vec<u8> {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        encode_bytes(self, encoder)
    }
}

impl TlDecode for Vec<u8> {
    fn decode(decoder: &mut Decoder<'_>) -> Result<Self, Error> {
        decode_bytes(decoder)
    }
}

impl<T: TlEncode, const N: usize> TlEncode for [T; N] {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        for value in self {
            value.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T: TlDecode, const N: usize> TlDecode for [T; N] {
    fn decode(decoder: &mut Decoder<'_>) -> Result<Self, Error> {
        let values = (0..N)
            .map(|_| T::decode(decoder))
            .collect::<Result<Vec<_>, _>>()?;
        values.try_into().map_err(|_| Error::DependentLength {
            declared: N,
            actual: N,
        })
    }
}

/// Return whether a conditional field controlled by `flags.bit` is present.
pub fn flag_is_set(flags: u32, bit: u32) -> bool {
    bit < 32 && flags & (1_u32 << bit) != 0
}

/// Validate a natural-number/dependent field before encoding it.
pub fn validate_dependent_len(declared: usize, actual: usize) -> Result<(), Error> {
    if declared == actual {
        Ok(())
    } else {
        Err(Error::DependentLength { declared, actual })
    }
}

/// Encode one complete bare value.
pub fn to_bytes<T: TlEncode + ?Sized>(value: &T) -> Result<Vec<u8>, Error> {
    let mut encoder = Encoder::new();
    value.encode(&mut encoder)?;
    Ok(encoder.into_bytes())
}

/// Decode one complete bare value, rejecting trailing input.
pub fn from_bytes<T: TlDecode>(input: &[u8], limits: Limits) -> Result<T, Error> {
    let mut decoder = Decoder::new(input, limits)?;
    let value = T::decode(&mut decoder)?;
    decoder.finish()?;
    Ok(value)
}
