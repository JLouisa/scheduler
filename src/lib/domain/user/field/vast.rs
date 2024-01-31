use crate::domain::UserError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Vast(bool);

impl Vast {
    pub fn new(vast: bool) -> Self {
        Self(vast)
    }
    pub fn into_inner(self) -> bool {
        self.0
    }
    pub fn flip(&mut self) -> bool {
        let new = self.0 = !self.0;
        return new;
    }
}

impl Default for Vast {
    fn default() -> Self {
        Self(false)
    }
}
