use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(Debug, PartialEq))]
#[cfg_attr(feature = "client", derive(Serialize))]
#[cfg_attr(feature = "server", derive(Deserialize))]
pub struct SecondFactorCode(pub [u8; 6]);

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Id(pub u64);

/// User information. This is sent from the server to the client.
#[cfg_attr(feature = "client", derive(Deserialize))]
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
