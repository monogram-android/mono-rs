//! Framing, control, HTTP, and property regressions.

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn word_framings_round_trip_arbitrary_payload(words in proptest::collection::vec(any::<u32>(), 1..256)) {
        let payload: Vec<u8> = words.into_iter().flat_map(u32::to_le_bytes).collect();
        for framing in [
            &mut Abridged::default() as &mut dyn Framing,
            &mut Intermediate::default(),
            &mut Full::default(),
        ] {
            let encoded = framing.encode(&payload).unwrap();
            let decoded = framing.decode(&encoded).unwrap();
            prop_assert_eq!(decoded.payload.as_slice(), payload.as_slice());
        }
    }
}

#[test]
fn framing_round_trips() {
    let payload = [7_u8; 508];
    for framing in [
        &mut Abridged::default() as &mut dyn Framing,
        &mut Intermediate::default(),
        &mut Full::default(),
    ] {
        let encoded = framing.encode(&payload).unwrap();
        let decoded = framing.decode(&encoded).unwrap();
        assert_eq!(decoded.payload, payload);
        assert_eq!(decoded.consumed, encoded.len());
    }
}
#[test]
fn full_rejects_corruption() {
    let mut framing = Full::default();
    let mut encoded = framing.encode(&[1, 2, 3, 4]).unwrap();
    encoded[8] ^= 1;
    assert!(matches!(
        framing.decode(&encoded),
        Err(Error::Checksum { .. })
    ));
}
#[test]
fn parses_http_response() {
    let response = b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\n\r\ntesttail";
    assert_eq!(
        http_response(response, 16).unwrap(),
        Packet {
            payload: b"test".to_vec(),
            consumed: 42
        }
    );
}

#[test]
fn quick_ack_and_transport_controls_are_distinct_from_payloads() {
    let mut abridged = Abridged::default();
    let encoded = abridged.encode_quick_ack(&[1, 2, 3, 4]).unwrap();
    assert_eq!(encoded[0], 0x81);
    assert_eq!(
        abridged_control(&[0x80, 0x12, 0x34, 0x56]).unwrap(),
        Some((Control::QuickAck(0x8012_3456), 4))
    );

    let mut intermediate = Intermediate::default();
    let encoded = intermediate.encode_quick_ack(&[1, 2, 3, 4]).unwrap();
    assert_eq!(
        u32::from_le_bytes(encoded[..4].try_into().unwrap()),
        0x8000_0004
    );
    assert_eq!(
        payload_control(&(-429_i32).to_le_bytes()),
        Some(Control::TransportError(-429))
    );
    let mut padded_ack = vec![0xff; 4];
    padded_ack.extend_from_slice(&7_u32.to_le_bytes());
    assert_eq!(payload_control(&padded_ack), Some(Control::QuickAck(7)));
}
