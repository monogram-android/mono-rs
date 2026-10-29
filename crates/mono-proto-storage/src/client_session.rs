use crate::collections::{HashMap, HashSet};
use crate::media_types::MediaRef;
use crate::peers::CachedPeer;
use crate::updates_state::UpdatesStateDto;
use mono_proto_session::Snapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NewSessionMetadata {
    pub first_msg_id: i64,
    pub unique_id: i64,
    pub server_salt: i64,
}

/// Imported master key for one foreign datacenter. Key bytes stay out of `Debug`.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DcAuthKey {
    pub key: Vec<u8>,
    pub salt: i64,
}

impl std::fmt::Debug for DcAuthKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DcAuthKey")
            .field("key_len", &self.key.len())
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChannelRecovery {
    pub chat_id: i64,
    pub due_at: u64,
    /// Completed open-channel subscription; drop when the user leaves it.
    pub watching: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientSession {
    pub snapshot: Snapshot,
    #[serde(default)]
    pub user_id: Option<i64>,
    #[serde(default)]
    pub peers: HashMap<i64, CachedPeer>,
    #[serde(default)]
    pub updates: Option<UpdatesStateDto>,
    /// Media download metadata keyed as "chatId:messageId".
    #[serde(default)]
    pub media: HashMap<String, MediaRef>,
    /// Oldest-first keys for `media`. Empty on sessions saved before retention.
    #[serde(default)]
    pub media_order: Vec<String>,
    /// Last applied per-channel pts, keyed by chat id.
    #[serde(default)]
    pub channel_pts: HashMap<i64, i32>,
    #[serde(default)]
    pub channel_recovery: std::collections::VecDeque<ChannelRecovery>,
    /// Deduplication keys for update events.
    #[serde(default)]
    pub seen_messages: HashSet<(i64, i32)>,
    /// Home auth key was rejected (401 AUTH_KEY_* / SESSION_* / USER_DEACTIVATED).
    #[serde(default)]
    pub session_dead: bool,
    /// Bounded future auth tokens returned by `auth.logOut`.
    #[serde(default)]
    pub logout_tokens: Vec<Vec<u8>>,
    /// Last `new_session_created` metadata observed on any lane.
    #[serde(default)]
    pub new_session: Option<NewSessionMetadata>,
    /// Connected to Telegram test DCs (`99966XYYYY`).
    #[serde(default)]
    pub test_dc: bool,
    /// Imported master keys for foreign datacenters, keyed by DC id.
    #[serde(default)]
    pub dc_auth_keys: HashMap<i32, DcAuthKey>,
    #[serde(default)]
    pub journal: crate::update_journal::UpdateJournal,
}
