use super::*;
use crate::client_session::{ChannelRecovery, DcAuthKey};
use crate::updates_state::UpdatesStateDto;
use mono_proto_session::OsRandom;

struct TestDir(PathBuf);

impl TestDir {
    fn new() -> Self {
        let mut nonce = [0_u8; 16];
        mono_proto_crypto::fill_random(&mut nonce).unwrap();
        let path = std::env::temp_dir().join(format!(
            "mtproto-session-{:032x}",
            u128::from_le_bytes(nonce)
        ));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    fn store(&self) -> FileSessionStore {
        FileSessionStore::new(self.0.join("session.json"))
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn seed(store: &FileSessionStore) -> ClientSession {
    let snapshot = Snapshot::new(2, &mut OsRandom).unwrap();
    fs::write(&store.path, serde_json::to_vec(&snapshot).unwrap()).unwrap();
    store.load().unwrap().unwrap()
}

#[test]
fn missing_session_is_absent_but_empty_and_corrupt_files_fail() {
    let dir = TestDir::new();
    let store = dir.store();
    assert!(store.load().unwrap().is_none());
    for bytes in [b"".as_slice(), b"{\"snapshot\":", b"{}"] {
        fs::write(&store.path, bytes).unwrap();
        assert!(store.load().is_err());
    }
}

#[test]
fn legacy_snapshot_and_full_session_round_trip() {
    let dir = TestDir::new();
    let store = dir.store();
    let mut session = seed(&store);
    assert!(session.dc_auth_keys.is_empty());
    assert!(session.channel_recovery.is_empty());
    session.dc_auth_keys.insert(
        4,
        DcAuthKey {
            key: vec![9; 256],
            salt: 11,
        },
    );
    session.user_id = Some(42);
    session.channel_pts.insert(-1000000000001, 17);
    session.channel_recovery.push_back(ChannelRecovery {
        chat_id: -1000000000001,
        due_at: 123,
        watching: false,
    });
    session.updates = Some(UpdatesStateDto {
        pts: 4,
        qts: 5,
        seq: 6,
        date: 7,
    });
    store.save(&session).unwrap();
    let loaded = store.load().unwrap().unwrap();
    assert_eq!(loaded.snapshot.session_id, session.snapshot.session_id);
    assert_eq!(loaded.user_id, Some(42));
    assert_eq!(loaded.channel_pts, session.channel_pts);
    assert_eq!(loaded.channel_recovery, session.channel_recovery);
    assert_eq!(loaded.updates.unwrap().qts, 5);
    assert_eq!(loaded.dc_auth_keys.len(), 1);
    assert_eq!(loaded.dc_auth_keys[&4].salt, 11);
    assert_eq!(loaded.dc_auth_keys[&4].key, vec![9; 256]);
    let legacy = serde_json::json!({"snapshot": loaded.snapshot});
    let without_keys: ClientSession = serde_json::from_value(legacy).unwrap();
    assert!(without_keys.dc_auth_keys.is_empty());
    store.clear().unwrap();
    store.clear().unwrap();
}

#[test]
fn concurrent_writers_never_share_or_leave_temporary_files() {
    let dir = TestDir::new();
    let store = dir.store();
    let session = seed(&store);
    std::thread::scope(|scope| {
        for user_id in 1..=8 {
            let mut session = session.clone();
            let path = store.path.clone();
            scope.spawn(move || {
                session.user_id = Some(user_id);
                let store = FileSessionStore::new(path);
                for _ in 0..8 {
                    store.save(&session).unwrap();
                    assert!(store.load().unwrap().unwrap().user_id.is_some());
                }
            });
        }
    });
    assert_eq!(fs::read_dir(&dir.0).unwrap().count(), 1);
}

#[test]
fn invalid_snapshot_does_not_replace_valid_session() {
    let dir = TestDir::new();
    let store = dir.store();
    let mut session = seed(&store);
    let original = fs::read(&store.path).unwrap();
    session.snapshot.auth_key = Some(vec![1; 3]);
    assert!(store.save(&session).is_err());
    assert_eq!(fs::read(&store.path).unwrap(), original);
}

#[test]
fn encryption_migrates_legacy_session_without_losing_identity() {
    let dir = TestDir::new();
    let store = dir.store();
    let mut session = seed(&store);
    session.user_id = Some(42);
    session.snapshot.auth_key = Some(vec![7; 256]);
    store.save(&session).unwrap();
    let key = crate::session_crypto::register(&store.path, &[9; 32]).unwrap();
    let restored = store.load().unwrap().unwrap();
    assert_eq!(restored.user_id, Some(42));
    assert_eq!(restored.snapshot.auth_key, session.snapshot.auth_key);
    let first = fs::read(&store.path).unwrap();
    assert!(first.starts_with(crate::session_crypto::MAGIC));
    assert!(serde_json::from_slice::<serde_json::Value>(&first).is_err());
    store.save(&restored).unwrap();
    assert_ne!(first, fs::read(&store.path).unwrap());
    assert_eq!(store.load().unwrap().unwrap().user_id, Some(42));
    drop(key);
    assert!(store.load().is_err());
    let _wrong_key = crate::session_crypto::register(&store.path, &[8; 32]).unwrap();
    assert!(store.load().is_err());
}

#[test]
fn encrypted_tampering_and_truncation_fail_without_replacing_session() {
    let dir = TestDir::new();
    let store = dir.store();
    let session = seed(&store);
    let _key = crate::session_crypto::register(&store.path, &[9; 32]).unwrap();
    store.save(&session).unwrap();
    let valid = fs::read(&store.path).unwrap();
    for index in [0, crate::session_crypto::MAGIC.len(), valid.len() - 1] {
        let mut damaged = valid.clone();
        damaged[index] ^= 1;
        fs::write(&store.path, &damaged).unwrap();
        assert!(store.load().is_err());
        assert_eq!(fs::read(&store.path).unwrap(), damaged);
    }
    fs::write(&store.path, &valid[..valid.len() - 1]).unwrap();
    assert!(store.load().is_err());
}

#[test]
fn encryption_reads_legacy_magic_and_rewrites_current() {
    let dir = TestDir::new();
    let store = dir.store();
    let mut session = seed(&store);
    session.user_id = Some(42);
    let key = crate::session_crypto::register(&store.path, &[9; 32]).unwrap();
    let plaintext = serde_json::to_vec(&session).unwrap();
    let encrypted = crate::session_crypto::encrypt_legacy(&plaintext, &key).unwrap();
    assert!(encrypted.starts_with(crate::session_crypto::LEGACY_MAGIC));
    fs::write(&store.path, &encrypted).unwrap();
    let restored = store.load().unwrap().unwrap();
    assert_eq!(restored.user_id, Some(42));
    store.save(&restored).unwrap();
    assert!(
        fs::read(&store.path)
            .unwrap()
            .starts_with(crate::session_crypto::MAGIC)
    );
}
