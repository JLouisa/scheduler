use crate::data::DbId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, PartialOrd)]
pub struct Id(DbId);

impl Id {
    pub fn new() -> Self {
        Self(DbId::new())
    }
    pub fn into_inner(self) -> DbId {
        self.0
    }
    pub fn to_the_string(&self) -> String {
        self.0.to_string().to_owned()
    }
}

impl From<DbId> for Id {
    fn from(id: DbId) -> Self {
        Self(id)
    }
}

impl Default for Id {
    fn default() -> Self {
        Self(DbId::nil())
    }
}
