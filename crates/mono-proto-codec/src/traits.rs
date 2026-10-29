//! Schema-neutral bare and boxed codec traits.

use crate::{Decoder, Encoder, Error};

/// A value that can be emitted in bare TL form.
pub trait TlEncode {
    /// Append the bare representation to `encoder`.
    fn encode(&self, encoder: &mut Encoder) -> Result<(), Error>;
}

/// A value that can be decoded from bare TL form.
pub trait TlDecode: Sized {
    /// Read one bare value.
    fn decode(decoder: &mut Decoder<'_>) -> Result<Self, Error>;
}

/// Constructor metadata for a boxed TL value.
pub trait Boxed: TlEncode {
    /// Constructor number written before the bare body.
    const CONSTRUCTOR_ID: u32;

    /// Encode constructor number followed by the bare body.
    fn encode_boxed(&self, encoder: &mut Encoder) -> Result<(), Error> {
        encoder.write_u32(Self::CONSTRUCTOR_ID);
        self.encode(encoder)
    }
}

/// Encode a constructor body without its constructor number.
pub trait BareEncode {
    /// Append only the fields belonging to this constructor.
    fn encode_bare(&self, encoder: &mut Encoder) -> Result<(), Error>;
}

/// Decode a constructor body whose type has an unambiguous bare constructor.
pub trait BareDecode: Sized {
    /// Read only the fields belonging to the constructor.
    fn decode_bare(decoder: &mut Decoder<'_>) -> Result<Self, Error>;
}

/// Decode a polymorphic boxed TL result without a hardcoded global table.
pub trait BoxedDecode: Sized {
    /// Dispatch a constructor number using the generated type's variants.
    fn decode_boxed(decoder: &mut Decoder<'_>) -> Result<Self, Error>;
}

/// Metadata and wire behavior for a generated Telegram RPC method.
pub trait RpcMethod: TlEncode {
    /// Decoded method result.
    type Response: TlDecode;
    /// Original namespace-qualified TL method name.
    const METHOD_NAME: &'static str;
    /// Result expression from the source schema.
    const RESULT_TYPE: &'static str;
}

impl<T: TlEncode + ?Sized> TlEncode for Box<T> {
    fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        (**self).encode(encoder)
    }
}

impl<T: TlDecode> TlDecode for Box<T> {
    fn decode(decoder: &mut Decoder<'_>) -> Result<Self, Error> {
        T::decode(decoder).map(Box::new)
    }
}

impl<T: BareEncode + ?Sized> BareEncode for Box<T> {
    fn encode_bare(&self, encoder: &mut Encoder) -> Result<(), Error> {
        (**self).encode_bare(encoder)
    }
}

impl<T: BareDecode> BareDecode for Box<T> {
    fn decode_bare(decoder: &mut Decoder<'_>) -> Result<Self, Error> {
        T::decode_bare(decoder).map(Box::new)
    }
}
