use super::*;
use mono_proto_crypto::{Direction, MessageToEncrypt, encrypt_message};
use mono_proto_session::OsRandom;

#[test]
fn clock_correction_requires_recent_matching_bad_message() {
    let now = std::time::Instant::now();
    let mut sent = std::collections::BTreeMap::new();
    sent.insert(42, now);
    let mut body = BAD_MSG_NOTIFICATION.to_le_bytes().to_vec();
    body.extend_from_slice(&42_i64.to_le_bytes());
    body.extend_from_slice(&1_i32.to_le_bytes());
    body.extend_from_slice(&16_i32.to_le_bytes());
    assert!(trusted_clock_correction(&body, &sent, now));
    assert!(!trusted_clock_correction(
        &body,
        &sent,
        now + std::time::Duration::from_secs(301)
    ));
    assert!(!trusted_clock_correction(
        &body,
        &std::collections::BTreeMap::new(),
        now
    ));
    body[16..20].copy_from_slice(&32_i32.to_le_bytes());
    assert!(!trusted_clock_correction(&body, &sent, now));
    body.truncate(19);
    assert!(!trusted_clock_correction(&body, &sent, now));
}

#[test]
fn old_authenticated_packet_cannot_change_clock_or_replay_state() {
    let clock = SystemClock;
    let mut snapshot = Snapshot::new(2, &mut OsRandom).unwrap();
    snapshot.set_auth_key((0_u8..=255).collect()).unwrap();
    snapshot.time_offset_micros = 7_000_000;
    let old_seconds = clock.unix_micros() / 1_000_000 - 600;
    let packet = encrypt_message(
        snapshot.auth_key.as_deref().unwrap(),
        MessageToEncrypt {
            server_salt: snapshot.server_salt,
            session_id: snapshot.session_id,
            message_id: (old_seconds << 32) | 1,
            sequence: 1,
            body: &[1, 2, 3, 4],
            padding: &[7; 12],
        },
        Direction::ServerToClient,
    )
    .unwrap();
    let mut engine = Engine::new(
        snapshot,
        ExponentialBackoff {
            timeout_micros: 1_000_000,
            initial_delay_micros: 1,
            max_attempts: 1,
        },
    )
    .unwrap();
    let inbound = engine.open_inbound(&packet, &clock, true, 1024).unwrap();
    assert_eq!(inbound.disposition, ReceivedMessageResult::InvalidTime);
    assert!(reject_invalid_time(inbound.disposition).is_err());
    assert_eq!(engine.session.time_offset_micros, 7_000_000);
    assert!(engine.session.received_message_ids.is_empty());
    assert!(engine.session.pending_acknowledgements.is_empty());
}
