//! Shared bounded TL codec used by the generated declarations.

pub use mono_proto_codec::*;

#[cfg(test)]
#[path = "../../tests/unit/codec_tests.rs"]
mod tests;
