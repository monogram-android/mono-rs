//! Testkit self-checks.

use super::*;
use mono_proto_session::RandomSource;
use mono_proto_transport::Connection;

#[test]
fn fake_splits_reads_without_losing_bytes() {
    let mut fake = FakeConnection::default();
    fake.push_read([1, 2, 3]);
    let mut first = [0; 2];
    let mut second = [0; 2];
    assert_eq!(fake.receive(&mut first).unwrap(), 2);
    assert_eq!(fake.receive(&mut second).unwrap(), 1);
    assert_eq!(first, [1, 2]);
    assert_eq!(second[0], 3);
}
#[test]
fn deterministic_sources_repeat() {
    let mut a = DeterministicRandom::new(7);
    let mut b = DeterministicRandom::new(7);
    let mut left = [0; 32];
    let mut right = [0; 32];
    a.fill(&mut left).unwrap();
    b.fill(&mut right).unwrap();
    assert_eq!(left, right);
}
