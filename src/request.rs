use crate::{chat, message::Message, user};
#[cfg(feature = "server")]
use serde::Deserialize;
#[cfg(feature = "client")]
use serde::Serialize;

/// Request from client to the server.
#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(feature = "server", derive(Deserialize))]
pub enum Request {
    /// Register a new user.
    Register(user::Registration),
    /// Log in.
    Login(user::Login),
    /// Log out.
    Logout,
    /// Update user preferences.
    UpdatePreferences(user::Preferences),
    /// Get a list of chats.
    ChatList,
    /// Get a chat.
    GetChat(chat::Id),
    /// Delete a chat.
    DeleteChat(chat::Id),
    /// Send a message.
    Message(chat::Id, Message),
    /// Get the session.
    Session,
}

#[derive(thiserror::Error, Debug)]
#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(feature = "server", derive(Deserialize))]
pub enum Error {
    #[error("missing token")]
    MissingToken,
    #[error("token has expired")]
    TokenExpired,
    #[error("invalid or inexistent IP address")]
    InvalidIp,
    #[error("invalid request")]
    InvalidRequest,
    #[error("suspicious activity detected")]
    Sus,
    #[error("You are banned because: {reason}.")]
    Banned { reason: String },
}
