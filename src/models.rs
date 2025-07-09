
use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Deserialize)]
pub struct PasteForm {
    pub title: String,
    pub content: String,
    pub expire: String,
    pub password: Option<String>,
}

pub struct Paste {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: OffsetDateTime,
    pub expires_at: Option<OffsetDateTime>,
    pub views: i32,
    pub password_hash: Option<String>,
}
