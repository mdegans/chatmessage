use crate::{
    chat::{self},
    user,
};
#[cfg(any(feature = "server", feature = "client"))]
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[cfg_attr(any(feature = "client", feature = "server"), derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[cfg_attr(test, derive(PartialEq))]
pub struct Session<'a> {
    pub token: String,
    pub user: user::User,
    pub active_chat: Option<chat::Id>,
    pub chat_list: chat::List<'a>,
}

impl Session<'_> {
    pub fn active_chat(&self) -> Option<&str> {
        self.active_chat
            .as_ref()
            .and_then(|id| self.chat_list.chats.get(id))
            .map(AsRef::as_ref)
    }

    pub fn chats(&self) -> impl Iterator<Item = &str> {
        self.chat_list.chats.values().map(AsRef::as_ref)
    }
}
