use std::time::{Duration, Instant};

use crate::collections::{HashMap, HashMapExt};

pub const ACK_DELAY: Duration = Duration::from_secs(10);

pub struct SilentRequests {
    sent: HashMap<i64, (Instant, bool)>,
    probes: HashMap<i64, Vec<i64>>,
    wire_ids: HashMap<i64, i64>,
    delay: Duration,
}

impl SilentRequests {
    pub fn new(media: bool) -> Self {
        Self {
            sent: HashMap::new(),
            probes: HashMap::new(),
            wire_ids: HashMap::new(),
            delay: Duration::from_secs(if media { 30 } else { 8 }),
        }
    }

    pub fn observe(&mut self, ids: &[i64], now: Instant) {
        self.wire_ids.retain(|id, _| ids.contains(id));
        for &id in ids {
            self.wire_ids.entry(id).or_insert(id);
        }
        self.sent
            .retain(|id, _| self.wire_ids.values().any(|wire| wire == id));
        for &id in self.wire_ids.values() {
            self.sent.entry(id).or_insert((now, false));
        }
    }

    pub fn resent(&mut self, old_id: i64, new_id: i64, now: Instant) {
        for wire in self.wire_ids.values_mut() {
            if *wire == old_id {
                *wire = new_id;
            }
        }
        if self.sent.remove(&old_id).is_some() {
            self.sent.insert(new_id, (now, false));
        }
    }

    pub fn next_deadline(&self) -> Option<Instant> {
        self.sent
            .values()
            .filter(|(_, probed)| !probed)
            .map(|(at, _)| *at + self.delay)
            .min()
    }

    pub fn due(&self, now: Instant, force: bool) -> Vec<i64> {
        self.sent
            .iter()
            .filter(|(_, (at, probed))| !probed && (force || now >= *at + self.delay))
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn probed(&mut self, probe: i64, ids: Vec<i64>) {
        for id in &ids {
            if let Some((_, done)) = self.sent.get_mut(id) {
                *done = true;
            }
        }
        self.probes.insert(probe, ids);
    }

    pub fn status(&mut self, probe: i64, info: &[u8]) -> Result<(), &'static str> {
        let Some(ids) = self.probes.remove(&probe) else {
            return Ok(());
        };
        if ids.len() != info.len() {
            return Err("invalid MTProto probe status count");
        }
        for (&id, &state) in ids.iter().zip(info) {
            if self.sent.contains_key(&id) && matches!(state & 7, 2 | 3) {
                return Err("RPC timeout: status probe confirms request not received");
            }
        }
        Ok(())
    }
}
