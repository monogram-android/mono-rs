use crate::{
    construct::create_client,
    persist::{flush_persist, persist_journal_sync},
    registry::{destroy_client, get_client},
    updates::{acknowledge_update_batch, pending_update_batches},
};
use mono_proto_storage::update_journal::*;
use mono_proto_types::{event_types::UpdateEventDto, updates_state::UpdatesStateDto};
use std::sync::Arc;

fn message_event(id: i32) -> UpdateEventDto {
    UpdateEventDto::MessagesDeleted {
        chat_id: Some(7),
        message_ids: vec![id],
    }
}

fn new_message(chat_id: i64, id: i32) -> UpdateEventDto {
    UpdateEventDto::NewMessage {
        message: serde_json::from_value(serde_json::json!({
            "chat_id": chat_id,
            "id": id,
            "text": "fixture",
            "date": 100,
            "outgoing": true,
            "noforwards": false,
            "supports_streaming": false,
            "replies_count": 0,
            "forum_topic": false
        }))
        .unwrap(),
    }
}

fn invalidation(kind: &str) -> UpdateEventDto {
    UpdateEventDto::Ignored { kind: kind.into() }
}

#[test]
fn filtered_global_search_dispatch_rejects_invalid_filters() {
    let path = temp_session("global-search-filter");
    let handle = create_client(1, "hash".into(), path.to_string_lossy().into());
    let err = crate::dispatch::search_global_filtered(
        handle,
        "query".into(),
        "pinned".into(),
        0,
        0,
        0,
        20,
        0,
    )
    .unwrap_err();
    assert!(
        err.to_string()
            .contains("unsupported global search filter: pinned")
    );
    destroy_client(handle);
    let _ = std::fs::remove_file(path);
}

#[test]
fn global_invalidation_resets_seen_at_its_position() {
    let mut seen = super::HashSet::default();
    seen.insert((7, 1));
    seen.insert((8, 1));
    let mut events = vec![
        new_message(7, 1),
        new_message(7, 2),
        invalidation("difference_too_long"),
        new_message(7, 1),
        new_message(8, 1),
        new_message(7, 1),
        UpdateEventDto::ChatsChanged,
        UpdateEventDto::ChatsChanged,
    ];

    super::filter_update_events(&mut events, &mut seen);

    assert!(matches!(
        events.as_slice(),
        [UpdateEventDto::NewMessage { message: before },
         UpdateEventDto::Ignored { kind },
         UpdateEventDto::NewMessage { message: first },
         UpdateEventDto::NewMessage { message: second },
         UpdateEventDto::ChatsChanged]
            if (before.chat_id, before.id) == (7, 2)
                && kind == "difference_too_long"
                && (first.chat_id, first.id) == (7, 1)
                && (second.chat_id, second.id) == (8, 1)
    ));
    assert_eq!(seen.len(), 2);
    assert!(seen.contains(&(7, 1)) && seen.contains(&(8, 1)));
}

#[test]
fn channel_invalidation_only_resets_selected_chat() {
    let mut seen = super::HashSet::default();
    seen.insert((-1007, 1));
    seen.insert((-1008, 1));
    let mut events = vec![
        new_message(-1007, 1),
        invalidation("channel_difference_too_long:not-a-chat"),
        new_message(-1007, 1),
        invalidation("channel_difference_too_long:-1007"),
        new_message(-1008, 1),
        new_message(-1007, 1),
        new_message(-1007, 1),
    ];

    super::filter_update_events(&mut events, &mut seen);

    assert!(matches!(
        events.as_slice(),
        [UpdateEventDto::Ignored { kind: invalid },
         UpdateEventDto::Ignored { kind: marker },
         UpdateEventDto::NewMessage { message }]
            if invalid == "channel_difference_too_long:not-a-chat"
                && marker == "channel_difference_too_long:-1007"
                && (message.chat_id, message.id) == (-1007, 1)
    ));
    assert_eq!(seen.len(), 2);
    assert!(seen.contains(&(-1007, 1)) && seen.contains(&(-1008, 1)));
}

fn cursor(pts: i32) -> Option<UpdatesStateDto> {
    Some(UpdatesStateDto {
        pts,
        qts: 0,
        date: 1,
        seq: 1,
    })
}

fn temp_session(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "monogram-journal-{name}-{}.session",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    path
}

fn queue(handle: u64, id: i32) -> DurableBatch {
    let client = get_client(handle).unwrap();
    let mut d = client.data.lock();
    let batch =
        Arc::make_mut(&mut d.journal).push(vec![message_event(id)], cursor(id), vec![(9, id)]);
    d.updates = cursor(id);
    d.persist_epoch += 1;
    batch
}

#[test]
fn pending_rechecks_batches_added_after_a_save() {
    let path = temp_session("pending-race");
    let handle = create_client(1, "hash".into(), path.to_string_lossy().into());
    let first = queue(handle, 1);
    let client = get_client(handle).unwrap();
    let mut calls = 0;
    let batches = super::pending_update_batches_with(&client, || {
        calls += 1;
        if calls == 1 {
            queue(handle, 2);
        }
    })
    .unwrap();
    assert_eq!(calls, 1);
    assert_eq!(batches.len(), 1);
    assert_eq!(batches[0].batch_id, first.batch_id);
    assert!(client.data.lock().journal.has_undelivered());
    let both = pending_update_batches(handle).unwrap();
    assert_eq!(both.len(), 2);
    crate::registry::CLIENTS.lock().remove(&handle);
    let restarted = create_client(1, "hash".into(), path.to_string_lossy().into());
    let restored = pending_update_batches(restarted).unwrap();
    assert_eq!(
        restored.iter().map(|b| b.batch_id).collect::<Vec<_>>(),
        both.iter().map(|b| b.batch_id).collect::<Vec<_>>()
    );
    destroy_client(restarted);
    let _ = std::fs::remove_file(path);
}

#[test]
fn pending_succeeds_when_unrelated_state_changes_after_save() {
    let path = temp_session("pending-busy");
    let handle = create_client(1, "hash".into(), path.to_string_lossy().into());
    let batch = queue(handle, 1);
    let client = get_client(handle).unwrap();
    let result = super::pending_update_batches_with(&client, || {
        client.data.lock().persist_epoch += 1;
    })
    .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].batch_id, batch.batch_id);
    assert!(!client.data.lock().journal.has_undelivered());
    destroy_client(handle);
    let _ = std::fs::remove_file(path);
}

#[test]
fn pending_does_not_resurrect_acknowledged_batches_or_deliver_new_ones() {
    let path = temp_session("pending-ack");
    let handle = create_client(1, "hash".into(), path.to_string_lossy().into());
    let first = queue(handle, 1);
    let client = get_client(handle).unwrap();
    let batches = super::pending_update_batches_with(&client, || {
        let mut data = client.data.lock();
        let generation = data.journal.account_generation;
        assert!(Arc::make_mut(&mut data.journal).acknowledge(generation, first.batch_id));
        Arc::make_mut(&mut data.journal).push(vec![message_event(2)], cursor(2), vec![]);
        data.persist_epoch += 1;
    })
    .unwrap();
    assert!(batches.is_empty());
    assert!(client.data.lock().journal.has_undelivered());
    destroy_client(handle);
    let _ = std::fs::remove_file(path);
}

#[test]
fn pending_drops_snapshot_after_account_reset() {
    let path = temp_session("pending-reset");
    let handle = create_client(1, "hash".into(), path.to_string_lossy().into());
    queue(handle, 1);
    let client = get_client(handle).unwrap();
    let batches = super::pending_update_batches_with(&client, || {
        let mut data = client.data.lock();
        Arc::make_mut(&mut data.journal).reset_account();
        Arc::make_mut(&mut data.journal).push(vec![message_event(2)], cursor(2), vec![]);
        data.persist_epoch += 1;
    })
    .unwrap();
    assert!(batches.is_empty());
    assert!(client.data.lock().journal.has_undelivered());
    destroy_client(handle);
    let _ = std::fs::remove_file(path);
}

#[test]
fn journal_survives_process_restart_and_is_replayable() {
    let path = temp_session("restart");
    let handle = create_client(1, "hash".into(), path.to_string_lossy().into());
    let batch = queue(handle, 5);
    let session_id = get_client(handle).unwrap().data.lock().home_session_id;
    persist_journal_sync(&get_client(handle).unwrap(), session_id).unwrap();
    destroy_client(handle);

    let restarted = create_client(1, "hash".into(), path.to_string_lossy().into());
    let drained = crate::updates::drain_updates(restarted).unwrap();
    assert!(
        drained.replay_required,
        "unacknowledged batch must block new delivery"
    );
    let pending = pending_update_batches(restarted).unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].batch_id, batch.batch_id);
    assert_eq!(pending[0].cursor_after.as_ref().map(|c| c.pts), Some(5));
    assert_eq!(pending[0].channel_pts_after[0].pts, 5);
    assert!(matches!(
        pending[0].events[0],
        UpdateEventDto::MessagesDeleted { .. }
    ));
    destroy_client(restarted);
    let _ = std::fs::remove_file(path);
}

#[test]
fn crash_before_the_journal_write_loses_nothing_the_cursor_did_not_claim() {
    let path = temp_session("before-save");
    let handle = create_client(1, "hash".into(), path.to_string_lossy().into());
    let session_id = get_client(handle).unwrap().data.lock().home_session_id;
    persist_journal_sync(&get_client(handle).unwrap(), session_id).unwrap();
    queue(handle, 3);
    // A hard kill: no orderly shutdown flush.
    crate::registry::CLIENTS.lock().remove(&handle);

    let restarted = create_client(1, "hash".into(), path.to_string_lossy().into());
    assert!(pending_update_batches(restarted).unwrap().is_empty());
    let client = get_client(restarted).unwrap();
    assert!(
        client.data.lock().updates.is_none(),
        "cursor stays behind the unwritten batch"
    );
    destroy_client(restarted);
    let _ = std::fs::remove_file(path);
}

#[test]
fn a_superseded_save_is_retried_until_the_batch_is_on_disk() {
    let path = temp_session("stale-retry");
    let handle = create_client(1, "hash".into(), path.to_string_lossy().into());
    let batch = queue(handle, 11);
    let client = get_client(handle).unwrap();
    let session_id = client.data.lock().home_session_id;
    crate::persist::FORCE_STALE_SAVES.store(1, std::sync::atomic::Ordering::SeqCst);
    persist_journal_sync(&client, session_id).expect("retry must write after a stale skip");
    assert_eq!(
        crate::persist::FORCE_STALE_SAVES.load(std::sync::atomic::Ordering::SeqCst),
        0
    );
    destroy_client(handle);

    let restarted = create_client(1, "hash".into(), path.to_string_lossy().into());
    let pending = pending_update_batches(restarted).unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].batch_id, batch.batch_id);
    destroy_client(restarted);
    let _ = std::fs::remove_file(path);
}

#[test]
fn an_older_snapshot_does_not_replace_a_newer_live_journal() {
    let mut older = UpdateJournal::default();
    let generation = older.account_generation;
    let mut newer = older.clone();
    newer.push(vec![message_event(1)], None, vec![]);
    let saved = crate::persist::journal_to_save(older, Some(newer.clone()));
    assert_eq!(saved.next_batch_id, newer.next_batch_id);
    assert_eq!(saved.batches.len(), 1);
    let reset = UpdateJournal::default();
    assert_ne!(reset.account_generation, generation);
    let explicit_reset = crate::persist::journal_to_save(reset.clone(), Some(newer));
    assert_eq!(explicit_reset.account_generation, reset.account_generation);
    assert!(explicit_reset.batches.is_empty());
}

#[test]
fn acknowledgement_survives_an_older_same_generation_snapshot() {
    let mut live = UpdateJournal::default();
    let generation = live.account_generation;
    let batch = live.push(vec![message_event(1)], cursor(1), vec![]);
    let snapshot = live.clone();
    assert!(live.acknowledge(generation, batch.batch_id));
    let saved = crate::persist::journal_to_save(snapshot, Some(live.clone()));
    assert!(
        saved.batches.is_empty(),
        "an older snapshot must not restore an ack"
    );
    assert_eq!(saved.next_batch_id, live.next_batch_id);
}

#[test]
fn drain_before_updates_start_creates_no_batch() {
    let path = temp_session("empty-drain");
    let handle = create_client(1, "hash".into(), path.to_string_lossy().into());
    let drained = crate::updates::drain_updates(handle).unwrap();
    assert!(drained.batches.is_empty());
    assert!(drained.live_events.is_empty());
    assert!(!drained.replay_required);
    assert!(
        get_client(handle)
            .unwrap()
            .data
            .lock()
            .journal
            .batches
            .is_empty()
    );
    destroy_client(handle);
    let _ = std::fs::remove_file(path);
}

#[test]
fn acknowledgement_is_idempotent_and_compaction_is_persisted() {
    let path = temp_session("ack");
    let handle = create_client(1, "hash".into(), path.to_string_lossy().into());
    let client = get_client(handle).unwrap();
    let batch = queue(handle, 8);
    let session_id = client.data.lock().home_session_id;
    persist_journal_sync(&client, session_id).unwrap();
    let generation = pending_update_batches(handle).unwrap()[0].account_generation;

    acknowledge_update_batch(handle, generation + 1, batch.batch_id).unwrap();
    assert_eq!(
        client.data.lock().journal.batches.len(),
        1,
        "foreign generation is ignored"
    );
    acknowledge_update_batch(handle, generation, batch.batch_id).unwrap();
    acknowledge_update_batch(handle, generation, batch.batch_id).unwrap();
    flush_persist(&client);
    destroy_client(handle);

    let restarted = create_client(1, "hash".into(), path.to_string_lossy().into());
    assert!(pending_update_batches(restarted).unwrap().is_empty());
    destroy_client(restarted);
    let _ = std::fs::remove_file(path);
}

#[test]
fn exhausted_persistence_reports_the_error_and_keeps_the_batch() {
    let path = temp_session("exhausted");
    let handle = create_client(1, "hash".into(), path.to_string_lossy().into());
    let client = get_client(handle).unwrap();
    queue(handle, 4);
    let session_id = client.data.lock().home_session_id;
    let blocker = std::env::temp_dir().join(format!("monogram-journal-dir-{}", std::process::id()));
    std::fs::create_dir_all(&blocker).unwrap();
    client.data.lock().session_path = blocker.clone();

    assert!(persist_journal_sync(&client, session_id).is_err());
    assert!(
        client
            .persist_exhausted
            .load(std::sync::atomic::Ordering::Acquire)
    );
    assert_eq!(client.data.lock().journal.batches.len(), 1);
    assert!(
        pending_update_batches(handle).is_err(),
        "an unwritten batch must not be handed out"
    );

    client.data.lock().session_path = path.clone();
    persist_journal_sync(&client, session_id).unwrap();
    assert!(
        !client
            .persist_exhausted
            .load(std::sync::atomic::Ordering::Acquire)
    );
    assert_eq!(pending_update_batches(handle).unwrap().len(), 1);
    destroy_client(handle);
    let _ = std::fs::remove_dir(blocker);
    let _ = std::fs::remove_file(path);
}

#[test]
fn old_session_files_without_a_journal_still_load() {
    let session = mono_proto_storage::client_session::ClientSession {
        snapshot: mono_proto_session::Snapshot::new(2, &mut mono_proto_session::OsRandom).unwrap(),
        user_id: None,
        peers: Default::default(),
        updates: None,
        media: Default::default(),
        media_order: Vec::new(),
        channel_pts: Default::default(),
        channel_recovery: Default::default(),
        seen_messages: Default::default(),
        session_dead: false,
        logout_tokens: Vec::new(),
        new_session: None,
        test_dc: false,
        dc_auth_keys: Default::default(),
        journal: Default::default(),
    };
    let mut value = serde_json::to_value(&session).unwrap();
    value.as_object_mut().unwrap().remove("journal");
    let loaded: mono_proto_storage::client_session::ClientSession =
        serde_json::from_value(value).unwrap();
    assert!(loaded.journal.batches.is_empty());
    assert_eq!(loaded.journal.next_batch_id, 1);
}
