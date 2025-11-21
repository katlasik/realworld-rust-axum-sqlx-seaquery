use crate::http::dto::article::ArticleListQuery as ArticleListQueryDto;
use crate::model::limit::Limit;
use crate::model::offset::Offset;
use crate::model::values::tag_name::TagName;
use crate::model::values::username::Username;

#[derive(Debug, Clone)]
pub struct ListArticlesQuery {
    pub tag: Option<TagName>,
    pub author: Option<Username>,
    pub favorited_by: Option<Username>,
    pub limit: Option<Limit>,
    pub offset: Option<Offset>,
}

impl ListArticlesQuery {
    pub fn from_request(dto: ArticleListQueryDto) -> Self {
        ListArticlesQuery {
            tag: dto.tag,
            author: dto.author,
            favorited_by: dto.favorited,
            limit: dto.limit,
            offset: dto.offset,
        }
    }
}
