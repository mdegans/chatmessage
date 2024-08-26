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
impl From<misanthropic::request::Message> for Message {
    fn from(msg: misanthropic::request::Message) -> Self {
        Self {
            role: match msg.role {
                misanthropic::request::message::Role::User => Role::User,
                misanthropic::request::message::Role::Assistant => Role::Agent,
            },
            content: msg.content.to_string(),
        }
    }
}

#[cfg(feature = "misanthropic")]
impl Into<misanthropic::request::Message> for Message {
    fn into(self) -> misanthropic::request::Message {
        misanthropic::request::Message {
            role: match self.role {
                Role::User => misanthropic::request::message::Role::User,
                Role::Agent => misanthropic::request::message::Role::Assistant,
            },
            content: misanthropic::request::message::Content::SinglePart(self.content),
        }
    }
}
