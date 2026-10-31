use crate::client::Client;
use crate::lanes::{Lane, SessionIo};
use crate::registry::CLIENTS;
use crate::session::fork_session;
use crate::session_store::FileSessionStore;
use crate::state::ClientData;
use mono_proto_media::media::MediaIndex;
use mono_proto_media::persistence::media_to_index;
use mono_proto_runtime::{scheduler, tcp};
use mono_proto_session::{OsRandom, Snapshot};
use mono_proto_types::collections::{HashMap, HashMapExt, HashSet, HashSetExt};
use parking_lot::Mutex;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64};

#[cfg(test)]
#[path = "../tests/unit/lifecycle_tests.rs"]
mod tests;

const DEFAULT_DC_ID: i32 = 2;

pub fn create_encrypted_client(
    api_id: i32,
    api_hash: String,
    session_path: String,
    key: Vec<u8>,
) -> Result<u64, mono_proto_types::MtprotoError> {
    let key = zeroize::Zeroizing::new(key);
    let guard =
        mono_proto_storage::session_crypto::register(std::path::Path::new(&session_path), &key)
            .map_err(|_| {
                mono_proto_types::MtprotoError::Message("session key unavailable".into())
            })?;
    FileSessionStore::new(&session_path)
        .load()
        .map_err(|_| mono_proto_types::MtprotoError::Message("session restore failed".into()))?;
    let handle = create_client(api_id, api_hash, session_path);
    drop(guard);
    Ok(handle)
}

pub fn create_client(api_id: i32, api_hash: String, session_path: String) -> u64 {
    let connections = Arc::new(tcp::ConnectionControl::default());
    let path = PathBuf::from(session_path);
    let store = FileSessionStore::new(&path);
    let (loaded, load_failed) = match store.load() {
        Ok(session) => (session, false),
        Err(_) => (None, true),
    };
    let channel_recovery = loaded
        .as_ref()
        .map(|s| s.channel_recovery.clone())
        .unwrap_or_default();
    let (
        snapshot,
        user_id,
        peers,
        updates,
        media,
        channel_pts,
        seen_messages,
        logout_tokens,
        new_session,
        session_dead,
        test_dc,
        dc_auth_keys,
        journal,
    ) = match loaded {
        Some(session) => (
            session.snapshot,
            session.user_id,
            session.peers,
            session.updates,
            media_to_index(&session.media, &session.media_order),
            session.channel_pts,
            session.seen_messages,
            session.logout_tokens,
            session.new_session,
            session.session_dead,
            session.test_dc,
            session.dc_auth_keys,
            session.journal,
        ),
        None => (
            Snapshot::new(DEFAULT_DC_ID, &mut OsRandom).expect("snapshot"),
            None,
            HashMap::new(),
            None,
            MediaIndex::default(),
            HashMap::new(),
            HashSet::new(),
            Vec::new(),
            None,
            load_failed,
            false,
            HashMap::new(),
            Default::default(),
        ),
    };
    mono_proto_runtime::rpc::set_use_test_dc(test_dc);
    let media_snapshot = fork_session(&snapshot);
    let rpc_snapshot = fork_session(&snapshot);
    let rpc_snapshot_b = fork_session(&snapshot);
    let media_snapshots: Vec<Snapshot> = (0..scheduler::MAX_MEDIA_LANES)
        .map(|index| {
            if index == 0 {
                media_snapshot.clone()
            } else {
                fork_session(&snapshot)
            }
        })
        .collect();
    let upload_lanes = core::array::from_fn(|_| {
        Lane::new(SessionIo {
            pending_push: Default::default(),
            last_difference: None,
            snapshot: fork_session(&snapshot),
            transport: None,
            temp_expires_at: 0,
            perm_key: None,
        })
    });
    let handle = CLIENTS.next_handle();
    CLIENTS.lock().insert(
        handle,
        Arc::new(Client {
            _session_key: mono_proto_storage::session_crypto::key_for(&path),
            connections,
            policy: mono_proto_runtime::transfer_policy::snapshot(),
            data: Mutex::new(ClientData {
                api_id,
                api_hash,
                session_path: path,
                user_id,
                peers,
                updates,
                media,
                updates_started: false,
                session_dead,
                session_dead_reason: None,
                last_history_chat_id: 0,
                channel_pts,
                channel_recovery,
                seen_messages,
                logout_tokens,
                new_session,
                home_snapshot: snapshot.clone(),
                home_dc: snapshot.dc_id,
                home_session_id: snapshot.session_id,
                home_auth_key: snapshot.auth_key.clone(),
                home_salt: snapshot.server_salt,
                home_time_offset: snapshot.time_offset_micros,
                test_dc,
                last_inline: None,
                dc_auth_keys,
                dc_temp_auth: HashMap::new(),
                persist_epoch: 0,
                persisted_epoch: 0,
                is_syncing: false,
                lazy_channel_updates: false,
                lazy_sync_exceptions: HashSet::new(),
                lazy_channel_recovery: HashSet::new(),
                lazy_retry_at: 0,
                journal: Arc::new(journal),
            }),
            main_gate: scheduler::LaneGate::new(),
            main: Mutex::new(SessionIo {
                pending_push: Default::default(),
                last_difference: None,
                snapshot,
                transport: None,
                temp_expires_at: 0,
                perm_key: None,
            }),
            rpc: [
                Lane::new(SessionIo {
                    pending_push: Default::default(),
                    last_difference: None,
                    snapshot: rpc_snapshot,
                    transport: None,
                    temp_expires_at: 0,
                    perm_key: None,
                }),
                Lane::new(SessionIo {
                    pending_push: Default::default(),
                    last_difference: None,
                    snapshot: rpc_snapshot_b,
                    transport: None,
                    temp_expires_at: 0,
                    perm_key: None,
                }),
            ],
            media: media_snapshots
                .into_iter()
                .map(|snapshot| {
                    Lane::new(SessionIo {
                        snapshot,
                        pending_push: Default::default(),
                        last_difference: None,
                        transport: None,
                        temp_expires_at: 0,
                        perm_key: None,
                    })
                })
                .collect(),
            upload: upload_lanes,
            file_cleanup_running: AtomicBool::new(false),
            media_open: std::sync::atomic::AtomicUsize::new(1),
            media_gate: scheduler::LaneGate::new(),
            persist_queued: AtomicBool::new(false),
            persist_running: AtomicBool::new(false),
            persist_exhausted: AtomicBool::new(false),
            interactive_waiters: AtomicU64::new(0),
            active_update_request: AtomicU64::new(0),
        }),
    );
    handle
}
