use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Active(bool);

impl Active {
    pub fn new(active: bool) -> Self {
        Self(active)
    }
    pub fn into_inner(self) -> bool {
        self.0
    }
    pub fn flip(self) -> bool {
        let new: bool = !self.0;
        return new;
    }
}

impl Default for Active {
    fn default() -> Self {
        Self(false)
    }
}
