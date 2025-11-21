use crate::app_error::AppError;
use crate::model::values::tag_name::TagName;
use crate::persistence::tag_repository::TagRepository;
use anyhow::Result;

#[derive(Clone)]
pub struct TagService {
    tag_repo: TagRepository,
}

impl TagService {
    pub fn new(tag_repo: TagRepository) -> Self {
        TagService { tag_repo }
    }

    pub async fn get_all_tags(&self) -> Result<Vec<TagName>, AppError> {
        self.tag_repo.get_all_tags().await
    }
}
