use nucleus_std::impl_model;
use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Track {
    pub id: i64,
    pub title: String,
    pub album_id: Option<i64>,
    pub artist_id: Option<i64>,
    pub path: String,
    pub duration: Option<i64>,
    pub track_number: Option<i64>,
    pub genre: Option<String>,
    pub rating: Option<i64>,
}

impl_model!(Track, "tracks");
