use crate::data::DbId;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Constructor)]
pub struct AvailabilityId(DbId);

impl AvailabilityId {
    pub fn into_inner(self) -> DbId {
        self.0
    }
}

impl From<DbId> for AvailabilityId {
    fn from(id: DbId) -> Self {
        Self(id)
    }
}

impl Default for AvailabilityId {
    fn default() -> Self {
        Self(DbId::nil())
    }
}
