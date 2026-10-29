use crate::client_session::ClientSession;
use crate::collections::{HashMap, HashMapExt, HashSet, HashSetExt};
use mono_proto_session::{Error as SessionError, Snapshot};
use std::fs;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;

pub struct FileSessionStore {
    path: PathBuf,
}

impl FileSessionStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn load(&self) -> Result<Option<ClientSession>, SessionError> {
        self.load_with_migration(|session| self.save(session))
    }

    pub fn load_with_migration(
        &self,
        save: impl Fn(&ClientSession) -> Result<(), SessionError>,
    ) -> Result<Option<ClientSession>, SessionError> {
        let bytes = match fs::read(&self.path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(SessionError::Persistence(error.to_string())),
        };
        if bytes.is_empty() {
            return Err(SessionError::Persistence("empty session file".into()));
        }
        let key = crate::session_crypto::key_for(&self.path);
        let encrypted = crate::session_crypto::is_encrypted(&bytes);
        let bytes = if encrypted {
            crate::session_crypto::decrypt(
                &bytes,
                key.as_deref()
                    .ok_or_else(|| SessionError::Persistence("session key unavailable".into()))?,
            )?
        } else {
            zeroize::Zeroizing::new(bytes)
        };
        if let Ok(session) = serde_json::from_slice::<ClientSession>(&bytes) {
            session.snapshot.validate()?;
            if !encrypted && key.is_some() {
                save(&session)?;
            }
            return Ok(Some(session));
        }
        let snapshot: Snapshot =
            serde_json::from_slice(&bytes).map_err(|e| SessionError::Persistence(e.to_string()))?;
        snapshot.validate()?;
        let session = ClientSession {
            snapshot,
            user_id: None,
            peers: HashMap::new(),
            updates: None,
            media: HashMap::new(),
            media_order: Vec::new(),
            channel_pts: HashMap::new(),
            channel_recovery: Default::default(),
            seen_messages: HashSet::new(),
            session_dead: false,
            logout_tokens: Vec::new(),
            new_session: None,
            test_dc: false,
            dc_auth_keys: HashMap::new(),
            journal: Default::default(),
        };
        if !encrypted && key.is_some() {
            save(&session)?;
        }
        Ok(Some(session))
    }

    pub fn save(&self, session: &ClientSession) -> Result<(), SessionError> {
        self.save_open(session)
    }

    pub fn save_open(&self, session: &ClientSession) -> Result<(), SessionError> {
        session.snapshot.validate()?;
        if let Some(parent) = self.path.parent().filter(|p| !p.as_os_str().is_empty()) {
            fs::create_dir_all(parent).map_err(|e| SessionError::Persistence(e.to_string()))?;
        }
        let plaintext = {
            let _span = crate::perf::span("session_save.encode");
            zeroize::Zeroizing::new(
                serde_json::to_vec(session)
                    .map_err(|_| SessionError::Persistence("session encoding failed".into()))?,
            )
        };
        let bytes = match crate::session_crypto::key_for(&self.path) {
            Some(key) => {
                let _span = crate::perf::span("session_save.encrypt");
                zeroize::Zeroizing::new(crate::session_crypto::encrypt(&plaintext, &key)?)
            }
            None => plaintext,
        };
        // Each writer owns its temporary file. Never truncate another writer's
        // snapshot, and flush its contents before the atomic replacement.
        let mut nonce = [0_u8; 16];
        mono_proto_crypto::fill_random(&mut nonce)
            .map_err(|e| SessionError::Persistence(e.to_string()))?;
        let suffix = u128::from_le_bytes(nonce);
        let tmp = self.path.with_extension(format!("{suffix:032x}.tmp"));
        let mut options = fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options
            .open(&tmp)
            .map_err(|e| SessionError::Persistence(e.to_string()))?;
        let result = (|| {
            {
                let _span = crate::perf::span("session_save.write");
                file.write_all(&bytes)?;
            }
            {
                let _span = crate::perf::span("session_save.fsync");
                file.sync_all()?;
            }
            drop(file);
            fs::rename(&tmp, &self.path)?;
            #[cfg(unix)]
            {
                let parent = self
                    .path
                    .parent()
                    .filter(|p| !p.as_os_str().is_empty())
                    .unwrap_or_else(|| std::path::Path::new("."));
                let _span = crate::perf::span("session_save.directory_sync");
                fs::File::open(parent)?.sync_all()?;
            }
            Ok::<_, std::io::Error>(())
        })();
        if result.is_err() {
            let _ = fs::remove_file(&tmp);
        }
        result.map_err(|e| SessionError::Persistence(e.to_string()))
    }

    pub fn clear(&self) -> Result<(), SessionError> {
        match fs::remove_file(&self.path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
            Err(error) => Err(SessionError::Persistence(error.to_string())),
        }
    }
}

#[cfg(test)]
#[path = "../tests/unit/session_store_tests.rs"]
mod tests;
