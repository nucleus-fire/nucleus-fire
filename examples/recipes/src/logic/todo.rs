use serde::{Deserialize, Serialize};
use nucleus_std::photon::query::Model;
use nucleus_std::impl_model;
use nucleus_std::server;
use nucleus_std::errors::Result;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub text: String,
    pub done: bool,
}

// 1. One-line Model Setup
impl_model!(Todo, "todos");

#[server]
pub async fn list_todos() -> Result<Vec<Todo>> {
    // 2. Beautiful Usage
    let todos = Todo::query()
        .order_by("id", "DESC")
        .all()
        .await?;
    Ok(todos)
}

#[server]
pub async fn add_todo(text: String) -> Result<()> {
    // 3. Create
    Todo::create()
        .value("text", text)
        .value("done", false)
        .execute()
        .await?;
    Ok(())
}
