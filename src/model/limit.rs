use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Limit(i64);

impl Limit {
    pub fn new(limit: i64) -> Self {
        Limit(limit)
    }

    pub(crate) fn value(&self) -> i64 {
        self.0
    }
}

impl Default for Limit {
  fn default() -> Self {
      Limit(50)
  }
}