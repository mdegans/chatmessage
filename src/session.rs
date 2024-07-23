use crate::{
    chat::{self, Chat},
    message::Message,
    user,
};
#[cfg(feature = "client")]
use serde::Deserialize;
#[cfg(feature = "server")]
use serde::Serialize;

#[cfg_attr(feature = "client", derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[cfg_attr(test, derive(PartialEq))]
pub struct Session {
    pub token: String,
    pub user: user::Info,
    pub active_chat: Option<crate::chat::Id>,
    pub chat_list: crate::chat::List,
}

impl Session {
    pub fn active_chat(&self) -> Option<&Chat> {
        self.active_chat
            .as_ref()
            .and_then(|id| self.chat_list.chats.get(id))
    }

    pub fn chats(&self) -> impl Iterator<Item = &Chat> {
        self.chat_list.chats.values()
    }

    pub fn chat_titles(&self) -> impl Iterator<Item = &str> {
        self.chats().map(|chat| chat.title.as_str())
    }
}
