//! Full user/chat/channel profiles and member lists.
//! https://core.telegram.org/method/users.getFullUser
//! https://core.telegram.org/method/messages.getFullChat
//! https://core.telegram.org/method/channels.getFullChannel

mod common;
mod full;
mod participants;
mod status;

pub use common::*;
pub use full::*;
pub use participants::*;
pub use status::*;

#[cfg(test)]
#[path = "../../tests/unit/profile_tests.rs"]
mod tests;
