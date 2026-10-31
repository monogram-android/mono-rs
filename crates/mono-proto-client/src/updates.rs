#[cfg(test)]
#[path = "../tests/unit/outgoing_receipts_tests.rs"]
mod outgoing_receipts_tests;
#[cfg(test)]
#[path = "../tests/unit/update_journal_tests.rs"]
mod update_journal_tests;
#[cfg(test)]
#[path = "../tests/unit/update_lane_admission_tests.rs"]
mod update_lane_admission_tests;

use crate::access::{interactive_request_pending, with_client_transport, with_read_lane};
use crate::client::Client;
use crate::lanes::{SessionIo, schedule_file_cleanup};
use crate::merge::{merge_changed_entries, prefer_newer_cursor};
use crate::persist::{
    expired_session_lease, persist_journal_sync, persist_updates_data, schedule_persist,
    session_lease_valid,
};
use crate::registry::get_client;
use crate::session::{
    apply_home_auth, call_with_migrate, ensure_auth_key_on, expire_temporary_lane,
    is_unrecoverable_session, mark_lane_session_dead, temporary_key_retry_error,
};
use crate::state::ClientState;
use mono_proto_runtime::{request_control, scheduler, tcp};
use mono_proto_storage::client_session::ChannelRecovery;
use mono_proto_storage::update_journal::{DurableBatch, is_durable};
use mono_proto_types::MtprotoError;
use mono_proto_types::collections::{HashMap, HashMapExt, HashSet, HashSetExt};
use mono_proto_types::dto::{ChannelCursorDto, UpdateBatchDto, UpdateDrainDto};
use mono_proto_types::events::{OutgoingMessageResult, UpdateEventDto};
use mono_proto_types::peers::channel_id_from_chat_id;
use mono_proto_types::updates_state::UpdatesStateDto;
use mono_proto_updates::{recovery::*, rpc as updates_rpc};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};

struct ActiveUpdateRequest<'a>(&'a AtomicU64);

impl Drop for ActiveUpdateRequest<'_> {
    fn drop(&mut self) {
        self.0.store(0, Ordering::Release);
    }
}

#[cfg(any(test, feature = "test-support"))]
std::thread_local! {
    static UPDATE_CACHE_CLONES: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

#[cfg(any(test, feature = "test-support"))]
pub fn take_update_cache_clones() -> u64 {
    UPDATE_CACHE_CLONES.with(|cell| cell.replace(0))
}

fn note_update_cache_clone() {
    mono_proto_media::perf::count("updates.cache_clone");
    #[cfg(any(test, feature = "test-support"))]
    UPDATE_CACHE_CLONES.with(|cell| cell.set(cell.get() + 1));
}

struct AppliedDrain {
    peers: HashMap<i64, mono_proto_types::peers::CachedPeer>,
    before_peers: HashMap<i64, mono_proto_types::peers::CachedPeer>,
    media: mono_proto_media::media::MediaIndex,
    before_media: mono_proto_media::media::MediaIndex,
    channel_pts: HashMap<i64, i32>,
    recovery: VecDeque<ChannelRecovery>,
    seen_messages: HashSet<(i64, i32)>,
    lazy_queue: HashSet<i64>,
    lazy_retry_at: u64,
    stopped_channel: Option<i64>,
    cursor: Option<UpdatesStateDto>,
}

pub fn start_updates(handle: u64) -> Result<(), MtprotoError> {
    let client = get_client(handle)?;
    {
        let mut data = client.data.lock();
        if data.session_dead {
            return Err(MtprotoError::Message(format!(
                "session invalidated: {}",
                data.session_dead_reason.as_deref().unwrap_or("UNKNOWN"),
            )));
        }
        data.updates_started = true;
    }
    Ok(())
}

/// A cursor becomes durable only after `getDifference` applies it. Fetching
/// `updates.getState` is used solely as the in-memory bootstrap cursor in
/// `drain_updates`; persisting that server head could skip a crash-window gap.
pub fn begin_updates(state: &mut ClientState) {
    state.updates_started = true;
}

pub fn clear_active_dialog(handle: u64) -> Result<(), MtprotoError> {
    let client = get_client(handle)?;
    let session_id = {
        let mut data = client.data.lock();
        data.last_history_chat_id = 0;
        // Keep server-driven recovery hints, but discard the periodic watcher
        // that existed only while this dialog was open.
        data.channel_recovery.retain(|entry| !entry.watching);
        data.home_session_id
    };
    persist_updates_data(&client, session_id).map(|_| ())
}

pub fn lock_updates_lane(
    client: &Client,
) -> Result<
    Option<(
        scheduler::LaneGuard<'_>,
        parking_lot::MutexGuard<'_, SessionIo>,
    )>,
    MtprotoError,
> {
    lock_updates_lane_with(client, || {})
}

fn lock_updates_lane_with(
    client: &Client,
    after_pending_check: impl FnOnce(),
) -> Result<
    Option<(
        scheduler::LaneGuard<'_>,
        parking_lot::MutexGuard<'_, SessionIo>,
    )>,
    MtprotoError,
> {
    // Background recovery must not wait ahead of interactive reads. If the lane is busy or an
    // interactive waiter is queued, the next loop retries after the request gets service.
    if interactive_request_pending(client) {
        return Ok(None);
    }
    after_pending_check();
    let Some(gate) = client
        .main_gate
        .try_acquire(scheduler::RequestClass::BackgroundRead)
    else {
        return Ok(None);
    };
    if interactive_request_pending(client) {
        drop(gate);
        return Ok(None);
    }
    let Some(io) = client.main.try_lock() else {
        drop(gate);
        return Ok(None);
    };
    Ok(Some((gate, io)))
}

fn batch_dto(generation: i64, batch: &DurableBatch) -> UpdateBatchDto {
    UpdateBatchDto {
        account_generation: generation,
        batch_id: batch.batch_id,
        events: batch.events.clone(),
        cursor_after: batch.cursor_after.clone(),
        channel_pts_after: batch
            .channel_pts_after
            .iter()
            .map(|(chat_id, pts)| ChannelCursorDto {
                chat_id: *chat_id,
                pts: *pts,
            })
            .collect(),
    }
}

pub fn stage_outgoing_result(
    client: &Client,
    expected_generation: i64,
    random_id: i64,
    result: OutgoingMessageResult,
) -> Result<i64, MtprotoError> {
    stage_outgoing_results(client, expected_generation, vec![(random_id, result)])
}

pub fn stage_outgoing_results(
    client: &Client,
    expected_generation: i64,
    results: Vec<(i64, OutgoingMessageResult)>,
) -> Result<i64, MtprotoError> {
    let mut data = client.data.lock();
    let session_id = data.home_session_id;
    if data.journal.account_generation != expected_generation
        || !session_lease_valid(&data, session_id)
    {
        return Err(expired_session_lease());
    }
    let ids = results
        .iter()
        .map(|(id, _)| *id)
        .collect::<std::collections::HashSet<_>>();
    if results.is_empty() || results.len() > 10 || ids.len() != results.len() || ids.contains(&0) {
        return Err(MtprotoError::Message(
            "outgoing receipts require distinct persisted random ids".into(),
        ));
    }
    let events = results
        .into_iter()
        .map(
            |(random_id, result)| UpdateEventDto::OutgoingMessageAcknowledged { random_id, result },
        )
        .collect();
    std::sync::Arc::make_mut(&mut data.journal).push(events, None, Vec::new());
    data.persist_epoch = data.persist_epoch.saturating_add(1);
    Ok(session_id)
}

pub fn pending_update_batches(handle: u64) -> Result<Vec<UpdateBatchDto>, MtprotoError> {
    let client = get_client(handle)?;
    let (session_id, dirty) = {
        let d = client.data.lock();
        if d.journal.batches.is_empty() {
            return Ok(Vec::new());
        }
        (d.home_session_id, d.persist_epoch != d.persisted_epoch)
    };
    if dirty {
        persist_journal_sync(&client, session_id)?;
    }
    let mut d = client.data.lock();
    if !session_lease_valid(&d, session_id) {
        return Err(expired_session_lease());
    }
    let generation = d.journal.account_generation;
    let batches = d
        .journal
        .pending()
        .iter()
        .map(|batch| batch_dto(generation, batch))
        .collect();
    std::sync::Arc::make_mut(&mut d.journal).mark_all_delivered();
    Ok(batches)
}

pub fn acknowledge_update_batch(
    handle: u64,
    account_generation: i64,
    batch_id: i64,
) -> Result<(), MtprotoError> {
    let client = get_client(handle)?;
    let (removed, session_id) = {
        let mut d = client.data.lock();
        let removed =
            std::sync::Arc::make_mut(&mut d.journal).acknowledge(account_generation, batch_id);
        if removed {
            d.persist_epoch = d.persist_epoch.saturating_add(1);
        }
        (removed, d.home_session_id)
    };
    if removed {
        schedule_persist(&client, session_id);
    }
    Ok(())
}

pub fn drain_updates(handle: u64) -> Result<UpdateDrainDto, MtprotoError> {
    let client = get_client(handle)?;
    if interactive_request_pending(&client) {
        return Ok(UpdateDrainDto::default());
    }
    {
        let d = client.data.lock();
        if d.journal.has_undelivered() {
            return Ok(UpdateDrainDto {
                replay_required: true,
                ..Default::default()
            });
        }
        if !d.journal.admission_open() {
            mono_proto_media::perf::count("journal.admission_closed");
            return Ok(UpdateDrainDto::default());
        }
    }
    if client
        .persist_exhausted
        .load(std::sync::atomic::Ordering::Acquire)
    {
        let session_id = client.data.lock().home_session_id;
        schedule_persist(&client, session_id);
    }
    let Some((gate, mut io)) = lock_updates_lane(&client)? else {
        return Ok(UpdateDrainDto::default());
    };
    let active_request = request_control::current_id();
    client
        .active_update_request
        .store(active_request, Ordering::Release);
    let _active_request = ActiveUpdateRequest(&client.active_update_request);
    let now = recovery_now();
    let (
        api_id,
        previous,
        home_dc,
        home_auth,
        home_salt,
        home_off,
        open_chat,
        session_id,
        due_recovery,
        lazy_pending,
    ) = {
        let d = client.data.lock();
        if d.session_dead {
            return Err(MtprotoError::Message(format!(
                "session invalidated: {}",
                d.session_dead_reason.as_deref().unwrap_or("UNKNOWN")
            )));
        }
        if d.user_id.is_none() || !d.updates_started {
            return Ok(UpdateDrainDto::default());
        }
        (
            d.api_id,
            d.updates.clone(),
            d.home_dc,
            d.home_auth_key.clone(),
            d.home_salt,
            d.home_time_offset,
            d.last_history_chat_id,
            d.home_session_id,
            next_channel_recovery(
                &d.channel_recovery,
                d.last_history_chat_id,
                &d.lazy_sync_exceptions,
                d.lazy_channel_updates,
                now,
            )
            .is_some()
                || (channel_id_from_chat_id(d.last_history_chat_id).is_some()
                    && !d
                        .channel_recovery
                        .iter()
                        .any(|entry| entry.chat_id == d.last_history_chat_id)),
            d.lazy_channel_updates && !d.lazy_channel_recovery.is_empty() && d.lazy_retry_at <= now,
        )
    };
    crate::bind_temp::ensure_main_temp(&client, &mut io, false)?;
    if io.temp_expires_at == 0 && io.perm_key.is_none() {
        apply_home_auth(
            &mut io.snapshot,
            home_dc,
            home_auth.clone(),
            home_salt,
            home_off,
        );
    }
    let mut needs_difference = io.transport.is_none()
        || io
            .last_difference
            .map(|last| last.elapsed() >= std::time::Duration::from_secs(60))
            .unwrap_or(true);
    let mut slot = io.transport.take();
    mono_proto_runtime::rpc::clear_new_session_metadata();
    let drained = with_client_transport(&client, &mut slot, || {
        // Receive stays on the idle budget; recovery RPCs use their own 8s scope
        // so a waiting interactive caller can take the lane between pages.
        ensure_auth_key_on(&mut io.snapshot)?;
        let mut cursor = previous.clone();
        if cursor.is_none() {
            cursor = Some(updates_rpc::get_updates_state(&mut io.snapshot, api_id)?);
        }
        if !needs_difference {
            let _span = mono_proto_media::perf::span("updates.receive");
            let pushes = mono_proto_runtime::rpc::receive_updates(&mut io.snapshot)?;
            io.pending_push.append(pushes);
        }
        let has_work =
            needs_difference || !io.pending_push.is_empty() || due_recovery || lazy_pending;
        if !has_work {
            mono_proto_media::perf::count("updates.empty_fast_path");
            return Ok((Vec::new(), None));
        }
        note_update_cache_clone();
        let mut recovery;
        let mut peers;
        let mut media;
        let mut channel_pts;
        let mut seen_messages;
        let mut lazy_queue;
        let mut lazy_retry_at;
        let lazy_enabled;
        let lazy_exceptions;
        {
            let d = client.data.lock();
            if d.session_dead {
                return Err(MtprotoError::Message(format!(
                    "session invalidated: {}",
                    d.session_dead_reason.as_deref().unwrap_or("UNKNOWN")
                )));
            }
            if d.home_dc != home_dc
                || d.home_auth_key != home_auth
                || !session_lease_valid(&d, session_id)
            {
                return Err(expired_session_lease());
            }
            recovery = d.channel_recovery.clone();
            peers = d.peers.clone();
            media = d.media.clone();
            channel_pts = d.channel_pts.clone();
            seen_messages = d.seen_messages.clone();
            lazy_queue = d.lazy_channel_recovery.clone();
            lazy_retry_at = d.lazy_retry_at;
            lazy_enabled = d.lazy_channel_updates;
            lazy_exceptions = d.lazy_sync_exceptions.clone();
        }
        let before_peers = peers.clone();
        let before_media = media.clone();
        let mut live = cursor.clone().unwrap();
        let mut pending_channels = Vec::new();
        let mut stopped_channel = None;
        let mut events = Vec::new();
        let sync_started = !client.data.lock().is_syncing
            && catch_up_active(
                !needs_difference,
                visible_gap_recovery_pending(&recovery, open_chat, &lazy_exceptions, lazy_enabled),
                false,
            );
        if sync_started {
            events.push(UpdateEventDto::SyncState { is_syncing: true });
        }
        if !needs_difference {
            let _span = mono_proto_media::perf::span("updates.apply");
            let resolve = io.pending_push.resolve(std::time::Instant::now(), |push| {
                match updates_rpc::apply_push(
                    push,
                    &mut peers,
                    &mut media,
                    &mut live,
                    &mut channel_pts,
                    &mut pending_channels,
                ) {
                    Ok(applied) => {
                        events.extend(applied);
                        Ok(true)
                    }
                    Err(MtprotoError::Message(message)) if message == "updates gap" => Ok(false),
                    Err(error) => Err(error),
                }
            });
            match resolve {
                Ok(recovery_required) => {
                    needs_difference = recovery_required;
                }
                Err(error) => {
                    // An invalid constructor or malformed packet cannot be
                    // retried from the reordering queue. Drop it and recover
                    // from the last committed cursor on a fresh connection.
                    io.pending_push = Default::default();
                    mono_proto_runtime::rpc::drop_live_transport();
                    needs_difference = true;
                    tcp::wait_reconnect(std::time::Duration::from_millis(50))
                        .map_err(|e| MtprotoError::Message(e.to_string()))?;
                    let _ = error;
                }
            }
        }
        let defer_recovery = events
            .iter()
            .any(|event| !matches!(event, UpdateEventDto::SyncState { .. }));
        if needs_difference {
            // Let a waiting interactive RPC take the home lane between
            // update recovery requests. Keep the cursor unchanged so the
            // next drain resumes recovery safely. Deliver already-applied
            // pushes this poll instead of holding the lane for getDifference.
            if interactive_request_pending(&client) || defer_recovery {
                mono_proto_media::perf::count("updates.defer_difference");
                needs_difference = false;
            }
        }
        if needs_difference {
            if !client.data.lock().is_syncing && !sync_started {
                events.push(UpdateEventDto::SyncState { is_syncing: true });
            }
            let _span = mono_proto_media::perf::span("updates.difference");
            let (page, has_more) = mono_proto_runtime::rpc::with_rpc_timeout_secs(8, || {
                updates_rpc::drain_difference(
                    &mut io.snapshot,
                    api_id,
                    &mut peers,
                    &mut media,
                    &mut live,
                    &mut pending_channels,
                )
            })?;
            // The difference is authoritative for the cursor. Discard
            // packets retained behind the gap before using the new state.
            io.pending_push = Default::default();
            events.extend(page);
            io.last_difference = (!has_more).then(std::time::Instant::now);
        }
        let now = recovery_now();
        let pending_channels = route_lazy_channels(
            pending_channels,
            open_chat,
            &lazy_exceptions,
            lazy_enabled,
            &mut lazy_queue,
        );
        pull_priority_channels(
            &mut lazy_queue,
            &mut recovery,
            open_chat,
            &lazy_exceptions,
            now,
        );
        prepare_channel_recovery(&mut recovery, pending_channels, open_chat, now);
        // Round robin pages, bounded per poll; a failed channel cannot
        // prevent the common cursor or another channel from progressing.
        let recovery_budget = std::time::Instant::now() + std::time::Duration::from_secs(8);
        for _ in 0..1 {
            if interactive_request_pending(&client)
                || defer_recovery
                || std::time::Instant::now() >= recovery_budget
            {
                break;
            }
            let Some(index) =
                next_channel_recovery(&recovery, open_chat, &lazy_exceptions, lazy_enabled, now)
            else {
                break;
            };
            let entry = recovery.remove(index).unwrap();
            if !entry.watching
                && !client.data.lock().is_syncing
                && !events
                    .iter()
                    .any(|event| matches!(event, UpdateEventDto::SyncState { is_syncing: true }))
            {
                events.push(UpdateEventDto::SyncState { is_syncing: true });
            }
            let chat_id = entry.chat_id;
            let pts = channel_pts.get(&chat_id).copied().unwrap_or(1);
            let _span = mono_proto_media::perf::span("updates.channel_diff");
            match mono_proto_runtime::rpc::with_rpc_timeout_secs(8, || {
                updates_rpc::drain_channel_difference(
                    &mut io.snapshot,
                    api_id,
                    &mut peers,
                    &mut media,
                    chat_id,
                    pts,
                )
            }) {
                Ok(page) => {
                    channel_pts.insert(chat_id, page.pts.max(pts));
                    events.extend(page.events);
                    finish_channel_recovery(
                        &mut recovery,
                        entry,
                        page.final_page,
                        page.timeout,
                        open_chat,
                        now,
                    );
                    let page_pending = route_lazy_channels(
                        page.pending_channels,
                        open_chat,
                        &lazy_exceptions,
                        lazy_enabled,
                        &mut lazy_queue,
                    );
                    prepare_channel_recovery(&mut recovery, page_pending, open_chat, now);
                }
                Err(err) => {
                    if is_unrecoverable_session(&err) {
                        return Err(err);
                    }
                    if !terminal_channel_error(&err) {
                        defer_channel_recovery(&mut recovery, entry, &err, recovery_now());
                    } else {
                        lazy_queue.remove(&chat_id);
                        if chat_id == open_chat {
                            stopped_channel = Some(chat_id);
                        }
                    }
                    mono_proto_runtime::rpc::drop_live_transport();
                    break;
                }
            }
        }
        pull_priority_channels(
            &mut lazy_queue,
            &mut recovery,
            if stopped_channel.is_some() {
                0
            } else {
                open_chat
            },
            &lazy_exceptions,
            recovery_now(),
        );
        let lazy_batch = if lazy_retry_at <= recovery_now()
            && !interactive_request_pending(&client)
            && !defer_recovery
            && !needs_difference
        {
            take_lazy_batch(
                &mut lazy_queue,
                |chat_id| mono_proto_types::peers::usable_cached_peer(&peers, chat_id).is_some(),
                LAZY_CHANNEL_BATCH,
            )
        } else {
            Vec::new()
        };
        if !lazy_batch.is_empty() {
            let _span = mono_proto_media::perf::span("updates.lazy_channels");
            match mono_proto_runtime::rpc::with_rpc_timeout_secs(8, || {
                updates_rpc::drain_lazy_channels(
                    &mut io.snapshot,
                    api_id,
                    &mut peers,
                    &mut media,
                    &mut channel_pts,
                    &lazy_batch,
                )
            }) {
                Ok(page) => {
                    lazy_retry_at = recovery_now().saturating_add(5);
                    restore_lazy_batch(&mut lazy_queue, &page.unresolved);
                    events.extend(lazy_dialog_events(page.chats));
                }
                Err(err) => {
                    let plan = plan_lazy_failure(is_unrecoverable_session(&err));
                    if !plan.restore {
                        return Err(err);
                    }
                    if !terminal_channel_error(&err) {
                        restore_lazy_batch(&mut lazy_queue, &lazy_batch);
                    }
                    lazy_retry_at = recovery_now().saturating_add(recovery_wait_secs(&err));
                    if plan.drop_transport {
                        mono_proto_runtime::rpc::drop_live_transport();
                    }
                }
            }
        }
        let mut saw_chats = false;
        if seen_messages.len() > 4_000 {
            seen_messages.clear();
        }
        events.retain(|event| match event {
            UpdateEventDto::ChatsChanged if saw_chats => false,
            UpdateEventDto::ChatsChanged => {
                saw_chats = true;
                true
            }
            UpdateEventDto::NewMessage { message } => {
                seen_messages.insert((message.chat_id, message.id))
            }
            UpdateEventDto::MessageEdited { message } => {
                seen_messages.insert((message.chat_id, message.id));
                true
            }
            UpdateEventDto::MessagesDeleted {
                chat_id,
                message_ids,
            } => {
                let inferred_chat = *chat_id;
                for message_id in message_ids {
                    if let Some(chat) = inferred_chat {
                        seen_messages.remove(&(chat, *message_id));
                    } else {
                        seen_messages.retain(|(_, id)| id != message_id);
                    }
                }
                true
            }
            _ => true,
        });
        Ok((
            events,
            Some(AppliedDrain {
                peers,
                before_peers,
                media,
                before_media,
                channel_pts,
                recovery,
                seen_messages,
                lazy_queue,
                lazy_retry_at,
                stopped_channel,
                cursor: Some(live),
            }),
        ))
    });
    let (mut events, applied) = match drained {
        Ok(pair) => {
            io.transport = slot;
            pair
        }
        Err(err) => {
            let temporary = expire_temporary_lane(&client, &mut io, &err);
            io.transport = None;
            drop(io);
            drop(gate);
            if temporary {
                return Err(temporary_key_retry_error());
            }
            if mark_lane_session_dead(&client, session_id, &err) {
                let _ = persist_updates_data(&client, session_id);
            }
            return Err(err);
        }
    };
    let new_session = mono_proto_runtime::rpc::take_new_session_metadata();
    let Some(applied) = applied else {
        drop(io);
        drop(gate);
        if let Some(metadata) = new_session {
            {
                let mut d = client.data.lock();
                if d.home_dc == home_dc
                    && d.home_auth_key == home_auth
                    && session_lease_valid(&d, session_id)
                {
                    d.new_session = Some(metadata);
                    d.persist_epoch = d.persist_epoch.saturating_add(1);
                }
            }
            schedule_persist(&client, session_id);
        }
        return Ok(UpdateDrainDto {
            live_events: events,
            ..Default::default()
        });
    };
    let persist_needed;
    let mut journal_batch = None;
    let mut journal_generation = 0;
    let is_syncing_now;
    let should_emit_sync_state;
    {
        let mut d = client.data.lock();
        // A completed old-account poll must not repopulate state after logout
        // or authorization migration on the main lane.
        if d.home_dc != home_dc
            || d.home_auth_key != home_auth
            || !session_lease_valid(&d, session_id)
        {
            io.transport = None;
            return Err(expired_session_lease());
        }
        is_syncing_now = catch_up_active(
            io.last_difference.is_some(),
            visible_gap_recovery_pending(
                &applied.recovery,
                d.last_history_chat_id,
                &d.lazy_sync_exceptions,
                d.lazy_channel_updates,
            ),
            false,
        );
        let peers_changed =
            merge_changed_entries(&mut d.peers, &applied.before_peers, applied.peers);
        let media_changed =
            merge_changed_entries(&mut d.media, &applied.before_media, applied.media);
        let next_cursor = prefer_newer_cursor(d.updates.clone(), applied.cursor);
        should_emit_sync_state = sync_edge(
            d.is_syncing
                || events
                    .iter()
                    .any(|event| matches!(event, UpdateEventDto::SyncState { is_syncing: true })),
            is_syncing_now,
        );
        d.is_syncing = is_syncing_now;
        let durable_events: Vec<UpdateEventDto>;
        (durable_events, events) = std::mem::take(&mut events)
            .into_iter()
            .partition(is_durable);
        if !durable_events.is_empty() {
            let channel_pts_after: Vec<(i64, i32)> = applied
                .channel_pts
                .iter()
                .filter(|(chat, pts)| d.channel_pts.get(*chat) != Some(*pts))
                .map(|(chat, pts)| (*chat, *pts))
                .collect();
            let journal = std::sync::Arc::make_mut(&mut d.journal);
            journal_generation = journal.account_generation;
            journal_batch =
                Some(journal.push(durable_events, next_cursor.clone(), channel_pts_after));
        }
        persist_needed = journal_batch.is_some()
            || new_session.is_some()
            || peers_changed
            || media_changed
            || d.channel_pts != applied.channel_pts
            || d.channel_recovery != applied.recovery
            || d.seen_messages != applied.seen_messages
            || next_cursor != d.updates;
        if persist_needed {
            d.persist_epoch = d.persist_epoch.saturating_add(1);
        }
        d.updates = next_cursor;
        d.channel_pts = applied.channel_pts;
        d.channel_recovery = applied.recovery;
        d.seen_messages = applied.seen_messages;
        d.lazy_channel_recovery = applied.lazy_queue;
        d.lazy_retry_at = applied.lazy_retry_at;
        if applied.stopped_channel == Some(d.last_history_chat_id) {
            d.last_history_chat_id = 0;
        }
        if new_session.is_some() {
            d.new_session = new_session.clone();
        }
        d.updates_started = true;
        // The updates lane can receive a fresher server salt/time correction.
        // Feed it back to the home lane on the next RPC instead of repeatedly
        // restoring stale values from the previous main-lane snapshot.
        if io.snapshot.dc_id == d.home_dc && io.snapshot.auth_key == d.home_auth_key {
            d.home_salt = io.snapshot.server_salt;
            d.home_time_offset = io.snapshot.time_offset_micros;
        }
    }
    drop(io);
    drop(gate);
    schedule_file_cleanup(&client);
    if should_emit_sync_state {
        events.push(UpdateEventDto::SyncState {
            is_syncing: is_syncing_now,
        });
    }
    let Some(batch) = journal_batch else {
        if persist_needed {
            mono_proto_media::perf::count("updates.persist");
            schedule_persist(&client, session_id);
        }
        return Ok(UpdateDrainDto {
            live_events: events,
            ..Default::default()
        });
    };
    mono_proto_media::perf::count("updates.persist");
    persist_journal_sync(&client, session_id)?;
    std::sync::Arc::make_mut(&mut client.data.lock().journal).mark_delivered(batch.batch_id);
    Ok(UpdateDrainDto {
        batches: vec![batch_dto(journal_generation, &batch)],
        live_events: events,
        replay_required: false,
    })
}

pub fn get_updates_state(handle: u64) -> Result<UpdatesStateDto, MtprotoError> {
    with_read_lane(handle, |state| {
        let cursor = call_with_migrate(state, |state| {
            updates_rpc::get_updates_state(&mut state.snapshot, state.api_id)
        })?;
        // Reading the server head is not applying its updates. Advancing the
        // recovery cursor here would skip messages on the next getDifference.
        Ok(cursor)
    })
}
