use super::*;
use crate::registry::{client_exists, destroy_client, get_client};

#[test]
fn encrypted_client_restores_session_and_rejects_wrong_key() {
    let dir = std::env::temp_dir().join(format!("mono-encrypted-lifecycle-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("session.json");
    let path_string = path.to_string_lossy().into_owned();
    assert!(create_encrypted_client(1, "test".into(), path_string.clone(), vec![1; 31]).is_err());
    let handle =
        create_encrypted_client(1, "test".into(), path_string.clone(), vec![1; 32]).unwrap();
    let client = get_client(handle).unwrap();
    {
        let mut data = client.data.lock();
        data.user_id = Some(42);
        data.persist_epoch += 1;
    }
    drop(client);
    destroy_client(handle);
    assert!(create_encrypted_client(1, "test".into(), path_string.clone(), vec![2; 32]).is_err());
    let restored = create_encrypted_client(1, "test".into(), path_string, vec![1; 32]).unwrap();
    assert_eq!(get_client(restored).unwrap().data.lock().user_id, Some(42));
    destroy_client(restored);
    std::fs::remove_file(path).unwrap();
    std::fs::remove_dir(dir).unwrap();
}

#[test]
fn client_lifecycle_persists_and_restores_session_identity() {
    let dir = std::env::temp_dir().join(format!("mono-client-lifecycle-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("session.json");
    let path_string = path.to_string_lossy().into_owned();
    let handle = create_client(1, "test".into(), path_string.clone());
    let client = get_client(handle).unwrap();
    let identity = client.data.lock().home_session_id;
    {
        let mut data = client.data.lock();
        data.user_id = Some(42);
        data.persist_epoch += 1;
    }
    drop(client);
    assert!(client_exists(handle));
    destroy_client(handle);
    assert!(!client_exists(handle));
    let restored = create_client(1, "test".into(), path_string);
    let restored_client = get_client(restored).unwrap();
    assert_eq!(restored_client.data.lock().home_session_id, identity);
    assert_eq!(restored_client.data.lock().user_id, Some(42));
    drop(restored_client);
    destroy_client(restored);
    std::fs::remove_file(path).unwrap();
    std::fs::remove_dir(dir).unwrap();
}
