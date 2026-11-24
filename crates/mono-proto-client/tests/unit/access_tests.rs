use super::*;
use mono_proto_types::peers::{CachedPeer, PeerKind};

fn client(name: &str) -> (u64, Arc<Client>) {
    let path = std::env::temp_dir().join(format!("mono-read-epoch-{name}-{}", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let handle = crate::construct::create_client(1, "test".into(), path.to_string_lossy().into());
    let client = get_client(handle).unwrap();
    let mut home = client.main.lock().snapshot.clone();
    home.auth_key = Some(vec![1; 256]);
    client.main.lock().snapshot = home.clone();
    {
        let mut d = client.data.lock();
        d.user_id = Some(42);
        d.home_auth_key = home.auth_key.clone();
        d.home_snapshot = home.clone();
    }
    for lane in &client.rpc {
        let mut io = lane.io.lock();
        io.snapshot.auth_key = Some(vec![2; 256]);
        io.temp_expires_at = crate::bind_temp::unix_now() + 3600;
        io.perm_key = home.auth_key.clone();
    }
    client
        .policy
        .main_session_allowance
        .store(3, Ordering::Relaxed);
    client
        .policy
        .bound_extra_sessions
        .store(2, Ordering::Relaxed);
    (handle, client)
}

fn finish(handle: u64, client: Arc<Client>) {
    crate::persist::flush_persist(&client);
    let path = client.data.lock().session_path.clone();
    crate::registry::destroy_client(handle);
    drop(client);
    let _ = std::fs::remove_file(path);
}

fn peer(hash: i64) -> CachedPeer {
    CachedPeer {
        kind: PeerKind::User,
        id: 7,
        access_hash: hash,
        min_hash: false,
    }
}

#[test]
fn shared_avatar_lookup_repairs_a_copy_from_a_matching_full_peer() {
    let (handle, client) = client("avatar-shared-repair");
    {
        let mut data = client.data.lock();
        data.peers.insert(7, peer(91));
        data.media.insert(
            (7, 0),
            mono_proto_media::media::media_ref_peer_photo(
                PeerKind::User,
                7,
                0,
                8,
                2,
                "avatar:7".into(),
                true,
                None,
            ),
        );
        let original = data.media.clone();
        for _ in 0..2 {
            let (_, _, media) = crate::media_download::indexed_media_for_download(
                &data,
                7,
                0,
                mono_proto_media::media::MediaDownloadKind::Display,
            )
            .expect("cached peer avoids main fallback")
            .unwrap();
            assert!(matches!(
                media.location,
                mono_proto_media::media::MediaLocation::PeerPhoto {
                    access_hash: 91,
                    ..
                }
            ));
        }
        assert_eq!(data.media, original);
        assert_eq!(data.persist_epoch, 0);
        assert!(
            crate::media_download::indexed_media_for_download(
                &data,
                7,
                0,
                mono_proto_media::media::MediaDownloadKind::Full,
            )
            .is_none(),
            "full avatar upgrade still uses repair path"
        );
        data.session_dead = true;
        assert!(
            crate::media_download::indexed_media_for_download(
                &data,
                7,
                0,
                mono_proto_media::media::MediaDownloadKind::Display,
            )
            .is_none()
        );
    }
    finish(handle, client);
}

#[test]
fn shared_avatar_repair_rejects_unusable_or_mismatched_peers() {
    let (handle, client) = client("avatar-shared-invalid");
    {
        let mut data = client.data.lock();
        data.media.insert(
            (7, 0),
            mono_proto_media::media::media_ref_peer_photo(
                PeerKind::User,
                7,
                0,
                8,
                2,
                "avatar:7".into(),
                false,
                None,
            ),
        );
        for cached in [
            None,
            Some(peer(0)),
            Some(CachedPeer {
                min_hash: true,
                ..peer(91)
            }),
            Some(CachedPeer { id: 8, ..peer(91) }),
            Some(CachedPeer {
                kind: PeerKind::Channel,
                ..peer(91)
            }),
        ] {
            data.peers.clear();
            if let Some(cached) = cached {
                data.peers.insert(7, cached);
            }
            assert!(
                crate::media_download::indexed_media_for_download(
                    &data,
                    7,
                    0,
                    mono_proto_media::media::MediaDownloadKind::Display,
                )
                .is_none()
            );
        }
        data.peers.insert(7, peer(91));
        data.media.insert(
            (7, 0),
            mono_proto_media::media::media_ref_peer_photo(
                PeerKind::User,
                7,
                55,
                8,
                2,
                "avatar:7".into(),
                false,
                None,
            ),
        );
        let (_, _, media) = crate::media_download::indexed_media_for_download(
            &data,
            7,
            0,
            mono_proto_media::media::MediaDownloadKind::Display,
        )
        .unwrap()
        .unwrap();
        assert!(matches!(
            media.location,
            mono_proto_media::media::MediaLocation::PeerPhoto {
                access_hash: 55,
                ..
            }
        ));
        data.user_id = None;
        assert!(
            crate::media_download::indexed_media_for_download(
                &data,
                7,
                0,
                mono_proto_media::media::MediaDownloadKind::Display,
            )
            .is_none()
        );
    }
    finish(handle, client);
}

#[test]
fn clean_reads_and_lane_identity_do_not_advance_epoch() {
    let (handle, client) = client("clean");
    with_read_lane(handle, |state| {
        assert!(EXTRA_READ_LANE.with(Cell::get));
        state.snapshot.session_id += 1;
        state.snapshot.server_salt += 1; // Temporary salt is not durable.
        Ok(())
    })
    .unwrap();
    assert_eq!(client.data.lock().persist_epoch, 0);
    assert!(!client.persist_running.load(Ordering::Acquire));
    assert!(!client.persist_queued.load(Ordering::Acquire));
    finish(handle, client);
}

#[test]
fn read_merge_keeps_concurrent_changes_and_applies_owned_deletions() {
    let (handle, client) = client("merge");
    {
        let mut d = client.data.lock();
        d.peers.insert(7, peer(1));
        d.peers.insert(8, peer(1));
        d.peers.insert(9, peer(1));
    }
    with_read_lane(handle, |state| {
        client.data.lock().peers.insert(7, peer(3));
        state.peers.insert(7, peer(2));
        state.peers.remove(&8);
        client.data.lock().peers.remove(&9);
        Ok(())
    })
    .unwrap();
    {
        let d = client.data.lock();
        assert_eq!(d.peers.get(&7), Some(&peer(3)));
        assert!(!d.peers.contains_key(&8));
        assert!(!d.peers.contains_key(&9));
        assert_eq!(d.persist_epoch, 1);
    }
    finish(handle, client);
}

#[test]
fn late_dialog_observations_do_not_overwrite_live_channel_pts() {
    use mono_proto_messages::dialogs::merge_channel_pts;
    use mono_proto_types::peers::chat_id_for_channel;

    let (handle, client) = client("channel-pts-race");
    let known = chat_id_for_channel(7);
    let concurrent = chat_id_for_channel(8);
    let unknown = chat_id_for_channel(9);
    client.data.lock().channel_pts.insert(known, 100);
    with_read_lane(handle, |stale| {
        assert!(EXTRA_READ_LANE.with(Cell::get));
        assert_eq!(stale.channel_pts.get(&known), Some(&100));
        assert!(!stale.channel_pts.contains_key(&concurrent));
        {
            let mut data = client.data.lock();
            data.channel_pts.insert(known, 103);
            data.channel_pts.insert(concurrent, 104);
        }
        for chat_id in [known, concurrent, unknown] {
            merge_channel_pts(&mut stale.channel_pts, chat_id, Some(105));
        }
        Ok(())
    })
    .unwrap();
    {
        let data = client.data.lock();
        assert_eq!(data.channel_pts.get(&known), Some(&103));
        assert_eq!(data.channel_pts.get(&concurrent), Some(&104));
        assert_eq!(data.channel_pts.get(&unknown), Some(&105));
    }
    finish(handle, client);
}

#[test]
fn actual_cursor_result_and_outstanding_dirty_work_control_saves() {
    use mono_proto_types::updates_state::UpdatesStateDto;
    let (handle, client) = client("outstanding");
    let newer = UpdatesStateDto {
        pts: 10,
        qts: 10,
        date: 10,
        seq: 10,
    };
    client.data.lock().updates = Some(newer.clone());
    with_read_lane(handle, |state| {
        state.updates = Some(UpdatesStateDto {
            pts: 1,
            qts: 1,
            date: 1,
            seq: 1,
        });
        Ok(())
    })
    .unwrap();
    assert_eq!(client.data.lock().persist_epoch, 0);
    assert_eq!(client.data.lock().updates, Some(newer));
    client.data.lock().persist_epoch = 1;
    with_read_lane(handle, |_| Ok(())).unwrap();
    crate::persist::flush_persist(&client);
    assert_eq!(client.data.lock().persist_epoch, 1);
    assert_eq!(client.data.lock().persisted_epoch, 1);
    finish(handle, client);
}

#[test]
fn failed_read_clock_and_tokens_remain_dirty_and_are_persisted() {
    let (handle, client) = client("error");
    let result: Result<(), _> = with_read_lane(handle, |state| {
        state.logout_tokens.push(vec![4]);
        state.snapshot.time_offset_micros = 1234;
        Err(MtprotoError::Message("read failed".into()))
    });
    assert!(result.is_err());
    assert_eq!(client.data.lock().persist_epoch, 1);
    crate::persist::flush_persist(&client);
    let path = client.data.lock().session_path.clone();
    let saved = crate::session_store::FileSessionStore::new(&path)
        .load()
        .unwrap()
        .unwrap();
    assert_eq!(saved.logout_tokens, vec![vec![4]]);
    assert_eq!(saved.snapshot.time_offset_micros, 1234);
    assert_eq!(saved.snapshot.auth_key, Some(vec![1; 256]));
    finish(handle, client);
}

#[test]
fn stale_read_lane_cannot_revert_a_newer_clock_correction() {
    for conflicting_correction in [false, true] {
        let (handle, client) = client(&format!("clock-race-{conflicting_correction}"));
        with_read_lane(handle, |stale| {
            assert_eq!(stale.snapshot.time_offset_micros, 0);
            // The outer request holds one lane while another learns a correction.
            with_read_lane(handle, |corrected| {
                assert_eq!(corrected.snapshot.time_offset_micros, 0);
                corrected.snapshot.time_offset_micros = 1234;
                Ok(())
            })?;
            if conflicting_correction {
                stale.snapshot.time_offset_micros = 5678;
            }
            Ok(())
        })
        .unwrap();
        {
            let data = client.data.lock();
            assert_eq!(data.home_time_offset, 1234);
            assert_eq!(data.home_snapshot.time_offset_micros, 1234);
            assert_eq!(data.persist_epoch, 1);
        }
        crate::persist::flush_persist(&client);
        // Reusing the stale bound lane without a new correction is also clean.
        with_read_lane(handle, |_| Ok(())).unwrap();
        crate::persist::flush_persist(&client);
        let path = client.data.lock().session_path.clone();
        let saved = crate::session_store::FileSessionStore::new(&path)
            .load()
            .unwrap()
            .unwrap();
        assert_eq!(saved.snapshot.time_offset_micros, 1234);
        assert_eq!(client.data.lock().persist_epoch, 1);
        assert_eq!(client.data.lock().persisted_epoch, 1);
        finish(handle, client);
    }
}

#[test]
fn logout_during_read_rejects_stale_merge() {
    let (handle, client) = client("logout");
    with_read_lane(handle, |state| {
        state.logout_tokens.push(vec![4]);
        let mut d = client.data.lock();
        d.home_session_id += 1;
        d.user_id = None;
        Ok(())
    })
    .unwrap();
    assert!(client.data.lock().logout_tokens.is_empty());
    assert_eq!(client.data.lock().persist_epoch, 0);
    finish(handle, client);
}

#[test]
fn each_durable_read_delta_advances_epoch() {
    use mono_proto_storage::client_session::{ChannelRecovery, DcAuthKey, NewSessionMetadata};
    for field in 0..9 {
        let (handle, client) = client(&format!("field-{field}"));
        with_read_lane(handle, |state| {
            match field {
                0 => {
                    state.channel_pts.insert(-7, 4);
                }
                1 => {
                    state.channel_recovery.push_back(ChannelRecovery {
                        chat_id: -7,
                        due_at: 1,
                        watching: true,
                    });
                }
                2 => {
                    state.seen_messages.insert((7, 2));
                }
                3 => {
                    state.logout_tokens.push(vec![5]);
                }
                4 => {
                    state.new_session = Some(NewSessionMetadata {
                        first_msg_id: 1,
                        unique_id: 2,
                        server_salt: 3,
                    });
                }
                5 => {
                    state.dc_auth_keys.insert(
                        2,
                        DcAuthKey {
                            key: vec![3; 256],
                            salt: 8,
                        },
                    );
                }
                6 => {
                    state.snapshot.time_offset_micros = 18;
                }
                7 => {
                    state.peers.insert(7, peer(8));
                }
                _ => {
                    state.session_dead = true;
                    state.session_dead_reason = Some("revoked".into());
                }
            }
            Ok(())
        })
        .unwrap();
        assert_eq!(client.data.lock().persist_epoch, 1, "field {field}");
        finish(handle, client);
    }
}

#[test]
fn unbound_read_lane_falls_back_to_main() {
    let (handle, client) = client("fallback");
    for lane in &client.rpc {
        let mut io = lane.io.lock();
        io.temp_expires_at = 0;
        io.perm_key = None;
    }
    with_read_lane(handle, |_| {
        assert!(!EXTRA_READ_LANE.with(Cell::get));
        Ok(())
    })
    .unwrap();
    finish(handle, client);
}
