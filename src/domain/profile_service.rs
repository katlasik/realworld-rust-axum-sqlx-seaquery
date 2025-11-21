use crate::app_error::AppError;
use crate::model::values::user_id::UserId;
use crate::persistence::profile_repository::ProfileRepository;
use anyhow::Result;

#[derive(Clone)]
pub struct ProfileService {
    profile_repo: ProfileRepository,
}

impl ProfileService {
    pub fn new(profile_repo: ProfileRepository) -> Self {
        ProfileService { profile_repo }
    }

    pub async fn follow_user(
        &self,
        follower_id: UserId,
        followee_id: UserId,
    ) -> Result<(), AppError> {
        if follower_id == followee_id {
            return Err(AppError::BadData("Cannot follow yourself".to_string()));
        }

        self.profile_repo
            .follow_user(follower_id, followee_id)
            .await
    }

    pub async fn unfollow_user(
        &self,
        follower_id: UserId,
        followee_id: UserId,
    ) -> Result<(), AppError> {
        self.profile_repo
            .unfollow_user(follower_id, followee_id)
            .await
    }

    pub async fn is_following(
        &self,
        follower_id: UserId,
        followee_id: UserId,
    ) -> Result<bool, AppError> {
        self.profile_repo
            .is_following(follower_id, followee_id)
            .await
    }
}
