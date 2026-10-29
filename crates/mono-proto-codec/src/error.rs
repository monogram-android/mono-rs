//! Codec limits, constants, and inspectable failures.

use thiserror::Error;

/// The universal boxed vector constructor.
pub const VECTOR_CONSTRUCTOR: u32 = 0x1cb5_c415;
/// Boxed TL `boolFalse`.
pub const BOOL_FALSE: u32 = 0xbc79_9737;
/// Boxed TL `boolTrue`.
pub const BOOL_TRUE: u32 = 0x9972_75b5;

/// Defensive limits applied before allocating or descending into input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    /// Largest permitted byte string.
    pub max_bytes: usize,
    /// Largest permitted vector element count.
    pub max_vector_len: usize,
    /// Maximum recursive nesting depth.
    pub max_depth: usize,
    /// Maximum total bytes consumed by one decoder.
    pub max_total_bytes: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_bytes: 16 * 1024 * 1024,
            max_vector_len: 1_000_000,
            max_depth: 64,
            max_total_bytes: 64 * 1024 * 1024,
        }
    }
}

/// A contextual TL codec failure.
#[derive(Debug, Error, Clone, Eq, PartialEq)]
pub enum Error {
    /// The input ended before a value was complete.
    #[error("Unexpected end of input at byte {offset}: needed {needed}, remaining {remaining}")]
    UnexpectedEof {
        /// Byte offset where the read was attempted.
        offset: usize,
        /// Number of requested bytes.
        needed: usize,
        /// Number of available bytes.
        remaining: usize,
    },
    /// A decoded length exceeded a configured bound.
    #[error("Length limit exceeded for {kind}: {actual} exceeds {limit}")]
    LimitExceeded {
        /// Kind of bounded value.
        kind: &'static str,
        /// Length found on the wire.
        actual: usize,
        /// Configured maximum.
        limit: usize,
    },
    /// A boxed value had an unsupported constructor number.
    #[error("Unknown constructor 0x{constructor:08x} at byte {offset}")]
    UnknownConstructor {
        /// Constructor number read from the wire.
        constructor: u32,
        /// Offset of the constructor.
        offset: usize,
    },
    /// A known constructor was expected but another was present.
    #[error("Constructor mismatch: expected 0x{expected:08x}, got 0x{actual:08x}")]
    ConstructorMismatch {
        /// Expected constructor.
        expected: u32,
        /// Actual constructor.
        actual: u32,
    },
    /// TL byte/string padding contained nonzero data.
    #[error("Nonzero TL padding at byte {offset}")]
    InvalidPadding {
        /// Offset of the first invalid padding byte.
        offset: usize,
    },
    /// The reserved byte-string marker 255 was encountered.
    #[error("Invalid TL byte-string length marker 255 at byte {offset}")]
    InvalidLengthMarker {
        /// Offset of the marker.
        offset: usize,
    },
    /// The four-byte representation was used for a length that has a shorter encoding.
    #[error("Non-canonical TL byte-string length {length} at byte {offset}")]
    NonCanonicalLength {
        /// Offset of the marker.
        offset: usize,
        /// Redundantly encoded length.
        length: usize,
    },
    /// UTF-8 validation failed for a TL string.
    #[error("Invalid UTF-8 string at byte {offset}")]
    InvalidUtf8 {
        /// Start of the string payload.
        offset: usize,
    },
    /// Input contains bytes after the requested top-level value.
    #[error("Trailing bytes after TL value: {remaining}")]
    TrailingBytes {
        /// Remaining byte count.
        remaining: usize,
    },
    /// A schema-dependent natural number did not match the actual value.
    #[error("Dependent length mismatch: declared {declared}, actual {actual}")]
    DependentLength {
        /// Length encoded in the dependent field.
        declared: usize,
        /// Actual element count.
        actual: usize,
    },
    /// A conditional field disagreed with the controlling flags word.
    #[error("Field `{field}` presence disagrees with `{flags}` bit {bit}")]
    InconsistentFlag {
        /// Generated Rust field name.
        field: &'static str,
        /// Controlling flags field name.
        flags: &'static str,
        /// Controlling bit number.
        bit: u32,
    },
    /// A bare type has more than one possible constructor.
    #[error("TL type `{type_name}` cannot be decoded bare because its constructor is ambiguous")]
    AmbiguousBareConstructor {
        /// Original TL result type.
        type_name: &'static str,
    },
    /// A natural-number expression cannot be represented by the generated field type.
    #[error("Dependent natural value {value} cannot be represented")]
    InvalidNatural {
        /// Value found or calculated.
        value: u64,
    },
    /// Nesting exceeded the decoder's recursion limit.
    #[error("TL nesting depth exceeds limit {limit}")]
    DepthExceeded {
        /// Configured limit.
        limit: usize,
    },
}
