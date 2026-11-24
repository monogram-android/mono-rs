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
fn dialog_refresh_before_channel_recovery_preserves_the_durable_gap() {
    let path = temp_session("dialog-refresh-channel-gap");
    let open_client = || create_client(1, "hash".into(), path.to_string_lossy().into());
    let handle = open_client();
    let client = get_client(handle).unwrap();
    let chat_id = mono_proto_types::peers::chat_id_for_channel(7);
    client.main.lock().snapshot.auth_key = Some(vec![1; 256]);
    {
        let mut data = client.data.lock();
        data.home_auth_key = Some(vec![1; 256]);
        data.user_id = Some(42);
        data.updates = cursor(1);
        data.channel_pts.insert(chat_id, 100);
        data.last_history_chat_id = chat_id;
    }
    client
        .policy
        .main_session_allowance
        .store(1, super::Ordering::Relaxed);
    super::start_updates(handle).unwrap();
    super::with_read_lane(handle, |state| {
        mono_proto_messages::dialogs::merge_channel_pts(&mut state.channel_pts, chat_id, Some(105));
        Ok(())
    })
    .unwrap();

    let drain = |handle, expected_pts| {
        let mut requests = Vec::new();
        let result = super::drain_updates_with(
            handle,
            super::recovery_now,
            std::time::Instant::now,
            |_, _| panic!("unexpected bootstrap"),
            |_, _, _, _, _, _| Ok((vec![], false)),
            |_, _, _, _, requested_chat, pts| {
                requests.push((requested_chat, pts));
                let events = if pts == 100 {
                    vec![UpdateEventDto::MessagesDeleted {
                        chat_id: Some(chat_id),
                        message_ids: vec![11, 12],
                    }]
                } else {
                    vec![]
                };
                Ok(mono_proto_updates::rpc::ChannelDifferencePage {
                    events,
                    pts: 105,
                    final_page: true,
                    timeout: None,
                    pending_channels: vec![],
                })
            },
        )
        .unwrap();
        assert_eq!(requests, [(chat_id, expected_pts)]);
        assert!(!result.replay_required);
        result.batches
    };
    let assert_gap_batch = |batch: &mono_proto_types::dto::UpdateBatchDto| {
        assert!(matches!(
            batch.events.as_slice(),
            [UpdateEventDto::MessagesDeleted { chat_id: Some(id), message_ids }]
                if *id == chat_id && message_ids == &[11, 12]
        ));
        assert_eq!(batch.channel_pts_after.len(), 1);
        assert_eq!(batch.channel_pts_after[0].chat_id, chat_id);
        assert_eq!(batch.channel_pts_after[0].pts, 105);
    };
    let batches = drain(handle, 100);
    assert_eq!(batches.len(), 1);
    assert_gap_batch(&batches[0]);

    // Reopen without a shutdown flush: the drain must already have saved the batch.
    crate::registry::CLIENTS.lock().remove(&handle);
    let restarted = open_client();
    assert!(super::drain_updates(restarted).unwrap().replay_required);
    let replay = pending_update_batches(restarted).unwrap();
    assert_eq!(replay.len(), 1);
    assert_eq!(replay[0].batch_id, batches[0].batch_id);
    assert_gap_batch(&replay[0]);
    acknowledge_update_batch(restarted, replay[0].account_generation, replay[0].batch_id).unwrap();
    destroy_client(restarted);

    let restarted = open_client();
    assert!(pending_update_batches(restarted).unwrap().is_empty());
    let client = get_client(restarted).unwrap();
    {
        let mut data = client.data.lock();
        data.last_history_chat_id = chat_id;
        data.channel_recovery.clear();
    }
    super::start_updates(restarted).unwrap();
    assert!(drain(restarted, 105).is_empty());
    assert!(pending_update_batches(restarted).unwrap().is_empty());
    destroy_client(restarted);
    let _ = std::fs::remove_file(path);
}

#[test]
fn bootstrap_retries_only_after_the_flood_wait_deadline() {
    use std::cell::Cell;
    use std::time::{Duration, Instant};

    let path = temp_session("bootstrap-flood-wait");
    let handle = create_client(1, "hash".into(), path.to_string_lossy().into());
    let client = get_client(handle).unwrap();
    client.main.lock().snapshot.auth_key = Some(vec![1; 256]);
    {
        let mut data = client.data.lock();
        data.home_auth_key = Some(vec![1; 256]);
        data.user_id = Some(42);
    }
    client
        .policy
        .main_session_allowance
        .store(1, super::Ordering::Relaxed);
    super::start_updates(handle).unwrap();

    let started = Instant::now();
    let received = started + Duration::from_millis(250);
    let now = Cell::new(started);
    let requests = Cell::new(0);
    let drain = || {
        super::drain_updates_with(
            handle,
            super::recovery_now,
            || now.get(),
            |_, _| {
                let request = requests.get();
                requests.set(request + 1);
                if request == 0 {
                    now.set(received);
                    return Err(super::MtprotoError::Message("FLOOD_WAIT_17".into()));
                }
                assert_eq!(now.get(), received + Duration::from_millis(17_000));
                Ok(cursor(100).unwrap())
            },
            |_, _, _, _, live, _| {
                assert_eq!(client.data.lock().updates, None);
                assert_eq!(live.pts, 100);
                live.pts = 105;
                Ok((vec![message_event(11)], false))
            },
            |_, _, _, _, _, _| panic!("unexpected channel difference"),
        )
    };
    assert!(matches!(
        drain(),
        Err(super::MtprotoError::Message(message)) if message == "FLOOD_WAIT_17"
    ));
    assert_eq!(client.data.lock().updates, None);

    now.set(received + Duration::from_millis(16_999));
    super::start_updates(handle).unwrap();
    super::clear_active_dialog(handle).unwrap();
    let pending = drain().unwrap();
    assert!(!pending.replay_required);
    assert!(pending.batches.is_empty());
    assert!(pending_update_batches(handle).unwrap().is_empty());
    assert_eq!(requests.get(), 1);
    assert_eq!(client.data.lock().updates, None);

    now.set(received + Duration::from_millis(17_000));
    let recovered = drain().unwrap();
    assert_eq!(requests.get(), 2);
    assert_eq!(client.data.lock().updates, cursor(105));
    assert_eq!(recovered.batches.len(), 1);
    assert_eq!(recovered.batches[0].cursor_after, cursor(105));
    destroy_client(handle);
    let _ = std::fs::remove_file(path);
}

#[test]
fn account_difference_retries_only_after_the_flood_wait_deadline() {
    use std::cell::Cell;
    use std::time::{Duration, Instant};

    let path = temp_session("account-difference-flood-wait");
    let handle = create_client(1, "hash".into(), path.to_string_lossy().into());
    let client = get_client(handle).unwrap();
    client.main.lock().snapshot.auth_key = Some(vec![1; 256]);
    {
        let mut data = client.data.lock();
        data.home_auth_key = Some(vec![1; 256]);
        data.user_id = Some(42);
        data.updates = cursor(100);
    }
    client
        .policy
        .main_session_allowance
        .store(1, super::Ordering::Relaxed);
    super::start_updates(handle).unwrap();

    let started = Instant::now();
    let received = started + Duration::from_millis(250);
    let now = Cell::new(started);
    let mut requests = Vec::new();
    let mut drain = || {
        super::drain_updates_with(
            handle,
            super::recovery_now,
            || now.get(),
            |_, _| panic!("unexpected bootstrap"),
            |_, _, _, _, live, _| {
                requests.push(live.pts);
                if requests.len() == 1 {
                    now.set(received);
                    return Err(super::MtprotoError::Message("FLOOD_WAIT_17".into()));
                }
                live.pts = 105;
                Ok((vec![message_event(11)], false))
            },
            |_, _, _, _, _, _| panic!("unexpected channel difference"),
        )
    };
    assert!(matches!(
        drain(),
        Err(super::MtprotoError::Message(message)) if message == "FLOOD_WAIT_17"
    ));
    now.set(received + Duration::from_millis(16_999));
    let pending = drain().unwrap();
    assert!(!pending.replay_required);
    assert!(pending.batches.is_empty());
    assert!(pending_update_batches(handle).unwrap().is_empty());
    assert_eq!(client.data.lock().updates, cursor(100));

    now.set(received + Duration::from_millis(17_000));
    let recovered = drain().unwrap();
    assert_eq!(requests, [100, 100]);
    assert_eq!(client.data.lock().updates, cursor(105));
    assert_eq!(recovered.batches.len(), 1);
    let batch = &recovered.batches[0];
    assert_eq!(batch.cursor_after, cursor(105));
    assert!(matches!(
        batch.events.as_slice(),
        [UpdateEventDto::MessagesDeleted { message_ids, .. }] if message_ids == &[11]
    ));
    acknowledge_update_batch(handle, batch.account_generation, batch.batch_id).unwrap();
    assert!(pending_update_batches(handle).unwrap().is_empty());
    destroy_client(handle);
    let _ = std::fs::remove_file(path);
}

#[test]
fn channel_flood_wait_survives_watch_changes_and_gaps_until_the_exact_deadline() {
    check_channel_flood_wait(false);
}

#[test]
fn channel_flood_wait_survives_restart_and_watch_changes() {
    check_channel_flood_wait(true);
}

fn check_channel_flood_wait(restart: bool) {
    use mono_proto_types::peers::chat_id_for_channel;
    use std::cell::Cell;
    use std::time::{Duration, Instant};

    let path = temp_session(if restart { "channel-flood-restart" } else { "channel-difference-flood-wait" });
    let handle = create_client(1, "hash".into(), path.to_string_lossy().into());
    let client = get_client(handle).unwrap();
    let chat = chat_id_for_channel(7);
    let other = chat_id_for_channel(8);
    client.main.lock().snapshot.auth_key = Some(vec![1; 256]);
    {
        let mut data = client.data.lock();
        data.home_auth_key = Some(vec![1; 256]);
        data.user_id = Some(42);
        data.updates = cursor(1);
        data.channel_pts.insert(chat, 100);
        data.channel_pts.insert(other, 10);
        data.last_history_chat_id = chat;
    }
    client
        .policy
        .main_session_allowance
        .store(1, super::Ordering::Relaxed);
    super::start_updates(handle).unwrap();

    let epoch = Instant::now();
    let elapsed = Cell::new(Duration::from_millis(500));
    let received = Duration::from_millis(750);
    let active_handle = Cell::new(handle);
    let drain = |pending: &[i64], expected| {
        let mut request = None;
        let drained = super::drain_updates_with(
            active_handle.get(),
            || 1_000 + elapsed.get().as_secs(),
            || epoch + elapsed.get(),
            |_, _| panic!("unexpected bootstrap"),
            |_, _, _, _, _, gaps| {
                gaps.extend_from_slice(pending);
                Ok((vec![], false))
            },
            |_, _, _, _, id, pts| {
                assert_eq!(request.replace((id, pts)), None);
                assert_eq!(request, expected);
                if elapsed.get() < received {
                    elapsed.set(received);
                    return Err(super::MtprotoError::Message(
                        "RPC 420: FLOOD_WAIT_17".into(),
                    ));
                }
                Ok(mono_proto_updates::rpc::ChannelDifferencePage {
                    events: if id == chat {
                        vec![UpdateEventDto::MessagesDeleted {
                            chat_id: Some(chat),
                            message_ids: vec![11],
                        }]
                    } else {
                        vec![]
                    },
                    pts: pts + 5,
                    final_page: true,
                    timeout: Some(3),
                    pending_channels: vec![],
                })
            },
        )
        .unwrap();
        assert_eq!(request, expected);
        assert!(!drained.replay_required);
        drained.batches
    };
    assert!(drain(&[], Some((chat, 100))).is_empty());
    assert!(client.data.lock().channel_recovery[0].watching);
    if restart {
        destroy_client(handle);
        let reopened = create_client(1, "hash".into(), path.to_string_lossy().into());
        active_handle.set(reopened);
        let restored = get_client(reopened).unwrap();
        restored.policy.main_session_allowance.store(1, super::Ordering::Relaxed);
        restored.data.lock().last_history_chat_id = chat;
        super::start_updates(reopened).unwrap();
        assert!(drain(&[], None).is_empty());
    }
    let handle = active_handle.get();
    let client = get_client(handle).unwrap();

    elapsed.set(received + Duration::from_millis(16_999));
    assert!(drain(&[], None).is_empty());
    super::clear_active_dialog(handle).unwrap();
    client.data.lock().last_history_chat_id = chat;
    assert!(drain(&[], None).is_empty());
    assert!(drain(&[chat, other], Some((other, 10))).is_empty());
    assert!(drain(&[chat], None).is_empty());
    super::clear_active_dialog(handle).unwrap();
    {
        let data = client.data.lock();
        assert_eq!(data.channel_pts[&chat], 100);
        assert_eq!(data.channel_pts[&other], 15);
        assert_eq!(data.channel_recovery.len(), 1);
        assert_eq!(data.channel_recovery[0].chat_id, chat);
        assert!(!data.channel_recovery[0].watching);
    }
    client.data.lock().last_history_chat_id = chat;

    elapsed.set(received + Duration::from_millis(17_000));
    let batches = drain(&[], Some((chat, 100)));
    assert_eq!(batches.len(), 1);
    let batch = &batches[0];
    assert!(matches!(
        batch.events.as_slice(),
        [UpdateEventDto::MessagesDeleted { chat_id: Some(id), message_ids }]
            if *id == chat && message_ids == &[11]
    ));
    assert_eq!(batch.channel_pts_after.len(), 1);
    assert_eq!(batch.channel_pts_after[0].chat_id, chat);
    assert_eq!(batch.channel_pts_after[0].pts, 105);
    acknowledge_update_batch(handle, batch.account_generation, batch.batch_id).unwrap();
    assert!(drain(&[], None).is_empty());
    {
        let data = client.data.lock();
        assert_eq!(data.channel_pts[&chat], 105);
        assert_eq!(data.channel_recovery.len(), 1);
        assert!(data.channel_recovery[0].watching);
    }
    destroy_client(handle);
    let _ = std::fs::remove_file(path);
}

#[test]
fn lazy_channel_refresh_waits_until_the_exact_monotonic_deadline() {
    use std::time::{Duration, Instant};

    let epoch = Instant::now();
    let received = epoch + Duration::from_millis(750);
    assert!(super::lazy_retry_due(None, epoch));

    let flood = super::MtprotoError::Message("FLOOD_WAIT_17".into());
    let short_flood = super::MtprotoError::Message("FLOOD_WAIT_2".into());
    for (error, wait) in [(None, 5), (Some(&short_flood), 5), (Some(&flood), 17)] {
        let deadline = Some(super::lazy_retry_deadline(received, error));
        let due = received + Duration::from_secs(wait);
        assert!(!super::lazy_retry_due(
            deadline,
            due - Duration::from_millis(1),
        ));
        assert!(super::lazy_retry_due(deadline, due));
    }
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
        channel_flood_until: Default::default(),
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
