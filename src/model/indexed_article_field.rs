use crate::persistence::schema::Articles;
use sea_query::IntoColumnRef;

pub enum IndexedArticleField {
    Slug,
    Id,
}

impl IndexedArticleField {
    pub(crate) fn to_field_name(&self) -> impl IntoColumnRef {
        match self {
            IndexedArticleField::Slug => (Articles::Table, Articles::Slug),
            IndexedArticleField::Id => (Articles::Table, Articles::Id),
        }
    }
}
