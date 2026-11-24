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
use crate::state::{ClientData, ClientState};
use mono_proto_runtime::{request_control, scheduler, tcp};
use mono_proto_storage::client_session::ChannelRecovery;
use mono_proto_storage::update_journal::{DurableBatch, is_durable};
use mono_proto_types::MtprotoError;
use mono_proto_types::collections::{HashMap, HashSet};
use mono_proto_types::dto::{ChannelCursorDto, UpdateBatchDto, UpdateDrainDto};
use mono_proto_types::events::{OutgoingMessageResult, UpdateEventDto};
use mono_proto_types::peers::channel_id_from_chat_id;
use mono_proto_types::updates_state::UpdatesStateDto;
use mono_proto_updates::{recovery::*, rpc as updates_rpc};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

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

#[derive(Default)]
struct DrainSync {
    started: bool,
}

impl DrainSync {
    fn start(&mut self, already_syncing: bool, events: &mut Vec<UpdateEventDto>) {
        if !already_syncing && !self.started {
            self.started = true;
            events.push(UpdateEventDto::SyncState { is_syncing: true });
        }
    }

    fn finish(self, committed: &mut bool, next: bool) -> Option<UpdateEventDto> {
        let emit = sync_edge(*committed || self.started, next);
        *committed = next;
        emit.then_some(UpdateEventDto::SyncState { is_syncing: next })
    }
}

struct AppliedDrain {
    sync: DrainSync,
    peers: HashMap<i64, mono_proto_types::peers::CachedPeer>,
    before_peers: HashMap<i64, mono_proto_types::peers::CachedPeer>,
    media: mono_proto_media::media::MediaIndex,
    before_media: mono_proto_media::media::MediaIndex,
    channel_pts: HashMap<i64, i32>,
    recovery: VecDeque<ChannelRecovery>,
    seen_messages: HashSet<(i64, i32)>,
    lazy_queue: HashSet<i64>,
    lazy_retry_at: Option<(Instant, Duration)>,
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
    pending_update_batches_with(&client, || {})
}

fn pending_update_batches_with(
    client: &Client,
    after_save: impl FnOnce(),
) -> Result<Vec<UpdateBatchDto>, MtprotoError> {
    // Later arrivals are not necessarily in the completed save and must stay pending.
    let (session_id, generation, captured, dirty) = {
        let d = client.data.lock();
        if d.journal.batches.is_empty() {
            return Ok(Vec::new());
        }
        (
            d.home_session_id,
            d.journal.account_generation,
            d.journal.pending(),
            d.persist_epoch != d.persisted_epoch,
        )
    };
    if dirty {
        persist_journal_sync(client, session_id)?;
        after_save();
    }
    let mut d = client.data.lock();
    if !session_lease_valid(&d, session_id) {
        return Err(expired_session_lease());
    }
    if d.journal.account_generation != generation {
        return Ok(Vec::new());
    }
    let current_ids: HashSet<_> = d
        .journal
        .batches
        .iter()
        .map(|batch| batch.batch_id)
        .collect();
    let batches: Vec<_> = captured
        .iter()
        .filter(|batch| current_ids.contains(&batch.batch_id))
        .map(|batch| batch_dto(generation, batch))
        .collect();
    if let Some(last) = batches.last() {
        std::sync::Arc::make_mut(&mut d.journal).mark_delivered(last.batch_id);
    }
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

fn filter_update_events(events: &mut Vec<UpdateEventDto>, seen_messages: &mut HashSet<(i64, i32)>) {
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
        UpdateEventDto::Ignored { kind } if kind == "difference_too_long" => {
            seen_messages.clear();
            true
        }
        UpdateEventDto::Ignored { kind } => {
            if let Some(chat_id) = kind
                .strip_prefix("channel_difference_too_long:")
                .and_then(|value| value.parse::<i64>().ok())
            {
                seen_messages.retain(|(chat, _)| *chat != chat_id);
            }
            true
        }
        _ => true,
    });
}

fn lazy_retry_due(deadline: Option<(Instant, Duration)>, now: Instant) -> bool {
    deadline.is_none_or(|(received, wait)| now.saturating_duration_since(received) >= wait)
}

fn lazy_retry_deadline(received: Instant, error: Option<&MtprotoError>) -> (Instant, Duration) {
    (
        received,
        Duration::from_secs(error.map_or(5, recovery_wait_secs)),
    )
}

pub fn drain_updates(handle: u64) -> Result<UpdateDrainDto, MtprotoError> {
    drain_updates_with(
        handle,
        recovery_now,
        Instant::now,
        updates_rpc::get_updates_state,
        updates_rpc::drain_difference,
        updates_rpc::drain_channel_difference,
    )
}

fn drain_updates_with(
    handle: u64,
    wall_now: impl Fn() -> u64,
    monotonic_now: impl Fn() -> Instant,
    get_updates_state: impl FnOnce(
        &mut mono_proto_session::Snapshot,
        i32,
    ) -> Result<UpdatesStateDto, MtprotoError>,
    drain_difference: impl FnOnce(
        &mut mono_proto_session::Snapshot,
        i32,
        &mut HashMap<i64, mono_proto_types::peers::CachedPeer>,
        &mut mono_proto_media::media::MediaIndex,
        &mut UpdatesStateDto,
        &mut Vec<i64>,
    ) -> Result<(Vec<UpdateEventDto>, bool), MtprotoError>,
    mut drain_channel_difference: impl FnMut(
        &mut mono_proto_session::Snapshot,
        i32,
        &mut HashMap<i64, mono_proto_types::peers::CachedPeer>,
        &mut mono_proto_media::media::MediaIndex,
        i64,
        i32,
    )
        -> Result<updates_rpc::ChannelDifferencePage, MtprotoError>,
) -> Result<UpdateDrainDto, MtprotoError> {
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
    let expire_channel_flood_waits = |d: &mut ClientData| {
        let now = monotonic_now();
        let before = d.channel_flood_until.len();
        d.channel_flood_until.retain(|chat_id, until| {
            // On first use after restore, translate the wall deadline once.
            let (received, wait) = d.channel_flood_waits.entry(*chat_id).or_insert_with(|| {
                (now, Duration::from_secs(until.saturating_sub(wall_now())))
            });
            if now.saturating_duration_since(*received) < *wait {
                true
            } else {
                d.channel_flood_waits.remove(chat_id);
                false
            }
        });
        if before != d.channel_flood_until.len() {
            d.persist_epoch = d.persist_epoch.saturating_add(1);
            schedule_persist(&client, d.home_session_id);
        }
    };
    let now = wall_now();
    let (
        api_id,
        previous,
        home_dc,
        home_auth,
        home_salt,
        home_off,
        open_chat,
        user_id,
        session_id,
        due_recovery,
        lazy_pending,
    ) = {
        let mut d = client.data.lock();
        if d.session_dead {
            return Err(MtprotoError::Message(format!(
                "session invalidated: {}",
                d.session_dead_reason.as_deref().unwrap_or("UNKNOWN")
            )));
        }
        if d.user_id.is_none() || !d.updates_started {
            return Ok(UpdateDrainDto::default());
        }
        if let Some((started, wait)) = d.difference_flood_wait {
            if monotonic_now().saturating_duration_since(started) < wait {
                return Ok(UpdateDrainDto::default());
            }
            d.difference_flood_wait = None;
        }
        expire_channel_flood_waits(&mut d);
        (
            d.api_id,
            d.updates.clone(),
            d.home_dc,
            d.home_auth_key.clone(),
            d.home_salt,
            d.home_time_offset,
            d.last_history_chat_id,
            d.user_id,
            d.home_session_id,
            next_channel_recovery(
                &d.channel_recovery,
                d.last_history_chat_id,
                &d.lazy_sync_exceptions,
                d.lazy_channel_updates,
                now,
                |chat_id| !d.channel_flood_waits.contains_key(&chat_id),
            )
            .is_some()
                || (channel_id_from_chat_id(d.last_history_chat_id).is_some()
                    && !d.channel_flood_waits.contains_key(&d.last_history_chat_id)
                    && !d
                        .channel_recovery
                        .iter()
                        .any(|entry| entry.chat_id == d.last_history_chat_id)),
            d.lazy_channel_updates
                && !d.lazy_channel_recovery.is_empty()
                && lazy_retry_due(d.lazy_retry_at, monotonic_now()),
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
    let record_flood_wait = |io: &mut SessionIo, error: &MtprotoError| {
        if let Some(wait) = mono_proto_media::download::flood_wait_secs(error) {
            client.data.lock().difference_flood_wait =
                Some((monotonic_now(), Duration::from_secs(wait)));
            // An interactive RPC can reopen the transport before the next drain.
            io.last_difference = None;
        }
    };
    let mut slot = io.transport.take();
    mono_proto_runtime::rpc::clear_new_session_metadata();
    let mut received_channel_wait = false;
    let drained = with_client_transport(&client, &mut slot, || {
        // Receive stays on the idle budget; recovery RPCs use their own 8s scope
        // so a waiting interactive caller can take the lane between pages.
        ensure_auth_key_on(&mut io.snapshot)?;
        let cursor = match previous.clone() {
            Some(cursor) => cursor,
            None => get_updates_state(&mut io.snapshot, api_id)
                .inspect_err(|error| record_flood_wait(&mut io, error))?,
        };
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
        let mut live = cursor;
        let mut pending_channels = Vec::new();
        let mut stopped_channel = None;
        let mut events = Vec::new();
        let mut sync = DrainSync::default();
        let already_syncing = client.data.lock().is_syncing;
        if !already_syncing
            && catch_up_active(
                !needs_difference,
                visible_gap_recovery_pending(&recovery, open_chat, &lazy_exceptions, lazy_enabled),
                false,
            )
        {
            sync.start(already_syncing, &mut events);
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
            sync.start(client.data.lock().is_syncing, &mut events);
            let _span = mono_proto_media::perf::span("updates.difference");
            let (page, has_more) = mono_proto_runtime::rpc::with_rpc_timeout_secs(8, || {
                drain_difference(
                    &mut io.snapshot,
                    api_id,
                    &mut peers,
                    &mut media,
                    &mut live,
                    &mut pending_channels,
                )
            })
            .inspect_err(|error| record_flood_wait(&mut io, error))?;
            // The difference is authoritative for the cursor. Discard
            // packets retained behind the gap before using the new state.
            io.pending_push = Default::default();
            events.extend(page);
            io.last_difference = (!has_more).then(std::time::Instant::now);
        }
        let now = wall_now();
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
            let index = {
                let mut d = client.data.lock();
                expire_channel_flood_waits(&mut d);
                let current_open = d.last_history_chat_id;
                recovery.retain(|entry| !entry.watching || entry.chat_id == current_open);
                next_channel_recovery(
                    &recovery,
                    current_open,
                    &lazy_exceptions,
                    lazy_enabled,
                    now,
                    |chat_id| !d.channel_flood_waits.contains_key(&chat_id),
                )
            };
            let Some(index) = index else {
                break;
            };
            let mut entry = recovery.remove(index).unwrap();
            if !entry.watching {
                sync.start(client.data.lock().is_syncing, &mut events);
            }
            let chat_id = entry.chat_id;
            let pts = channel_pts.get(&chat_id).copied().unwrap_or(1);
            let _span = mono_proto_media::perf::span("updates.channel_diff");
            match mono_proto_runtime::rpc::with_rpc_timeout_secs(8, || {
                drain_channel_difference(
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
                        if let Some(wait) = mono_proto_media::download::flood_wait_secs(&err) {
                            let received = monotonic_now();
                            let wait = wait.max(5);
                            let mut d = client.data.lock();
                            if d.user_id != user_id || !session_lease_valid(&d, session_id) {
                                return Err(expired_session_lease());
                            }
                            d.channel_flood_waits.insert(chat_id, (received, Duration::from_secs(wait)));
                            d.channel_flood_until.insert(chat_id, wall_now().saturating_add(wait));
                            d.persist_epoch = d.persist_epoch.saturating_add(1);
                            received_channel_wait = true;
                            // Only the monotonic gate controls flood retry eligibility.
                            entry.due_at = 0;
                        } else {
                            entry.due_at = wall_now().saturating_add(recovery_wait_secs(&err));
                        }
                        recovery.push_back(entry);
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
            wall_now(),
        );
        let lazy_batch = if lazy_retry_due(lazy_retry_at, monotonic_now())
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
                    &channel_pts,
                    &lazy_batch,
                )
            }) {
                Ok(page) => {
                    lazy_retry_at = Some(lazy_retry_deadline(monotonic_now(), None));
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
                    lazy_retry_at = Some(lazy_retry_deadline(monotonic_now(), Some(&err)));
                    if plan.drop_transport {
                        mono_proto_runtime::rpc::drop_live_transport();
                    }
                }
            }
        }
        filter_update_events(&mut events, &mut seen_messages);
        Ok((
            events,
            Some(AppliedDrain {
                sync,
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
            if received_channel_wait {
                persist_journal_sync(&client, session_id)?;
            }
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
    let Some(mut applied) = applied else {
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
    let sync_event;
    {
        let mut d = client.data.lock();
        // A completed old-account poll must not repopulate state after logout
        // or authorization migration on the main lane.
        if d.user_id != user_id
            || d.home_dc != home_dc
            || d.home_auth_key != home_auth
            || !session_lease_valid(&d, session_id)
        {
            io.transport = None;
            return Err(expired_session_lease());
        }
        applied.recovery.retain(|entry| !entry.watching || entry.chat_id == d.last_history_chat_id);
        let is_syncing_now = catch_up_active(
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
        sync_event = applied.sync.finish(&mut d.is_syncing, is_syncing_now);
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
    if let Some(event) = sync_event {
        events.push(event);
    }
    let Some(batch) = journal_batch else {
        if received_channel_wait {
            persist_journal_sync(&client, session_id)?;
        } else if persist_needed {
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
