use super::*;

#[test]
fn nested_profile_and_temporary_chunk_restore_the_previous_window() {
    assert!(bound_download_window().is_none());
    let outer = bind_download_profile(DownloadProfile::Ordinary);
    let original = bound_download_window().unwrap();
    assert_eq!(forced_chunk(), original.chunk);
    {
        let _inner = bind_download_profile(DownloadProfile::User);
        let inner = bound_download_window().unwrap();
        assert_eq!(forced_chunk(), inner.chunk);
        {
            let _restore = preserve_download_window();
            set_forced_chunk(17);
        }
        assert_eq!(forced_chunk(), inner.chunk);
    }
    assert_eq!(bound_download_window(), Some(original));
    assert_eq!(forced_chunk(), original.chunk);
    drop(outer);
    assert!(bound_download_window().is_none());
    assert_eq!(forced_chunk(), 0);
}
