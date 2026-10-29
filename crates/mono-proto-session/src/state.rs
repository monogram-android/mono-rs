//! Persistable session snapshot and checked transitions.

use crate::{Clock, Error, RECEIVED_MESSAGE_WINDOW, RandomSource, ReceivedMessageResult};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use zeroize::Zeroize;

/// Reconnection progress, safe to persist between attempts.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReconnectState {
    /// Number of consecutive unsuccessful connections.
    pub attempts: u32,
    /// Application clock timestamp before which another attempt should not occur.
    pub retry_at_micros: Option<i64>,
}

/// Stable representation of session state.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    /// Data-center identifier.
    pub dc_id: i32,
    /// The 2048-bit authorization key, absent before authorization.
    pub auth_key: Option<Vec<u8>>,
    /// Current server salt.
    pub server_salt: i64,
    /// Random session identifier.
    pub session_id: i64,
    /// Last locally generated message identifier.
    pub last_message_id: i64,
    /// Number of content-related messages sent.
    pub content_sequence: u32,
    /// Estimated server minus local time in microseconds.
    pub time_offset_micros: i64,
    /// Message identifiers waiting to be acknowledged.
    pub pending_acknowledgements: BTreeSet<i64>,
    /// Recent authenticated incoming identifiers and whether they require acknowledgment.
    #[serde(default)]
    pub received_message_ids: BTreeMap<i64, bool>,
    /// Reconnection policy state.
    pub reconnect: ReconnectState,
}

impl std::fmt::Debug for Snapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Snapshot { .. }")
    }
}

impl Drop for Snapshot {
    fn drop(&mut self) {
        self.clear_auth_key();
    }
}

impl Snapshot {
    /// Construct an unauthorized session using caller-controlled randomness.
    pub fn new(dc_id: i32, random: &mut impl RandomSource) -> Result<Self, Error> {
        let mut bytes = [0_u8; 8];
        random.fill(&mut bytes)?;
        Ok(Self {
            dc_id,
            auth_key: None,
            server_salt: 0,
            session_id: i64::from_le_bytes(bytes),
            last_message_id: 0,
            content_sequence: 0,
            time_offset_micros: 0,
            pending_acknowledgements: BTreeSet::new(),
            received_message_ids: BTreeMap::new(),
            reconnect: ReconnectState::default(),
        })
    }

    /// Validate state loaded from an untrusted backend.
    pub fn validate(&self) -> Result<(), Error> {
        if let Some(key) = &self.auth_key {
            if key.len() != 256 {
                return Err(Error::InvalidAuthKey(key.len()));
            }
        }
        if self.last_message_id & 3 != 0 {
            return Err(Error::InvalidServerMessageId(self.last_message_id));
        }
        if self.received_message_ids.len() > RECEIVED_MESSAGE_WINDOW
            || self
                .received_message_ids
                .keys()
                .any(|message_id| message_id & 1 == 0)
        {
            return Err(Error::InvalidServerMessageId(
                self.received_message_ids
                    .keys()
                    .copied()
                    .find(|id| id & 1 == 0)
                    .unwrap_or(0),
            ));
        }
        Ok(())
    }

    /// Retire the owned key without changing session identity or protocol state.
    pub fn clear_auth_key(&mut self) {
        self.auth_key.zeroize();
    }

    /// Install a newly negotiated authorization key, wiping the replaced key.
    pub fn set_auth_key(&mut self, mut key: Vec<u8>) -> Result<(), Error> {
        if key.len() != 256 {
            let length = key.len();
            key.zeroize();
            return Err(Error::InvalidAuthKey(length));
        }
        self.clear_auth_key();
        self.auth_key = Some(key);
        Ok(())
    }

    /// Generate a strictly increasing client message id divisible by four.
    pub fn next_message_id(&mut self, clock: &impl Clock) -> Result<i64, Error> {
        let adjusted = clock
            .unix_micros()
            .checked_add(self.time_offset_micros)
            .ok_or(Error::MessageIdOverflow)?;
        let seconds = adjusted.div_euclid(1_000_000);
        let micros = adjusted.rem_euclid(1_000_000);
        let candidate = (i128::from(seconds) * (1_i128 << 32)
            + i128::from(micros) * (1_i128 << 32) / 1_000_000)
            & !3;
        let candidate = i64::try_from(candidate).map_err(|_| Error::MessageIdOverflow)?;
        let next = if candidate > self.last_message_id {
            candidate
        } else {
            self.last_message_id
                .checked_add(4)
                .ok_or(Error::MessageIdOverflow)?
        };
        self.last_message_id = next;
        Ok(next)
    }

    /// Allocate a sequence number. Content-related messages use odd numbers.
    pub fn next_sequence(&mut self, content_related: bool) -> Result<i32, Error> {
        let base = self
            .content_sequence
            .checked_mul(2)
            .ok_or(Error::SequenceOverflow)?;
        let sequence = base
            .checked_add(u32::from(content_related))
            .ok_or(Error::SequenceOverflow)?;
        if content_related {
            self.content_sequence = self
                .content_sequence
                .checked_add(1)
                .ok_or(Error::SequenceOverflow)?;
        }
        i32::try_from(sequence).map_err(|_| Error::SequenceOverflow)
    }

    /// Queue a valid server message for acknowledgement.
    pub fn acknowledge(&mut self, message_id: i64) -> Result<(), Error> {
        if message_id & 1 == 0 {
            return Err(Error::InvalidServerMessageId(message_id));
        }
        self.pending_acknowledgements.insert(message_id);
        Ok(())
    }

    /// Register an authenticated server message against replay and time windows.
    ///
    /// When `check_time` is false, parity and replay checks still apply; this is
    /// intended only for salt/time-correction service messages.
    pub fn register_received_message(
        &mut self,
        message_id: i64,
        needs_acknowledgement: bool,
        clock: &impl Clock,
        check_time: bool,
    ) -> Result<ReceivedMessageResult, Error> {
        if message_id & 1 == 0 {
            return Err(Error::InvalidServerMessageId(message_id));
        }
        if self.received_message_ids.contains_key(&message_id) {
            if needs_acknowledgement {
                self.pending_acknowledgements.insert(message_id);
            }
            return Ok(ReceivedMessageResult::Duplicate);
        }
        if check_time {
            let server_seconds = message_id >> 32;
            let adjusted_micros = clock.unix_micros().saturating_add(self.time_offset_micros);
            let local_seconds = adjusted_micros.div_euclid(1_000_000);
            if server_seconds > local_seconds.saturating_add(30)
                || server_seconds < local_seconds.saturating_sub(300)
            {
                return Ok(ReceivedMessageResult::InvalidTime);
            }
        }
        if self.received_message_ids.len() >= RECEIVED_MESSAGE_WINDOW
            && self
                .received_message_ids
                .first_key_value()
                .is_some_and(|(minimum, _)| message_id < *minimum)
        {
            return Ok(ReceivedMessageResult::TooOld);
        }
        self.received_message_ids
            .insert(message_id, needs_acknowledgement);
        while self.received_message_ids.len() > RECEIVED_MESSAGE_WINDOW {
            self.received_message_ids.pop_first();
        }
        if needs_acknowledgement {
            self.pending_acknowledgements.insert(message_id);
        }
        Ok(ReceivedMessageResult::Accepted)
    }

    /// Return whether a retained incoming message requires acknowledgment.
    pub fn received_message_needs_acknowledgement(&self, message_id: i64) -> Option<bool> {
        self.received_message_ids.get(&message_id).copied()
    }

    /// Remove and return at most `maximum` acknowledgements in stable order.
    pub fn take_acknowledgements(&mut self, maximum: usize) -> Vec<i64> {
        let values: Vec<_> = self
            .pending_acknowledgements
            .iter()
            .copied()
            .take(maximum)
            .collect();
        for value in &values {
            self.pending_acknowledgements.remove(value);
        }
        values
    }

    /// Apply a `bad_server_salt` response and make the failed message retryable.
    pub fn apply_bad_server_salt(&mut self, new_salt: i64) {
        self.server_salt = new_salt;
    }

    /// Apply clock-related `bad_msg_notification` corrections.
    pub fn apply_bad_message(
        &mut self,
        code: i32,
        bad_message_id: i64,
        server_message_id: i64,
    ) -> Result<(), Error> {
        match code {
            16 | 17 => {
                if server_message_id & 1 == 0 {
                    return Err(Error::InvalidServerMessageId(server_message_id));
                }
                let difference = (i128::from(server_message_id) - i128::from(bad_message_id))
                    * 1_000_000
                    / (1_i128 << 32);
                self.time_offset_micros = self.time_offset_micros.saturating_add(
                    i64::try_from(difference).unwrap_or(if difference < 0 {
                        i64::MIN
                    } else {
                        i64::MAX
                    }),
                );
                self.last_message_id = 0;
                Ok(())
            }
            32 => {
                self.content_sequence = self.content_sequence.saturating_add(1);
                Ok(())
            }
            33 => {
                self.content_sequence = self.content_sequence.saturating_sub(1);
                Ok(())
            }
            34 | 35 => Ok(()),
            other => Err(Error::UnsupportedBadMessage(other)),
        }
    }

    /// Move to another DC, clearing credentials and DC-specific sequencing.
    pub fn migrate(&mut self, dc_id: i32, random: &mut impl RandomSource) -> Result<(), Error> {
        let mut replacement = Self::new(dc_id, random)?;
        replacement.reconnect = self.reconnect.clone();
        *self = replacement;
        Ok(())
    }

    /// Record a failed connection and its externally calculated retry time.
    pub fn reconnect_failed(&mut self, retry_at_micros: i64) {
        self.reconnect.attempts = self.reconnect.attempts.saturating_add(1);
        self.reconnect.retry_at_micros = Some(retry_at_micros);
    }

    /// Reset reconnection state after a successful connection.
    pub fn reconnected(&mut self) {
        self.reconnect = ReconnectState::default();
    }
}
