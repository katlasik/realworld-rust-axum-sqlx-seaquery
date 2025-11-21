use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default)]
pub struct Offset(i64);

impl Offset {
    pub fn new(limit: i64) -> Self {
        Offset(limit)
    }
  
    pub(crate) fn value(&self) -> i64 {
        self.0
    }
}
