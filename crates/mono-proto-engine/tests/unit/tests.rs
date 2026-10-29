//! Engine correlation, retry, envelope, and wire regressions.

use super::*;
use crate::wire::VECTOR_ID;
use mono_proto_session::{
    Clock, Error as SessionError, RandomSource, ReceivedMessageResult, Snapshot,
};
use std::io::Write;
struct C(i64);

#[test]
fn authorization_corrects_dh_clock_before_next_plain_and_new_key_messages() {
    use crate::authorization::{Authorization, AuthorizationAdapter};
    struct Adapter(bool);
    impl AuthorizationAdapter for Adapter {
        fn request_parameters(&mut self) -> Result<Vec<u8>, Error> {
            Ok(vec![])
        }
        fn accept_parameters(&mut self, _: &[u8]) -> Result<Vec<u8>, Error> {
            Ok(vec![])
        }
        fn accept_dh(&mut self, _: &[u8]) -> Result<Vec<u8>, Error> {
            self.0 = true;
            Ok(vec![])
        }
        fn accept_authorization(&mut self, _: &[u8]) -> Result<(Vec<u8>, i64), Error> {
            Ok((vec![1; 256], 7))
        }
        fn time_offset_micros(&self) -> Option<i64> {
            self.0.then_some(-100_000_000_000_000)
        }
    }
    let clock = C(1_800_000_000_000_000);
    let mut session = Snapshot::new(2, &mut R).unwrap();
    let old = session.next_message_id(&clock).unwrap();
    let mut exchange = Authorization::new(Adapter(false));
    exchange.start().unwrap();
    exchange.advance(&[], &mut session).unwrap();
    assert_eq!(session.time_offset_micros, 0);
    exchange.advance(&[], &mut session).unwrap();
    assert!(session.auth_key.is_none());
    assert_eq!(session.time_offset_micros, -100_000_000_000_000);
    let corrected = session.next_message_id(&clock).unwrap();
    assert_eq!(corrected >> 32, 1_700_000_000);
    assert!(corrected < old);
    exchange.advance(&[], &mut session).unwrap();
    assert_eq!(session.auth_key.as_deref(), Some(&[1_u8; 256][..]));
    assert_eq!(session.server_salt, 7);
    assert_eq!(
        session.next_message_id(&clock).unwrap() >> 32,
        1_700_000_000
    );
    assert!(exchange.into_adapter().0);
}
impl Clock for C {
    fn unix_micros(&self) -> i64 {
        self.0
    }
}
struct R;
impl RandomSource for R {
    fn fill(&mut self, out: &mut [u8]) -> Result<(), SessionError> {
        out.fill(1);
        Ok(())
    }
}
struct Ping;
impl Method for Ping {
    type Response = Vec<u8>;
    fn encode_request(&self) -> Result<Vec<u8>, mono_proto_codec::Error> {
        Ok(vec![1, 2, 3, 4])
    }
    fn decode_response(bytes: &[u8]) -> Result<Self::Response, mono_proto_codec::Error> {
        Ok(bytes.to_vec())
    }
}
#[test]
fn correlates_typed_response() {
    let session = Snapshot::new(2, &mut R).unwrap();
    let clock = C(1_700_000_000_000_000);
    let mut engine = Engine::new(
        session,
        ExponentialBackoff {
            timeout_micros: 10,
            initial_delay_micros: 1,
            max_attempts: 2,
        },
    )
    .unwrap();
    let handle = engine.invoke(&Ping, &clock).unwrap();
    engine.next_outbound().unwrap();
    engine
        .receive_result(handle.message_id(), vec![9], 5)
        .unwrap();
    assert_eq!(
        engine.take_response::<Ping>(&handle).unwrap(),
        Some(vec![9])
    );
}
#[test]
fn typed_handle_survives_wire_retry() {
    let session = Snapshot::new(2, &mut R).unwrap();
    let initial = C(1_700_000_000_000_000);
    let mut engine = Engine::new(
        session,
        ExponentialBackoff {
            timeout_micros: 10,
            initial_delay_micros: 1,
            max_attempts: 2,
        },
    )
    .unwrap();
    let handle = engine.invoke(&Ping, &initial).unwrap();
    let first = engine.next_outbound().unwrap();
    assert!(engine.poll(&C(initial.0 + 10)).unwrap().is_empty());
    assert!(engine.poll(&C(initial.0 + 11)).unwrap().is_empty());
    let retry = engine.next_outbound().unwrap();
    assert_ne!(retry.message_id, first.message_id);
    engine.receive_result(retry.message_id, vec![8], 5).unwrap();
    assert_eq!(
        engine.take_response::<Ping>(&handle).unwrap(),
        Some(vec![8])
    );
}
#[test]
fn extracts_rpc_advice() {
    assert_eq!(
        RpcError::new(420, "FLOOD_WAIT_17").flood_wait_seconds,
        Some(17)
    );
    assert_eq!(RpcError::new(303, "PHONE_MIGRATE_4").migrate_to_dc, Some(4));
}

#[test]
fn raw_container_and_acknowledgements_match_wire_layout() {
    let messages = [
        ContainerMessage {
            message_id: 4,
            sequence: 1,
            body: vec![1, 2, 3, 4],
        },
        ContainerMessage {
            message_id: 8,
            sequence: 3,
            body: vec![5, 6, 7, 8],
        },
    ];
    let encoded = encode_container(&messages).unwrap();
    assert_eq!(decode_container(&encoded, 1024, 1024).unwrap(), messages);
    validate_container_for_outer(&messages, 12).unwrap();
    assert!(validate_container_for_outer(&messages, 8).is_err());
    let empty = encode_container(&[]).unwrap();
    assert_eq!(
        decode_container(&empty, 1024, 1024).unwrap(),
        Vec::<ContainerMessage>::new()
    );

    let acknowledgements = encode_acknowledgements(&[5, 9]).unwrap();
    assert_eq!(&acknowledgements[..4], &MSGS_ACK_ID.to_le_bytes());
    assert_eq!(&acknowledgements[4..8], &VECTOR_ID.to_le_bytes());
    assert_eq!(&acknowledgements[8..12], &2_u32.to_le_bytes());
}

#[test]
fn gzip_packed_is_bounded_and_dispatch_ready() {
    let object = [0x78, 0x56, 0x34, 0x12];
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
    encoder.write_all(&object).unwrap();
    let packed = encoder.finish().unwrap();
    let mut tl = mono_proto_codec::Encoder::new();
    tl.write_u32(GZIP_PACKED_ID);
    tl.write_bytes(&packed).unwrap();
    let wire = tl.into_bytes();

    assert_eq!(unpack_gzip_packed(&wire, object.len()).unwrap(), object);
    assert!(matches!(
        unpack_gzip_packed(&wire, object.len() - 1),
        Err(Error::MalformedMessage(
            "gzip_packed output exceeds configured limit"
        ))
    ));
}

#[test]
fn inbound_envelope_is_authenticated_and_replay_registered() {
    let clock = C(1_700_000_000_000_000);
    let mut session = Snapshot::new(2, &mut R).unwrap();
    session.set_auth_key((0_u8..=255).collect()).unwrap();
    let packet = mono_proto_crypto::encrypt_message(
        session.auth_key.as_deref().unwrap(),
        mono_proto_crypto::MessageToEncrypt {
            server_salt: 9,
            session_id: session.session_id,
            message_id: (1_700_000_000_i64 << 32) | 1,
            sequence: 1,
            body: &[1, 2, 3, 4],
            padding: &[7; 12],
        },
        mono_proto_crypto::Direction::ServerToClient,
    )
    .unwrap();
    let mut engine = Engine::new(
        session,
        ExponentialBackoff {
            timeout_micros: 10,
            initial_delay_micros: 1,
            max_attempts: 2,
        },
    )
    .unwrap();
    assert_eq!(
        engine
            .open_inbound(&packet, &clock, true, 1024)
            .unwrap()
            .disposition,
        ReceivedMessageResult::Accepted
    );
    assert_eq!(
        engine
            .open_inbound(&packet, &clock, true, 1024)
            .unwrap()
            .disposition,
        ReceivedMessageResult::Duplicate
    );
}
