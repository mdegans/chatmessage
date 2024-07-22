use crate::{
    chat::{self, Chat},
    message::{self, Message},
    user, Session,
};
use serde::{Deserialize, Serialize};

/// Response from the server to the client.
#[cfg_attr(feature = "client", derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
pub enum Response {
    /// Logout response.
    Logout,
    /// [`user::Info`] response. Sent in response to
    /// [`Request::Info`](crate::request::Request::Info),
    /// [`Request::Login`](crate::request::Request::Login), and
    /// [`Request::Register`](crate::request::Request::Register).
    Info(user::Info),
    /// User preferences response.
    Preferences(user::Preferences),
    /// [`chat::List`] response. Sent in response to
    /// [`Request::ListChats`](crate::request::Request::ListChats),
    /// and [`Request::DeleteChat`](crate::request::Request::DeleteChat).
    ChatList(chat::List),
    /// Chat response.
    Chat(Chat),
    /// Message response.
    Message(chat::Id, Message),
    /// Session response.
    Session(Session),
}
