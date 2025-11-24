use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Limit(u64);

impl Limit {
    pub fn new(limit: u64) -> Self {
        Limit(limit)
    }

    pub(crate) fn value(&self) -> u64 {
        self.0
    }
}

impl Default for Limit {
    fn default() -> Self {
        Limit(50)
    }
}
