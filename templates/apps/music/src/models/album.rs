use nucleus_std::impl_model;
use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Album {
    pub id: i64,
    pub title: String,
    pub artist_id: i64,
    pub year: Option<i64>, // SQLite integers are i64
    pub cover_path: Option<String>,
}

impl_model!(Album, "albums");
