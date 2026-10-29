#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatDto {
    pub id: i64,
    pub title: String,
    pub is_channel: bool,
    pub is_group: bool,
    pub is_forum: bool,
    /// True if the current user is not a participant of this dialog.
    pub left: bool,
    pub unread_count: i32,
    pub last_message_preview: Option<String>,
    pub last_message_date: Option<i64>,
    pub archived: bool,
    pub muted: bool,
    pub is_contact: bool,
    pub is_bot: bool,
    pub is_verified: bool,
    pub photo_cache_key: Option<String>,
    pub pinned: bool,
    pub read_inbox_max_id: i32,
    pub read_outbox_max_id: i32,
    pub peer_status: Option<String>,
    pub peer_status_at: Option<i64>,
    pub last_media_thumb_cache_key: Option<String>,
    pub last_message_id: i32,
    pub can_view: bool,
    pub can_send_plain: bool,
    pub can_send_photos: bool,
    #[serde(default)]
    pub can_send_polls: bool,
    pub can_forward: bool,
    pub can_delete_others: bool,
    /// Custom emoji / collectible document for the peer name.
    pub emoji_status_document_id: Option<i64>,
    /// Whether the last dialog message was outgoing.
    pub last_message_outgoing: bool,
    /// Whether the dialog carries its own mute setting instead of inheriting the type default.
    pub mute_override: bool,
    /// `dialog.unread_mark`: the user manually marked the dialog unread.
    pub unread_mark: bool,
    /// `dialog.unread_mentions_count`.
    pub unread_mentions_count: i32,
    /// `dialog.unread_reactions_count`.
    pub unread_reactions_count: i32,
    /// Creator or `manage_topics` admin right.
    pub can_manage_topics: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MessageDto {
    pub chat_id: i64,
    pub id: i32,
    pub sender_id: Option<i64>,
    pub text: Option<String>,
    pub date: i64,
    pub edit_date: Option<i64>,
    pub outgoing: bool,
    pub media_kind: Option<String>,
    pub media_cache_key: Option<String>,
    pub thumb_cache_key: Option<String>,
    pub media_duration: Option<i32>,
    pub media_width: Option<i32>,
    pub media_height: Option<i32>,
    pub reply_quote: Option<String>,
    /// Compact JSON list of `{kind,offset,length,url?}` entities for `reply_quote`.
    pub reply_quote_entities_json: Option<String>,
    /// Compact JSON list of `{kind,offset,length,url?}` message entities.
    pub entities_json: Option<String>,
    pub noforwards: bool,
    pub reply_to_msg_id: Option<i32>,
    pub reply_to_top_id: Option<i32>,
    pub fwd_from: Option<String>,
    pub fwd_from_id: Option<i64>,
    pub fwd_date: Option<i64>,
    pub via_bot: Option<String>,
    pub sender_name: Option<String>,
    pub sender_emoji_status_document_id: Option<i64>,
    pub grouped_id: Option<i64>,
    pub file_name: Option<String>,
    pub file_size: Option<i64>, // album / document meta
    pub supports_streaming: bool,
    pub reactions_json: Option<String>,
    pub replies_count: i32,
    pub discussion_peer_id: Option<i64>,
    /// Compact JSON for reply/inline keyboards. Never log.
    pub reply_markup_json: Option<String>,
    pub forum_topic: bool,
    /// Avatar already indexed at `(sender_id, 0)` when its key starts with `avatar:`.
    #[serde(default)]
    pub sender_photo_cache_key: Option<String>,
    /// `None` means the cached record predates media privacy metadata.
    #[serde(default)]
    pub media_preview_restricted: Option<bool>,
}
