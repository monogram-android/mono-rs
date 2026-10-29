//! Encrypted MTProto RPC over padded-intermediate framing using Mono Proto Engine.
//! https://core.telegram.org/mtproto/description
//! https://core.telegram.org/mtproto/mtproto-transports
//! https://core.telegram.org/mtproto/service_messages

use crate::dc;
mod framing;
mod inbound;
mod invoke;
mod live;
mod main_policy;
pub use crate::salts;
mod supervisor;
use crate::rpc_timeout as timeout;

pub use dc::{dc_endpoints, rotated_endpoints, same_ip_endpoints, set_use_test_dc, use_test_dc};
pub use framing::SystemClock;
pub use inbound::{
    BoxedQuery, NewSessionMetadata, clear_new_session_metadata, encode_boxed_bytes,
    take_new_session_metadata,
};
pub use inbound::{gzip_if_smaller, ungzip_if_needed};
pub use invoke::invoke_raw;
pub use live::queue_rpc_updates;
pub use live::{LiveTransport, drop_live_transport, with_live_transport};

pub fn enqueue_response(
    dc_id: i32,
    updates: &mono_proto::latest::api::Updates,
) -> Result<(), crate::MtprotoError> {
    let mut encoder = mono_proto::codec::Encoder::new();
    mono_proto::codec::TlEncode::encode(updates, &mut encoder)
        .map_err(|error| crate::MtprotoError::Message(error.to_string()))?;
    queue_rpc_updates(dc_id, encoder.into_bytes())
}
pub use timeout::rpc_timeout_secs;
pub use timeout::with_rpc_timeout_secs;

pub use invoke::{
    flush_pending_acks, invoke_batch_raw_with_retry, invoke_batch_raw_with_retry_streaming,
    invoke_raw_with_retry_factory, ping_existing_rtt, receive_updates,
};
pub use main_policy::ACK_DELAY;
pub use timeout::trim_padded_mtproto_packet;

#[cfg(test)]
use crate::MtprotoError;
#[cfg(test)]
fn timeout_idle_needs_probe(err: &MtprotoError) -> bool {
    match err {
        MtprotoError::Message(message) => timeout::timeout_idle_needs_probe(message),
        _ => false,
    }
}
#[cfg(test)]
use crate::tcp;
#[cfg(test)]
use mono_proto::codec::{Decoder, Limits, TlDecode};
#[cfg(test)]
use mono_proto::transport::{
    GzipPackedConstructor, MsgResendReqConstructor, NewSessionCreatedConstructor, PingRequest,
    PongConstructor, RpcErrorConstructor, Vector,
};
#[cfg(test)]
use mono_proto_engine::{Engine, ExponentialBackoff};
#[cfg(test)]
use mono_proto_session::{Clock, OsRandom, ReceivedMessageResult, Snapshot};
#[cfg(test)]
use mono_proto_transport::{Framing, PaddedIntermediate};
#[cfg(test)]
use std::io::Read;

#[cfg(test)]
pub use dc::{extra_reconnect_same_host, reconnect_backoff, reconnect_delay, rotated_same_ip};
#[cfg(any(test, feature = "test-support"))]
pub use framing::open_live_addr;
#[cfg(test)]
pub use framing::{
    bad_msg_should_reconnect, clock_delta_micros, detailed_answer_request, queue_update,
    recreate_session_after_bad_message,
};
#[cfg(test)]
pub use inbound::{
    BAD_MSG_NOTIFICATION, BAD_SERVER_SALT, InboundEvent, MAX_UNPACKED_BYTES, MAX_WRAPPER_DEPTH,
    MSG_CONTAINER, MSG_COPY, MSGS_ACK, NEW_SESSION_CREATED, PONG, RPC_RESULT,
    apply_new_session_salt, is_updates_type, parse_authenticated, parse_service_or_result,
    should_process_inbound,
};
#[cfg(test)]
pub use invoke::{RawMethod, RpcRetryPolicy, may_reconnect_request};
#[cfg(test)]
pub use live::{live_stack_depth, live_top_occupied};
#[cfg(test)]
pub use supervisor::{
    ConnectionState, ConnectionSupervisor, FailureClass, failure_class, is_transport_error,
    is_waitable_io,
};
#[cfg(test)]
pub use timeout::{
    KEEPALIVE_PING_SECS, fail_fast_idle, idle_after_complete_frames, idle_empty_first_byte,
    idle_needs_liveness_probe, idle_reused_socket, is_mid_frame, keepalive_probe_failed,
    leftover_frame_grace, live_transport_stale, note_inbound_liveness, recv_wait_deadline,
    rpc_attempt_budget, rpc_timeout_message, subscribed_read_deadline,
};

#[cfg(test)]
#[path = "../../tests/unit/rpc_container_tests.rs"]
mod container_tests;
#[cfg(test)]
#[path = "../../tests/unit/rpc_dc_tests.rs"]
mod dc_tests;
#[cfg(test)]
#[path = "../../tests/unit/rpc_gzip_tests.rs"]
mod gzip_tests;
#[cfg(test)]
#[path = "../../tests/unit/rpc_push_tests.rs"]
mod push_tests;
#[cfg(test)]
#[path = "../../tests/unit/rpc_wire_tests.rs"]
mod wire_tests;

#[cfg(test)]
#[path = "../../tests/unit/rpc_session_tests.rs"]
mod session_tests;
#[cfg(test)]
#[path = "../../tests/unit/rpc_timeout_tests.rs"]
mod timeout_tests;
