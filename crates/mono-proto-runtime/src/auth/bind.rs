use super::bind_crypto::encrypt_bind_message;
use crate::MtprotoError;
use crate::auth_key::create_temp_auth_key;
use crate::rpc::SystemClock;
use mono_proto::codec::{Boxed, Decoder, Limits, TlDecode};
use mono_proto::transport::BindAuthKeyInnerConstructor;
#[cfg(test)]
use mono_proto_crypto::{aes_ige_decrypt, sha1};
use mono_proto_crypto::{auth_key_id, fill_random};
use mono_proto_session::{Clock, OsRandom, Snapshot};
use mono_proto_transport::{Connection, Framing, PaddedIntermediate};

const TEMP_EXPIRES_IN: i32 = 24 * 60 * 60;
const BOOL_TRUE: u32 = 0x9972_75b5;
const BOOL_FALSE: u32 = 0xbc79_9737;
const RPC_RESULT: u32 = 0xf35c_6d01;
const RPC_ERROR: u32 = 0x2144_ca19;
const MSG_CONTAINER: u32 = 0x73f1_f8dc;

pub struct BoundTemp {
    pub snapshot: Snapshot,
    pub expires_at: i32,
}

struct DeadlineConnection<'a> {
    inner: &'a mut crate::tcp::ObfuscatedTcp,
    deadline: std::time::Instant,
}

impl DeadlineConnection<'_> {
    fn prepare(&mut self) -> Result<(), mono_proto_transport::Error> {
        let remaining = self
            .deadline
            .saturating_duration_since(std::time::Instant::now());
        if remaining.is_zero() {
            return Err(mono_proto_transport::Error::Connection(
                "temporary key negotiation timed out".into(),
            ));
        }
        self.inner
            .set_io_timeout_ms(remaining.as_millis().min(u128::from(u64::MAX)) as u64);
        Ok(())
    }
}

impl Connection for DeadlineConnection<'_> {
    fn send(&mut self, packet: &[u8]) -> Result<(), mono_proto_transport::Error> {
        self.prepare()?;
        self.inner.send(packet)
    }

    fn receive(&mut self, output: &mut [u8]) -> Result<usize, mono_proto_transport::Error> {
        self.prepare()?;
        self.inner.receive(output)
    }

    fn close(&mut self) -> Result<(), mono_proto_transport::Error> {
        self.inner.close()
    }
}

/// Handshake a temporary key and bind it. Leaves the permanent key untouched.
pub fn negotiate_bound_temp(
    perm_key: &[u8],
    dc_id: i32,
    time_offset_micros: i64,
) -> Result<BoundTemp, MtprotoError> {
    if perm_key.len() != 256 {
        return Err(MtprotoError::Message("permanent key required".into()));
    }
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    let mut snapshot =
        Snapshot::new(dc_id, &mut OsRandom).map_err(|e| MtprotoError::Message(e.to_string()))?;
    snapshot.time_offset_micros = time_offset_micros;
    let mut conn = crate::tcp::connect_obfuscated_dc(crate::rpc::dc_endpoints(dc_id))
        .map_err(|e| MtprotoError::Message(e.to_string()))?;
    let mut conn = DeadlineConnection {
        inner: &mut conn,
        deadline,
    };
    let mut framing = PaddedIntermediate::default();
    let expires_at = create_temp_auth_key(&mut conn, &mut framing, &mut snapshot, TEMP_EXPIRES_IN)?;
    bind_temporary_key(
        &mut conn,
        &mut framing,
        &mut snapshot,
        perm_key,
        expires_at,
        &SystemClock,
    )?;
    Ok(BoundTemp {
        snapshot,
        expires_at,
    })
}

fn bind_temporary_key(
    conn: &mut dyn Connection,
    framing: &mut PaddedIntermediate,
    snapshot: &mut Snapshot,
    perm_key: &[u8],
    expires_at: i32,
    clock: &impl Clock,
) -> Result<(), MtprotoError> {
    let temp_key = zeroize::Zeroizing::new(
        snapshot
            .auth_key
            .clone()
            .ok_or_else(|| MtprotoError::Message("temp key missing".into()))?,
    );
    if temp_key.as_slice() == perm_key {
        return Err(MtprotoError::Message(
            "temp key matched permanent key".into(),
        ));
    }
    let msg_id = snapshot
        .next_message_id(clock)
        .map_err(|e| MtprotoError::Message(e.to_string()))?;
    let mut nonce_bytes = [0_u8; 8];
    fill_random(&mut nonce_bytes).map_err(|e| MtprotoError::Message(e.to_string()))?;
    let mut prefix = [0_u8; 16];
    fill_random(&mut prefix).map_err(|e| MtprotoError::Message(e.to_string()))?;
    let inner = BindAuthKeyInnerConstructor {
        nonce: i64::from_le_bytes(nonce_bytes),
        temp_auth_key_id: auth_key_id(&temp_key),
        perm_auth_key_id: auth_key_id(perm_key),
        temp_session_id: snapshot.session_id,
        expires_at,
    };
    let mut padding = [0_u8; 8];
    fill_random(&mut padding).map_err(|e| MtprotoError::Message(e.to_string()))?;
    let encrypted = encrypt_bind_message(perm_key, msg_id, &inner, &prefix, &padding)?;
    let request = mono_proto::latest::api::AuthBindTempAuthKeyRequest {
        perm_auth_key_id: inner.perm_auth_key_id,
        nonce: inner.nonce,
        expires_at,
        encrypted_message: encrypted,
    };
    let body = crate::rpc::encode_boxed_bytes(&request)?;
    let seq = snapshot
        .next_sequence(true)
        .map_err(|e| MtprotoError::Message(e.to_string()))?;
    send_v2(conn, framing, &temp_key, snapshot, msg_id, seq, &body)?;
    read_bind_result(conn, framing, &temp_key, snapshot, msg_id, clock)
}

fn send_v2(
    conn: &mut dyn Connection,
    framing: &mut PaddedIntermediate,
    auth_key: &[u8],
    snapshot: &mut Snapshot,
    message_id: i64,
    sequence: i32,
    body: &[u8],
) -> Result<(), MtprotoError> {
    let pad_len = v2_pad_len(body.len());
    let mut padding = vec![0_u8; pad_len];
    fill_random(&mut padding).map_err(|e| MtprotoError::Message(e.to_string()))?;
    let packet = mono_proto_crypto::encrypt_message(
        auth_key,
        mono_proto_crypto::MessageToEncrypt {
            server_salt: snapshot.server_salt,
            session_id: snapshot.session_id,
            message_id,
            sequence,
            body,
            padding: &padding,
        },
        mono_proto_crypto::Direction::ClientToServer,
    )
    .map_err(|e| MtprotoError::Message(e.to_string()))?;
    let framed = framing
        .encode(&packet)
        .map_err(|e| MtprotoError::Message(e.to_string()))?;
    conn.send(&framed)
        .map_err(|e| MtprotoError::Message(e.to_string()))
}

fn v2_pad_len(body_len: usize) -> usize {
    let mut pad = 12;
    while (32 + body_len + pad) % 16 != 0 {
        pad += 1;
    }
    pad
}

fn read_bind_result(
    conn: &mut dyn Connection,
    framing: &mut PaddedIntermediate,
    temp_key: &[u8],
    snapshot: &mut Snapshot,
    msg_id: i64,
    clock: &impl Clock,
) -> Result<(), MtprotoError> {
    let mut buf = vec![0_u8; 64 * 1024];
    let mut input = Vec::new();
    for _ in 0..8 {
        loop {
            match framing.decode(&input) {
                Ok(packet) => {
                    let consumed = packet.consumed;
                    let payload = crate::rpc::trim_padded_mtproto_packet(&packet.payload);
                    input.drain(..consumed.min(input.len()));
                    if let Some(control) = mono_proto_transport::payload_control(payload) {
                        match control {
                            mono_proto_transport::Control::QuickAck(_) => break,
                            mono_proto_transport::Control::TransportError(code) => {
                                return Err(MtprotoError::Message(format!(
                                    "temp bind transport {code}"
                                )));
                            }
                        }
                    }
                    let message = mono_proto_crypto::decrypt_message(
                        temp_key,
                        payload,
                        mono_proto_crypto::Direction::ServerToClient,
                        snapshot.session_id,
                        1024 * 1024,
                    )
                    .map_err(|e| MtprotoError::Message(e.to_string()))?;
                    if message.sequence < 0 {
                        return Err(malformed_bind());
                    }
                    let received = snapshot
                        .register_received_message(
                            message.message_id,
                            message.sequence & 1 != 0,
                            clock,
                            true,
                        )
                        .map_err(|e| MtprotoError::Message(e.to_string()))?;
                    if received != mono_proto_session::ReceivedMessageResult::Accepted {
                        break;
                    }
                    let response =
                        bind_response(&message.body, msg_id, snapshot, clock, message.message_id)?;
                    if let Some(salt) = response.server_salt {
                        snapshot.server_salt = salt;
                    }
                    if response.accepted {
                        return Ok(());
                    }
                    break;
                }
                Err(mono_proto_transport::Error::Incomplete { .. }) => {
                    let n = conn
                        .receive(&mut buf)
                        .map_err(|e| MtprotoError::Message(e.to_string()))?;
                    if n == 0 {
                        return Err(MtprotoError::Message("temp bind connection closed".into()));
                    }
                    input.extend_from_slice(&buf[..n]);
                }
                Err(e) => return Err(MtprotoError::Message(e.to_string())),
            }
        }
    }
    Err(MtprotoError::Message("temp bind had no result".into()))
}

#[derive(Default)]
struct BindResponse {
    accepted: bool,
    server_salt: Option<i64>,
}

fn bind_response(
    body: &[u8],
    msg_id: i64,
    snapshot: &mut Snapshot,
    clock: &impl Clock,
    outer_id: i64,
) -> Result<BindResponse, MtprotoError> {
    bind_response_at_depth(body, msg_id, 0, &mut 1024, snapshot, clock, outer_id)
}

fn malformed_bind() -> MtprotoError {
    MtprotoError::Message("malformed temporary key binding response".into())
}

fn bind_response_at_depth(
    body: &[u8],
    msg_id: i64,
    depth: usize,
    remaining: &mut usize,
    snapshot: &mut Snapshot,
    clock: &impl Clock,
    outer_id: i64,
) -> Result<BindResponse, MtprotoError> {
    if depth > 8 || *remaining == 0 || body.len() < 4 || body.len() % 4 != 0 {
        return Err(malformed_bind());
    }
    *remaining -= 1;
    let ctor = u32::from_le_bytes(body[..4].try_into().expect("ctor"));
    if ctor == MSG_CONTAINER {
        return container_accepted(body, msg_id, depth, remaining, snapshot, clock, outer_id);
    }
    if ctor == RPC_RESULT {
        return Ok(BindResponse {
            accepted: rpc_result_accepted(body, msg_id)?,
            ..Default::default()
        });
    }
    if ctor == 0x9ec2_0908 {
        if body.len() != 28 {
            return Err(malformed_bind());
        }
        return Ok(BindResponse {
            server_salt: Some(i64::from_le_bytes(body[20..28].try_into().expect("salt"))),
            ..Default::default()
        });
    }
    Ok(BindResponse::default())
}

fn container_accepted(
    body: &[u8],
    msg_id: i64,
    depth: usize,
    remaining: &mut usize,
    snapshot: &mut Snapshot,
    clock: &impl Clock,
    outer_id: i64,
) -> Result<BindResponse, MtprotoError> {
    if body.len() < 8 {
        return Err(malformed_bind());
    }
    let count = u32::from_le_bytes(body[4..8].try_into().expect("count")) as usize;
    if count > *remaining || count > (body.len() - 8) / 20 {
        return Err(malformed_bind());
    }
    let mut offset = 8;
    let mut response = BindResponse::default();
    for _ in 0..count {
        if offset + 16 > body.len() {
            return Err(malformed_bind());
        }
        let child_id = i64::from_le_bytes(
            body[offset..offset + 8]
                .try_into()
                .expect("child message id"),
        );
        let sequence =
            i32::from_le_bytes(body[offset + 8..offset + 12].try_into().expect("sequence"));
        if child_id >= outer_id || child_id & 1 == 0 || sequence < 0 {
            return Err(malformed_bind());
        }
        offset += 8 + 4;
        let len = u32::from_le_bytes(body[offset..offset + 4].try_into().expect("len")) as usize;
        offset += 4;
        let end = offset.checked_add(len).ok_or_else(malformed_bind)?;
        if end > body.len() || len < 4 || len % 4 != 0 {
            return Err(malformed_bind());
        }
        // Fresh containers can retransmit old children; their IDs still need
        // replay registration and content-related acknowledgements.
        let disposition = snapshot
            .register_received_message(child_id, sequence & 1 != 0, clock, false)
            .map_err(|_| malformed_bind())?;
        if disposition == mono_proto_session::ReceivedMessageResult::Accepted {
            let child = bind_response_at_depth(
                &body[offset..end],
                msg_id,
                depth + 1,
                remaining,
                snapshot,
                clock,
                child_id,
            )?;
            response.accepted |= child.accepted;
            response.server_salt = child.server_salt.or(response.server_salt);
        } else {
            *remaining = remaining.checked_sub(1).ok_or_else(malformed_bind)?;
        }
        offset = end;
    }
    if offset != body.len() {
        return Err(malformed_bind());
    }
    Ok(response)
}

fn rpc_result_accepted(body: &[u8], msg_id: i64) -> Result<bool, MtprotoError> {
    if body.len() < 16 {
        return Err(malformed_bind());
    }
    let req = i64::from_le_bytes(body[4..12].try_into().expect("req"));
    if req != msg_id {
        return Ok(false);
    }
    let inner = u32::from_le_bytes(body[12..16].try_into().expect("inner"));
    if inner == BOOL_TRUE || inner == BOOL_FALSE {
        if body.len() != 16 {
            return Err(malformed_bind());
        }
        return if inner == BOOL_TRUE {
            Ok(true)
        } else {
            Err(MtprotoError::Message("temp bind rejected".into()))
        };
    }
    if inner == RPC_ERROR {
        if body.len() > 148 {
            return Err(malformed_bind());
        }
        let mut decoder =
            Decoder::new(&body[16..], Limits::default()).map_err(|_| malformed_bind())?;
        let error = mono_proto::transport::RpcErrorConstructor::decode(&mut decoder)
            .map_err(|_| malformed_bind())?;
        decoder.finish().map_err(|_| malformed_bind())?;
        let token = if error.error_message.len() <= 128
            && error
                .error_message
                .bytes()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == b'_')
        {
            error.error_message.as_str()
        } else {
            "UNKNOWN"
        };
        return Err(MtprotoError::Message(format!(
            "temp bind RPC {}: {token}",
            error.error_code
        )));
    }
    Err(MtprotoError::Message("temp bind result".into()))
}

#[cfg(test)]
#[path = "../../tests/unit/bind_tests.rs"]
mod tests;
