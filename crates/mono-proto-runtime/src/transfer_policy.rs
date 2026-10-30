//! Per-client transfer limits.
//! https://core.telegram.org/api/files
//! https://core.telegram.org/api/optimisation

use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock};

use crate::HashMapExt;
use parking_lot::RwLock;

pub static VISIBLE_TRANSFER_WAITERS: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

pub const DEFAULT_PIPELINE_PARTS: usize = 4;
pub const MAX_PIPELINE_PARTS: usize = 8;
pub const DEFAULT_MEDIA_LANES: usize = 2;
pub const MAX_MEDIA_LANES: usize = 8;
use crate::transfer_window::DEFAULT_CHUNK;

use crate::upload_limits::{FILE_PART_FAST, FILE_PART_SMALL};

pub type ProgressCallback = Arc<dyn Fn(&str, i64, i64) + Send + Sync + 'static>;

pub struct TransferPolicy {
    pub main_session_allowance: AtomicI32,
    pub bound_extra_sessions: AtomicUsize,
    media_lanes: AtomicUsize,
    pub premium: AtomicBool,
    pub upload_parts: parking_lot::Mutex<(i32, i32)>,
    pipeline_parts: AtomicUsize,
    chunk_size: AtomicI32,
    file_part: AtomicUsize,
    progress: RwLock<Option<ProgressCallback>>,
    pub floods: parking_lot::Mutex<crate::transfer_window::FloodScope>,
    pub queue_limits: parking_lot::Mutex<(usize, usize)>,
    downloads: parking_lot::Mutex<ActiveDownloads>,
    download_wake: parking_lot::Condvar,
    pub upload_rtt: parking_lot::Mutex<Option<(i32, std::time::Instant, std::time::Duration)>>,
    pub config_due: std::sync::atomic::AtomicU64,
    media_lanes_owned: AtomicBool,
    pipeline_owned: AtomicBool,
    chunk_owned: AtomicBool,
    file_part_owned: AtomicBool,
    progress_owned: AtomicBool,
}

impl TransferPolicy {
    pub fn stock() -> Self {
        Self {
            main_session_allowance: AtomicI32::new(-1),
            bound_extra_sessions: AtomicUsize::new(0),
            media_lanes: AtomicUsize::new(DEFAULT_MEDIA_LANES),
            premium: AtomicBool::new(false),
            upload_parts: parking_lot::Mutex::new((4_000, 8_000)),
            pipeline_parts: AtomicUsize::new(DEFAULT_PIPELINE_PARTS),
            chunk_size: AtomicI32::new(DEFAULT_CHUNK),
            file_part: AtomicUsize::new(FILE_PART_FAST),
            progress: RwLock::new(None),
            floods: parking_lot::Mutex::new(crate::transfer_window::FloodScope::new()),
            queue_limits: parking_lot::Mutex::new((5, 2)),
            downloads: parking_lot::Mutex::new(ActiveDownloads::default()),
            download_wake: parking_lot::Condvar::new(),
            upload_rtt: parking_lot::Mutex::new(None),
            config_due: std::sync::atomic::AtomicU64::new(0),
            media_lanes_owned: AtomicBool::new(false),
            pipeline_owned: AtomicBool::new(false),
            chunk_owned: AtomicBool::new(false),
            file_part_owned: AtomicBool::new(false),
            progress_owned: AtomicBool::new(false),
        }
    }

    pub fn upload_max_parts(&self) -> i32 {
        let limits = *self.upload_parts.lock();
        if self.premium.load(Ordering::Relaxed) {
            limits.1
        } else {
            limits.0
        }
    }

    pub fn media_lanes(&self) -> usize {
        self.media_lanes
            .load(Ordering::Relaxed)
            .clamp(1, MAX_MEDIA_LANES)
    }

    pub fn admit_download(
        self: &Arc<Self>,
        dc: i32,
        large: bool,
    ) -> Result<DownloadAdmission, crate::MtprotoError> {
        let priority = crate::request_class::current_class().priority();
        let mut active = self.downloads.lock();
        let ticket = active.next_ticket;
        active.next_ticket = active.next_ticket.wrapping_add(1);
        active.waiters.push(DownloadWaiter {
            ticket,
            priority,
            dc,
            large,
            queued_at: std::time::Instant::now(),
        });
        loop {
            if let Err(error) = crate::request_control::check() {
                active.waiters.retain(|waiter| waiter.ticket != ticket);
                self.download_wake.notify_all();
                return Err(crate::MtprotoError::Message(error.to_string()));
            }
            let limits = *self.queue_limits.lock();
            let limit = if large { limits.1.min(2) } else { limits.0 };
            let count = active.counts.get(&(dc, large)).copied().unwrap_or(0);
            let best = active
                .waiters
                .iter()
                .filter(|waiter| waiter.dc == dc && waiter.large == large)
                .min_by_key(|waiter| waiter.order_at(std::time::Instant::now()))
                .map(|waiter| waiter.ticket);
            let recovering = self.floods.lock().recovering_download(dc);
            let dc_count: usize = active
                .counts
                .iter()
                .filter(|((active_dc, _), _)| *active_dc == dc)
                .map(|(_, count)| *count)
                .sum();
            if count < limit.max(1) && (!recovering || dc_count == 0) && best == Some(ticket) {
                *active.counts.entry((dc, large)).or_insert(0) += 1;
                active.waiters.retain(|waiter| waiter.ticket != ticket);
                return Ok(DownloadAdmission {
                    policy: self.clone(),
                    dc,
                    large,
                });
            }
            self.download_wake
                .wait_for(&mut active, std::time::Duration::from_millis(50));
        }
    }

    pub fn pipeline_parts(&self) -> usize {
        self.pipeline_parts
            .load(Ordering::Relaxed)
            .clamp(1, MAX_PIPELINE_PARTS)
    }

    pub fn chunk_size(&self) -> i32 {
        let size = self.chunk_size.load(Ordering::Relaxed);
        if valid_chunk(size) {
            size
        } else {
            DEFAULT_CHUNK
        }
    }

    pub fn file_part(&self) -> usize {
        self.file_part
            .load(Ordering::Relaxed)
            .clamp(FILE_PART_SMALL, FILE_PART_FAST)
    }

    pub fn assign_media_lanes(&self, lanes: usize) {
        self.media_lanes
            .store(lanes.clamp(1, MAX_MEDIA_LANES), Ordering::Relaxed);
    }

    pub fn assign_pipeline_parts(&self, parts: usize) {
        self.pipeline_parts
            .store(parts.clamp(1, MAX_PIPELINE_PARTS), Ordering::Relaxed);
    }

    pub fn assign_chunk_size(&self, size: i32) {
        self.chunk_size
            .store(normalize_chunk(size), Ordering::Relaxed);
    }

    pub fn assign_file_part_kib(&self, kib: i32) {
        self.file_part
            .store(normalize_file_part_kib(kib), Ordering::Relaxed);
    }

    pub fn progress_callback(&self) -> Option<ProgressCallback> {
        self.progress.read().clone()
    }

    pub fn assign_progress(&self, callback: Option<ProgressCallback>) {
        *self.progress.write() = callback;
    }

    pub fn follow_media_lanes(&self, lanes: usize) {
        if !self.media_lanes_owned.load(Ordering::Relaxed) {
            self.assign_media_lanes(lanes);
        }
    }

    pub fn follow_pipeline_parts(&self, parts: usize) {
        if !self.pipeline_owned.load(Ordering::Relaxed) {
            self.assign_pipeline_parts(parts);
        }
    }

    pub fn follow_chunk_size(&self, size: i32) {
        if !self.chunk_owned.load(Ordering::Relaxed) {
            self.assign_chunk_size(size);
        }
    }

    pub fn follow_file_part_kib(&self, kib: i32) {
        if !self.file_part_owned.load(Ordering::Relaxed) {
            self.assign_file_part_kib(kib);
        }
    }

    pub fn follow_progress(&self, callback: &Option<ProgressCallback>) {
        if !self.progress_owned.load(Ordering::Relaxed) {
            self.assign_progress(callback.clone());
        }
    }

    pub fn own_media_lanes(&self, lanes: usize) {
        self.media_lanes_owned.store(true, Ordering::Relaxed);
        self.assign_media_lanes(lanes);
    }

    pub fn own_pipeline_parts(&self, parts: usize) {
        self.pipeline_owned.store(true, Ordering::Relaxed);
        self.assign_pipeline_parts(parts);
    }

    pub fn own_chunk_size(&self, size: i32) {
        self.chunk_owned.store(true, Ordering::Relaxed);
        self.assign_chunk_size(size);
    }

    pub fn is_chunk_owned(&self) -> bool {
        self.chunk_owned.load(Ordering::Relaxed)
    }

    pub fn own_file_part_kib(&self, kib: i32) {
        self.file_part_owned.store(true, Ordering::Relaxed);
        self.assign_file_part_kib(kib);
    }

    pub fn own_progress(&self, callback: Option<ProgressCallback>) {
        self.progress_owned.store(true, Ordering::Relaxed);
        self.assign_progress(callback);
    }

    pub fn notify(&self, path: &str, downloaded: i64, total: i64) {
        if let Some(callback) = self.progress.read().as_ref() {
            callback(path, downloaded, total);
        }
    }
}

pub struct DownloadAdmission {
    policy: Arc<TransferPolicy>,
    dc: i32,
    large: bool,
}

impl Drop for DownloadAdmission {
    fn drop(&mut self) {
        let mut active = self.policy.downloads.lock();
        if let Some(count) = active.counts.get_mut(&(self.dc, self.large)) {
            *count = count.saturating_sub(1);
        }
        self.policy.download_wake.notify_all();
    }
}

#[derive(Default)]
struct ActiveDownloads {
    counts: crate::HashMap<(i32, bool), usize>,
    waiters: Vec<DownloadWaiter>,
    next_ticket: u64,
}

struct DownloadWaiter {
    ticket: u64,
    priority: u8,
    dc: i32,
    large: bool,
    queued_at: std::time::Instant,
}

impl DownloadWaiter {
    fn order_at(&self, now: std::time::Instant) -> (u8, u64) {
        let priority = if self.priority > 2 {
            let age = (now.saturating_duration_since(self.queued_at).as_secs() / 5).min(255) as u8;
            self.priority.saturating_sub(age).max(1)
        } else {
            self.priority
        };
        (priority, self.ticket)
    }
}

#[cfg(test)]
#[path = "../tests/unit/transfer_policy_aging_tests.rs"]
mod tests;

fn valid_chunk(size: i32) -> bool {
    matches!(size, 131_072 | 262_144 | 524_288)
}

pub fn normalize_chunk(size: i32) -> i32 {
    if valid_chunk(size) {
        size
    } else {
        DEFAULT_CHUNK
    }
}

pub fn normalize_file_part_kib(kib: i32) -> usize {
    if kib >= 512 {
        FILE_PART_FAST
    } else {
        FILE_PART_SMALL
    }
}

pub fn default_policy() -> Arc<TransferPolicy> {
    static POLICY: OnceLock<Arc<TransferPolicy>> = OnceLock::new();
    POLICY
        .get_or_init(|| Arc::new(TransferPolicy::stock()))
        .clone()
}

pub fn snapshot() -> Arc<TransferPolicy> {
    let defaults = default_policy();
    let policy = TransferPolicy::stock();
    policy.assign_media_lanes(defaults.media_lanes());
    policy.assign_pipeline_parts(defaults.pipeline_parts());
    policy.assign_chunk_size(defaults.chunk_size());
    policy
        .file_part
        .store(defaults.file_part(), Ordering::Relaxed);
    policy.assign_progress(defaults.progress.read().clone());
    Arc::new(policy)
}

thread_local! {
    static BOUND: RefCell<Option<Arc<TransferPolicy>>> = RefCell::new(None);
}

pub struct PolicyGuard {
    previous: Option<Arc<TransferPolicy>>,
}

impl Drop for PolicyGuard {
    fn drop(&mut self) {
        let previous = self.previous.take();
        BOUND.with(|slot| *slot.borrow_mut() = previous);
    }
}

pub fn bind(policy: Arc<TransferPolicy>) -> PolicyGuard {
    BOUND.with(|slot| PolicyGuard {
        previous: slot.borrow_mut().replace(policy),
    })
}

pub fn current() -> Arc<TransferPolicy> {
    BOUND
        .with(|slot| slot.borrow().clone())
        .unwrap_or_else(default_policy)
}
