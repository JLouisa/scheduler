use crate::data::DbId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekId(DbId);

impl WeekId {
    pub fn new() -> Self {
        Self(DbId::new())
    }
}

impl WeekId {
    pub fn into_inner(self) -> DbId {
        self.0
    }
}

impl From<DbId> for WeekId {
    fn from(id: DbId) -> Self {
        Self(id)
    }
}

impl Default for WeekId {
    fn default() -> Self {
        Self(DbId::nil())
    }
}
