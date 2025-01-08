use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Hash, PartialOrd)]
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

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
