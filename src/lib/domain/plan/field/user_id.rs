use crate::data::DbId;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserId(DbId);

impl UserId {
    pub fn new() -> Self {
        Self(DbId::new())
    }
}

impl UserId {
    pub fn into_inner(self) -> DbId {
        self.0
    }
}

impl From<DbId> for UserId {
    fn from(id: DbId) -> Self {
        Self(id)
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self(DbId::nil())
    }
}
