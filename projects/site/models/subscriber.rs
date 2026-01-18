//! Subscriber model
//!
//! Represents newsletter subscribers.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscriber {
    pub id: i64,
    pub email: String,
    pub created_at: String,
}
