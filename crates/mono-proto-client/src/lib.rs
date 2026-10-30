pub use mono_proto_media::wallpaper::WallpaperCatalogDto;
pub use mono_proto_messages::instant_view::InstantViewDto;
pub use mono_proto_messages::messages::{
    checklist_creation::ChecklistSendSpec, contact_send::ContactSendSpec,
    location_send::LocationSendSpec, poll_creation::PollSendSpec,
};
pub use mono_proto_runtime::send_options::SendOptions;
pub mod adapter {
    pub mod protocol {
        pub use mono_proto::LATEST_API_LAYER;
        pub use mono_proto::latest;
    }
    pub mod media {
        pub use mono_proto_media::{location, profile, wallpaper};
    }
    pub mod messages {
        pub use mono_proto_messages::{
            contacts, extras_rpc, instant_view, messages_checklist_creation, messages_contact_send,
            messages_location_send, messages_poll_creation, messages_read_receipts_rpc,
            messages_send_result, notifications, upload_rpc,
        };
    }
    pub mod runtime {
        pub use mono_proto_runtime::{
            api_invoke, connection, request_control, scheduler, send_options, transfer_policy,
            transfer_window,
        };
    }
    pub mod observability {
        pub use mono_proto_observability::perf;
    }
    pub mod socket {
        pub use mono_proto_socket::{ProxyConfig, ProxyKind, TransportMode};
    }
    pub mod types {
        pub use mono_proto_types::{
            auth_types, collections, dto_types, error, event_types, init_connection, message_types,
            updates_state,
        };
    }
}

pub use mono_proto_types::{
    MtprotoError, collections::CompactString, dto_types::*, event_types::*, message_types::*,
};

pub use auth::{connect, is_authorized, send_auth_code, sign_in};
pub use construct::{create_client, create_encrypted_client};
pub use dispatch::{get_chats, get_history};
pub use updates::start_updates;

pub mod access;
pub mod auth;
pub mod bind_temp;
pub mod chat_invites;
pub mod checklists;
pub mod client;
pub mod config;
pub mod construct;
pub mod contacts;
pub mod dispatch;
pub mod drafts;
pub mod extras;
pub mod forum_management;
pub mod group_management;
pub mod lanes;
pub mod management;
pub mod media_download;
mod media_support;
pub mod membership;
pub mod merge;
mod message_ops;
pub mod passkey_mapping;
pub mod passkeys;
pub mod persist;
pub mod polls;
pub mod registry;
pub mod scheduled;
pub mod session;
pub mod session_store;
pub mod shared_folders;
pub mod state;
pub mod updates;
