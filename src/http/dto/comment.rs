use crate::http::dto::profile::Profile;
use crate::model::values::comment_body::CommentBody;
use crate::model::values::comment_id::CommentId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::model::persistence::comment_view::CommentView;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentResponse {
    pub comment: CommentItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentsResponse {
    pub comments: Vec<CommentItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentItem {
    pub id: CommentId,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub body: CommentBody,
    pub author: Profile,
}

impl CommentItem {
    pub fn from_comment_view(view: CommentView) -> CommentItem {
        CommentItem {
            id: view.id,
            created_at: view.created_at,
            updated_at: view.updated_at,
            body: view.body,
            author: Profile {
                username: view.author,
                bio: view.author_bio,
                image: view.author_image,
                following: view.following,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    pub comment: CreateComment,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateComment {
    pub body: CommentBody,
}
