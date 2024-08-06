use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "dioxus", derive(Debug, PartialEq))]
pub enum Role {
    // The client does not need to craft agent messages. The server will also
    // not accept agent messages from the client.
    #[cfg_attr(feature = "client", serde(skip_serializing))]
    Agent,
    User,
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "dioxus", derive(Debug, PartialEq))]
pub struct Message {
    pub role: Role,
    pub content: String,
}
