use serde::{Deserialize, Serialize};
use crate::http::dto::profile::Profile;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentResponse {
    pub comment: Comment,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentsResponse {
    pub comments: Vec<Comment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub body: String,
    pub author: Profile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    pub comment: CreateComment,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateComment {
    pub body: String,
}
