//! Codec unit and property regressions.

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn byte_strings_round_trip(value in proptest::collection::vec(any::<u8>(), 0..1000)) {
        let mut encoder = Encoder::new();
        encoder.write_bytes(&value).unwrap();
        let bytes = encoder.into_bytes();
        prop_assert_eq!(bytes.len() % 4, 0);
        let mut decoder = Decoder::new(&bytes, Limits::default()).unwrap();
        prop_assert_eq!(decoder.read_bytes().unwrap(), value);
        decoder.finish().unwrap();
    }

    #[test]
    fn arbitrary_input_never_panics_or_exceeds_bounds(input in proptest::collection::vec(any::<u8>(), 0..4096)) {
        use crate::transport::Pong;
        let limits = Limits { max_bytes: 1024, max_vector_len: 32, max_depth: 8, max_total_bytes: 4096 };
        if let Ok(mut decoder) = Decoder::new(&input, limits) {
            let _ = Pong::decode(&mut decoder);
        }
    }
}

#[test]
fn fixed_width_big_integers_preserve_twos_complement_bytes() {
    use bnum::types::{I128, I256, I512};

    fn round_trip<T>(value: &T, expected_len: usize)
    where
        T: TlEncode + TlDecode + std::fmt::Debug + PartialEq,
    {
        let mut encoder = Encoder::new();
        value.encode(&mut encoder).unwrap();
        let wire = encoder.into_bytes();
        assert_eq!(wire.len(), expected_len, "Fixed integer width changed");
        let mut decoder = Decoder::new(&wire, Limits::default()).unwrap();
        assert_eq!(&T::decode(&mut decoder).unwrap(), value);
        decoder.finish().unwrap();
    }

    round_trip(&I128::try_from(-1_i8).unwrap(), 16);
    round_trip(&I256::MIN, 32);
    round_trip(&I512::MAX, 64);
}

#[test]
fn generated_constructor_dispatch_round_trips() {
    use crate::transport::{Pong, PongConstructor};

    let value = Pong::Pong(PongConstructor {
        msg_id: 7,
        ping_id: 42,
    });
    let mut encoder = Encoder::new();
    value.encode(&mut encoder).unwrap();
    let wire = encoder.into_bytes();
    assert_eq!(&wire[..4], &PongConstructor::ID.to_le_bytes());

    let mut decoder = Decoder::new(&wire, Limits::default()).unwrap();
    assert_eq!(Pong::decode(&mut decoder).unwrap(), value);
    decoder.finish().unwrap();
}

#[test]
fn generated_repetitions_validate_limits_before_allocation() {
    use crate::transport::VectorConstructor;

    let wire = 10_u32.to_le_bytes();
    let mut decoder = Decoder::new(
        &wire,
        Limits {
            max_vector_len: 2,
            ..Limits::default()
        },
    )
    .unwrap();
    assert!(matches!(
        VectorConstructor::<i32>::decode(&mut decoder),
        Err(Error::LimitExceeded {
            kind: "vector",
            actual: 10,
            limit: 2
        })
    ));
}

#[cfg(feature = "layer-232")]
#[test]
fn generated_flags_reject_inconsistent_presence() {
    use crate::latest::api::InputMessagesFilterPhoneCallsConstructor;

    let value = InputMessagesFilterPhoneCallsConstructor {
        flags: InputMessagesFilterPhoneCallsConstructor::MISSED_FLAG,
        missed: None,
    };
    let mut encoder = Encoder::new();
    assert!(matches!(
        value.encode(&mut encoder),
        Err(Error::InconsistentFlag {
            field: "missed",
            ..
        })
    ));
}

#[test]
fn rejects_nonzero_padding_and_reserved_marker() {
    let mut decoder = Decoder::new(&[1, b'x', 1, 0], Limits::default()).unwrap();
    assert!(matches!(
        decoder.read_bytes(),
        Err(Error::InvalidPadding { .. })
    ));
    let mut decoder = Decoder::new(&[255, 0, 0, 0], Limits::default()).unwrap();
    assert!(matches!(
        decoder.read_bytes(),
        Err(Error::InvalidLengthMarker { .. })
    ));
    let mut decoder = Decoder::new(&[254, 1, 0, 0, b'x', 0, 0, 0], Limits::default()).unwrap();
    assert!(matches!(
        decoder.read_bytes(),
        Err(Error::NonCanonicalLength { length: 1, .. })
    ));
}

#[test]
fn primitive_vector_round_trip() {
    let mut encoder = Encoder::new();
    encode_vector(&[1_i64, 2, 3], &mut encoder, true).unwrap();
    let bytes = encoder.into_bytes();
    let mut decoder = Decoder::new(&bytes, Limits::default()).unwrap();
    assert_eq!(decode_vector::<i64>(&mut decoder, true).unwrap(), [1, 2, 3]);
}
