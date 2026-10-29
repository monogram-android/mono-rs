use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MediaLocation {
    Photo {
        id: i64,
        access_hash: i64,
        file_reference: Vec<u8>,
        thumb_size: String,
        dc_id: i32,
    },
    Document {
        id: i64,
        access_hash: i64,
        file_reference: Vec<u8>,
        thumb_size: String,
        dc_id: i32,
        mime_type: String,
    },
    PeerPhoto {
        peer_kind: crate::peers::PeerKind,
        peer_id: i64,
        access_hash: i64,
        photo_id: i64,
        big: bool,
        dc_id: i32,
    },
    Inline {
        bytes: Vec<u8>,
        extension: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaRef {
    #[serde(default)]
    pub file_size: Option<i64>,
    pub kind: String,
    pub cache_key: String,
    pub location: MediaLocation,
    #[serde(default)]
    pub thumb_cache_key: Option<String>,
    #[serde(default)]
    pub thumb_location: Option<MediaLocation>,
    #[serde(default)]
    pub display_cache_key: Option<String>,
    #[serde(default)]
    pub display_location: Option<MediaLocation>,
    #[serde(default)]
    pub sticker_set_id: Option<i64>,
    #[serde(default)]
    pub sticker_set_access_hash: Option<i64>,
    #[serde(default)]
    pub source_url: Option<String>,
}
