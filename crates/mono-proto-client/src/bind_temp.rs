//! Bind a temporary auth key so an extra home-DC session does not reuse the
//! permanent key. https://core.telegram.org/api/pfs
//! https://core.telegram.org/method/auth.bindTempAuthKey
//! https://core.telegram.org/mtproto/description_v1

use mono_proto_session::{Clock, Snapshot};
use mono_proto_transport::Connection;

use mono_proto_types::MtprotoError;

pub fn unix_now() -> i32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| i32::try_from(d.as_secs()).unwrap_or(i32::MAX))
        .unwrap_or(0)
}

struct SystemClock;
impl Clock for SystemClock {
    fn unix_micros(&self) -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| i64::try_from(d.as_micros()).unwrap_or(i64::MAX))
            .unwrap_or(0)
    }
}

pub use mono_proto_runtime::auth::bind::{BoundTemp, negotiate_bound_temp};

pub fn temp_key_valid(snapshot: &Snapshot, expires_at: i32) -> bool {
    temp_key_valid_at(snapshot, expires_at, &SystemClock)
}

fn temp_key_valid_at(snapshot: &Snapshot, expires_at: i32, clock: &impl Clock) -> bool {
    let server_now = clock
        .unix_micros()
        .saturating_add(snapshot.time_offset_micros)
        .div_euclid(1_000_000);
    i64::from(expires_at) > server_now.saturating_add(60)
}

pub fn ensure_main_temp(
    client: &crate::client::Client,
    io: &mut crate::lanes::SessionIo,
    activate: bool,
) -> Result<bool, MtprotoError> {
    ensure_main_temp_with(client, io, activate, &SystemClock, negotiate_bound_temp)
}

pub fn ensure_main_temp_with(
    client: &crate::client::Client,
    io: &mut crate::lanes::SessionIo,
    activate: bool,
    clock: &impl Clock,
    negotiate: impl FnOnce(&[u8], i32, i64) -> Result<BoundTemp, MtprotoError>,
) -> Result<bool, MtprotoError> {
    client
        .connections
        .wait(std::time::Duration::ZERO)
        .map_err(|e| MtprotoError::Message(e.to_string()))?;
    if !activate && io.temp_expires_at == 0 && io.perm_key.is_none() {
        return Ok(false);
    }
    let dc = {
        let data = client.data.lock();
        if data.user_id.is_none() || data.session_dead {
            return Ok(false);
        }
        data.home_dc
    };
    ensure_dc_temp_with(client, io, dc, None, clock, negotiate)
}

pub fn dc_permanent_key(data: &crate::state::ClientData, dc: i32) -> Option<&[u8]> {
    let key = if dc == data.home_dc {
        data.home_auth_key.as_deref()
    } else {
        data.dc_auth_keys.get(&dc).map(|entry| entry.key.as_slice())
    };
    key.filter(|key| key.len() == 256)
}

pub fn ensure_dc_temp_with(
    client: &crate::client::Client,
    io: &mut crate::lanes::SessionIo,
    dc: i32,
    expected_home: Option<i64>,
    clock: &impl Clock,
    negotiate: impl FnOnce(&[u8], i32, i64) -> Result<BoundTemp, MtprotoError>,
) -> Result<bool, MtprotoError> {
    client
        .connections
        .wait(std::time::Duration::ZERO)
        .map_err(|e| MtprotoError::Message(e.to_string()))?;
    let (permanent, owner, user, cached) = {
        let data = client.data.lock();
        if data.user_id.is_none()
            || data.session_dead
            || expected_home.is_some_and(|session| session != data.home_session_id)
        {
            return Err(crate::persist::expired_session_lease());
        }
        let permanent = dc_permanent_key(&data, dc)
            .ok_or_else(|| MtprotoError::Message("permanent session identity unavailable".into()))?
            .to_vec();
        let cached = expected_home
            .and_then(|_| data.dc_temp_auth.get(&dc))
            .filter(|entry| {
                entry.perm_key == permanent
                    && temp_key_valid_at(&entry.snapshot, entry.temp_expires_at, clock)
            })
            .cloned();
        (permanent, data.home_snapshot.clone(), data.user_id, cached)
    };
    if io.perm_key.as_deref() == Some(permanent.as_slice())
        && io.snapshot.dc_id == dc
        && io
            .snapshot
            .auth_key
            .as_ref()
            .is_some_and(|key| key.len() == 256 && key != &permanent)
        && temp_key_valid_at(&io.snapshot, io.temp_expires_at, clock)
    {
        return Ok(false);
    }
    mono_proto_runtime::request_control::check()
        .map_err(|e| MtprotoError::Message(e.to_string()))?;
    let bound = if let Some(cached) = cached {
        BoundTemp {
            snapshot: crate::session::fork_session(&cached.snapshot),
            expires_at: cached.temp_expires_at,
        }
    } else {
        mono_proto_runtime::tcp::with_connection_control(&client.connections, || {
            negotiate(&permanent, dc, owner.time_offset_micros)
        })?
    };
    mono_proto_runtime::request_control::check()
        .map_err(|e| MtprotoError::Message(e.to_string()))?;
    bound
        .snapshot
        .validate()
        .map_err(|e| MtprotoError::Message(e.to_string()))?;
    if bound.snapshot.dc_id != dc
        || bound.snapshot.session_id == owner.session_id
        || !bound
            .snapshot
            .auth_key
            .as_ref()
            .is_some_and(|key| key.len() == 256 && key != &permanent)
        || !temp_key_valid_at(&bound.snapshot, bound.expires_at, clock)
    {
        return Err(MtprotoError::Message(
            "invalid bound temporary session".into(),
        ));
    }
    let mut data = client.data.lock();
    if !crate::persist::session_lease_valid(&data, owner.session_id)
        || data.user_id != user
        || data.home_dc != owner.dc_id
        || data.home_auth_key != owner.auth_key
        || dc_permanent_key(&data, dc) != Some(permanent.as_slice())
    {
        return Err(crate::persist::expired_session_lease());
    }
    client
        .connections
        .while_open(|| {
            client.connections.wait(std::time::Duration::ZERO)?;
            if let Some(mut previous) = io.transport.take() {
                let _ = previous.conn.close();
            }
            mono_proto_runtime::rpc::salts::forget_session(io.snapshot.session_id);
            if expected_home.is_some() {
                data.dc_temp_auth.insert(
                    dc,
                    crate::state::DcTempAuth {
                        snapshot: bound.snapshot.clone(),
                        temp_expires_at: bound.expires_at,
                        perm_key: permanent.clone(),
                    },
                );
            }
            io.snapshot = bound.snapshot;
            io.temp_expires_at = bound.expires_at;
            io.perm_key = Some(permanent);
            io.pending_push = Default::default();
            io.last_difference = None;
            Ok::<_, mono_proto_transport::Error>(())
        })
        .map_err(|e| MtprotoError::Message(e.to_string()))?
        .map_err(|e| MtprotoError::Message(e.to_string()))?;
    Ok(true)
}

fn promote_main(
    client: &crate::client::Client,
    negotiate: &mut impl FnMut(&[u8], i32, i64) -> Result<BoundTemp, MtprotoError>,
) -> Result<bool, MtprotoError> {
    let _gate = client
        .main_gate
        .acquire(mono_proto_runtime::scheduler::RequestClass::BackgroundRead)?;
    let mut main = crate::lanes::lock_request_lane(&client.main, "lane_wait.pfs")?;
    ensure_main_temp_with(client, &mut main, true, &SystemClock, negotiate)
}

fn bind_lane(
    client: &crate::client::Client,
    lane: &crate::lanes::Lane,
    negotiate: &mut impl FnMut(&[u8], i32, i64) -> Result<BoundTemp, MtprotoError>,
) -> bool {
    let Some(_gate) = lane
        .gate
        .try_acquire(mono_proto_runtime::scheduler::RequestClass::BackgroundRead)
    else {
        return false;
    };
    let Some(mut io) = lane.io.try_lock() else {
        return false;
    };
    ensure_main_temp_with(client, &mut io, true, &SystemClock, negotiate).is_ok()
        && io.perm_key.is_some()
}

/// Additional main sessions are admitted only after their temporary key is bound.
pub fn prepare_extra_main_sessions(
    handle: u64,
    mut negotiate: impl FnMut(&[u8], i32, i64) -> Result<BoundTemp, MtprotoError>,
) -> Result<(), MtprotoError> {
    let client = crate::registry::get_client(handle)?;
    let _policy = mono_proto_runtime::transfer_policy::bind(client.policy.clone());
    {
        let data = client.data.lock();
        if data.user_id.is_none() || data.session_dead || data.home_auth_key.is_none() {
            mono_proto_runtime::scheduler::set_bound_extra_sessions(0);
            return Ok(());
        }
    }
    promote_main(&client, &mut negotiate)?;
    for lane in &client.upload {
        let _ = bind_lane(&client, lane, &mut negotiate);
    }
    let allowance = mono_proto_runtime::scheduler::main_session_allowance();
    if allowance <= 1 {
        mono_proto_runtime::scheduler::set_bound_extra_sessions(0);
        return Ok(());
    }
    let want = usize::try_from(allowance.saturating_sub(1)).unwrap_or(0);
    let want = want.min(mono_proto_runtime::scheduler::READ_LANES);
    let mut bound = 0_usize;
    for lane in client.rpc.iter().take(want) {
        if bind_lane(&client, lane, &mut negotiate) {
            bound += 1;
        }
    }
    mono_proto_runtime::scheduler::set_bound_extra_sessions(bound);
    Ok(())
}
