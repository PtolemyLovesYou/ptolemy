use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::error::ParseError;

#[derive(Debug, Clone, Copy, PartialEq, Hash, PartialOrd, Serialize, Deserialize)]
pub struct Id(Uuid);

impl From<Uuid> for Id {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl Into<Uuid> for Id {
    fn into(self) -> Uuid {
        self.0
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
