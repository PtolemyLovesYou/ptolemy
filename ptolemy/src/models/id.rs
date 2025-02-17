use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ParseError;

#[derive(Debug, Clone, Copy, PartialEq, Hash, PartialOrd, Serialize, Deserialize)]
pub struct Id(Uuid);

impl Id {
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl std::ops::Deref for Id {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Uuid> for Id {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<Id> for Uuid {
    fn from(val: Id) -> Self {
        val.0
    }
}

impl TryFrom<String> for Id {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Id(
            Uuid::parse_str(&value).map_err(|_| ParseError::InvalidUuid)?
        ))
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
