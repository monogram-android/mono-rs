use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, LazyLock};

use mono_proto_types::collections::HashMap;
use parking_lot::{Mutex, MutexGuard};

#[cfg(test)]
#[path = "../tests/unit/registry_tests.rs"]
mod tests;

pub static CLIENTS: LazyLock<ClientRegistry<crate::client::Client>> =
    LazyLock::new(ClientRegistry::default);

pub fn get_client(
    handle: u64,
) -> Result<Arc<crate::client::Client>, mono_proto_types::MtprotoError> {
    CLIENTS
        .lock()
        .get(&handle)
        .cloned()
        .ok_or(mono_proto_types::MtprotoError::UnknownClient)
}

pub fn client_exists(handle: u64) -> bool {
    CLIENTS.lock().contains_key(&handle)
}

pub fn destroy_client(handle: u64) {
    let client = CLIENTS.lock().remove(&handle);
    if let Some(client) = client {
        client.connections.begin_shutdown();
        crate::persist::flush_persist(&client);
        client.connections.close();
    }
}

pub fn visit_policies(visit: impl Fn(&mono_proto_runtime::transfer_policy::TransferPolicy)) {
    for client in CLIENTS.lock().values() {
        visit(&client.policy);
    }
}

pub struct ClientRegistry<T> {
    next_handle: AtomicU64,
    clients: Mutex<HashMap<u64, Arc<T>>>,
}

impl<T> Default for ClientRegistry<T> {
    fn default() -> Self {
        Self {
            next_handle: AtomicU64::new(1),
            clients: Mutex::new(HashMap::default()),
        }
    }
}

impl<T> ClientRegistry<T> {
    pub fn next_handle(&self) -> u64 {
        self.next_handle.fetch_add(1, Ordering::Relaxed)
    }

    pub fn lock(&self) -> MutexGuard<'_, HashMap<u64, Arc<T>>> {
        self.clients.lock()
    }
}
