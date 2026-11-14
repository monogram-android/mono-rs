use crate::collections::{HashMap, IndexMap};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use mono_proto::latest::api::{
    InputDocumentFileLocationConstructor, InputFileLocation, InputPeer,
    InputPeerChannelConstructor, InputPeerChatConstructor, InputPeerPhotoFileLocationConstructor,
    InputPeerUserConstructor, InputPhotoFileLocationConstructor, True, TrueConstructor,
};

use crate::MtprotoError;
use crate::media_thumbs::is_getfile_thumb_size;

pub use crate::media_types::{MediaLocation, MediaRef, MediaRefreshSource};

// Zero and -1 are reserved for avatars and Instant View; queue media uses the remaining negatives.
pub fn scheduled_media_id(id: i32) -> Result<i32, MtprotoError> {
    if id <= 0 {
        return Err(MtprotoError::Message("invalid scheduled message id".into()));
    }
    Ok(-1 - id)
}

pub fn scheduled_message_id(media_id: i32) -> Option<i32> {
    (media_id < -1).then(|| -1 - media_id)
}

/// Snapshot clones share payloads until mutable access detaches the map.
/// Owned `IndexMap` callers can convert with `From`/`Into` in either direction.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MediaIndex(Arc<IndexMap<(i64, i32), MediaRef>>);

impl MediaIndex {
    pub fn shares_storage_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Deref for MediaIndex {
    type Target = IndexMap<(i64, i32), MediaRef>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MediaIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::make_mut(&mut self.0)
    }
}

impl IntoIterator for MediaIndex {
    type Item = ((i64, i32), MediaRef);
    type IntoIter = <IndexMap<(i64, i32), MediaRef> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Arc::unwrap_or_clone(self.0).into_iter()
    }
}

impl<'a> IntoIterator for &'a MediaIndex {
    type Item = (&'a (i64, i32), &'a MediaRef);
    type IntoIter = <&'a IndexMap<(i64, i32), MediaRef> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut MediaIndex {
    type Item = (&'a (i64, i32), &'a mut MediaRef);
    type IntoIter = <&'a mut IndexMap<(i64, i32), MediaRef> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Arc::make_mut(&mut self.0).iter_mut()
    }
}

impl From<IndexMap<(i64, i32), MediaRef>> for MediaIndex {
    fn from(index: IndexMap<(i64, i32), MediaRef>) -> Self {
        Self(Arc::new(index))
    }
}

impl From<MediaIndex> for IndexMap<(i64, i32), MediaRef> {
    fn from(index: MediaIndex) -> Self {
        Arc::unwrap_or_clone(index.0)
    }
}

impl FromIterator<((i64, i32), MediaRef)> for MediaIndex {
    fn from_iter<T: IntoIterator<Item = ((i64, i32), MediaRef)>>(iter: T) -> Self {
        Self(Arc::new(iter.into_iter().collect()))
    }
}

/// Avatar `InputPeerPhotoFileLocation` cannot use hash 0 / min.
/// https://core.telegram.org/api/min
pub fn fill_zero_peer_photo_hashes(
    media: &mut MediaIndex,
    peers: &HashMap<i64, crate::peers::CachedPeer>,
) {
    for ((chat_id, _), item) in media.iter_mut() {
        fill_peer_photo_hash(item, peers, *chat_id);
    }
}

/// Repair only the requested entry, without walking the whole media index.
pub fn fill_zero_peer_photo_hash(
    media: &mut MediaIndex,
    peers: &HashMap<i64, crate::peers::CachedPeer>,
    chat_id: i64,
    message_id: i32,
) {
    if let Some(item) = media.get_mut(&(chat_id, message_id)) {
        fill_peer_photo_hash(item, peers, chat_id);
    }
}

pub fn fill_peer_photo_hash(
    item: &mut MediaRef,
    peers: &HashMap<i64, crate::peers::CachedPeer>,
    chat_id: i64,
) {
    if let Some(peer) = crate::peers::usable_cached_peer(peers, chat_id) {
        if let MediaLocation::PeerPhoto { peer_kind, peer_id, access_hash, .. } = &mut item.location
            && *peer_kind == peer.kind && *peer_id == peer.id && *access_hash == 0 {
                *access_hash = peer.access_hash;
            }
        if let Some(MediaLocation::PeerPhoto { peer_kind, peer_id, access_hash, .. }) = &mut item.display_location
            && *peer_kind == peer.kind && *peer_id == peer.id && *access_hash == 0 {
                *access_hash = peer.access_hash;
            }
    }
}

pub fn inline_payload_missing(media: &MediaRef) -> bool {
    matches!(media.location, MediaLocation::Inline { ref bytes, .. } if bytes.is_empty())
}

pub fn peer_photo_needs_hash(media: &MediaRef) -> bool {
    matches!(
        media.location,
        MediaLocation::PeerPhoto { access_hash: 0, peer_kind, .. }
            if peer_kind != crate::peers::PeerKind::Chat
    )
}

pub fn input_location(loc: &MediaLocation) -> Result<InputFileLocation, MtprotoError> {
    match loc {
        MediaLocation::Photo {
            id,
            access_hash,
            file_reference,
            thumb_size,
            ..
        } => Ok(InputFileLocation::InputPhotoFileLocation(
            InputPhotoFileLocationConstructor {
                id: *id,
                access_hash: *access_hash,
                file_reference: file_reference.clone(),
                thumb_size: thumb_size.clone(),
            },
        )),
        MediaLocation::Document {
            id,
            access_hash,
            file_reference,
            thumb_size,
            ..
        } => Ok(InputFileLocation::InputDocumentFileLocation(
            InputDocumentFileLocationConstructor {
                id: *id,
                access_hash: *access_hash,
                file_reference: file_reference.clone(),
                thumb_size: thumb_size.clone(),
            },
        )),
        MediaLocation::PeerPhoto {
            peer_kind,
            peer_id,
            access_hash,
            photo_id,
            big,
            ..
        } => {
            let peer = match peer_kind {
                crate::peers::PeerKind::User => {
                    InputPeer::InputPeerUser(InputPeerUserConstructor {
                        user_id: *peer_id,
                        access_hash: *access_hash,
                    })
                }
                crate::peers::PeerKind::Chat => {
                    InputPeer::InputPeerChat(InputPeerChatConstructor { chat_id: *peer_id })
                }
                crate::peers::PeerKind::Channel => {
                    InputPeer::InputPeerChannel(InputPeerChannelConstructor {
                        channel_id: *peer_id,
                        access_hash: *access_hash,
                    })
                }
            };
            let big = if *big {
                Some(Box::new(True::True(TrueConstructor {})))
            } else {
                None
            };
            let flags = if big.is_some() { 1 } else { 0 };
            Ok(InputFileLocation::InputPeerPhotoFileLocation(
                InputPeerPhotoFileLocationConstructor {
                    flags,
                    big,
                    peer: Box::new(peer),
                    photo_id: *photo_id,
                },
            ))
        }
        MediaLocation::Inline { .. } => Err(MtprotoError::Message(
            "inline media does not need upload.getFile".into(),
        )),
    }
}

pub fn location_is_inline(loc: &MediaLocation) -> bool {
    matches!(loc, MediaLocation::Inline { .. })
}

pub fn media_dc(loc: &MediaLocation) -> Option<i32> {
    match loc {
        MediaLocation::Photo { dc_id, .. }
        | MediaLocation::Document { dc_id, .. }
        | MediaLocation::PeerPhoto { dc_id, .. } => Some(*dc_id),
        MediaLocation::Inline { .. } => None,
    }
}

/// DC that should own `upload.getFile` for this location. 0 / missing → home DC.
pub fn download_dc(home_dc: i32, loc: &MediaLocation) -> i32 {
    media_dc(loc).filter(|dc| *dc != 0).unwrap_or(home_dc)
}

pub fn is_file_reference_error(err: &MtprotoError) -> bool {
    matches!(err, MtprotoError::Message(message) if {
        let upper = message.to_ascii_uppercase();
        upper.contains("FILE_REFERENCE")
    })
}

/// Compare locations after a file-ref refresh. Peer photos have no `file_reference`.
/// https://core.telegram.org/api/file-references
pub fn location_token(loc: &MediaLocation) -> Vec<u8> {
    match loc {
        MediaLocation::Photo {
            id,
            file_reference,
            thumb_size,
            ..
        } => token_parts(*id, file_reference, thumb_size),
        MediaLocation::Document {
            id,
            file_reference,
            thumb_size,
            ..
        } => token_parts(*id, file_reference, thumb_size),
        MediaLocation::PeerPhoto {
            photo_id,
            access_hash,
            ..
        } => {
            let mut out = photo_id.to_le_bytes().to_vec();
            out.extend_from_slice(&access_hash.to_le_bytes());
            out
        }
        MediaLocation::Inline { bytes, .. } => bytes.clone(),
    }
}

thread_local! {
    static TOKEN_SCRATCH: std::cell::RefCell<Vec<u8>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

pub fn token_parts(id: i64, file_reference: &[u8], thumb_size: impl AsRef<str>) -> Vec<u8> {
    TOKEN_SCRATCH.with(|cell| {
        let mut out = cell.borrow_mut();
        out.clear();
        out.extend_from_slice(&id.to_le_bytes());
        out.extend_from_slice(file_reference);
        out.extend_from_slice(thumb_size.as_ref().as_bytes());
        out.clone()
    })
}

pub fn location_is_getfile_thumb(loc: &MediaLocation) -> bool {
    match loc {
        MediaLocation::Inline { bytes, .. } => !bytes.is_empty(),
        MediaLocation::Photo { thumb_size, .. } | MediaLocation::Document { thumb_size, .. } => {
            is_getfile_thumb_size(thumb_size)
        }
        MediaLocation::PeerPhoto { big, .. } => !*big,
    }
}

pub fn is_file_id_invalid(err: &MtprotoError) -> bool {
    matches!(err, MtprotoError::Message(message) if {
        message.to_ascii_uppercase().contains("FILE_ID_INVALID")
    })
}

/// Other getFile letters to try after `FILE_ID_INVALID` on the picked thumb.

pub fn location_thumb_size(loc: &MediaLocation) -> Option<&str> {
    match loc {
        MediaLocation::Photo { thumb_size, .. } | MediaLocation::Document { thumb_size, .. } => {
            Some(thumb_size.as_str())
        }
        _ => None,
    }
}

pub fn with_thumb_size(loc: &MediaLocation, thumb_size: &str) -> Option<MediaLocation> {
    match loc {
        MediaLocation::Photo {
            id,
            access_hash,
            file_reference,
            dc_id,
            ..
        } => Some(MediaLocation::Photo {
            id: *id,
            access_hash: *access_hash,
            file_reference: file_reference.clone(),
            thumb_size: thumb_size.to_string(),
            dc_id: *dc_id,
        }),
        MediaLocation::Document {
            id,
            access_hash,
            file_reference,
            dc_id,
            mime_type,
            ..
        } => Some(MediaLocation::Document {
            id: *id,
            access_hash: *access_hash,
            file_reference: file_reference.clone(),
            thumb_size: thumb_size.to_string(),
            dc_id: *dc_id,
            mime_type: mime_type.clone(),
        }),
        _ => None,
    }
}

/// Persist a working getFile letter so later thumb fetches skip the invalid size.
pub fn remember_thumb_location(media: &mut MediaRef, loc: MediaLocation) {
    if media.thumb_location.is_some() {
        media.thumb_location = Some(loc);
    } else {
        media.location = loc;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaDownloadKind {
    Full,
    Thumb,
    Display,
}

/// Thumb downloads must use an inline blob or a real getFile size.
/// Falling back to the full document/photo is how thumbs hit `FILE_ID_INVALID`.
pub fn media_for_download(
    media: &MediaRef,
    kind: MediaDownloadKind,
) -> Result<MediaRef, MtprotoError> {
    match kind {
        MediaDownloadKind::Full => Ok(media.clone()),
        MediaDownloadKind::Display => {
            if let Some(loc) = &media.display_location
                && location_is_getfile_thumb(loc) {
                    let mut copy = media.clone();
                    copy.location = loc.clone();
                    copy.file_size = None;
                    copy.cache_key = media
                        .display_cache_key
                        .clone()
                        .unwrap_or_else(|| format!("{}:display", media.cache_key));
                    return Ok(copy);
                }
            Err(MtprotoError::Message("no display size".into()))
        }
        MediaDownloadKind::Thumb => {
            if let Some(loc) = &media.thumb_location
                && location_is_getfile_thumb(loc) {
                    let mut copy = media.clone();
                    copy.location = loc.clone();
                    copy.file_size = None;
                    copy.cache_key = media
                        .thumb_cache_key
                        .clone()
                        .unwrap_or_else(|| media.cache_key.clone());
                    return Ok(copy);
                }
            // Only size is already a getFile photo thumb (`m`/`x`/…); not a full document.
            if location_is_getfile_thumb(&media.location) {
                return Ok(media.clone());
            }
            if matches!(
                &media.location,
                MediaLocation::Inline { bytes, .. } if !bytes.is_empty()
            ) {
                return Ok(media.clone());
            }
            Err(MtprotoError::Message("no downloadable thumb".into()))
        }
    }
}

/// Expanded JPEG for `photoStrippedSize` / cached thumbs. No `upload.getFile`.
pub fn inline_thumb_jpeg(media: &MediaRef) -> Option<Vec<u8>> {
    let download = media_for_download(media, MediaDownloadKind::Thumb).ok()?;
    match download.location {
        MediaLocation::Inline { bytes, .. } if !bytes.is_empty() => Some(bytes),
        _ => None,
    }
}

#[cfg(test)]
#[path = "../tests/unit/media_location_tests.rs"]
mod tests;
