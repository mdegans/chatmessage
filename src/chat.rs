use std::{borrow::Cow, collections::BTreeMap};

use crate::message::Message;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub (u64, u64));
/// Public-facing chat struct. This is user-exportable.
#[derive(Clone)]
#[cfg_attr(any(feature = "client", feature = "server"), derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[cfg_attr(test, derive(PartialEq))]
pub struct Chat<'a> {
    pub title: Cow<'a, str>,
    pub messages: Vec<Message<'a>>,
    /// Temporarily disable agent memory (except safety-related information).
    pub incognito: bool,
}

#[cfg(feature = "misanthropic")]
impl<'a, 'b> From<&misanthropic::prompt::Prompt<'a>> for Chat<'b>
where
    'a: 'b,
{
    fn from(chat: &misanthropic::prompt::Prompt<'a>) -> Self {
        Self {
            title: "Untitled".into(),
            messages: chat.messages.iter().map(Message::from).collect::<Vec<_>>(),
            incognito: false,
        }
    }
}

/// List of chats.
#[derive(Clone)]
#[cfg_attr(any(feature = "client", feature = "server"), derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize, Default))]
#[cfg_attr(test, derive(PartialEq))]
pub struct List<'a> {
    pub chats: BTreeMap<Id, Cow<'a, str>>,
}
