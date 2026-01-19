use nucleus_std::impl_model;
use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Artist {
    pub id: i64,
    pub name: String,
    pub bio: Option<String>,
    pub image_url: Option<String>,
}

impl_model!(Artist, "artists");
