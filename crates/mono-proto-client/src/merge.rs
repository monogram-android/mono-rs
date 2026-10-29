use mono_proto_runtime::client_session::ChannelRecovery;
use mono_proto_types::collections::{HashMap, IndexMap};
use mono_proto_types::updates_state::UpdatesStateDto;
use std::collections::VecDeque;

pub fn prefer_newer_cursor(
    current: Option<UpdatesStateDto>,
    incoming: Option<UpdatesStateDto>,
) -> Option<UpdatesStateDto> {
    match (current, incoming) {
        (None, x) | (x, None) => x,
        (Some(a), Some(b)) => Some(UpdatesStateDto {
            pts: a.pts.max(b.pts),
            qts: a.qts.max(b.qts),
            date: a.date.max(b.date),
            seq: a.seq.max(b.seq),
        }),
    }
}

// Commit only this lane's changes. An unchanged cloned entry must never
// overwrite data fetched by another lane while this lane was on the network.
pub trait MergeMap<K, V> {
    fn contains_key(&self, key: &K) -> bool;
    fn get(&self, key: &K) -> Option<&V>;
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn retain(&mut self, f: &mut dyn FnMut(&K, &mut V) -> bool);
    fn for_each_entry(self, f: &mut dyn FnMut(K, V));
}

impl<K, V> MergeMap<K, V> for HashMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn contains_key(&self, key: &K) -> bool {
        HashMap::contains_key(self, key)
    }
    fn get(&self, key: &K) -> Option<&V> {
        HashMap::get(self, key)
    }
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        HashMap::insert(self, key, value)
    }
    fn retain(&mut self, f: &mut dyn FnMut(&K, &mut V) -> bool) {
        HashMap::retain(self, |key, value| f(key, value));
    }
    fn for_each_entry(self, f: &mut dyn FnMut(K, V)) {
        for (key, value) in self {
            f(key, value);
        }
    }
}

impl<K, V> MergeMap<K, V> for IndexMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn contains_key(&self, key: &K) -> bool {
        IndexMap::contains_key(self, key)
    }
    fn get(&self, key: &K) -> Option<&V> {
        IndexMap::get(self, key)
    }
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        IndexMap::insert(self, key, value)
    }
    fn retain(&mut self, f: &mut dyn FnMut(&K, &mut V) -> bool) {
        IndexMap::retain(self, |key, value| f(key, value));
    }
    fn for_each_entry(self, f: &mut dyn FnMut(K, V)) {
        for (key, value) in self {
            f(key, value);
        }
    }
}

impl MergeMap<i64, ChannelRecovery> for VecDeque<ChannelRecovery> {
    fn contains_key(&self, key: &i64) -> bool {
        self.iter().any(|entry| entry.chat_id == *key)
    }
    fn get(&self, key: &i64) -> Option<&ChannelRecovery> {
        self.iter().find(|entry| entry.chat_id == *key)
    }
    fn insert(&mut self, key: i64, value: ChannelRecovery) -> Option<ChannelRecovery> {
        if let Some(entry) = self.iter_mut().find(|entry| entry.chat_id == key) {
            Some(std::mem::replace(entry, value))
        } else {
            self.push_back(value);
            None
        }
    }
    fn retain(&mut self, f: &mut dyn FnMut(&i64, &mut ChannelRecovery) -> bool) {
        self.retain_mut(|entry| {
            let key = entry.chat_id;
            f(&key, entry)
        });
    }
    fn for_each_entry(self, f: &mut dyn FnMut(i64, ChannelRecovery)) {
        for entry in self {
            f(entry.chat_id, entry);
        }
    }
}

// Commit only this lane's changes. An unchanged cloned entry must never
// overwrite data fetched by another lane while this lane was on the network.
pub fn merge_changed_entries<K, V, M>(current: &mut M, before: &M, incoming: M) -> bool
where
    K: Eq + std::hash::Hash,
    V: PartialEq,
    M: MergeMap<K, V>,
{
    let mut changed = false;
    current.retain(&mut |key, value| {
        let keep = incoming.contains_key(key) || before.get(key) != Some(value);
        if !keep {
            changed = true;
        }
        keep
    });
    incoming.for_each_entry(&mut |key, value| {
        if before.get(&key) != Some(&value) && current.get(&key) == before.get(&key) {
            current.insert(key, value);
            changed = true;
        }
    });
    changed
}
