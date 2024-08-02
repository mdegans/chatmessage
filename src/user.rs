use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(Debug, PartialEq))]
#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(feature = "server", derive(Deserialize))]
pub struct SecondFactorCode(pub [u8; 6]);

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(Debug))]
#[cfg_attr(feature = "dioxus", derive(Clone, Copy, Debug))]
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

/// User information. This is sent from the server to the client.
#[cfg_attr(any(feature = "client", feature = "server"), derive(Deserialize))]
#[cfg_attr(feature = "server", derive(Serialize))]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Info {
    pub id: Id,
    pub username: String,
    pub email: String,
    pub verified: bool,
    pub preferences: Preferences,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(feature = "server", derive(Deserialize))]
pub struct Registration {
    pub username: String,
    pub email: String,
    /// Note that this is salted before being stored in the database. Encryption
    /// protects this in transit.
    pub hashed_password: String,
    // TODO: Add second factor code
    // /// Second factor code
    // pub second_factor_code: SecondFactorCode,
}

#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(feature = "server", derive(Deserialize))]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Login {
    pub email_or_username: String,
    /// Note that this is salted before being stored in the database. Encryption
    /// protects this in transit.
    pub hashed_password: String,
    // TODO: Add second factor code
    /// Reset password request.
    pub reset: bool,
}

#[derive(Serialize, Deserialize, Default)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Preferences {
    /// Whether the agent has stores information about the user. Note that some
    /// safety-related information is always stored, like the user's karma and
    /// reports.
    pub agent_memory: bool,
    pub dark_mode: bool,
}
