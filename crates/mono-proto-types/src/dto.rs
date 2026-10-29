use crate::event_types::UpdateEventDto;
use crate::message_types::MessageDto;
use crate::updates_state::UpdatesStateDto;

#[derive(Debug, Clone)]
pub struct FolderDto {
    pub id: i32,
    pub title: String,
    pub chat_ids: Vec<i64>,
    pub exclude_chat_ids: Vec<i64>,
    pub include_contacts: bool,
    pub include_non_contacts: bool,
    pub include_groups: bool,
    pub include_channels: bool,
    pub include_bots: bool,
    pub exclude_muted: bool,
    pub exclude_read: bool,
    pub exclude_archived: bool,
    pub emoticon: String,
    /// `dialogFilter.pinned_peers`, distinct from include peers in [chat_ids].
    pub pinned_chat_ids: Vec<i64>,
}

#[derive(Debug, Clone)]
pub struct ProfileDto {
    pub id: i64,
    pub kind: String,
    pub title: String,
    pub username: Option<String>,
    pub about: Option<String>,
    pub avatar_cache_key: Option<String>,
    pub is_self: bool,
    pub status: Option<String>,
    pub status_at: Option<i64>,
    /// Compact JSON of extra profile facts (members, phone, badges). Never log.
    pub extra_json: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MessagesPageDto {
    pub messages: Vec<MessageDto>,
    pub total_count: i32,
}

#[derive(Debug, Clone)]
pub struct SavedGifDto {
    pub document_id: i64,
    pub cache_key: String,
    pub thumb_cache_key: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct ForumTopicDto {
    pub id: i32,
    pub title: String,
    pub icon_color: i32,
    pub icon_emoji_id: Option<i64>,
    pub top_message: i32,
    pub date: i32,
    pub unread_count: i32,
    pub unread_mentions_count: i32,
    pub unread_reactions_count: i32,
    pub read_inbox_max_id: i32,
    pub pinned: bool,
    pub closed: bool,
    pub hidden: bool,
    pub short: bool,
    pub deleted: bool,
    pub last_message_preview: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ForumTopicsPageDto {
    pub count: i32,
    pub topics: Vec<ForumTopicDto>,
}

#[derive(Debug, Clone)]
pub struct DiscussionDto {
    pub chat_id: i64,
    pub message_id: i32,
}

#[derive(Debug, Clone)]
pub struct ReactionChoiceDto {
    pub emoticon: String,
    pub document_id: i64,
}

#[derive(Debug, Clone)]
pub struct ChannelCursorDto {
    pub chat_id: i64,
    pub pts: i32,
}

#[derive(Debug, Clone)]
pub struct UpdateBatchDto {
    pub account_generation: i64,
    pub batch_id: i64,
    pub events: Vec<UpdateEventDto>,
    pub cursor_after: Option<UpdatesStateDto>,
    pub channel_pts_after: Vec<ChannelCursorDto>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateDrainDto {
    pub batches: Vec<UpdateBatchDto>,
    pub live_events: Vec<UpdateEventDto>,
    pub replay_required: bool,
}

#[derive(Debug, Clone, Default)]
pub struct NotifySettingsDto {
    pub show_previews: Option<bool>,
    pub silent: bool,
    pub mute_until: i32,
    pub stories_muted: bool,
    pub stories_hide_sender: bool,
    pub sound: String,
}

#[derive(Debug, Clone)]
pub struct NotifyExceptionDto {
    pub peer_kind: String,
    pub chat_id: i64,
    pub topic_id: Option<i32>,
    pub show_previews: Option<bool>,
    pub silent: bool,
    pub mute_until: i32,
    pub stories_muted: bool,
    pub stories_hide_sender: bool,
    pub sound: String,
}

#[derive(Debug, Clone)]
pub struct StickerPackDto {
    pub id: i64,
    pub access_hash: i64,
    pub title: String,
    pub short_name: String,
    pub count: i32,
    pub is_emoji: bool,
    pub preview_document_ids: Vec<i64>,
}

#[derive(Debug, Clone)]
pub struct StickerCatalogDto {
    pub hash: i64,
    pub not_modified: bool,
    pub sets: Vec<StickerPackDto>,
}

#[derive(Debug, Clone)]
pub struct StickerListDto {
    pub hash: i64,
    pub not_modified: bool,
    pub document_ids: Vec<i64>,
}

#[derive(Debug, Clone)]
pub struct ResolvedPeerDto {
    pub peer_id: i64,
    pub username: Option<String>,
    pub title: String,
    pub is_bot: bool,
}

#[derive(Debug, Clone)]
pub struct InlineBotResultDto {
    pub id: String,
    pub kind: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub document_id: Option<i64>,
    pub thumb_cache_key: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InlineBotResultsDto {
    pub query_id: i64,
    pub gallery: bool,
    pub next_offset: Option<String>,
    pub cache_time: i32,
    pub results: Vec<InlineBotResultDto>,
}

#[derive(Debug, Clone)]
pub struct SearchPeerDto {
    pub peer_id: i64,
    pub title: String,
    pub username: Option<String>,
    pub kind: String,
    pub is_bot: bool,
    pub is_group: bool,
    pub is_channel: bool,
}

#[derive(Debug, Clone)]
pub struct ContactsSearchDto {
    pub people: Vec<SearchPeerDto>,
    pub chats: Vec<SearchPeerDto>,
}

#[derive(Debug, Clone)]
pub struct GlobalMessageSearchDto {
    pub messages: Vec<MessageDto>,
    pub next_rate: i32,
    pub next_peer_id: i64,
    pub next_offset_id: i32,
}

#[derive(Debug, Clone)]
pub struct BotCallbackAnswerDto {
    pub alert: bool,
    pub message: Option<String>,
    pub url: Option<String>,
    pub cache_time: i32,
}

#[derive(Debug, Clone)]
pub struct UploadItemDto {
    pub path: String,
    pub kind: String,
    pub mime_type: String,
    pub file_name: String,
    pub caption: String,
    pub entities_json: Option<String>,
    pub duration: i32,
    pub width: i32,
    pub height: i32,
    pub random_id: i64,
}

#[derive(Debug, Clone)]
pub struct JoinChatDto {
    pub joined: bool,
    pub can_send_plain: bool,
    pub can_send_photos: bool,
    pub can_send_polls: bool,
}

impl JoinChatDto {
    pub fn pending() -> Self {
        Self {
            joined: false,
            can_send_plain: false,
            can_send_photos: false,
            can_send_polls: false,
        }
    }
}
