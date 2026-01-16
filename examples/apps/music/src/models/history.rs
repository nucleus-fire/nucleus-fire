use serde::Serialize;
use nucleus_std::impl_model;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PlaybackHistory {
    pub id: i64,
    pub media_type: String,
    pub media_id: i64,
    pub position: i64,
    pub updated_at: String, // SQLite returns strings for DATETIME by default usually
}

impl_model!(PlaybackHistory, "playback_history");
