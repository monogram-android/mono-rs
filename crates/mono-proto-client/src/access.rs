use crate::client::Client;
use crate::lanes::{acquire_read_lane, lock_request_lane, schedule_file_cleanup};
use crate::merge::{merge_changed_entries, prefer_newer_cursor};
use crate::persist::{EXTRA_READ_LANE, durable_snapshot, schedule_persist};
use crate::registry::get_client;
use crate::session::{
    ensure_auth_key, expire_temporary_lane, fork_session, kills_session, mark_session_dead,
    temporary_key_failure, temporary_key_retry_error,
};
use crate::state::{ClientData, ClientState};
use mono_proto_runtime::{request_control, scheduler, tcp};
use mono_proto_session::{OsRandom, Snapshot};
use mono_proto_types::MtprotoError;
use std::cell::Cell;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

pub fn interactive_request_pending(client: &Client) -> bool {
    client.interactive_waiters.load(Ordering::Acquire) != 0
}

fn cancel_active_update(client: &Client) {
    let request_id = client.active_update_request.load(Ordering::Acquire);
    if request_id != 0 {
        request_control::cancel(request_id);
    }
}

pub struct InteractiveWaiter<'a>(pub &'a AtomicU64);

impl Drop for InteractiveWaiter<'_> {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::AcqRel);
    }
}

pub fn with_interactive_client_mut<T>(
    handle: u64,
    f: impl FnOnce(&mut ClientState) -> Result<T, MtprotoError>,
) -> Result<T, MtprotoError> {
    with_client_mut(handle, f)
}

pub fn with_client_transport<T>(
    client: &Client,
    slot: &mut Option<mono_proto_runtime::rpc::LiveTransport>,
    body: impl FnOnce() -> T,
) -> T {
    tcp::with_connection_control(&client.connections, || {
        mono_proto_runtime::rpc::with_live_transport(slot, body)
    })
}

pub fn client_api_id(handle: u64) -> i32 {
    get_client(handle)
        .map(|c| c.data.lock().api_id)
        .unwrap_or(0)
}

pub fn client_uses_test_dc(handle: u64) -> bool {
    get_client(handle)
        .map(|c| c.data.lock().test_dc)
        .unwrap_or(false)
}

pub fn with_client_mut<T>(
    handle: u64,
    f: impl FnOnce(&mut ClientState) -> Result<T, MtprotoError>,
) -> Result<T, MtprotoError> {
    with_client_mut_binding(handle, f, crate::bind_temp::negotiate_bound_temp)
}

pub fn with_client_mut_binding<T>(
    handle: u64,
    f: impl FnOnce(&mut ClientState) -> Result<T, MtprotoError>,
    negotiate: impl FnOnce(&[u8], i32, i64) -> Result<crate::bind_temp::BoundTemp, MtprotoError>,
) -> Result<T, MtprotoError> {
    let client = get_client(handle)?;
    let _policy = mono_proto_runtime::transfer_policy::bind(std::sync::Arc::clone(&client.policy));
    let class = scheduler::current_class();
    let _waiter = match class {
        scheduler::RequestClass::InteractiveRead | scheduler::RequestClass::InteractiveWrite => {
            client.interactive_waiters.fetch_add(1, Ordering::AcqRel);
            cancel_active_update(&client);
            Some(InteractiveWaiter(&client.interactive_waiters))
        }
        _ => None,
    };
    let _gate = {
        let _wait = mono_proto_media::perf::span("gate_wait.main");
        client.main_gate.acquire(class)?
    };
    let mut io = lock_request_lane(&client.main, "lane_wait.main")?;
    crate::bind_temp::ensure_main_temp_with(
        &client,
        &mut io,
        false,
        &mono_proto_runtime::rpc::SystemClock,
        negotiate,
    )?;
    let mut state = {
        let d = client.data.lock();
        ClientState {
            api_id: d.api_id,
            api_hash: d.api_hash.clone(),
            session_path: d.session_path.clone(),
            snapshot: io.snapshot.clone(),
            user_id: d.user_id,
            peers: d.peers.clone(),
            updates: d.updates.clone(),
            media: d.media.clone(),
            updates_started: d.updates_started,
            channel_pts: d.channel_pts.clone(),
            channel_recovery: d.channel_recovery.clone(),
            seen_messages: d.seen_messages.clone(),
            logout_tokens: d.logout_tokens.clone(),
            new_session: d.new_session.clone(),
            session_dead: d.session_dead,
            session_dead_reason: d.session_dead_reason.clone(),
            test_dc: d.test_dc,
            last_inline: d.last_inline.clone(),
            permanent: (io.temp_expires_at != 0 || io.perm_key.is_some())
                .then(|| d.home_snapshot.clone()),
            temp_expires_at: io.temp_expires_at,
            dc_auth_keys: d.dc_auth_keys.clone(),
            journal: Arc::clone(&d.journal),
        }
    };
    mono_proto_runtime::rpc::set_use_test_dc(state.test_dc);
    mono_proto_runtime::rpc::clear_new_session_metadata();
    let before_peers = state.peers.clone();
    let before_media = state.media.clone();
    let before_channel_pts = state.channel_pts.clone();
    let before_recovery = state.channel_recovery.clone();
    let before_user_id = state.user_id;
    let before_updates = state.updates.clone();
    let before_logout = state.logout_tokens.clone();
    let before_new_session = state.new_session.clone();
    let before_dead = state.session_dead;
    let before_test_dc = state.test_dc;
    let before_dc_auth_keys = state.dc_auth_keys.clone();
    let mut slot = io.transport.take();
    let before_snapshot = io.snapshot.clone();
    let result = with_client_transport(&client, &mut slot, || f(&mut state));
    let temp_failed = result
        .as_ref()
        .err()
        .is_some_and(|err| temporary_key_failure(&state, err));
    if !temp_failed && !state.session_dead {
        if let Err(error) = &result {
            if kills_session(state.user_id, error) {
                mark_session_dead(&mut state, error);
            }
        }
    }
    if temp_failed {
        expire_temporary_lane(&client, &mut io, result.as_ref().err().unwrap());
        state.temp_expires_at = 1;
    }
    if let Some(metadata) = mono_proto_runtime::rpc::take_new_session_metadata() {
        state.new_session = Some(metadata);
    }
    // A session/auth-key or session_id change invalidates the old TCP lane.
    // Never put a transport bound to the previous session back into the slot
    // after auth restart, bad-msg recovery, DC migration, or key recreation.
    let snapshot_changed = before_snapshot.dc_id != state.snapshot.dc_id
        || before_snapshot.auth_key != state.snapshot.auth_key
        || before_snapshot.session_id != state.snapshot.session_id;
    io.transport = if snapshot_changed || state.session_dead || temp_failed {
        None
    } else {
        slot
    };
    io.snapshot = state.snapshot.clone();
    if snapshot_changed || state.session_dead {
        io.pending_push = Default::default();
        io.last_difference = None;
    }
    let identity_changed =
        before_user_id != state.user_id || before_snapshot.auth_key != state.snapshot.auth_key;
    let drop_dc_auth = identity_changed || before_test_dc != state.test_dc;
    if drop_dc_auth {
        state.dc_auth_keys.clear();
    }
    let dc_keys_changed = before_dc_auth_keys != state.dc_auth_keys;
    if identity_changed {
        client
            .policy
            .bound_extra_sessions
            .store(0, Ordering::Release);
        client
            .policy
            .main_session_allowance
            .store(-1, Ordering::Release);
        if state.user_id.is_none() || before_user_id.is_some() {
            client.policy.premium.store(false, Ordering::Relaxed);
        }
        *client.policy.upload_rtt.lock() = None;
    }
    let updates_changed = before_updates != state.updates;
    let tokens_changed = before_logout != state.logout_tokens;
    let session_meta_changed = before_new_session != state.new_session
        || before_dead != state.session_dead
        || before_test_dc != state.test_dc;
    let home = durable_snapshot(&state);
    if let Some(permanent) = &state.permanent {
        io.perm_key = permanent.auth_key.clone();
        io.temp_expires_at = state.temp_expires_at.max(1);
    } else if state.snapshot.auth_key != before_snapshot.auth_key
        || state.snapshot.dc_id != before_snapshot.dc_id
    {
        io.temp_expires_at = 0;
        io.perm_key = None;
    }
    let persist_needed;
    {
        let mut d = client.data.lock();
        d.user_id = state.user_id;
        let maps_changed = if before_user_id != state.user_id {
            d.peers = state.peers;
            d.media = state.media;
            d.updates = state.updates;
            d.channel_pts = state.channel_pts;
            d.channel_recovery = state.channel_recovery;
            d.seen_messages = state.seen_messages;
            d.updates_started = state.updates_started;
            if before_user_id.is_some() {
                Arc::make_mut(&mut d.journal).reset_account();
            }
            true
        } else {
            let peers_changed = merge_changed_entries(&mut d.peers, &before_peers, state.peers);
            let media_changed = merge_changed_entries(&mut d.media, &before_media, state.media);
            let channels_changed =
                merge_changed_entries(&mut d.channel_pts, &before_channel_pts, state.channel_pts);
            let recovery_changed = state.channel_recovery != before_recovery
                && merge_changed_entries(
                    &mut d.channel_recovery,
                    &before_recovery,
                    state.channel_recovery,
                );
            let seen_before = d.seen_messages.len();
            d.seen_messages.extend(state.seen_messages);
            d.updates = prefer_newer_cursor(d.updates.clone(), state.updates);
            d.updates_started |= state.updates_started;
            peers_changed
                || media_changed
                || channels_changed
                || recovery_changed
                || seen_before != d.seen_messages.len()
                || updates_changed
        };
        d.logout_tokens = state.logout_tokens;
        d.new_session = state.new_session;
        d.session_dead = if before_dead {
            state.session_dead
        } else {
            d.session_dead || state.session_dead
        };
        if before_dead || state.session_dead || !d.session_dead {
            d.session_dead_reason = if state.session_dead {
                state.session_dead_reason.clone()
            } else {
                None
            };
        }
        if d.session_dead {
            io.transport = None;
        }
        d.home_dc = home.dc_id;
        d.home_time_offset = home.time_offset_micros;
        d.home_session_id = home.session_id;
        d.home_auth_key = home.auth_key.clone();
        d.home_salt = home.server_salt;
        d.home_snapshot = home;
        d.test_dc = state.test_dc;
        d.last_inline = state.last_inline;
        if dc_keys_changed {
            d.dc_auth_keys = state.dc_auth_keys.clone();
        }
        if drop_dc_auth {
            d.dc_temp_auth.clear();
        }
        persist_needed = result.is_ok()
            && (identity_changed
                || snapshot_changed
                || maps_changed
                || tokens_changed
                || session_meta_changed
                || dc_keys_changed);
        if persist_needed {
            d.persist_epoch = d.persist_epoch.saturating_add(1);
        }
    }
    if identity_changed {
        for lane in client.rpc.iter() {
            if let Some(mut rpc) = lane.io.try_lock() {
                rpc.temp_expires_at = 0;
                rpc.transport = None;
                rpc.pending_push = Default::default();
                rpc.last_difference = None;
                if io.temp_expires_at == 0 && io.perm_key.is_none() {
                    rpc.snapshot = fork_session(&io.snapshot);
                }
            }
        }
    }
    if persist_needed {
        let persist_id = client.data.lock().home_session_id;
        schedule_persist(&client, persist_id);
    }
    schedule_file_cleanup(&client);
    if temp_failed {
        Err(temporary_key_retry_error())
    } else {
        result
    }
}

pub fn home_fork_source(data: &ClientData) -> Option<Snapshot> {
    let auth = data.home_auth_key.clone()?;
    if auth.len() != 256 || data.user_id.is_none() || data.session_dead {
        return None;
    }
    let mut snap = Snapshot::new(data.home_dc, &mut OsRandom).ok()?;
    snap.auth_key = Some(auth);
    snap.server_salt = data.home_salt;
    snap.time_offset_micros = data.home_time_offset;
    snap.session_id = data.home_session_id;
    Some(snap)
}

pub fn with_read_lane<T>(
    handle: u64,
    mut f: impl FnMut(&mut ClientState) -> Result<T, MtprotoError>,
) -> Result<T, MtprotoError> {
    let client = get_client(handle)?;
    let _policy = mono_proto_runtime::transfer_policy::bind(client.policy.clone());
    // A non-media DC permits a single main session. Extra read lanes are home-DC
    // sessions, so they run only when the server granted them (`config.tmp_sessions`).
    if scheduler::extra_main_sessions() == 0 {
        return with_client_mut(handle, f);
    }
    let home = {
        let d = client.data.lock();
        home_fork_source(&d)
    };
    let Some(home) = home else {
        return with_client_mut(handle, f);
    };
    // Reads wait for a read lane; the `with_client_mut` paths below are authorization
    // repairs, not contention fallbacks.
    let mut io = acquire_read_lane(&client)?;
    crate::bind_temp::ensure_main_temp(&client, &mut io, false)?;
    let bound_temp = crate::bind_temp::temp_key_valid(&io.snapshot, io.temp_expires_at)
        && io.snapshot.auth_key.as_ref() != home.auth_key.as_ref()
        && io.snapshot.auth_key.as_ref().map(Vec::len) == Some(256);
    if !bound_temp {
        drop(io);
        return with_client_mut(handle, f);
    }
    let home_session_id = home.session_id;
    let home_dc = home.dc_id;
    let home_auth = home.auth_key.clone();
    let built = {
        let d = client.data.lock();
        if d.session_dead || d.home_session_id != home_session_id {
            None
        } else {
            Some(ClientState {
                api_id: d.api_id,
                api_hash: d.api_hash.clone(),
                session_path: d.session_path.clone(),
                snapshot: io.snapshot.clone(),
                user_id: d.user_id,
                peers: d.peers.clone(),
                updates: d.updates.clone(),
                media: d.media.clone(),
                updates_started: d.updates_started,
                channel_pts: d.channel_pts.clone(),
                channel_recovery: d.channel_recovery.clone(),
                seen_messages: d.seen_messages.clone(),
                logout_tokens: d.logout_tokens.clone(),
                new_session: d.new_session.clone(),
                session_dead: d.session_dead,
                session_dead_reason: d.session_dead_reason.clone(),
                test_dc: d.test_dc,
                last_inline: d.last_inline.clone(),
                permanent: (io.temp_expires_at != 0 || io.perm_key.is_some())
                    .then(|| d.home_snapshot.clone()),
                temp_expires_at: io.temp_expires_at,
                dc_auth_keys: d.dc_auth_keys.clone(),
                journal: Arc::clone(&d.journal),
            })
        }
    };
    let mut state = match built {
        Some(state) => state,
        None => {
            drop(io);
            return with_client_mut(handle, f);
        }
    };
    mono_proto_runtime::rpc::set_use_test_dc(state.test_dc);
    mono_proto_runtime::rpc::clear_new_session_metadata();
    let before_peers = state.peers.clone();
    let before_media = state.media.clone();
    let before_channel_pts = state.channel_pts.clone();
    let before_recovery = state.channel_recovery.clone();
    let before_dc_auth_keys = state.dc_auth_keys.clone();
    let mut slot = io.transport.take();
    let before_snapshot = io.snapshot.clone();
    let result = EXTRA_READ_LANE.with(|cell| {
        struct Restore<'a>(&'a Cell<bool>, bool);
        impl Drop for Restore<'_> {
            fn drop(&mut self) {
                self.0.set(self.1);
            }
        }
        let previous = cell.replace(true);
        let _restore = Restore(cell, previous);
        mono_proto_runtime::api_invoke::with_invoke_without_updates(|| {
            with_client_transport(&client, &mut slot, || f(&mut state))
        })
    });
    if let Some(metadata) = mono_proto_runtime::rpc::take_new_session_metadata() {
        state.new_session = Some(metadata);
    }
    let temp_failed = result
        .as_ref()
        .err()
        .is_some_and(|err| temporary_key_failure(&state, err));
    if temp_failed {
        expire_temporary_lane(&client, &mut io, result.as_ref().err().unwrap());
        return Err(temporary_key_retry_error());
    }
    let still_temp = (io.temp_expires_at != 0 || io.perm_key.is_some())
        && state.snapshot.auth_key.as_ref() != home_auth.as_ref();
    let diverged =
        state.snapshot.dc_id != home_dc || (state.snapshot.auth_key != home_auth && !still_temp);
    if diverged {
        io.temp_expires_at = 0;
        io.perm_key = None;
        io.snapshot = fork_session(&home);
        io.transport = None;
        drop(io);
        return with_client_mut(handle, f);
    }
    let snapshot_changed = before_snapshot.dc_id != state.snapshot.dc_id
        || before_snapshot.auth_key != state.snapshot.auth_key
        || before_snapshot.session_id != state.snapshot.session_id;
    io.transport = if snapshot_changed || state.session_dead {
        None
    } else {
        slot
    };
    io.snapshot = state.snapshot.clone();
    if snapshot_changed || state.session_dead {
        io.pending_push = Default::default();
        io.last_difference = None;
    }
    {
        let mut d = client.data.lock();
        if d.home_session_id == home_session_id && !d.session_dead {
            let _ = merge_changed_entries(&mut d.peers, &before_peers, state.peers);
            let _ = merge_changed_entries(&mut d.media, &before_media, state.media);
            let _ =
                merge_changed_entries(&mut d.channel_pts, &before_channel_pts, state.channel_pts);
            if state.channel_recovery != before_recovery {
                merge_changed_entries(
                    &mut d.channel_recovery,
                    &before_recovery,
                    state.channel_recovery,
                );
            }
            d.seen_messages.extend(state.seen_messages);
            d.updates = prefer_newer_cursor(d.updates.clone(), state.updates);
            d.updates_started |= state.updates_started;
            d.logout_tokens = state.logout_tokens;
            if state.new_session.is_some() {
                d.new_session = state.new_session;
            }
            let terminal_changed = !d.session_dead && state.session_dead;
            d.session_dead |= state.session_dead;
            if state.session_dead {
                d.session_dead_reason = state.session_dead_reason.clone();
            }
            d.last_inline = state.last_inline;
            if state.permanent.is_none() && state.snapshot.auth_key == d.home_auth_key {
                d.home_salt = state.snapshot.server_salt;
                d.home_snapshot.server_salt = state.snapshot.server_salt;
            }
            d.home_time_offset = state.snapshot.time_offset_micros;
            if before_dc_auth_keys != state.dc_auth_keys {
                d.dc_auth_keys = state.dc_auth_keys.clone();
            }
            if result.is_ok() || terminal_changed {
                d.persist_epoch = d.persist_epoch.saturating_add(1);
            }
        }
    }
    drop(io);
    schedule_file_cleanup(&client);
    if result
        .as_ref()
        .err()
        .and_then(mono_proto_runtime::api_invoke::migrate_dc)
        .is_none()
    {
        schedule_persist(&client, home_session_id);
    }
    if result
        .as_ref()
        .err()
        .and_then(mono_proto_runtime::api_invoke::migrate_dc)
        .is_some()
    {
        return with_client_mut(handle, f);
    }
    result
}

pub fn ensure_ready(state: &mut ClientState) -> Result<(), MtprotoError> {
    ensure_auth_key(state)
}
