use std::sync::{LazyLock, Mutex};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TxtEndpoint {
    pub dc_id: i32,
    pub addr: String,
    pub secret: Option<[u8; 16]>,
}

pub static TXT_ENDPOINTS: LazyLock<Mutex<Vec<TxtEndpoint>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));
pub static CDN_ENDPOINTS: LazyLock<Mutex<Vec<TxtEndpoint>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

pub fn endpoints_for(dc_id: i32) -> Vec<TxtEndpoint> {
    TXT_ENDPOINTS
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .filter(|e| e.dc_id == dc_id)
        .cloned()
        .collect()
}

pub fn cdn_endpoints_for(dc_id: i32) -> Vec<TxtEndpoint> {
    CDN_ENDPOINTS
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .filter(|e| e.dc_id == dc_id)
        .cloned()
        .collect()
}

/// Obfuscation secret from sidecar / `help.getConfig` for this exact DC address.
/// First hop uses this; other IPs stay unused with the home auth key (`AUTH_KEY_DUPLICATED`).
pub fn secret_for(dc_id: i32, addr: &str) -> Option<[u8; 16]> {
    TXT_ENDPOINTS
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .find(|e| e.dc_id == dc_id && e.addr == addr)
        .and_then(|e| e.secret)
}

/// Fresh temp-key handshake target when live `help.getConfig` fails.
/// Prefer a different address so we do not retry the dead home socket.
pub fn pick_backup_endpoint(
    home_dc: i32,
    home_addr: &str,
    endpoints: &[TxtEndpoint],
) -> Option<TxtEndpoint> {
    endpoints
        .iter()
        .find(|e| e.dc_id == home_dc && e.addr != home_addr)
        .or_else(|| endpoints.iter().find(|e| e.addr != home_addr))
        .or_else(|| endpoints.iter().find(|e| e.secret.is_some()))
        .cloned()
}
