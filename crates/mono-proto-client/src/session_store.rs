use mono_proto_session::Error as SessionError;
use mono_proto_storage::client_session::ClientSession;
use std::path::PathBuf;

pub struct FileSessionStore {
    path: PathBuf,
}

impl FileSessionStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn load(&self) -> Result<Option<ClientSession>, SessionError> {
        self.inner()
            .load_with_migration(|session| self.save(session))
    }

    pub fn save(&self, session: &ClientSession) -> Result<(), SessionError> {
        mono_proto_runtime::tcp::while_client_open(|| self.save_open(session))
            .map_err(|_| SessionError::Persistence("client closed".into()))?
    }

    pub fn save_open(&self, session: &ClientSession) -> Result<(), SessionError> {
        self.inner().save_open(session)
    }

    pub fn clear(&self) -> Result<(), SessionError> {
        self.inner().clear()
    }

    fn inner(&self) -> mono_proto_storage::session_store::FileSessionStore {
        mono_proto_storage::session_store::FileSessionStore::new(&self.path)
    }
}
