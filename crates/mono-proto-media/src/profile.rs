use crate::transfer_policy;
use crate::transfer_window::*;

std::thread_local! {
    static BOUND_WINDOW: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
    static FORCED_CHUNK: std::cell::Cell<i32> = const { std::cell::Cell::new(0) };
    static REQUEST_PROFILE: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
}

pub fn set_request_profile(profile: Option<DownloadProfile>) {
    REQUEST_PROFILE.with(|cell| cell.set(profile.map(profile_tag).unwrap_or(0)));
}

pub fn request_profile() -> Option<DownloadProfile> {
    REQUEST_PROFILE.with(|cell| profile_from_tag(cell.get()))
}

fn profile_tag(profile: DownloadProfile) -> u8 {
    match profile {
        DownloadProfile::Thumb => 1,
        DownloadProfile::Ordinary => 2,
        DownloadProfile::Background => 3,
        DownloadProfile::Visible => 4,
        DownloadProfile::Playing => 5,
        DownloadProfile::User => 6,
    }
}

fn profile_from_tag(tag: u8) -> Option<DownloadProfile> {
    Some(match tag {
        1 => DownloadProfile::Thumb,
        2 => DownloadProfile::Ordinary,
        3 => DownloadProfile::Background,
        4 => DownloadProfile::Visible,
        5 => DownloadProfile::Playing,
        6 => DownloadProfile::User,
        _ => return None,
    })
}

pub struct DownloadWindowGuard {
    previous_profile: u8,
    previous_chunk: i32,
}

impl Drop for DownloadWindowGuard {
    fn drop(&mut self) {
        BOUND_WINDOW.with(|cell| cell.set(self.previous_profile));
        FORCED_CHUNK.with(|cell| cell.set(self.previous_chunk));
    }
}

pub fn bind_download_profile(profile: DownloadProfile) -> DownloadWindowGuard {
    let window = configured_download_window(profile);
    let previous_profile = BOUND_WINDOW.with(|cell| cell.replace(profile_tag(profile)));
    let previous_chunk = FORCED_CHUNK.with(|cell| cell.replace(window.chunk));
    DownloadWindowGuard {
        previous_profile,
        previous_chunk,
    }
}

pub fn bound_download_window() -> Option<DownloadWindow> {
    BOUND_WINDOW
        .with(|cell| profile_from_tag(cell.get()))
        .map(configured_download_window)
}

pub fn configured_download_window(profile: DownloadProfile) -> DownloadWindow {
    let maximum = download_window(profile);
    let policy = crate::transfer_policy::current();
    DownloadWindow {
        chunk: if policy.is_chunk_owned() {
            maximum.chunk.min(policy.chunk_size())
        } else if maximum.chunk >= FAST_CHUNK {
            maximum.chunk
        } else {
            maximum.chunk.min(policy.chunk_size())
        },
        in_flight: maximum.in_flight.min(policy.pipeline_parts()),
    }
}

pub fn download_in_flight() -> usize {
    bound_download_window()
        .map(|window| window.in_flight)
        .unwrap_or_else(|| crate::transfer_policy::current().pipeline_parts())
}

/// Parts before the aligned seek, or past the in-flight window, are not requested.
/// A non-zero seek stays on the origin DC.
/// https://core.telegram.org/api/files
/// https://core.telegram.org/cdn
pub fn cancel_outside_seek_window(
    seek: i64,
    chunk: i32,
    window_chunks: usize,
    inflight: &[i64],
) -> Vec<i64> {
    let step = i64::from(chunk.max(1));
    let start = seek - seek.rem_euclid(step);
    let end = start.saturating_add(step.saturating_mul(window_chunks.max(1) as i64));
    inflight
        .iter()
        .copied()
        .filter(|off| *off < start || *off >= end)
        .collect()
}

pub fn preserve_download_window() -> DownloadWindowGuard {
    DownloadWindowGuard {
        previous_profile: BOUND_WINDOW.with(|cell| cell.get()),
        previous_chunk: FORCED_CHUNK.with(|cell| cell.get()),
    }
}

pub fn forced_chunk() -> i32 {
    FORCED_CHUNK.with(|cell| cell.get())
}

pub fn set_forced_chunk(chunk: i32) {
    FORCED_CHUNK.with(|cell| cell.set(chunk));
}

#[cfg(test)]
#[path = "../tests/unit/download_profile_tests.rs"]
mod tests;
