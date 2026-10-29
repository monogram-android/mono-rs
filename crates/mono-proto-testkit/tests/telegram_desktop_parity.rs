//! Behavioral parity checks derived from Telegram Desktop's MTProto implementation.
//!
//! Audited revision: telegramdesktop/tdesktop
//! `8e18cb71103d83d7d98994ff27f0a2bca55c489c`.

use mono_proto_crypto::{parse_rsa_public_key, rsa_public_key_fingerprint};
use mono_proto_session::{
    Clock, RECEIVED_MESSAGE_WINDOW, RandomSource, ReceivedMessageResult, Snapshot,
};
use mono_proto_transport::{Framing, ObfuscatedProtocol, ObfuscatedStream, PaddedIntermediate};

struct FixedClock(i64);
impl Clock for FixedClock {
    fn unix_micros(&self) -> i64 {
        self.0
    }
}

struct ZeroRandom;
impl RandomSource for ZeroRandom {
    fn fill(&mut self, output: &mut [u8]) -> Result<(), mono_proto_session::Error> {
        output.fill(0);
        Ok(())
    }
}

#[test]
fn received_id_window_matches_desktop_manager() {
    let seconds = 1_700_000_000_i64;
    let clock = FixedClock(seconds * 1_000_000);
    let mut session = Snapshot::new(2, &mut ZeroRandom).unwrap();
    for index in 0..=RECEIVED_MESSAGE_WINDOW {
        let message_id = (seconds << 32) | (i64::try_from(index).unwrap() * 4 + 1);
        assert_eq!(
            session
                .register_received_message(message_id, index % 2 == 0, &clock, true)
                .unwrap(),
            ReceivedMessageResult::Accepted
        );
    }
    assert_eq!(session.received_message_ids.len(), RECEIVED_MESSAGE_WINDOW);
    let evicted = (seconds << 32) | 1;
    assert_eq!(
        session
            .register_received_message(evicted, false, &clock, true)
            .unwrap(),
        ReceivedMessageResult::TooOld
    );
    let newest = (seconds << 32) | (i64::try_from(RECEIVED_MESSAGE_WINDOW).unwrap() * 4 + 1);
    assert_eq!(
        session
            .register_received_message(newest, true, &clock, true)
            .unwrap(),
        ReceivedMessageResult::Duplicate
    );
}

#[test]
fn production_rsa_key_fingerprint_matches_desktop() {
    let key = parse_rsa_public_key(
        "-----BEGIN RSA PUBLIC KEY-----\n\
         MIIBCgKCAQEA6LszBcC1LGzyr992NzE0ieY+BSaOW622Aa9Bd4ZHLl+TuFQ4lo4g\n\
         5nKaMBwK/BIb9xUfg0Q29/2mgIR6Zr9krM7HjuIcCzFvDtr+L0GQjae9H0pRB2OO\n\
         62cECs5HKhT5DZ98K33vmWiLowc621dQuwKWSQKjWf50XYFw42h21P2KXUGyp2y/\n\
         +aEyZ+uVgLLQbRA1dEjSDZ2iGRy12Mk5gpYc397aYp438fsJoHIgJ2lgMv5h7WY9\n\
         t6N/byY9Nw9p21Og3AoXSL2q/2IJ1WRUhebgAdGVMlV1fkuOQoEzR7EdpqtQD9Cs\n\
         5+bfo3Nhmcyvk5ftB0WkJ9z6bNZ7yxrP8wIDAQAB\n\
         -----END RSA PUBLIC KEY-----",
    )
    .unwrap();
    assert_eq!(rsa_public_key_fingerprint(&key), -3_414_540_481_677_951_611);
}

#[test]
fn padded_intermediate_matches_desktop_layout() {
    let mut framing = PaddedIntermediate::default();
    let encoded = framing
        .encode_with_padding(&[1, 2, 3, 4], &[9, 8, 7])
        .unwrap();
    assert_eq!(&encoded[..4], &7_u32.to_le_bytes());
    assert_eq!(&encoded[4..], &[1, 2, 3, 4, 9, 8, 7]);
    assert_eq!(framing.decode(&encoded).unwrap().payload, encoded[4..]);
}

#[test]
fn obfuscated_ctr_state_is_continuous_across_writes() {
    let mut nonce = [0_u8; 64];
    for (index, byte) in nonce.iter_mut().enumerate() {
        *byte = u8::try_from(index + 1).unwrap();
    }
    let mut split = ObfuscatedStream::new_client(
        nonce,
        ObfuscatedProtocol::PaddedIntermediate,
        Some(2),
        Some(&[0x99; 16]),
    )
    .unwrap();
    let mut joined = ObfuscatedStream::new_client(
        nonce,
        ObfuscatedProtocol::PaddedIntermediate,
        Some(2),
        Some(&[0x99; 16]),
    )
    .unwrap();
    assert_eq!(split.header(), joined.header());
    let mut split_bytes = [0x55; 32];
    split.encrypt(&mut split_bytes[..7]);
    split.encrypt(&mut split_bytes[7..]);
    let mut joined_bytes = [0x55; 32];
    joined.encrypt(&mut joined_bytes);
    assert_eq!(split_bytes, joined_bytes);
}
