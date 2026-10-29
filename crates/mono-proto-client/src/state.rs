use mono_proto_media::media::MediaIndex;
use mono_proto_runtime::client_session::{ChannelRecovery, DcAuthKey, NewSessionMetadata};
use mono_proto_session::Snapshot;
use mono_proto_storage::update_journal::UpdateJournal;
use mono_proto_types::collections::{CompactString, HashMap, HashSet};
use mono_proto_types::peers::CachedPeer;
use mono_proto_types::updates_state::UpdatesStateDto;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Arc;

pub struct ClientData {
    pub api_id: i32,
    pub api_hash: String,
    pub session_path: PathBuf,
    pub user_id: Option<i64>,
    pub peers: HashMap<i64, CachedPeer>,
    pub updates: Option<UpdatesStateDto>,
    pub media: MediaIndex,
    pub updates_started: bool,
    pub session_dead: bool,
    /// Catalog token of the error that killed the session (`AUTH_KEY_DUPLICATED`).
    pub session_dead_reason: Option<String>,
    pub last_history_chat_id: i64,
    pub channel_pts: HashMap<i64, i32>,
    pub channel_recovery: VecDeque<ChannelRecovery>,
    pub seen_messages: HashSet<(i64, i32)>,
    pub logout_tokens: Vec<Vec<u8>>,
    pub new_session: Option<NewSessionMetadata>,
    pub home_snapshot: Snapshot,
    pub home_dc: i32,
    pub home_session_id: i64,
    pub home_auth_key: Option<Vec<u8>>,
    pub home_salt: i64,
    pub home_time_offset: i64,
    pub test_dc: bool,
    pub last_inline: Option<LastInlineQuery>,
    /// Imported foreign-DC master keys. Not a temporary PFS cache.
    pub dc_auth_keys: HashMap<i32, DcAuthKey>,
    /// Bound temporary keys for media lanes. Process-local; never persisted.
    pub dc_temp_auth: HashMap<i32, DcTempAuth>,
    pub persist_epoch: u64,
    pub persisted_epoch: u64,
    /// In-memory catch-up edge. Not written into the session file.
    pub is_syncing: bool,
    pub lazy_channel_updates: bool,
    pub lazy_sync_exceptions: HashSet<i64>,
    /// Optional metadata previews; gaps remain durable in `channel_recovery`.
    pub lazy_channel_recovery: HashSet<i64>,
    pub lazy_retry_at: u64,
    pub journal: Arc<UpdateJournal>,
}

#[derive(Clone)]
pub struct LastInlineQuery {
    pub chat_id: i64,
    pub bot_id: i64,
    pub query: CompactString,
    pub offset: CompactString,
    pub last_refresh: Option<std::time::Instant>,
}

/// Combined view for existing RPC closures. Snapshot lives on the main lane;
/// `data` is copied in/out so the data mutex is not held across TCP.
pub struct ClientState {
    pub api_id: i32,
    pub api_hash: String,
    pub session_path: PathBuf,
    pub snapshot: Snapshot,
    pub user_id: Option<i64>,
    pub peers: HashMap<i64, CachedPeer>,
    pub updates: Option<UpdatesStateDto>,
    pub media: MediaIndex,
    pub updates_started: bool,
    pub channel_pts: HashMap<i64, i32>,
    pub channel_recovery: VecDeque<ChannelRecovery>,
    pub seen_messages: HashSet<(i64, i32)>,
    pub logout_tokens: Vec<Vec<u8>>,
    pub new_session: Option<NewSessionMetadata>,
    /// Home auth key is dead (401 AUTH_KEY_* / SESSION_*). Fail RPCs without more network.
    pub session_dead: bool,
    pub session_dead_reason: Option<String>,
    pub test_dc: bool,
    pub last_inline: Option<LastInlineQuery>,
    /// Set while the live snapshot uses a PFS temp key. Disk writes keep this key.
    pub permanent: Option<Snapshot>,
    pub temp_expires_at: i32,
    pub dc_auth_keys: HashMap<i32, DcAuthKey>,
    pub journal: Arc<UpdateJournal>,
}

/// In-memory bound temporary key. Deliberately not `Debug`: it holds key material.
#[derive(Clone)]
pub struct DcTempAuth {
    pub snapshot: Snapshot,
    pub temp_expires_at: i32,
    pub perm_key: Vec<u8>,
}
