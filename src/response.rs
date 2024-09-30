use std::borrow::Cow;

use crate::{
    chat::{self},
    message::Message,
    session::Session,
    user,
};
#[cfg(feature = "client")]
use serde::Deserialize;
#[cfg(feature = "server")]
use serde::Serialize;

/// Response from the server to the client.
#[cfg_attr(feature = "client", derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize, Clone, Debug))]
pub enum Response<'a> {
    /// Stream Connected. This is not sent from the server but generated by the
    /// client to indicate that the stream is connected.
    Connected,
    /// Logout response.
    Logout,
    /// Narration is a small blurb underneath a message to indicate actions.
    Narration(chat::Id, Cow<'a, str>),
    /// Preferences updated.
    Preferences(user::Preferences),
    /// Chat deleted.
    ChatDeleted(chat::Id),
    /// Message response.
    Message(chat::Id, Message<'a>),
    /// Session response. Sent in response to:
    /// [`Request::Login`](crate::request::Request::Login), and
    /// [`Request::Register`](crate::request::Request::Register).
    Session(Session<'a>),
}
