use nucleus_std::impl_model;
use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Playlist {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub created_at: String, // String for SQLite datetime
}

impl_model!(Playlist, "playlists");
