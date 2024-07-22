use std::collections::BTreeMap;

use crate::message::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(Debug))]
pub struct Id(pub u64);

/// Public-facing chat struct. This is user-exportable.
#[cfg_attr(feature = "client", derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Chat {
    pub id: Id,
    pub title: String,
    pub messages: Vec<Message>,
    /// Temporarily disable agent memory (except safety-related information).
    pub incognito: bool,
}

/// List of chats.
#[cfg_attr(feature = "client", derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct List {
    pub chats: BTreeMap<Id, String>,
}
