use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "dioxus", derive(Debug, PartialEq))]
pub enum Role {
    // The client does not need to craft agent messages. The server will also
    // not accept agent messages from the client.
    #[cfg_attr(feature = "client", serde(skip_serializing))]
    Agent,
    User,
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "dioxus", derive(Debug, PartialEq))]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[cfg(feature = "misanthropic")]
impl<'a> From<misanthropic::prompt::Message<'a>> for Message {
    fn from(msg: misanthropic::prompt::Message) -> Self {
        Self {
            role: match msg.role {
                misanthropic::prompt::message::Role::User => Role::User,
                misanthropic::prompt::message::Role::Assistant => Role::Agent,
            },
            content: msg.content.to_string(),
        }
    }
}

#[cfg(feature = "misanthropic")]
impl<'a> Into<misanthropic::prompt::Message<'a>> for Message {
    fn into(self) -> misanthropic::prompt::Message<'a> {
        misanthropic::prompt::Message {
            role: match self.role {
                Role::User => misanthropic::prompt::message::Role::User,
                Role::Agent => misanthropic::prompt::message::Role::Assistant,
            },
            content: misanthropic::prompt::message::Content::SinglePart(self.content.into()),
        }
    }
}
