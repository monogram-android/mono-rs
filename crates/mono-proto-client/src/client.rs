use crate::lanes::{Lane, SessionIo};
use crate::state::ClientData;
use mono_proto_runtime::{scheduler, tcp};
use mono_proto_types::collections::SmallVec;
use parking_lot::Mutex;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64};

pub struct Client {
    pub _session_key: Option<Arc<mono_proto_storage::session_crypto::SessionKey>>,
    pub connections: Arc<tcp::ConnectionControl>,
    pub policy: std::sync::Arc<mono_proto_runtime::transfer_policy::TransferPolicy>,
    pub data: Mutex<ClientData>,
    pub main_gate: scheduler::LaneGate,
    pub main: Mutex<SessionIo>,
    /// Overlapping reads; never the updates subscriber.
    pub rpc: [Lane; scheduler::READ_LANES],
    /// File RPCs on separate sessions.
    pub media: SmallVec<[Lane; scheduler::MAX_MEDIA_LANES]>,
    pub upload: [Lane; scheduler::UPLOAD_LANES],
    pub file_cleanup_running: AtomicBool,
    pub media_open: std::sync::atomic::AtomicUsize,
    pub media_gate: scheduler::LaneGate,
    pub persist_queued: AtomicBool,
    pub persist_running: AtomicBool,
    pub persist_exhausted: AtomicBool,
    pub interactive_waiters: AtomicU64,
    pub active_update_request: AtomicU64,
}
