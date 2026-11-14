//! Run explicitly with --release --ignored --nocapture --test-threads=1.
//! Full saves include production validation, JSON, AEAD, atomic rename and fsync.
//! Pure operations and full saves overlap in work and MUST NOT be added together.
use crate as storage;
#[path = "native_workload_support.rs"]
mod support;
use crate::{
    client_session::ClientSession,
    event_types::UpdateEventDto,
    session_crypto,
    session_store::FileSessionStore,
    update_journal::{DurableBatch, MAX_UNACKED_BATCHES, UpdateJournal},
    updates_state::UpdatesStateDto,
};
use std::{fs, io::Write};
use support::{CLASSES, TempDir, fixture, measure};

fn cursor(n: usize) -> UpdatesStateDto {
    UpdatesStateDto {
        pts: n as i32,
        qts: 0,
        seq: n as i32,
        date: 1_700_000_000 + n as i32,
    }
}
fn append(session: &mut ClientSession, n: usize) -> DurableBatch {
    let after = cursor(n);
    let batch = session.journal.push(
        vec![UpdateEventDto::MessagesDeleted {
            chat_id: Some(7),
            message_ids: (1..=64).map(|i| n as i32 * 64 + i).collect(),
        }],
        Some(after.clone()),
        vec![(7, after.pts)],
    );
    session.updates = Some(after);
    session.channel_pts.insert(7, n as i32);
    batch
}
fn journal_fixture(count: usize) -> ClientSession {
    let mut session = fixture(8);
    for n in 1..=count {
        append(&mut session, n);
    }
    session
}
fn verify_replay(session: &ClientSession, count: usize) {
    assert_eq!(session.journal.next_batch_id, count as i64 + 1);
    let batches = session.journal.pending();
    assert_eq!(batches.len(), count);
    for (index, batch) in batches.iter().enumerate() {
        let n = index + 1;
        assert_eq!(batch.batch_id, n as i64);
        let after = batch.cursor_after.as_ref().unwrap();
        assert_eq!(
            (after.pts, after.seq, after.date),
            (n as i32, n as i32, cursor(n).date)
        );
        assert_eq!(batch.channel_pts_after, vec![(7, n as i32)]);
        assert!(
            matches!(&batch.events[..], [UpdateEventDto::MessagesDeleted { chat_id: Some(7), message_ids }]
            if message_ids.len() == 64 && message_ids[0] == n as i32 * 64 + 1)
        );
    }
    assert_eq!(session.updates.as_ref().unwrap().pts, count as i32);
    assert_eq!(session.channel_pts[&7], count as i32);
}
/// Synthetic application's commit barrier, always before acknowledging a batch.
fn commit_receipt(dir: &TempDir, batch_id: i64) {
    let mut receipt = fs::File::create(dir.0.join("application-receipt")).unwrap();
    receipt.write_all(&batch_id.to_le_bytes()).unwrap();
    receipt.sync_all().unwrap();
    #[cfg(unix)]
    fs::File::open(&dir.0).unwrap().sync_all().unwrap();
}

#[test]
#[ignore = "bounded storage workload benchmark; run explicitly in release"]
fn native_workload_session_stages() {
    assert!(!cfg!(debug_assertions), "run with --release");
    for (class, count) in CLASSES {
        let session = fixture(count);
        let plaintext = serde_json::to_vec(&session).unwrap();
        let bytes = plaintext.len();
        let dir = TempDir::new();
        let path = dir.0.join("session.json");
        let key = session_crypto::register(&path, &[0x52; 32]).unwrap();
        let store = FileSessionStore::new(&path);
        let encrypted = session_crypto::encrypt(&plaintext, &key).unwrap();
        measure(
            "session_json_encode",
            class,
            count,
            bytes,
            || (),
            |_| serde_json::to_vec(&session),
            |result| assert_eq!(result.as_ref().unwrap().len(), bytes),
        );
        measure(
            "session_authenticated_encrypt",
            class,
            count,
            bytes,
            || (),
            |_| session_crypto::encrypt(&plaintext, &key),
            |result| {
                let output = result.as_ref().unwrap();
                assert!(session_crypto::is_encrypted(output));
                assert_eq!(&*session_crypto::decrypt(output, &key).unwrap(), &plaintext);
            },
        );
        measure(
            "session_authenticated_decrypt",
            class,
            count,
            encrypted.len(),
            || (),
            |_| session_crypto::decrypt(&encrypted, &key),
            |result| assert_eq!(&**result.as_ref().unwrap(), &plaintext),
        );
        // Seed once so every timed save measures atomic replacement, not creation.
        store.save(&session).unwrap();
        measure(
            "session_atomic_encrypted_save_total",
            class,
            count,
            bytes,
            || (),
            |_| store.save(&session),
            |result| {
                result.as_ref().unwrap();
                let restored = store.load().unwrap().unwrap();
                assert_eq!(restored.peers.len(), count);
                assert_eq!(restored.media.len(), count);
                assert_eq!(restored.media_order, session.media_order);
                assert_eq!(restored.snapshot.auth_key, session.snapshot.auth_key);
            },
        );
        measure(
            "session_encrypted_load_total",
            class,
            count,
            encrypted.len(),
            || (),
            |_| store.load(),
            |result| {
                let restored = result.as_ref().unwrap().as_ref().unwrap();
                assert_eq!(restored.peers.len(), count);
                assert_eq!(restored.media.len(), count);
            },
        );
        assert_eq!(
            fs::read_dir(&dir.0).unwrap().count(),
            1,
            "no leaked atomic temp files"
        );
    }
}

#[test]
#[ignore = "bounded journal workload benchmark; run explicitly in release"]
fn native_workload_journal_stages() {
    assert!(!cfg!(debug_assertions), "run with --release");
    for (class, count) in [
        ("small", 8),
        ("realistic", 64),
        ("stress", MAX_UNACKED_BATCHES),
    ] {
        let dir = TempDir::new();
        let path = dir.0.join("journal.session");
        let _key = session_crypto::register(&path, &[0x63; 32]).unwrap();
        let store = FileSessionStore::new(&path);
        let base = journal_fixture(count);
        let bytes = serde_json::to_vec(&base).unwrap().len();
        store.save(&base).unwrap();
        let restored = store.load().unwrap().unwrap();
        verify_replay(&restored, count);
        assert!(restored.journal.has_undelivered());
        commit_receipt(&dir, 1);
        let generation = base.journal.account_generation;
        let mut before_append = base.clone();
        before_append.journal.batches.pop_back();
        before_append.journal.next_batch_id = count as i64;
        before_append.updates = Some(cursor(count - 1));
        before_append.channel_pts.insert(7, count as i32 - 1);
        measure(
            "journal_append_memory",
            class,
            count,
            bytes,
            || before_append.clone(),
            |mut session| {
                let batch = append(&mut session, count);
                (session, batch)
            },
            |(session, batch)| {
                assert_eq!(batch.batch_id, count as i64);
                verify_replay(session, count);
            },
        );
        measure(
            "journal_pending_clone_memory",
            class,
            count,
            bytes,
            || (),
            |_| base.journal.pending(),
            |batches| {
                assert_eq!(batches.len(), count);
                assert_eq!(batches[0].batch_id, 1);
                assert_eq!(batches[count - 1].batch_id, count as i64);
            },
        );
        measure(
            "journal_ack_memory",
            class,
            count,
            bytes,
            || base.journal.clone(),
            |mut journal| {
                let removed = journal.acknowledge(generation, 1);
                (journal, removed)
            },
            |(journal, removed)| {
                assert!(*removed);
                assert_eq!(journal.batches.len(), count - 1);
                assert_eq!(journal.batches.front().unwrap().batch_id, 2);
            },
        );
        measure(
            "journal_append_atomic_save_total",
            class,
            count,
            bytes,
            || {
                store.save(&before_append).unwrap();
                before_append.clone()
            },
            |mut session| {
                append(&mut session, count);
                let saved = store.save(&session);
                (session, saved)
            },
            |(_, result)| {
                result.as_ref().unwrap();
                verify_replay(&store.load().unwrap().unwrap(), count);
            },
        );
        measure(
            "journal_disk_replay_total",
            class,
            count,
            bytes,
            || (),
            |_| store.load(),
            |result| verify_replay(result.as_ref().unwrap().as_ref().unwrap(), count),
        );
        measure(
            "journal_ack_atomic_compaction_total",
            class,
            count,
            bytes,
            || {
                store.save(&base).unwrap();
                let mut acknowledged = base.clone();
                assert!(acknowledged.journal.acknowledge(generation, 1));
                acknowledged
            },
            |session| {
                let saved = store.save(&session);
                (session, saved)
            },
            |(_, result)| {
                result.as_ref().unwrap();
                let replay = store.load().unwrap().unwrap();
                assert_eq!(replay.journal.batches.len(), count - 1);
                assert_eq!(replay.journal.batches.front().unwrap().batch_id, 2);
                assert_eq!(replay.journal.next_batch_id, count as i64 + 1);
                assert_eq!(replay.updates.as_ref().unwrap().seq, count as i32);
                assert!(fs::metadata(&path).unwrap().len() < (bytes + 48) as u64);
            },
        );
    }
}

#[test]
fn native_workload_journal_durability_and_rejection() {
    let dir = TempDir::new();
    let path = dir.0.join("journal.session");
    let key = session_crypto::register(&path, &[0x74; 32]).unwrap();
    let store = FileSessionStore::new(&path);
    let mut session = journal_fixture(8);
    let generation = session.journal.account_generation;
    session.journal.mark_all_delivered();
    assert!(!session.journal.has_undelivered());
    store.save(&session).unwrap();
    let original = fs::read(&path).unwrap();
    let restored = store.load().unwrap().unwrap();
    verify_replay(&restored, 8);
    assert!(
        restored.journal.has_undelivered(),
        "delivery restarts after reload"
    );
    assert!(!session.journal.acknowledge(generation + 1, 1));
    assert!(!session.journal.acknowledge(generation, 9));
    assert_eq!(session.journal.batches.len(), 8);
    commit_receipt(&dir, 1);
    assert!(session.journal.acknowledge(generation, 1));
    assert!(!session.journal.acknowledge(generation, 1));
    // Before the next durable save a restart must still replay the receipt safely.
    assert_eq!(
        store
            .load()
            .unwrap()
            .unwrap()
            .journal
            .batches
            .front()
            .unwrap()
            .batch_id,
        1
    );
    store.save(&session).unwrap();
    let compacted = fs::read(&path).unwrap();
    assert!(compacted.len() < original.len());
    let restored = store.load().unwrap().unwrap();
    assert_eq!(restored.journal.batches.front().unwrap().batch_id, 2);
    assert_eq!(restored.updates.as_ref().unwrap().seq, 8);
    session.snapshot.auth_key = Some(vec![1; 3]);
    assert!(store.save(&session).is_err());
    assert_eq!(fs::read(&path).unwrap(), compacted);
    for index in [session_crypto::MAGIC.len(), compacted.len() - 1] {
        let mut corrupt = compacted.clone();
        corrupt[index] ^= 1;
        assert!(session_crypto::decrypt(&corrupt, &key).is_err());
        fs::write(&path, &corrupt).unwrap();
        assert!(store.load().is_err());
        assert_eq!(fs::read(&path).unwrap(), corrupt);
    }
    fs::write(&path, &compacted[..compacted.len() - 1]).unwrap();
    assert!(store.load().is_err());
    let mut full = journal_fixture(MAX_UNACKED_BATCHES);
    assert!(!full.journal.admission_open());
    let generation = full.journal.account_generation;
    store.save(&full).unwrap();
    commit_receipt(&dir, 1);
    assert!(full.journal.acknowledge(generation, 1));
    assert!(full.journal.admission_open());
    let blocker = dir.0.join("not-a-directory");
    fs::write(&blocker, b"synthetic blocker").unwrap();
    assert!(
        FileSessionStore::new(blocker.join("session"))
            .save(&full)
            .is_err()
    );
    // A failed save never acknowledges or mutates the caller's journal.
    assert_eq!(full.journal.batches.len(), MAX_UNACKED_BATCHES - 1);
    assert_eq!(
        store.load().unwrap().unwrap().journal.batches.len(),
        MAX_UNACKED_BATCHES
    );
    let encoded = serde_json::to_vec(&full.journal).unwrap();
    let replay: UpdateJournal = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(replay.next_batch_id, MAX_UNACKED_BATCHES as i64 + 1);
}

#[test]
fn native_workload_atomic_replace_failure_cleans_temp_without_acknowledging() {
    let dir = TempDir::new();
    let target = dir.0.join("occupied-directory");
    fs::create_dir(&target).unwrap();
    fs::write(target.join("sentinel"), b"synthetic original").unwrap();
    let _key = session_crypto::register(&target, &[0x75; 32]).unwrap();
    let session = journal_fixture(8);
    // Encoding, encryption, file write and file fsync succeed. Replacing a
    // nonempty directory with the session file must fail and clean its temp.
    assert!(FileSessionStore::new(&target).save(&session).is_err());
    assert_eq!(
        fs::read(target.join("sentinel")).unwrap(),
        b"synthetic original"
    );
    assert_eq!(fs::read_dir(&dir.0).unwrap().count(), 1);
    assert_eq!(session.journal.batches.len(), 8);
    assert_eq!(session.journal.batches.front().unwrap().batch_id, 1);
    verify_replay(&session, 8);
}
