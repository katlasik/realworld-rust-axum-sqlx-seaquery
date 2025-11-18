use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TagsResponse {
    pub tags: Vec<String>,
}
