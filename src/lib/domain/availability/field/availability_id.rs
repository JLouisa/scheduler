use crate::data::DbId;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Constructor, Copy, PartialEq, PartialOrd)]
pub struct AvailabilityId(DbId);

impl AvailabilityId {
    pub fn into_inner(self) -> DbId {
        self.0
    }
    pub fn to_the_string(&self) -> String {
        self.0.to_string().to_owned()
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
