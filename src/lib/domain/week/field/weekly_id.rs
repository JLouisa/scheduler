use crate::data::DbWeekId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyId(DbWeekId);

impl WeeklyId {
    pub fn new() -> Self {
        Self(DbWeekId::new())
    }
}

impl WeeklyId {
    pub fn into_inner(self) -> DbWeekId {
        self.0
    }
}

impl From<DbWeekId> for WeeklyId {
    fn from(id: DbWeekId) -> Self {
        Self(id)
    }
}
