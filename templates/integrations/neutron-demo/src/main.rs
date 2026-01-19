mod logic;
use axum::{
    extract::{Form, Path, State},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use logic::store::TodoStore;
use serde::Deserialize;
use std::sync::Arc;
use tower_http::services::ServeDir;

// Start: Shared State
struct AppState {
    store: Arc<TodoStore>,
}

#[tokio::main]
async fn main() {
    println!("⚛️  Neutron Web Demo starting on http://0.0.0.0:3000");

    let store = Arc::new(TodoStore::new(Vec::new(), "all".to_string()));

    // Seed some data
    store.add_todo("Learn Nucleus".to_string());
    store.add_todo("Build something cool".to_string());

    let app_state = Arc::new(AppState { store });

    let app = Router::new()
        .route("/", get(index))
        .route("/add", post(add_todo))
        .route("/toggle/:id", post(toggle_todo))
        .route("/filter/:name", get(set_filter))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let html = render_page(&state.store);
    Html(html)
}

#[derive(Deserialize)]
struct AddTodoForm {
    text: String,
}

async fn add_todo(
    State(state): State<Arc<AppState>>,
    Form(form): Form<AddTodoForm>,
) -> impl IntoResponse {
    if !form.text.is_empty() {
        state.store.add_todo(form.text);
    }
    Redirect::to("/")
}

async fn toggle_todo(State(state): State<Arc<AppState>>, Path(id): Path<u64>) -> impl IntoResponse {
    state.store.toggle(id);
    Redirect::to("/")
}

async fn set_filter(
    State(state): State<Arc<AppState>>,
    Path(filter): Path<String>,
) -> impl IntoResponse {
    state.store.set_filter(filter);
    Redirect::to("/")
}

// --- View Logic (SSR) ---
// In a real app, use a template engine like Askama or Tera.
// Here we just use format! for 0-dependency simplicity.

fn render_page(store: &TodoStore) -> String {
    let todos = store.filtered_todos();
    let current_filter = store.filter.get();

    let list_html: String = todos.iter().map(|todo| {
        let class = if todo.completed { "todo-item completed" } else { "todo-item" };
        // We wrap the item in a form to support "click to toggle" via POST
        // For better UX, we'd use HTMX/JS, but plain HTML forms work too.
        // Actually, let's just make the whole item a button or link for simplicity, 
        // but strict POST is better for actions.
        format!(r#"
            <form action="/toggle/{id}" method="POST" style="margin:0">
                <button type="submit" class="{class}" style="width:100%; text-align:left; font:inherit; color:inherit;">
                    <div class="checkbox"></div>
                    <span>{text}</span>
                </button>
            </form>
        "#, id=todo.id, text=todo.text, class=class)
    }).collect();

    let tab_class = |name: &str| {
        if current_filter == name {
            "tab active"
        } else {
            "tab"
        }
    };

    format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Neutron Todo</title>
    <link rel="stylesheet" href="/static/style.css">
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet">
</head>
<body>
    <div class="container">
        <div class="card">
            <h1>⚛️ Neutron Todo</h1>
            <p class="subtitle">Reactive State Management Demo</p>

            <form action="/add" method="POST" class="input-group">
                <input type="text" name="text" placeholder="What needs to be done?" autofocus autocomplete="off">
                <button type="submit" class="primary">Add</button>
            </form>

            <div class="tabs">
                <a href="/filter/all" class="{active_all}">All</a>
                <a href="/filter/active" class="{active_active}">Active</a>
                <a href="/filter/completed" class="{active_completed}">Completed</a>
            </div>

            <div class="todo-list">
                {list}
            </div>
            
            <div class="stats">
                {count} items left
            </div>
        </div>
    </div>
</body>
</html>
"#,
        list = list_html,
        active_all = tab_class("all"),
        active_active = tab_class("active"),
        active_completed = tab_class("completed"),
        count = todos.len()
    )
}
