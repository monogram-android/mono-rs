//! Typed RPC correlation, timeout, and retry engine.

use crate::Error;
use mono_proto_session::{Clock, ReceivedMessageResult, Snapshot};
use std::collections::{BTreeMap, VecDeque};
use std::marker::PhantomData;

/// Schema-generated method adapter. No global constructor table is required.
pub trait Method {
    /// Strongly typed generated result.
    type Response;
    /// Encode the boxed method invocation for the selected declaration layer.
    fn encode_request(&self) -> Result<Vec<u8>, mono_proto_codec::Error>;
    /// Decode its typed result from a boxed TL payload.
    fn decode_response(bytes: &[u8]) -> Result<Self::Response, mono_proto_codec::Error>;
}

/// Typed handle returned when a method is queued.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RequestHandle<T> {
    message_id: i64,
    marker: PhantomData<fn() -> T>,
}
impl<T> RequestHandle<T> {
    /// MTProto message id used for correlation.
    pub const fn message_id(&self) -> i64 {
        self.message_id
    }
}

/// Retry decision made without binding to a timer runtime.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetryDecision {
    /// Retry at the supplied application-clock timestamp.
    RetryAt(i64),
    /// Surface the failure immediately.
    Fail,
}

/// Application-defined timeout and retry behavior.
pub trait RetryPolicy {
    /// Deadline for an attempt sent at `now_micros`.
    fn deadline(&self, attempt: u32, now_micros: i64) -> i64;
    /// Decide whether and when an unsuccessful request is retryable.
    fn after_failure(&self, attempt: u32, now_micros: i64, error: &Error) -> RetryDecision;
}

/// A bounded exponential policy using caller clock units (microseconds).
#[derive(Clone, Copy, Debug)]
pub struct ExponentialBackoff {
    /// Timeout per individual attempt.
    pub timeout_micros: i64,
    /// Initial retry delay.
    pub initial_delay_micros: i64,
    /// Maximum number of sends, including the first.
    pub max_attempts: u32,
}
impl RetryPolicy for ExponentialBackoff {
    fn deadline(&self, _attempt: u32, now: i64) -> i64 {
        now.saturating_add(self.timeout_micros.max(0))
    }
    fn after_failure(&self, attempt: u32, now: i64, _error: &Error) -> RetryDecision {
        if attempt >= self.max_attempts {
            return RetryDecision::Fail;
        }
        let shift = attempt.saturating_sub(1).min(62);
        RetryDecision::RetryAt(
            now.saturating_add(
                self.initial_delay_micros
                    .max(0)
                    .saturating_mul(1_i64 << shift),
            ),
        )
    }
}

/// Plain MTProto message ready for encryption and transport framing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutboundMessage {
    /// Unique message id.
    pub message_id: i64,
    /// MTProto sequence number.
    pub sequence: i32,
    /// Boxed TL method or service payload.
    pub body: Vec<u8>,
}

/// Authenticated inbound message and replay-window disposition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InboundMessage {
    /// Decrypted and authenticated envelope.
    pub message: mono_proto_crypto::DecryptedMessage,
    /// Whether the identifier is new, duplicate, too old, or mistimed.
    pub disposition: ReceivedMessageResult,
}

#[derive(Clone, Debug)]
struct Pending {
    logical_id: i64,
    body: Vec<u8>,
    attempts: u32,
    deadline: i64,
    retry_at: Option<i64>,
}

/// Correlation and retry engine. It deliberately performs no socket or timer I/O.
#[derive(Debug)]
pub struct Engine<P> {
    /// Mutable protocol session.
    pub session: Snapshot,
    policy: P,
    pending: BTreeMap<i64, Pending>,
    outbound: VecDeque<OutboundMessage>,
    responses: BTreeMap<i64, Vec<u8>>,
}

impl<P: RetryPolicy> Engine<P> {
    /// Create an engine around validated session state.
    pub fn new(session: Snapshot, policy: P) -> Result<Self, Error> {
        session.validate()?;
        Ok(Self {
            session,
            policy,
            pending: BTreeMap::new(),
            outbound: VecDeque::new(),
            responses: BTreeMap::new(),
        })
    }

    /// Encode and queue a generated API method.
    pub fn invoke<M: Method>(
        &mut self,
        method: &M,
        clock: &impl Clock,
    ) -> Result<RequestHandle<M::Response>, Error> {
        let body = method.encode_request()?;
        let now = clock.unix_micros();
        let message_id = self.session.next_message_id(clock)?;
        let sequence = self.session.next_sequence(true)?;
        self.pending.insert(
            message_id,
            Pending {
                logical_id: message_id,
                body: body.clone(),
                attempts: 1,
                deadline: self.policy.deadline(1, now),
                retry_at: None,
            },
        );
        self.outbound.push_back(OutboundMessage {
            message_id,
            sequence,
            body,
        });
        Ok(RequestHandle {
            message_id,
            marker: PhantomData,
        })
    }

    /// Take the next payload for encryption and transport.
    pub fn next_outbound(&mut self) -> Option<OutboundMessage> {
        self.outbound.pop_front()
    }

    /// Encrypt an outbound engine message with the active session key.
    pub fn seal_outbound(
        &self,
        message: &OutboundMessage,
        random_padding: &[u8],
    ) -> Result<Vec<u8>, Error> {
        let auth_key = self
            .session
            .auth_key
            .as_deref()
            .ok_or_else(|| Error::Authorization("session has no authorization key".into()))?;
        mono_proto_crypto::encrypt_message(
            auth_key,
            mono_proto_crypto::MessageToEncrypt {
                server_salt: self.session.server_salt,
                session_id: self.session.session_id,
                message_id: message.message_id,
                sequence: message.sequence,
                body: &message.body,
                padding: random_padding,
            },
            mono_proto_crypto::Direction::ClientToServer,
        )
        .map_err(Error::from)
    }

    /// Decrypt, authenticate, session-check, and replay-register an inbound message.
    pub fn open_inbound(
        &mut self,
        packet: &[u8],
        clock: &impl Clock,
        check_time: bool,
        max_body_length: usize,
    ) -> Result<InboundMessage, Error> {
        let auth_key = self
            .session
            .auth_key
            .as_deref()
            .ok_or_else(|| Error::Authorization("session has no authorization key".into()))?;
        let message = mono_proto_crypto::decrypt_message(
            auth_key,
            packet,
            mono_proto_crypto::Direction::ServerToClient,
            self.session.session_id,
            max_body_length,
        )?;
        let disposition = self.session.register_received_message(
            message.message_id,
            message.sequence & 1 != 0,
            clock,
            check_time,
        )?;
        Ok(InboundMessage {
            message,
            disposition,
        })
    }

    /// Correlate an `rpc_result` body with its request.
    pub fn receive_result(
        &mut self,
        request_message_id: i64,
        response_body: Vec<u8>,
        server_message_id: i64,
    ) -> Result<(), Error> {
        let pending = self
            .pending
            .remove(&request_message_id)
            .ok_or(Error::UnknownRequest(request_message_id))?;
        self.session.acknowledge(server_message_id)?;
        self.responses.insert(pending.logical_id, response_body);
        Ok(())
    }

    /// Decode and consume a typed response.
    pub fn take_response<M: Method>(
        &mut self,
        handle: &RequestHandle<M::Response>,
    ) -> Result<Option<M::Response>, Error> {
        self.responses
            .remove(&handle.message_id)
            .map(|bytes| M::decode_response(&bytes).map_err(Error::from))
            .transpose()
    }

    /// Mark a request retryable after reconnect, salt correction, or a transient failure.
    pub fn fail_request(
        &mut self,
        message_id: i64,
        now_micros: i64,
        error: &Error,
    ) -> Result<(), Error> {
        let pending = self
            .pending
            .get_mut(&message_id)
            .ok_or(Error::UnknownRequest(message_id))?;
        match self
            .policy
            .after_failure(pending.attempts, now_micros, error)
        {
            RetryDecision::RetryAt(at) => {
                pending.retry_at = Some(at);
                Ok(())
            }
            RetryDecision::Fail => {
                let attempts = pending.attempts;
                self.pending.remove(&message_id);
                Err(Error::RetryExhausted {
                    message_id,
                    attempts,
                })
            }
        }
    }

    /// Advance time, emitting retries with fresh message ids and sequence numbers.
    pub fn poll(&mut self, clock: &impl Clock) -> Result<Vec<Error>, Error> {
        let now = clock.unix_micros();
        let mut failures = Vec::new();
        let due: Vec<i64> = self
            .pending
            .iter()
            .filter(|(_, request)| {
                request.retry_at.is_some_and(|at| at <= now) || request.deadline <= now
            })
            .map(|(id, _)| *id)
            .collect();
        for old_id in due {
            let mut request = self
                .pending
                .remove(&old_id)
                .expect("collected existing keys");
            if request.retry_at.is_none() {
                let timeout = Error::Timeout {
                    message_id: old_id,
                    attempts: request.attempts,
                };
                match self.policy.after_failure(request.attempts, now, &timeout) {
                    RetryDecision::RetryAt(at) if at > now => {
                        request.retry_at = Some(at);
                        self.pending.insert(old_id, request);
                        continue;
                    }
                    RetryDecision::Fail => {
                        failures.push(timeout);
                        continue;
                    }
                    RetryDecision::RetryAt(_) => {}
                }
            }
            request.attempts = request.attempts.saturating_add(1);
            request.retry_at = None;
            request.deadline = self.policy.deadline(request.attempts, now);
            let new_id = self.session.next_message_id(clock)?;
            let sequence = self.session.next_sequence(true)?;
            self.outbound.push_back(OutboundMessage {
                message_id: new_id,
                sequence,
                body: request.body.clone(),
            });
            self.pending.insert(new_id, request);
        }
        Ok(failures)
    }

    /// Number of calls awaiting a response or retry.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
}
