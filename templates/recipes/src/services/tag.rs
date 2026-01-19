use nucleus_std::impl_model;
use nucleus_std::photon::query::Model;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: String,
}

// Active Record Implementation
impl_model!(Tag, "tags");
