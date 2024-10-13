use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Id(pub (u64, u64));

impl Id {
    pub const fn from_uuid(uuid: uuid::Uuid) -> Self {
        Self(uuid.as_u64_pair())
    }

    pub const fn into_uuid(self) -> uuid::Uuid {
        uuid::Uuid::from_u64_pair(self.0 .0, self.0 .1)
    }
}

impl From<uuid::Uuid> for Id {
    fn from(uuid: uuid::Uuid) -> Self {
        Self::from_uuid(uuid)
    }
}

impl Into<uuid::Uuid> for Id {
    fn into(self) -> uuid::Uuid {
        self.into_uuid()
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::from_uuid(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_uuid())
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.into_uuid().to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Id, D::Error>
    where
        D: Deserializer<'de>,
    {
        let uuid = uuid::Uuid::parse_str(&String::deserialize(deserializer)?)
            .map_err(serde::de::Error::custom)?;
        Ok(Self::from_uuid(uuid))
    }
}
