use crate::domain::UserError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Admin(bool);

impl Admin {
    pub fn new(admin: bool) -> Self {
        Self(admin)
    }
    pub fn into_inner(self) -> bool {
        self.0
    }
    pub fn flip(&mut self) -> bool {
        let new = self.0 = !self.0;
        return new;
    }
}

impl Default for Admin {
    fn default() -> Self {
        Self(false)
    }
}
