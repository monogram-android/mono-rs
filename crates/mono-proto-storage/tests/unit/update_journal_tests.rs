use super::*;
use crate::event_types::UpdateEventDto;
use crate::updates_state::UpdatesStateDto;

fn message_event(id: i32) -> UpdateEventDto {
    UpdateEventDto::MessagesDeleted {
        chat_id: Some(7),
        message_ids: vec![id],
    }
}

fn cursor(pts: i32) -> Option<UpdatesStateDto> {
    Some(UpdatesStateDto {
        pts,
        qts: 0,
        date: 1,
        seq: 1,
    })
}

#[test]
fn batches_get_increasing_ids_and_ack_is_exact() {
    let mut journal = UpdateJournal::default();
    let generation = journal.account_generation;
    let first = journal.push(vec![message_event(1)], cursor(1), vec![]);
    let second = journal.push(vec![message_event(2)], cursor(2), vec![]);
    assert!(second.batch_id > first.batch_id);
    assert!(!journal.acknowledge(generation + 1, first.batch_id));
    assert_eq!(journal.batches.len(), 2);
    assert!(journal.acknowledge(generation, first.batch_id));
    assert!(!journal.acknowledge(generation, first.batch_id));
    assert_eq!(journal.pending().len(), 1);
    assert_eq!(journal.pending()[0].batch_id, second.batch_id);
}

#[test]
fn delivery_tracking_follows_acknowledgement() {
    let mut journal = UpdateJournal::default();
    let generation = journal.account_generation;
    let a = journal.push(vec![message_event(1)], None, vec![]);
    let b = journal.push(vec![message_event(2)], None, vec![]);
    assert!(journal.has_undelivered());
    journal.mark_delivered(a.batch_id);
    assert!(journal.has_undelivered());
    journal.acknowledge(generation, a.batch_id);
    assert!(journal.has_undelivered(), "b is still undelivered");
    journal.mark_delivered(b.batch_id);
    assert!(!journal.has_undelivered());
}

#[test]
fn admission_closes_at_the_batch_bound_and_reopens_after_ack() {
    let mut journal = UpdateJournal::default();
    let generation = journal.account_generation;
    let mut first = 0;
    for index in 0..MAX_UNACKED_BATCHES {
        let batch = journal.push(vec![message_event(index as i32)], None, vec![]);
        if index == 0 {
            first = batch.batch_id;
        }
    }
    assert!(!journal.admission_open());
    assert!(journal.acknowledge(generation, first));
    assert!(journal.admission_open());
}

#[test]
fn account_reset_drops_batches_and_changes_generation() {
    let mut journal = UpdateJournal::default();
    let previous = journal.account_generation;
    journal.push(vec![message_event(1)], None, vec![]);
    journal.reset_account();
    assert_ne!(journal.account_generation, previous);
    assert!(journal.batches.is_empty());
}

#[test]
fn only_state_mutations_are_durable() {
    assert!(is_durable(&message_event(1)));
    assert!(is_durable(&UpdateEventDto::ChatsChanged));
    assert!(is_durable(&UpdateEventDto::ReadInbox {
        chat_id: 1,
        max_id: 2,
        still_unread: 0
    }));
    assert!(!is_durable(&UpdateEventDto::SyncState { is_syncing: true }));
    assert!(!is_durable(&UpdateEventDto::PeerTyping {
        chat_id: 1,
        user_id: 2,
        typing: true,
        action: String::new()
    }));
    assert!(!is_durable(&UpdateEventDto::Ignored { kind: "x".into() }));
    assert!(is_durable(&UpdateEventDto::Ignored {
        kind: "unread_mentions_delta:5:1".into()
    }));
}
