use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Admin(bool);

impl Admin {
    pub fn new(admin: bool) -> Self {
        Self(admin)
    }
    pub fn into_inner(self) -> bool {
        self.0
    }
    pub fn flip(self) -> bool {
        let new: bool = !self.0;
        return new;
    }
}

impl Default for Admin {
    fn default() -> Self {
        Self(false)
    }
}
