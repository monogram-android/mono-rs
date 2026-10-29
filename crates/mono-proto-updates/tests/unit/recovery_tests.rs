use super::*;
use mono_proto_types::peers::chat_id_for_channel;

#[test]
fn active_channel_is_scheduled_before_other_due_channels() {
    let other = chat_id_for_channel(1);
    let active = chat_id_for_channel(2);
    let mut queue = VecDeque::new();
    prepare_channel_recovery(&mut queue, vec![other, active], active, 100);
    assert_eq!(
        next_channel_recovery(&queue, active, &HashSet::default(), false, 99),
        None
    );
    assert_eq!(
        next_channel_recovery(&queue, active, &HashSet::default(), false, 100),
        Some(1)
    );
}

#[test]
fn completed_open_channel_is_polled_again_after_timeout() {
    let active = chat_id_for_channel(1);
    let mut queue = VecDeque::new();
    prepare_channel_recovery(&mut queue, vec![active], active, 100);
    let entry = queue.pop_front().unwrap();
    finish_channel_recovery(&mut queue, entry, true, Some(5), active, 100);
    assert!(queue[0].watching);
    assert_eq!(queue[0].due_at, 105);
    assert!(!gap_recovery_pending(&queue));
}
