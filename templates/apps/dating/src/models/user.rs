use nucleus_std::photon::Model;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, Model)]
#[model(table_name = "users")]
pub struct User {
    pub id: u32,
    pub username: String,
    pub age: u8,
    pub bio: String,
    pub avatar_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Model)]
#[model(table_name = "matches")]
pub struct Match {
    pub id: u32,
    pub user_a_id: u32,
    pub user_b_id: u32,
    pub score: u32,
    pub created_at: String,
}
