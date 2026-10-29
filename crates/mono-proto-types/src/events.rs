use crate::message_types::{ChatDto, MessageDto};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CloudDraft {
    pub chat_id: i64,
    pub thread_id: i32,
    pub text: String,
    pub entities_json: Option<String>,
    pub reply_to_message_id: i32,
    pub date: i32,
    pub no_webpage: bool,
    pub has_unsupported_content: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScheduledMessageDto {
    pub message: MessageDto,
    #[serde(default = "unsupported_by_default")]
    pub has_unsupported_content: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum OutgoingMessageResult {
    Sent { message: MessageDto },
    Scheduled { message: ScheduledMessageDto },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum UpdateEventDto {
    PollChanged {
        poll_id: i64,
    },
    ChatsChanged,
    FoldersChanged,
    DraftChanged {
        draft: CloudDraft,
    },
    OutgoingMessageAcknowledged {
        random_id: i64,
        result: OutgoingMessageResult,
    },
    ScheduledMessageChanged {
        message: ScheduledMessageDto,
    },
    ScheduledMessagesDeleted {
        chat_id: i64,
        message_ids: Vec<i32>,
        sent_message_ids: Option<Vec<i32>>,
    },
    NewMessage {
        message: MessageDto,
    },
    MessageEdited {
        message: MessageDto,
    },
    MessagesDeleted {
        chat_id: Option<i64>,
        message_ids: Vec<i32>,
    },
    PeerTyping {
        chat_id: i64,
        user_id: i64,
        typing: bool,
        action: String,
    },
    PeerStatus {
        user_id: i64,
        status: Option<String>,
        status_at: Option<i64>,
    },
    PeerEmojiStatus {
        user_id: i64,
        document_id: Option<i64>,
    },
    ReadInbox {
        chat_id: i64,
        max_id: i32,
        still_unread: i32,
    },
    ReadOutbox {
        chat_id: i64,
        max_id: i32,
    },
    SavedGifsChanged,
    MessageReactions {
        chat_id: i64,
        message_id: i32,
        reactions_json: String,
    },
    DiscussionInbox {
        channel_id: i64,
        top_message_id: i32,
        read_max_id: i32,
    },
    /// Dialog rows from a lazy `messages.getPeerDialogs` batch. Not a full chat refresh.
    DialogsPatched {
        chats: Vec<ChatDto>,
    },
    /// Edge-triggered catch-up signal. True while difference or channel recovery is in flight.
    SyncState {
        is_syncing: bool,
    },
    Ignored {
        kind: String,
    },
}

fn unsupported_by_default() -> bool {
    true
}
