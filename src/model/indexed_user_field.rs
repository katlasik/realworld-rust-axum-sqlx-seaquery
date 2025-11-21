pub enum IndexedUserField {
    Email,
    Username,
    Id,
}

impl IndexedUserField {
    pub(crate) fn to_field_name(&self) -> &str {
        match self {
            IndexedUserField::Email => "email",
            IndexedUserField::Username => "username",
            IndexedUserField::Id => "id",
        }
    }
}
