use crate::{
    chat::{self, Chat},
    user,
};
#[cfg(any(feature = "server", feature = "client"))]
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[cfg_attr(any(feature = "client", feature = "server"), derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[cfg_attr(test, derive(PartialEq))]
pub struct Session {
    pub token: String,
    pub user: user::User,
    pub active_chat: Option<chat::Id>,
    pub chat_list: chat::List,
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
