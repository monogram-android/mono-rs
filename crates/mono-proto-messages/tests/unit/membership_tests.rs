use super::*;

#[test]
fn absent_membership_does_not_grant_access() {
    let result = membership_granted(-123, &[]);
    assert!(!result.joined);
    assert!(!result.can_send_plain);
    assert!(!result.can_send_photos);
    assert!(!result.can_send_polls);
}
