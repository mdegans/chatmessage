use std::collections::BTreeMap;

use crate::message::Message;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub u64);

#[cfg(feature = "dioxus")]
impl std::str::FromStr for Id {
    type Err = <u64 as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Id(s.parse()?))
    }
}
#[cfg(feature = "dioxus")]
impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Public-facing chat struct. This is user-exportable.
#[cfg_attr(any(feature = "client", feature = "server"), derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "dioxus", derive(Clone, Debug, PartialEq))]
pub struct Chat {
    pub id: Id,
    pub title: String,
    pub messages: Vec<Message>,
    /// Temporarily disable agent memory (except safety-related information).
    pub incognito: bool,
}

/// List of chats.
#[cfg_attr(any(feature = "client", feature = "server"), derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "dioxus", derive(Clone, Debug, PartialEq))]
pub struct List {
    pub chats: BTreeMap<Id, crate::chat::Chat>,
}
