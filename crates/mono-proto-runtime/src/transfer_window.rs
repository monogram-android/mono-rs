use crate::MtprotoError;
use crate::collections::HashMapExt;

pub const DEFAULT_CHUNK: i32 = 128 * 1024;
pub const CHUNK: i32 = DEFAULT_CHUNK;
pub const STREAM_WINDOW_CHUNKS: usize = 4;
pub const FAST_CHUNK: i32 = 512 * 1024;
/// Telegram keeps foreground file sockets up and pings about every 3 minutes.
pub const DOWNLOAD_SESSION_IDLE: std::time::Duration = std::time::Duration::from_secs(180);

/// https://core.telegram.org/api/files
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DownloadProfile {
    Thumb,
    Ordinary,
    Background,
    Visible,
    Playing,
    User,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DownloadWindow {
    pub chunk: i32,
    pub in_flight: usize,
}

/// Visible, playing, and user files use 512 KiB and 8 parts. Other files stay
/// at 128 KiB, and a thumb is one request.
/// https://core.telegram.org/api/files
pub fn window_candidate(
    chunk: i32,
    in_flight: usize,
    alignment_proven: bool,
) -> Option<DownloadWindow> {
    let aligned = chunk > 0 && chunk % 4_096 == 0;
    match (chunk, in_flight, alignment_proven && aligned) {
        (131_072, 4, _) | (524_288, 8, _) => Some(DownloadWindow { chunk, in_flight }),
        (1_048_576, 12, true) => Some(DownloadWindow { chunk, in_flight }),
        _ => None,
    }
}

pub fn download_window(profile: DownloadProfile) -> DownloadWindow {
    match profile {
        DownloadProfile::Thumb => DownloadWindow {
            chunk: DEFAULT_CHUNK,
            in_flight: 1,
        },
        DownloadProfile::Ordinary | DownloadProfile::Background => DownloadWindow {
            chunk: DEFAULT_CHUNK,
            in_flight: 4,
        },
        DownloadProfile::Visible | DownloadProfile::Playing | DownloadProfile::User => {
            DownloadWindow {
                chunk: FAST_CHUNK,
                in_flight: 8,
            }
        }
    }
}

/// `LIMIT_INVALID` retries a smaller part and fewer of them in flight.
/// https://core.telegram.org/api/files
pub fn limit_invalid_window(window: DownloadWindow) -> Option<DownloadWindow> {
    if window.chunk <= DEFAULT_CHUNK {
        return None;
    }
    let in_flight = (window.in_flight / 2).clamp(1, 4);
    let in_flight = if in_flight < window.in_flight {
        in_flight
    } else {
        1
    };
    Some(DownloadWindow {
        chunk: DEFAULT_CHUNK,
        in_flight,
    })
}

/// Server `FLOOD_WAIT` seconds, not a 15 second cap.
/// https://core.telegram.org/api/errors
pub fn flood_wait_duration(secs: u64) -> std::time::Duration {
    std::time::Duration::from_secs(secs.max(1))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TransferClass {
    Download,
    Upload,
}

#[derive(Clone, Debug)]
pub struct FloodScope {
    slow_until: crate::HashMap<(i32, TransferClass), FloodBackoff>,
}

#[derive(Clone, Debug)]
struct FloodBackoff {
    window: DownloadWindow,
    until: std::time::Instant,
    premium: bool,
    recover_after: std::time::Instant,
    successes: usize,
}

impl FloodScope {
    pub fn new() -> Self {
        Self {
            slow_until: crate::HashMap::new(),
        }
    }

    /// Record the server deadline. The caller unwinds its lease on the error.
    pub fn park(
        &mut self,
        dc_id: i32,
        class: TransferClass,
        seconds: u64,
        premium: bool,
        now: std::time::Instant,
        current: DownloadWindow,
    ) {
        let key = (dc_id, class);
        let previous = self.slow_until.get(&key);
        let slowed = if class == TransferClass::Download {
            DownloadWindow {
                chunk: current.chunk,
                in_flight: if premium {
                    1
                } else {
                    (previous.map_or(current.in_flight, |state| state.window.in_flight) / 2)
                        .clamp(1, 2)
                },
            }
        } else if premium {
            DownloadWindow {
                chunk: DEFAULT_CHUNK,
                in_flight: 1,
            }
        } else {
            DownloadWindow {
                chunk: current.chunk.min(DEFAULT_CHUNK),
                in_flight: current.in_flight.min(4),
            }
        };
        let seconds = seconds.max(1);
        let deadline =
            (now + flood_wait_duration(seconds)).max(previous.map_or(now, |state| state.until));
        let premium = previous
            .filter(|state| state.until > now)
            .map_or(premium, |state| state.premium || premium);
        self.slow_until.insert(
            key,
            FloodBackoff {
                window: slowed,
                until: deadline,
                premium,
                recover_after: deadline + std::time::Duration::from_secs(30),
                successes: 0,
            },
        );
    }

    pub fn window(
        &mut self,
        dc_id: i32,
        class: TransferClass,
        now: std::time::Instant,
        fallback: DownloadWindow,
    ) -> DownloadWindow {
        let expired = self
            .slow_until
            .get(&(dc_id, class))
            .is_some_and(|state| now >= state.until);
        if expired && class != TransferClass::Download {
            self.slow_until.remove(&(dc_id, class));
            return fallback;
        }
        self.slow_until
            .get(&(dc_id, class))
            .map(|state| DownloadWindow {
                chunk: state.window.chunk.min(fallback.chunk),
                in_flight: state.window.in_flight.min(fallback.in_flight),
            })
            .unwrap_or(fallback)
    }

    pub fn recovering_download(&self, dc_id: i32) -> bool {
        self.slow_until
            .contains_key(&(dc_id, TransferClass::Download))
    }

    pub fn download_succeeded(&mut self, dc_id: i32, now: std::time::Instant) {
        let Some(state) = self.slow_until.get_mut(&(dc_id, TransferClass::Download)) else {
            return;
        };
        if now < state.until {
            return;
        }
        state.successes = state.successes.saturating_add(1);
        if state.successes >= 32 && now >= state.recover_after && state.window.in_flight < 4 {
            state.window.in_flight += 1;
            state.successes = 0;
            state.recover_after = now + std::time::Duration::from_secs(30);
        }
    }

    pub fn pending_error(
        &self,
        dc_id: i32,
        class: TransferClass,
        now: std::time::Instant,
    ) -> Option<MtprotoError> {
        let state = self.slow_until.get(&(dc_id, class))?;
        let remaining = state.until.checked_duration_since(now)?;
        if remaining.is_zero() {
            return None;
        }
        let seconds = remaining
            .as_secs()
            .saturating_add(u64::from(remaining.subsec_nanos() > 0));
        Some(MtprotoError::Message(format!(
            "RPC 420: {}_{seconds}",
            if state.premium {
                "FLOOD_PREMIUM_WAIT"
            } else {
                "FLOOD_WAIT"
            }
        )))
    }
}
