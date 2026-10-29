use mono_proto_runtime::client_session::ChannelRecovery;
use mono_proto_types::collections::HashSet;
use mono_proto_types::error::MtprotoError;
use mono_proto_types::events::UpdateEventDto;
use mono_proto_types::message::ChatDto;
use mono_proto_types::peers::channel_id_from_chat_id;
use std::collections::VecDeque;

#[cfg(test)]
#[path = "../tests/unit/recovery_tests.rs"]
mod tests;

pub fn recovery_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn defer_channel_recovery(
    queue: &mut VecDeque<ChannelRecovery>,
    mut entry: ChannelRecovery,
    error: &MtprotoError,
    now: u64,
) {
    entry.due_at = now.saturating_add(recovery_wait_secs(error));
    queue.push_back(entry);
}

pub fn terminal_channel_error(error: &MtprotoError) -> bool {
    matches!(error, MtprotoError::Message(message) if
        message.contains("CHANNEL_PRIVATE") || message.contains("CHANNEL_INVALID")
        || message.contains("PEER_ID_INVALID"))
}

pub fn recovery_wait_secs(error: &MtprotoError) -> u64 {
    let wait = match error {
        MtprotoError::Message(message) => ["FLOOD_WAIT_", "FLOOD_PREMIUM_WAIT_"]
            .iter()
            .filter_map(|prefix| message.split_once(prefix))
            .filter_map(|(_, suffix)| {
                suffix
                    .split(|c: char| !c.is_ascii_digit())
                    .next()?
                    .parse::<u64>()
                    .ok()
            })
            .max()
            .unwrap_or(5),
        _ => 5,
    };
    wait.max(5)
}

pub fn prepare_channel_recovery(
    queue: &mut VecDeque<ChannelRecovery>,
    mut pending: Vec<i64>,
    open_chat: i64,
    now: u64,
) {
    queue.retain(|entry| !entry.watching || entry.chat_id == open_chat);
    for chat_id in pending
        .drain(..)
        .filter(|id| channel_id_from_chat_id(*id).is_some())
    {
        if let Some(entry) = queue.iter_mut().find(|entry| entry.chat_id == chat_id) {
            if entry.watching {
                entry.watching = false;
                entry.due_at = now;
            }
        } else {
            queue.push_back(ChannelRecovery {
                chat_id,
                due_at: now,
                watching: false,
            });
        }
    }
    if channel_id_from_chat_id(open_chat).is_some()
        && !queue.iter().any(|entry| entry.chat_id == open_chat)
    {
        queue.push_back(ChannelRecovery {
            chat_id: open_chat,
            due_at: now,
            watching: true,
        });
    }
}

pub fn finish_channel_recovery(
    queue: &mut VecDeque<ChannelRecovery>,
    mut entry: ChannelRecovery,
    final_page: bool,
    timeout: Option<i32>,
    open_chat: i64,
    now: u64,
) {
    if !final_page || entry.chat_id == open_chat {
        entry.watching = final_page;
        entry.due_at = now.saturating_add(if final_page {
            timeout.unwrap_or(1).max(1) as u64
        } else {
            0
        });
        queue.push_back(entry);
    }
}

pub const LAZY_CHANNEL_BATCH: usize = 100;

pub fn sync_edge(previous: bool, now: bool) -> bool {
    previous != now
}

/// Difference still running or a channel gap still queued.
/// A `watching` entry is the open channel's short poll, not catch-up.
pub fn catch_up_active(
    difference_settled: bool,
    recovery_pending: bool,
    lazy_pending: bool,
) -> bool {
    !difference_settled || recovery_pending || lazy_pending
}

pub fn gap_recovery_pending(recovery: &VecDeque<ChannelRecovery>) -> bool {
    recovery.iter().any(|entry| !entry.watching)
}

pub fn visible_gap_recovery_pending(
    recovery: &VecDeque<ChannelRecovery>,
    open_chat: i64,
    exceptions: &HashSet<i64>,
    lazy: bool,
) -> bool {
    recovery.iter().any(|entry| {
        !entry.watching
            && (!lazy || entry.chat_id == open_chat || exceptions.contains(&entry.chat_id))
    })
}

pub fn next_channel_recovery(
    recovery: &VecDeque<ChannelRecovery>,
    open_chat: i64,
    exceptions: &HashSet<i64>,
    lazy: bool,
    now: u64,
) -> Option<usize> {
    recovery
        .iter()
        .position(|entry| entry.due_at <= now && entry.chat_id == open_chat)
        .or_else(|| {
            recovery.iter().position(|entry| {
                entry.due_at <= now && (!lazy || exceptions.contains(&entry.chat_id))
            })
        })
}

/// Chats the user opened, or listed as exceptions, must not stay on the lazy
/// batch. They go to `getChannelDifference` even if an earlier poll queued them.
pub fn pull_priority_channels(
    lazy_queue: &mut HashSet<i64>,
    recovery: &mut VecDeque<ChannelRecovery>,
    open_chat: i64,
    exceptions: &HashSet<i64>,
    now: u64,
) {
    let promoted: Vec<i64> = lazy_queue
        .iter()
        .copied()
        .filter(|id| *id == open_chat || exceptions.contains(id))
        .collect();
    for chat_id in promoted {
        lazy_queue.remove(&chat_id);
        if let Some(entry) = recovery.iter_mut().find(|entry| entry.chat_id == chat_id) {
            if entry.watching {
                entry.watching = false;
                entry.due_at = now;
            }
        } else {
            recovery.push_back(ChannelRecovery {
                chat_id,
                due_at: now,
                watching: false,
            });
        }
    }
}

pub struct LazyFailurePlan {
    pub restore: bool,
    pub drop_transport: bool,
}

/// A lazy `getPeerDialogs` failure restores the batch. It does not drop the
/// update transport. An unrecoverable session still fails the drain.
pub fn plan_lazy_failure(unrecoverable: bool) -> LazyFailurePlan {
    if unrecoverable {
        LazyFailurePlan {
            restore: false,
            drop_transport: false,
        }
    } else {
        LazyFailurePlan {
            restore: true,
            drop_transport: false,
        }
    }
}

/// Retain offscreen gaps durably until the channel is opened or lazy mode is disabled.
pub fn route_lazy_channels(
    pending: Vec<i64>,
    open_chat: i64,
    exceptions: &HashSet<i64>,
    lazy_enabled: bool,
    lazy_queue: &mut HashSet<i64>,
) -> Vec<i64> {
    if !lazy_enabled {
        return pending;
    }
    let mut full = Vec::new();
    for chat_id in pending {
        if chat_id == open_chat || exceptions.contains(&chat_id) {
            full.push(chat_id);
        } else if channel_id_from_chat_id(chat_id).is_some() {
            lazy_queue.insert(chat_id);
            full.push(chat_id);
        }
    }
    full
}

/// Takes at most `limit` peers that can be sent. Ids without a usable peer stay queued.
pub fn take_lazy_batch(
    queue: &mut HashSet<i64>,
    mut usable: impl FnMut(i64) -> bool,
    limit: usize,
) -> Vec<i64> {
    let mut ids: Vec<i64> = queue.iter().copied().collect();
    ids.sort_unstable();
    let mut ready = Vec::new();
    for id in ids {
        if ready.len() == limit {
            break;
        }
        if usable(id) {
            queue.remove(&id);
            ready.push(id);
        }
    }
    ready
}

pub fn restore_lazy_batch(queue: &mut HashSet<i64>, batch: &[i64]) {
    queue.extend(batch.iter().copied());
}

pub fn lazy_dialog_events(chats: Vec<ChatDto>) -> Vec<UpdateEventDto> {
    if chats.is_empty() {
        Vec::new()
    } else {
        vec![UpdateEventDto::DialogsPatched { chats }]
    }
}
