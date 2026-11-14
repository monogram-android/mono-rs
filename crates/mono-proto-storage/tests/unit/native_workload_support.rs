// Shared synthetic workload and timing support. No wire buffers or network I/O.
// Only the requested operation is timed; setup, validation and output drops are not.
use super::storage::client_session::ClientSession;
use mono_proto_session::{OsRandom, Snapshot};
use mono_proto_types::{
    collections::{HashMap, HashMapExt, HashSet, HashSetExt},
    media_types::{MediaLocation, MediaRef},
    peers::{CachedPeer, PeerKind},
};
use std::{hint::black_box, path::PathBuf, time::Instant};

pub const CLASSES: [(&str, usize); 3] = [("small", 8), ("realistic", 8_702), ("stress", 34_808)];
pub const WARM: usize = 3;
pub const SAMPLES: usize = 11;

pub struct TempDir(pub PathBuf);
impl TempDir {
    pub fn new() -> Self {
        let mut nonce = [0_u8; 16];
        mono_proto_crypto::fill_random(&mut nonce).unwrap();
        let path = std::env::temp_dir().join(format!(
            "mono-storage-workload-{}-{:032x}",
            std::process::id(),
            u128::from_le_bytes(nonce)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Opaque synthetic file references are DTO data, never invented serialized state.
/// A full snapshot always comes from Snapshot::new and is validated before use.
pub fn fixture(count: usize) -> ClientSession {
    let mut snapshot = Snapshot::new(2, &mut OsRandom).unwrap();
    snapshot.auth_key = Some(vec![0x31; 256]);
    snapshot.validate().unwrap();
    let mut session = ClientSession {
        snapshot,
        user_id: Some(42),
        peers: HashMap::with_capacity(count),
        updates: None,
        media: HashMap::with_capacity(count),
        media_order: Vec::with_capacity(count),
        channel_pts: HashMap::new(),
        channel_recovery: Default::default(),
        seen_messages: HashSet::new(),
        session_dead: false,
        logout_tokens: Vec::new(),
        new_session: None,
        test_dc: true,
        dc_auth_keys: HashMap::new(),
        journal: Default::default(),
    };
    for n in 1..=count {
        let id = n as i64;
        session.peers.insert(
            id,
            CachedPeer {
                kind: PeerKind::User,
                id,
                access_hash: id * 31,
                min_hash: false,
            },
        );
        let key = format!("{id}:1");
        session.media_order.push(key.clone());
        session.media.insert(
            key,
            MediaRef {
                file_size: Some(1_048_576 + id),
                kind: "document".into(),
                cache_key: format!("synthetic-document:{id}"),
                location: MediaLocation::Document {
                    id,
                    access_hash: id * 17,
                    file_reference: (0..384).map(|i| ((i + n) % 256) as u8).collect(),
                    thumb_size: String::new(),
                    dc_id: 2,
                    mime_type: "application/octet-stream".into(),
                },
                thumb_cache_key: None,
                thumb_location: None,
                display_cache_key: None,
                display_location: None,
                sticker_set_id: None,
                sticker_set_access_hash: None,
                source_url: None,
            },
        );
    }
    session
}

pub fn measure<S, T>(
    op: &str,
    class: &str,
    count: usize,
    bytes: usize,
    mut setup: impl FnMut() -> S,
    mut run: impl FnMut(S) -> T,
    mut check: impl FnMut(&T),
) {
    let mut raw = Vec::with_capacity(SAMPLES);
    for sample in 0..WARM + SAMPLES {
        let input = setup();
        let start = Instant::now();
        let output = black_box(run(black_box(input)));
        let ns = start.elapsed().as_nanos();
        check(&output);
        drop(output);
        if sample >= WARM {
            raw.push(ns);
        }
    }
    let mut sorted = raw.clone();
    sorted.sort_unstable();
    println!(
        "NATIVE_WORKLOAD op={op} class={class} count={count} bytes={bytes} warm={WARM} samples={SAMPLES} unit=ns raw={raw:?} median={} p95={}",
        sorted[SAMPLES / 2],
        sorted[(SAMPLES * 95).div_ceil(100) - 1]
    );
}
