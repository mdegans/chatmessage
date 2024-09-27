use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Role {
    // The client does not need to craft agent messages. The server will also
    // not accept agent messages from the client.
    #[cfg_attr(feature = "client", serde(skip_serializing))]
    Agent,
    User,
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Message<'a> {
    pub role: Role,
    pub content: Cow<'a, str>,
}

#[cfg(feature = "misanthropic")]
impl<'a, 'b> From<&misanthropic::prompt::Message<'a>> for Message<'b>
where
    'a: 'b,
{
    fn from(msg: &misanthropic::prompt::Message<'a>) -> Self {
        Self {
            role: match msg.role {
                misanthropic::prompt::message::Role::User => Role::User,
                misanthropic::prompt::message::Role::Assistant => Role::Agent,
            },
            content: match &msg.content {
                misanthropic::prompt::message::Content::SinglePart(content) => content.clone(),
                misanthropic::prompt::message::Content::MultiPart(_) => {
                    Cow::Owned(msg.content.to_string())
                }
            },
        }
    }
}

#[cfg(feature = "misanthropic")]
impl<'a, 'b> Into<misanthropic::prompt::Message<'a>> for &Message<'b>
where
    'b: 'a,
{
    fn into(self) -> misanthropic::prompt::Message<'a> {
        misanthropic::prompt::Message {
            role: match self.role {
                Role::User => misanthropic::prompt::message::Role::User,
                Role::Agent => misanthropic::prompt::message::Role::Assistant,
            },
            content: misanthropic::prompt::message::Content::SinglePart(self.content.clone()),
        }
    }
}
