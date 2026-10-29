use super::*;

#[test]
fn stale_staging_preserves_existing_destination_and_removes_partial_bytes() {
    let dest = std::env::temp_dir().join(format!("mono-proto-staging-{}", std::process::id()));
    fs::write(&dest, b"previous").unwrap();
    let staged = StagedDownload::new(&dest).unwrap();
    let path = staged.path().to_owned();
    fs::write(&path, b"uncommitted").unwrap();
    drop(staged);
    assert!(!path.exists());
    assert_eq!(fs::read(&dest).unwrap(), b"previous");
    fs::remove_file(dest).unwrap();
}

#[test]
fn cancel_before_publication_cleans_staging_without_publishing() {
    let dest =
        std::env::temp_dir().join(format!("mono-proto-publish-cancel-{}", std::process::id()));
    let staged = StagedDownload::new(&dest).unwrap();
    let path = staged.path().to_owned();
    fs::write(&path, b"complete").unwrap();
    fs::write(cancel_marker(&dest), []).unwrap();
    let ranges = ranges_path(&path);
    fs::write(&ranges, b"ranges").unwrap();
    assert!(staged.publish(&dest).is_err());
    assert!(path.exists());
    assert!(ranges.exists());
    assert!(!dest.exists());
    let _ = fs::remove_file(&path);
    let _ = fs::remove_file(&ranges);
    fs::remove_file(cancel_marker(&dest)).unwrap();
}

#[test]
fn successful_staging_publishes_complete_file() {
    let dest = std::env::temp_dir().join(format!("mono-proto-publish-{}", std::process::id()));
    let staged = StagedDownload::new(&dest).unwrap();
    let path = staged.path().to_owned();
    fs::write(&path, b"complete").unwrap();
    let ranges = ranges_path(&path);
    fs::write(&ranges, b"stale").unwrap();
    staged.publish(&dest).unwrap();
    assert!(!path.exists());
    assert!(!ranges.exists());
    assert_eq!(fs::read(&dest).unwrap(), b"complete");
    fs::remove_file(dest).unwrap();
}

#[test]
fn resume_reopens_the_longest_part_and_cancel_deletes_it() {
    let dest = std::env::temp_dir().join(format!("mono-proto-resume-{}", std::process::id()));
    let short = StagedDownload::new(&dest).unwrap();
    fs::write(short.path(), vec![1; 32]).unwrap();
    let short_path = short.path().to_owned();
    short.keep();
    let long = StagedDownload::new(&dest).unwrap();
    fs::write(long.path(), vec![2; 128 * 1024 + 10]).unwrap();
    let long_path = long.path().to_owned();
    long.keep();
    let resumed = StagedDownload::resume_or_create(&dest).unwrap();
    assert_eq!(resumed.path(), long_path);
    assert!(!short_path.exists());
    assert_eq!(aligned_prefix(128 * 1024 + 10, CHUNK), 128 * 1024);
    assert_eq!(aligned_prefix(10, CHUNK), 0);
    resumed.keep();
    fs::write(cancel_marker(&dest), []).unwrap();
    assert!(StagedDownload::resume_or_create(&dest).is_err());
    assert!(long_path.exists(), "cancel keeps the partial like Android");
    let _ = fs::remove_file(&long_path);
    let _ = fs::remove_file(cancel_marker(&dest));
    let _ = fs::remove_file(&dest);
}

#[test]
fn android_range_file_round_trips_and_rejects_a_negative_count() {
    let ranges = MissingRanges::span(128 * 1024, 512 * 1024);
    let bytes = ranges.to_bytes().unwrap();
    assert_eq!(i32::from_be_bytes(bytes[0..4].try_into().unwrap()), 1);
    assert_eq!(
        i64::from_be_bytes(bytes[4..12].try_into().unwrap()),
        128 * 1024
    );
    assert_eq!(
        i64::from_be_bytes(bytes[12..20].try_into().unwrap()),
        512 * 1024
    );
    assert_eq!(MissingRanges::load(&bytes), Some(ranges));
    assert!(MissingRanges::load(&(-1i32).to_be_bytes()).is_none());
    let mut huge = vec![0x7f, 0xff, 0xff, 0xff];
    huge.extend_from_slice(&[0; 4]);
    assert!(MissingRanges::load(&huge).is_none());
    assert_eq!(
        resume_start(MissingRanges::load(&bytes).as_ref(), 0, CHUNK),
        128 * 1024
    );
    assert_eq!(resume_start(None, 128 * 1024 + 10, CHUNK), 128 * 1024);
    let mut hole = MissingRanges::span(100, 200);
    assert!(preserved_end(1_000, 200, Some(1_000), Some(&hole)).is_err());
    hole.cover(100, 200);
    assert_eq!(
        preserved_end(1_000, 200, Some(1_000), Some(&hole)),
        Ok(1_000)
    );
    let leading = MissingRanges::span(0, CHUNK as i64);
    assert!(preserved_end(1_048_576, 0, Some(1_048_576), Some(&leading)).is_err());
    let mut unknown = MissingRanges::span(100, 200);
    unknown.cover(400, i64::MAX);
    assert!(preserved_end(1_000, 180, None, Some(&unknown)).is_err());
    unknown.cover(100, 200);
    unknown.clip_to_end(180);
    assert_eq!(preserved_end(1_000, 180, None, Some(&unknown)), Ok(1_000));
    assert!(keep_partial(&crate::MtprotoError::Message(
        "download incomplete: received 200 of 1000 bytes".into()
    )));
    assert!(keep_partial(&crate::MtprotoError::Message(
        "401: AUTH_KEY_UNREGISTERED".into()
    )));
    assert!(keep_partial(&crate::MtprotoError::Message(
        "401: AUTH_KEY_INVALID".into()
    )));
    assert!(keep_partial(&crate::MtprotoError::Message(
        "406: AUTH_KEY_DUPLICATED".into()
    )));
    let staging = std::path::PathBuf::from("video.mp4.100.2.part");
    let sidecar = ranges_path(&staging);
    assert!(sidecar.ends_with("video.mp4.100.2.ranges.part"));
    assert!(!is_native_staging_name(
        "video.mp4.100.2.ranges.part",
        "video.mp4"
    ));
}

#[test]
fn retryable_failure_keeps_the_part_and_cancel_does_not() {
    let dest = std::env::temp_dir().join(format!("mono-proto-keep-{}", std::process::id()));
    let staged = StagedDownload::new(&dest).unwrap();
    let path = staged.path().to_owned();
    fs::write(&path, b"partial").unwrap();
    assert!(keep_partial(&crate::MtprotoError::Message(
        "FLOOD_WAIT_3".into()
    )));
    assert!(keep_partial(&crate::MtprotoError::Message(
        "FILE_REFERENCE_EXPIRED".into()
    )));
    assert!(!keep_partial(&crate::MtprotoError::Message(
        "cancelled".into()
    )));
    staged.keep();
    assert!(path.exists());
    let _ = fs::remove_file(&path);
    let _ = fs::remove_file(&dest);
}

#[test]
fn one_destination_has_one_writer() {
    let dest = std::env::temp_dir().join(format!("mono-proto-lock-{}", std::process::id()));
    let first = DestinationLock::acquire(&dest);
    let dest_for_thread = dest.clone();
    let (started, started_rx) = std::sync::mpsc::channel();
    let thread = std::thread::spawn(move || {
        started.send(()).unwrap();
        let _second = DestinationLock::acquire(&dest_for_thread);
        started.send(()).unwrap();
    });
    started_rx
        .recv_timeout(std::time::Duration::from_secs(1))
        .unwrap();
    assert!(
        started_rx
            .recv_timeout(std::time::Duration::from_millis(50))
            .is_err()
    );
    drop(first);
    started_rx
        .recv_timeout(std::time::Duration::from_secs(1))
        .unwrap();
    thread.join().unwrap();
}
