use crate::data::DbId;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Constructor)]
pub struct UserID(DbId);

impl UserID {
    pub fn into_inner(self) -> DbId {
        self.0
    }
}

impl From<DbId> for UserID {
    fn from(id: DbId) -> Self {
        Self(id)
    }
}

impl Default for UserID {
    fn default() -> Self {
        Self(DbId::nil())
    }
}
