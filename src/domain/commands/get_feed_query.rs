use crate::http::dto::article::ArticleListQuery as ArticleListQueryDto;
use crate::model::limit::Limit;
use crate::model::offset::Offset;
use crate::model::values::user_id::UserId;

#[derive(Debug, Clone)]
pub struct GetFeedQuery {
    pub user_id: UserId,
    pub limit: Option<Limit>,
    pub offset: Option<Offset>,
}

impl GetFeedQuery {
    pub fn from_request(dto: ArticleListQueryDto, user_id: UserId) -> Self {
        GetFeedQuery {
            user_id,
            limit: dto.limit,
            offset: dto.offset,
        }
    }
}
