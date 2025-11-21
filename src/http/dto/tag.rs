use crate::model::values::tag_name::TagName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TagsResponse {
    pub tags: Vec<TagName>,
}
