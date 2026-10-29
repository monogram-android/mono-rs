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
