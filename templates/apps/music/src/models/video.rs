use nucleus_std::impl_model;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Video {
    pub id: i64,
    pub title: String,
    pub path: String,
    pub duration: Option<i64>,
    pub year: Option<i64>,
    pub plot: Option<String>,
    pub cast: Option<String>, // JSON string
    pub rating: Option<i64>,
    pub cover_path: Option<String>,
}

impl_model!(Video, "videos");
