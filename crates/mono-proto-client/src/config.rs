use crate::access::with_client_mut;
use crate::registry::get_client;
use mono_proto_runtime::scheduler;
use mono_proto_storage::client_session::ChannelRecovery;
use std::sync::Arc;
use std::sync::atomic::Ordering;

pub fn schedule_transfer_config(handle: u64) {
    let Ok(client) = get_client(handle) else {
        return;
    };
    if client.data.lock().user_id.is_none() {
        return;
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let due = client.policy.config_due.load(Ordering::Acquire);
    if due > now
        || client
            .policy
            .config_due
            .compare_exchange(due, now + 3600, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
    {
        return;
    }
    let weak = Arc::downgrade(&client);
    let spawned = std::thread::Builder::new()
        .name("mtproto-transfer-config".into())
        .spawn(move || {
            let result = scheduler::with_class(scheduler::RequestClass::BackgroundRead, || {
                with_client_mut(handle, |state| {
                    mono_proto_runtime::rpc::with_rpc_timeout_secs(8, || {
                        mono_proto_messages::extras_rpc::animated_emoji_max(
                            &mut state.snapshot,
                            state.api_id,
                        )
                    })
                })
            });
            if result.is_err() {
                if let Some(client) = weak.upgrade() {
                    client.policy.config_due.store(now + 60, Ordering::Release);
                }
            }
        });
    if spawned.is_err() {
        client.policy.config_due.store(now + 60, Ordering::Release);
    }
}

/// Offscreen previews leave gaps durable; only the open chat and exceptions recover them.
pub fn update_lazy_sync_config(handle: u64, lazy: bool, exceptions: Vec<i64>) {
    let Ok(client) = get_client(handle) else {
        return;
    };
    let mut d = client.data.lock();
    if lazy && !d.lazy_channel_updates {
        let pending: Vec<_> = d
            .channel_recovery
            .iter()
            .filter(|entry| {
                !entry.watching
                    && entry.chat_id != d.last_history_chat_id
                    && !exceptions.contains(&entry.chat_id)
            })
            .map(|entry| entry.chat_id)
            .collect();
        d.lazy_channel_recovery.extend(pending);
    }
    d.lazy_channel_updates = lazy;
    d.lazy_sync_exceptions = exceptions.into_iter().collect();
    if lazy {
        return;
    }
    let pending = std::mem::take(&mut d.lazy_channel_recovery);
    for chat_id in pending {
        if d.channel_recovery
            .iter()
            .any(|entry| entry.chat_id == chat_id)
        {
            continue;
        }
        d.channel_recovery.push_back(ChannelRecovery {
            chat_id,
            due_at: 0,
            watching: false,
        });
    }
}
