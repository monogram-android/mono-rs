//! Generated-declaration authorization transition orchestration.

use crate::Error;
use mono_proto_session::Snapshot;

/// Authorization phases driven by an adapter that owns generated handshake types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthorizationPhase {
    /// No nonce sent.
    Initial,
    /// Waiting for server parameters.
    WaitingForParameters,
    /// Waiting for DH parameters.
    WaitingForDh,
    /// Waiting for DH acceptance.
    WaitingForAcceptance,
    /// Authorization key is installed.
    Authorized,
}

/// Generated-declaration authorization adapter; cryptographic validation stays in
/// `mono-proto-crypto`, while this engine enforces transition ordering.
pub trait AuthorizationAdapter {
    /// Produce the initial `req_pq_multi` body.
    fn request_parameters(&mut self) -> Result<Vec<u8>, Error>;
    /// Validate parameters and produce the encrypted DH request.
    fn accept_parameters(&mut self, response: &[u8]) -> Result<Vec<u8>, Error>;
    /// Validate server DH data and produce the client DH request.
    fn accept_dh(&mut self, response: &[u8]) -> Result<Vec<u8>, Error>;
    /// Validate completion and return `(auth_key, server_salt)`.
    fn accept_authorization(&mut self, response: &[u8]) -> Result<(Vec<u8>, i64), Error>;
    /// Clock correction derived from authenticated server DH data.
    fn time_offset_micros(&self) -> Option<i64> {
        None
    }
}

/// Ordered authorization orchestrator independent of a Telegram declaration layer.
pub struct Authorization<A> {
    phase: AuthorizationPhase,
    adapter: A,
}
impl<A: AuthorizationAdapter> Authorization<A> {
    /// Create a new authorization exchange.
    pub const fn new(adapter: A) -> Self {
        Self {
            phase: AuthorizationPhase::Initial,
            adapter,
        }
    }
    /// Current state.
    pub const fn phase(&self) -> AuthorizationPhase {
        self.phase
    }
    /// Recover exchange-specific metadata after completion.
    pub fn into_adapter(self) -> A {
        self.adapter
    }
    /// Begin authorization.
    pub fn start(&mut self) -> Result<Vec<u8>, Error> {
        if self.phase != AuthorizationPhase::Initial {
            return Err(Error::Authorization("exchange already started".into()));
        }
        let body = self.adapter.request_parameters()?;
        self.phase = AuthorizationPhase::WaitingForParameters;
        Ok(body)
    }
    /// Accept the next response and optionally install the completed session key.
    pub fn advance(
        &mut self,
        response: &[u8],
        session: &mut Snapshot,
    ) -> Result<Option<Vec<u8>>, Error> {
        match self.phase {
            AuthorizationPhase::WaitingForParameters => {
                let body = self.adapter.accept_parameters(response)?;
                self.phase = AuthorizationPhase::WaitingForDh;
                Ok(Some(body))
            }
            AuthorizationPhase::WaitingForDh => {
                let body = self.adapter.accept_dh(response)?;
                if let Some(offset) = self.adapter.time_offset_micros() {
                    session.time_offset_micros = offset;
                    session.last_message_id = 0;
                }
                self.phase = AuthorizationPhase::WaitingForAcceptance;
                Ok(Some(body))
            }
            AuthorizationPhase::WaitingForAcceptance => {
                let (key, salt) = self.adapter.accept_authorization(response)?;
                session.set_auth_key(key)?;
                session.server_salt = salt;
                if let Some(offset) = self.adapter.time_offset_micros() {
                    session.time_offset_micros = offset;
                    // Unencrypted IDs generated using an incorrect host clock must
                    // not force the new key's encrypted IDs into the future.
                    session.last_message_id = 0;
                }
                self.phase = AuthorizationPhase::Authorized;
                Ok(None)
            }
            _ => Err(Error::Authorization(
                "response is invalid in current phase".into(),
            )),
        }
    }
}
