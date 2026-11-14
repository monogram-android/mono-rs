//! Deterministic, network-free microbench support. Timings exclude fixture reset
//! and returned-output destruction. These are not binary before/after tests.
use mono_proto_types::collections::{HashMap, HashMapExt, IndexMap};
use mono_proto_types::media_types::{MediaLocation, MediaRef};
use mono_proto_types::peers::{CachedPeer, PeerKind};
use std::hint::black_box;
use std::time::Instant;

pub const COUNT: usize = 8_702;
pub const SAMPLES: usize = 11;
pub const WARM: usize = 3;
pub type Index = IndexMap<(i64, i32), MediaRef>;

pub fn fixture() -> (HashMap<i64, CachedPeer>, Index, usize) {
    let mut peers = HashMap::with_capacity(COUNT);
    let mut media = Index::default();
    let mut bytes = 0;
    for n in 1..=COUNT {
        let id = n as i64;
        peers.insert(id, CachedPeer { kind: PeerKind::User, id, access_hash: id * 31, min_hash: false });
        // Mixed unresolved/resolved avatars with small inline JPEG-like previews;
        // every fourth photo has a full 4096-byte inline payload.
        let len = if n % 4 == 0 { 4096 } else { 96 + n % 417 };
        let mut payload: Vec<u8> = (0..len).map(|i| ((i * 17 + n) % 256) as u8).collect();
        payload[..2].copy_from_slice(&[0xff, 0xd8]);
        bytes += len;
        let location = if n % 4 == 0 {
            MediaLocation::Inline { bytes: payload.clone(), extension: "jpg".into() }
        } else {
            MediaLocation::PeerPhoto { peer_kind: PeerKind::User, peer_id: id, access_hash: 0, photo_id: id * 7, big: false, dc_id: 2 }
        };
        media.insert((id, 0), MediaRef {
            file_size: None, kind: "photo".into(), cache_key: format!("avatar:{id}"), location,
            thumb_cache_key: Some(format!("avatar:{id}:thumb")),
            thumb_location: Some(MediaLocation::Inline { bytes: payload, extension: "jpg".into() }),
            display_cache_key: None, display_location: None, sticker_set_id: None,
            sticker_set_access_hash: None, source_url: None,
        });
        if n % 4 == 0 { bytes += len; }
    }
    (peers, media, bytes)
}

pub fn bench<T>(name: &str, size: usize, mut run: impl FnMut() -> T) {
    bench_reset(name, size, || (), |_| run());
}

pub fn bench_reset<S, T>(name: &str, size: usize, mut setup: impl FnMut() -> S, mut run: impl FnMut(S) -> T) {
    let mut times = Vec::with_capacity(SAMPLES);
    for i in 0..WARM + SAMPLES {
        let input = setup();
        let start = Instant::now();
        let output = black_box(run(black_box(input)));
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        drop(output);
        if i >= WARM { times.push(elapsed); }
    }
    times.sort_by(f64::total_cmp);
    let p95 = (SAMPLES * 95).div_ceil(100) - 1;
    println!("NATIVE_BENCH name={name} count={COUNT} size_bytes={size} warm={WARM} samples={SAMPLES} unit=ms median={:.6} p95={:.6} min={:.6} max={:.6}", times[SAMPLES / 2], times[p95], times[0], times[SAMPLES - 1]);
}
