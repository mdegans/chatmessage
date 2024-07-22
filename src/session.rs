use crate::user;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "client", derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Session {
    pub token: String,
    pub user: user::Info,
    pub active_chat: Option<crate::chat::Chat>,
    pub chat_list: crate::chat::List,
}
