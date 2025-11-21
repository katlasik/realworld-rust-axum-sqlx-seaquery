pub enum IndexedArticleField {
    Slug,
    Id,
}

impl IndexedArticleField {
    pub(crate) fn to_field_name(&self) -> &str {
        match self {
            IndexedArticleField::Slug => "slug",
            IndexedArticleField::Id => "id",
        }
    }
}
