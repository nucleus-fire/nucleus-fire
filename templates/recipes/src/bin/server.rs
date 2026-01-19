#![allow(unused_imports)]
use axum::{
    extract::{Form, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;

// --- Static Assets (Zero-Allocation) ---
#[allow(non_snake_case, unreachable_code, unused_variables)]
async fn handle_counter(
    headers: axum::http::HeaderMap,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    let mut html_body = String::new();
    html_body.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Recipes</title><meta name=\"description\" content=\"Built with Nucleus\"></head><body>");
    html_body.push_str("<div");
    html_body.push_str(" class=\"app-layout\"");
    html_body.push_str(" style=\"display: flex; flex-direction: column; min-height: 100vh; font-family: system-ui, sans-serif; background: #111; color: white;\"");
    html_body.push('>');
    html_body.push_str("<nav");
    html_body.push_str(" style=\"border-bottom: 1px solid #333; padding: 1rem; display: flex; gap: 1rem; align-items: center;\"");
    html_body.push('>');
    html_body.push_str("<span");
    html_body.push_str(" style=\"font-weight: bold; font-size: 1.2rem;\"");
    html_body.push('>');
    html_body.push_str("🍳 Nucleus Recipes");
    html_body.push_str("</span>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Home");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/todo\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Todo");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/counter\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Counter");
    html_body.push_str("</a>");
    html_body.push_str("</nav>");
    html_body.push_str("<main");
    html_body.push_str(" style=\"flex: 1; padding: 2rem;\"");
    html_body.push('>');
    html_body.push_str("<div");
    html_body.push_str(" style=\"text-align: center; margin-top: 5rem;\"");
    html_body.push('>');
    html_body.push_str("<h1");
    html_body.push('>');
    html_body.push_str("Interactive Counter (WASM)");
    html_body.push_str("</h1>");
    html_body.push_str("<div");
    html_body.push_str(" id=\"counter-app\"");
    html_body.push('>');
    html_body.push_str("<button");
    html_body.push_str(" id=\"decrement\"");
    html_body.push_str(" style=\"padding: 10px 20px; font-size: 1.5rem;\"");
    html_body.push('>');
    html_body.push('-');
    html_body.push_str("</button>");
    html_body.push_str("<span");
    html_body.push_str(" id=\"count\"");
    html_body.push_str(" style=\"font-size: 3rem; margin: 0 2rem;\"");
    html_body.push('>');
    html_body.push('0');
    html_body.push_str("</span>");
    html_body.push_str("<button");
    html_body.push_str(" id=\"increment\"");
    html_body.push_str(" style=\"padding: 10px 20px; font-size: 1.5rem;\"");
    html_body.push('>');
    html_body.push('+');
    html_body.push_str("</button>");
    html_body.push_str("</div>");
    html_body.push_str("<p");
    html_body.push_str(" style=\"margin-top: 2rem; color: #888;\"");
    html_body.push('>');
    html_body.push_str("This logic runs in your browser via WebAssembly.");
    html_body.push_str("</p>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/\"");
    html_body.push_str(" style=\"color: #4facfe;\"");
    html_body.push('>');
    html_body.push_str("&larr; Back to Recipes");
    html_body.push_str("</a>");
    html_body.push_str("</div>");
    html_body.push_str("<!-- Client-Side Logic -->");
    html_body.push_str("");
    html_body.push_str("</main>");
    html_body.push_str("<footer");
    html_body.push_str(" style=\"border-top: 1px solid #333; padding: 1rem; text-align: center; color: #666; font-size: 0.8rem;\"");
    html_body.push('>');
    html_body.push_str("Built with Nucleus V3\n        ");
    html_body.push_str("</footer>");
    html_body.push_str("</div>");
    html_body.push_str("</body></html>");

    axum::response::Html(html_body).into_response()
}

#[allow(non_snake_case, unreachable_code, unused_variables)]
async fn handle_todo(
    headers: axum::http::HeaderMap,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    let mut html_body = String::new();
    html_body.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Recipes</title><meta name=\"description\" content=\"Built with Nucleus\"></head><body>");
    html_body.push_str("<div");
    html_body.push_str(" class=\"app-layout\"");
    html_body.push_str(" style=\"display: flex; flex-direction: column; min-height: 100vh; font-family: system-ui, sans-serif; background: #111; color: white;\"");
    html_body.push('>');
    html_body.push_str("<nav");
    html_body.push_str(" style=\"border-bottom: 1px solid #333; padding: 1rem; display: flex; gap: 1rem; align-items: center;\"");
    html_body.push('>');
    html_body.push_str("<span");
    html_body.push_str(" style=\"font-weight: bold; font-size: 1.2rem;\"");
    html_body.push('>');
    html_body.push_str("🍳 Nucleus Recipes");
    html_body.push_str("</span>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Home");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/todo\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Todo");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/counter\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Counter");
    html_body.push_str("</a>");
    html_body.push_str("</nav>");
    html_body.push_str("<main");
    html_body.push_str(" style=\"flex: 1; padding: 2rem;\"");
    html_body.push('>');
    html_body.push_str("<!-- Server Side Model Loading -->");
    html_body.push_str("<n:model");
    html_body.push_str(" todos=\"crate::logic::todo::list_todos().await\"");
    html_body.push('>');
    html_body.push_str("</n:model>");
    html_body.push_str("<!-- Action Handler (via Query Param for GET - Proto implementation) -->");
    html_body.push_str("<!-- In V4 this would be a Form Action -->");
    html_body.push_str("<div");
    html_body.push_str(" class=\"container\"");
    html_body.push('>');
    html_body.push_str("<h1");
    html_body.push('>');
    html_body.push_str("✅ Todo List");
    html_body.push_str("</h1>");
    html_body.push_str("<!-- Add Form -->");
    html_body.push_str("<form");
    html_body.push_str(" action=\"/todo\"");
    html_body.push_str(" method=\"get\"");
    html_body.push_str(" style=\"display: flex; gap: 1rem; margin-bottom: 2rem;\"");
    html_body.push('>');
    html_body.push_str("<input");
    html_body.push_str(" type=\"text\"");
    html_body.push_str(" name=\"text\"");
    html_body.push_str(" placeholder=\"What needs doing?\"");
    html_body.push_str(" required=\"true\"");
    html_body.push_str(" style=\"flex: 1; padding: 0.5rem;\"");
    html_body.push('>');
    html_body.push_str("</input>");
    html_body.push_str("<button");
    html_body.push_str(" type=\"submit\"");
    html_body.push_str(
        " style=\"padding: 0.5rem 1rem; background: #4facfe; border: none; color: white;\"",
    );
    html_body.push('>');
    html_body.push_str("Add");
    html_body.push_str("</button>");
    html_body.push_str("</form>");
    html_body.push_str("<ul");
    html_body.push_str(" style=\"list-style: none; padding: 0;\"");
    html_body.push('>');
    html_body.push_str("<n:for");
    html_body.push_str(" item=\"todo\"");
    html_body.push_str(" in=\"todos\"");
    html_body.push('>');
    html_body.push_str("<li");
    html_body.push_str(
        " style=\"background: #222; padding: 1rem; margin-bottom: 0.5rem; border-radius: 4px;\"",
    );
    html_body.push('>');
    html_body.push_str("<span");
    html_body.push_str(" class=\"id\"");
    html_body.push('>');
    html_body.push('#');
    html_body.push_str("<n:text");
    html_body.push_str(" value=\"todo.id\"");
    html_body.push('>');
    html_body.push_str("</n:text>");
    html_body.push_str("</span>");
    html_body.push_str("<strong");
    html_body.push_str(" class=\"text\"");
    html_body.push('>');
    html_body.push_str("<n:text");
    html_body.push_str(" value=\"todo.text\"");
    html_body.push('>');
    html_body.push_str("</n:text>");
    html_body.push_str("</strong>");
    html_body.push_str("</li>");
    html_body.push_str("</n:for>");
    html_body.push_str("</ul>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/\"");
    html_body.push_str(" style=\"color: #4facfe;\"");
    html_body.push('>');
    html_body.push_str("&larr; Back to Recipes");
    html_body.push_str("</a>");
    html_body.push_str("</div>");
    html_body.push_str("<link");
    html_body.push_str(" rel=\"stylesheet\"");
    html_body.push_str(" href=\"/static/css/style-75a7cec0c43ec1a.css\"");
    html_body.push('>');
    html_body.push_str("</link>");
    html_body.push_str("</main>");
    html_body.push_str("<footer");
    html_body.push_str(" style=\"border-top: 1px solid #333; padding: 1rem; text-align: center; color: #666; font-size: 0.8rem;\"");
    html_body.push('>');
    html_body.push_str("Built with Nucleus V3\n        ");
    html_body.push_str("</footer>");
    html_body.push_str("</div>");
    html_body.push_str("</body></html>");

    axum::response::Html(html_body).into_response()
}

#[allow(non_snake_case, unreachable_code, unused_variables)]
async fn handle_login(
    headers: axum::http::HeaderMap,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    let mut html_body = String::new();
    html_body.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Recipes</title><meta name=\"description\" content=\"Built with Nucleus\"></head><body>");
    html_body.push_str("<div");
    html_body.push_str(" class=\"app-layout\"");
    html_body.push_str(" style=\"display: flex; flex-direction: column; min-height: 100vh; font-family: system-ui, sans-serif; background: #111; color: white;\"");
    html_body.push('>');
    html_body.push_str("<nav");
    html_body.push_str(" style=\"border-bottom: 1px solid #333; padding: 1rem; display: flex; gap: 1rem; align-items: center;\"");
    html_body.push('>');
    html_body.push_str("<span");
    html_body.push_str(" style=\"font-weight: bold; font-size: 1.2rem;\"");
    html_body.push('>');
    html_body.push_str("🍳 Nucleus Recipes");
    html_body.push_str("</span>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Home");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/todo\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Todo");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/counter\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Counter");
    html_body.push_str("</a>");
    html_body.push_str("</nav>");
    html_body.push_str("<main");
    html_body.push_str(" style=\"flex: 1; padding: 2rem;\"");
    html_body.push('>');
    html_body.push_str("<div");
    html_body.push_str(" style=\"font-family: system-ui; max-width: 400px; margin: 4rem auto; padding: 2rem; border: 1px solid #333; border-radius: 8px; background: #111; color: white;\"");
    html_body.push('>');
    html_body.push_str("<h1");
    html_body.push_str(" style=\"text-align: center;\"");
    html_body.push('>');
    html_body.push_str("🔐 Login");
    html_body.push_str("</h1>");
    html_body.push_str("<form");
    html_body.push_str(" action=\"/login\"");
    html_body.push_str(" method=\"get\"");
    html_body.push_str(" style=\"display: flex; flex-direction: column; gap: 1rem;\"");
    html_body.push('>');
    html_body.push_str("<input");
    html_body.push_str(" name=\"user\"");
    html_body.push_str(" placeholder=\"Username\"");
    html_body.push_str(" required=\"true\"");
    html_body.push_str(" style=\"padding: 0.8rem; border-radius: 4px; border: 1px solid #444; background: #222; color: white;\"");
    html_body.push('>');
    html_body.push_str("</input>");
    html_body.push_str("<button");
    html_body.push_str(" type=\"submit\"");
    html_body.push_str(" style=\"padding: 0.8rem; background: #4facfe; color: white; border: none; border-radius: 4px; font-weight: bold; cursor: pointer;\"");
    html_body.push('>');
    html_body.push_str("Sign In");
    html_body.push_str("</button>");
    html_body.push_str("</form>");
    html_body.push_str("</div>");
    html_body.push_str("</main>");
    html_body.push_str("<footer");
    html_body.push_str(" style=\"border-top: 1px solid #333; padding: 1rem; text-align: center; color: #666; font-size: 0.8rem;\"");
    html_body.push('>');
    html_body.push_str("Built with Nucleus V3\n        ");
    html_body.push_str("</footer>");
    html_body.push_str("</div>");
    html_body.push_str("</body></html>");

    axum::response::Html(html_body).into_response()
}

#[allow(non_snake_case, unreachable_code, unused_variables)]
async fn handle_index(
    headers: axum::http::HeaderMap,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    let mut html_body = String::with_capacity(30000);
    html_body.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Recipes</title><meta name=\"description\" content=\"Built with Nucleus\"></head><body>");
    html_body.push_str("<div");
    html_body.push_str(" class=\"app-layout\"");
    html_body.push_str(" style=\"display: flex; flex-direction: column; min-height: 100vh; font-family: system-ui, sans-serif; background: #111; color: white;\"");
    html_body.push('>');
    html_body.push_str("<nav");
    html_body.push_str(" style=\"border-bottom: 1px solid #333; padding: 1rem; display: flex; gap: 1rem; align-items: center;\"");
    html_body.push('>');
    html_body.push_str("<span");
    html_body.push_str(" style=\"font-weight: bold; font-size: 1.2rem;\"");
    html_body.push('>');
    html_body.push_str("🍳 Nucleus Recipes");
    html_body.push_str("</span>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Home");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/todo\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Todo");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/counter\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Counter");
    html_body.push_str("</a>");
    html_body.push_str("</nav>");
    html_body.push_str("<main");
    html_body.push_str(" style=\"flex: 1; padding: 2rem;\"");
    html_body.push('>');
    html_body.push_str("<div");
    html_body.push_str(" class=\"container\"");
    html_body.push('>');
    html_body.push_str("<h1");
    html_body.push('>');
    html_body.push_str("🍳 Nucleus Recipes");
    html_body.push_str("</h1>");
    html_body.push_str("<p");
    html_body.push('>');
    html_body.push_str("A collection of examples demonstrating Nucleus capabilities.");
    html_body.push_str("</p>");
    html_body.push_str("<div");
    html_body.push_str(" class=\"grid\"");
    html_body.push('>');
    html_body.push_str("<a");
    html_body.push_str(" href=\"/hello\"");
    html_body.push_str(" class=\"card\"");
    html_body.push('>');
    html_body.push_str("<h2");
    html_body.push('>');
    html_body.push_str("👋 Hello World");
    html_body.push_str("</h2>");
    html_body.push_str("<p");
    html_body.push('>');
    html_body.push_str("Basic routing and Title rendering.");
    html_body.push_str("</p>");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/counter\"");
    html_body.push_str(" class=\"card\"");
    html_body.push('>');
    html_body.push_str("<h2");
    html_body.push('>');
    html_body.push_str("🔢 Counter");
    html_body.push_str("</h2>");
    html_body.push_str("<p");
    html_body.push('>');
    html_body.push_str("Client-side interactivity (WASM).");
    html_body.push_str("</p>");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/todo\"");
    html_body.push_str(" class=\"card\"");
    html_body.push('>');
    html_body.push_str("<h2");
    html_body.push('>');
    html_body.push_str("✅ Todo App");
    html_body.push_str("</h2>");
    html_body.push_str("<p");
    html_body.push('>');
    html_body.push_str("Database (SQLite) and Form handling.");
    html_body.push_str("</p>");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/auth/login\"");
    html_body.push_str(" class=\"card\"");
    html_body.push('>');
    html_body.push_str("<h2");
    html_body.push('>');
    html_body.push_str("🔐 Authentication");
    html_body.push_str("</h2>");
    html_body.push_str("<p");
    html_body.push('>');
    html_body.push_str("Guards and Sessions.");
    html_body.push_str("</p>");
    html_body.push_str("</a>");
    html_body.push_str("</div>");
    html_body.push_str("<link");
    html_body.push_str(" rel=\"stylesheet\"");
    html_body.push_str(" href=\"/static/css/style-bf6ac5547bf247d9.css\"");
    html_body.push('>');
    html_body.push_str("</link>");
    html_body.push_str("</div>");
    html_body.push_str("</main>");
    html_body.push_str("<footer");
    html_body.push_str(" style=\"border-top: 1px solid #333; padding: 1rem; text-align: center; color: #666; font-size: 0.8rem;\"");
    html_body.push('>');
    html_body.push_str("Built with Nucleus V3\n        ");
    html_body.push_str("</footer>");
    html_body.push_str("</div>");
    html_body.push_str("</body></html>");

    axum::response::Html(html_body).into_response()
}

#[allow(non_snake_case, unreachable_code, unused_variables)]
async fn handle_island_test(
    headers: axum::http::HeaderMap,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    let mut html_body = String::new();
    html_body.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Recipes</title><meta name=\"description\" content=\"Built with Nucleus\"></head><body>");
    html_body.push_str("<div");
    html_body.push_str(" class=\"app-layout\"");
    html_body.push_str(" style=\"display: flex; flex-direction: column; min-height: 100vh; font-family: system-ui, sans-serif; background: #111; color: white;\"");
    html_body.push('>');
    html_body.push_str("<nav");
    html_body.push_str(" style=\"border-bottom: 1px solid #333; padding: 1rem; display: flex; gap: 1rem; align-items: center;\"");
    html_body.push('>');
    html_body.push_str("<span");
    html_body.push_str(" style=\"font-weight: bold; font-size: 1.2rem;\"");
    html_body.push('>');
    html_body.push_str("🍳 Nucleus Recipes");
    html_body.push_str("</span>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Home");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/todo\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Todo");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/counter\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Counter");
    html_body.push_str("</a>");
    html_body.push_str("</nav>");
    html_body.push_str("<main");
    html_body.push_str(" style=\"flex: 1; padding: 2rem;\"");
    html_body.push('>');
    html_body.push_str("<div");
    html_body.push_str(" style=\"padding: 2rem;\"");
    html_body.push('>');
    html_body.push_str("<h1");
    html_body.push('>');
    html_body.push_str("🏝\u{fe0f} Islands Architecture");
    html_body.push_str("</h1>");
    html_body.push_str("<p");
    html_body.push('>');
    html_body.push_str("Below is a counter rendered as an island component.");
    html_body.push_str("</p>");
    html_body.push_str("<div");
    html_body.push_str(
        " style=\"height: 150vh; background: #222; margin: 2rem 0; padding: 1rem; color: #666;\"",
    );
    html_body.push('>');
    html_body.push_str("Scroll down for lazy island...\n        ");
    html_body.push_str("</div>");
    html_body.push_str("</div>");
    html_body.push_str("</main>");
    html_body.push_str("<footer");
    html_body.push_str(" style=\"border-top: 1px solid #333; padding: 1rem; text-align: center; color: #666; font-size: 0.8rem;\"");
    html_body.push('>');
    html_body.push_str("Built with Nucleus V3\n        ");
    html_body.push_str("</footer>");
    html_body.push_str("</div>");
    html_body.push_str("</body></html>");

    axum::response::Html(html_body).into_response()
}

#[allow(non_snake_case)]
async fn handle_counter_widget() -> impl IntoResponse {
    Html("No view found in src/views/components/counter_widget.ncl")
}
#[allow(non_snake_case, unreachable_code, unused_variables)]
async fn handle_ts_test(
    headers: axum::http::HeaderMap,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    let mut html_body = String::new();
    html_body.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Recipes</title><meta name=\"description\" content=\"Built with Nucleus\"></head><body>");
    html_body.push_str("<div");
    html_body.push_str(" class=\"app-layout\"");
    html_body.push_str(" style=\"display: flex; flex-direction: column; min-height: 100vh; font-family: system-ui, sans-serif; background: #111; color: white;\"");
    html_body.push('>');
    html_body.push_str("<nav");
    html_body.push_str(" style=\"border-bottom: 1px solid #333; padding: 1rem; display: flex; gap: 1rem; align-items: center;\"");
    html_body.push('>');
    html_body.push_str("<span");
    html_body.push_str(" style=\"font-weight: bold; font-size: 1.2rem;\"");
    html_body.push('>');
    html_body.push_str("🍳 Nucleus Recipes");
    html_body.push_str("</span>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Home");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/todo\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Todo");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/counter\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Counter");
    html_body.push_str("</a>");
    html_body.push_str("</nav>");
    html_body.push_str("<main");
    html_body.push_str(" style=\"flex: 1; padding: 2rem;\"");
    html_body.push('>');
    html_body.push_str("<div");
    html_body.push_str(" class=\"container\"");
    html_body.push('>');
    html_body.push_str("<h1");
    html_body.push('>');
    html_body.push_str("TypeScript Integration Test");
    html_body.push_str("</h1>");
    html_body.push_str("<p");
    html_body.push('>');
    html_body.push_str("Check the console for a message from Lodash.");
    html_body.push_str("</p>");
    html_body.push_str("<div");
    html_body.push_str(" id=\"ts-output\"");
    html_body.push('>');
    html_body.push_str("Waiting for TS...");
    html_body.push_str("</div>");
    html_body.push_str("</div>");
    html_body.push_str("<script");
    html_body.push_str(" src=\"/static/js/ts_test-44a4cd6f22b37d59.js\"");
    html_body.push_str(" type=\"module\"");
    html_body.push_str(" defer=\"true\"");
    html_body.push('>');
    html_body.push_str("</script>");
    html_body.push_str("<link");
    html_body.push_str(" rel=\"stylesheet\"");
    html_body.push_str(" href=\"/static/css/style-568f2a01c17cc4ab.css\"");
    html_body.push('>');
    html_body.push_str("</link>");
    html_body.push_str("</main>");
    html_body.push_str("<footer");
    html_body.push_str(" style=\"border-top: 1px solid #333; padding: 1rem; text-align: center; color: #666; font-size: 0.8rem;\"");
    html_body.push('>');
    html_body.push_str("Built with Nucleus V3\n        ");
    html_body.push_str("</footer>");
    html_body.push_str("</div>");
    html_body.push_str("</body></html>");

    axum::response::Html(html_body).into_response()
}

#[allow(non_snake_case, unreachable_code, unused_variables)]
async fn handle_action_success(
    headers: axum::http::HeaderMap,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    let mut html_body = String::new();
    html_body.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Recipes</title><meta name=\"description\" content=\"Built with Nucleus\"></head><body>");
    html_body.push_str("<div");
    html_body.push_str(" class=\"app-layout\"");
    html_body.push_str(" style=\"display: flex; flex-direction: column; min-height: 100vh; font-family: system-ui, sans-serif; background: #111; color: white;\"");
    html_body.push('>');
    html_body.push_str("<nav");
    html_body.push_str(" style=\"border-bottom: 1px solid #333; padding: 1rem; display: flex; gap: 1rem; align-items: center;\"");
    html_body.push('>');
    html_body.push_str("<span");
    html_body.push_str(" style=\"font-weight: bold; font-size: 1.2rem;\"");
    html_body.push('>');
    html_body.push_str("🍳 Nucleus Recipes");
    html_body.push_str("</span>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Home");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/todo\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Todo");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/counter\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Counter");
    html_body.push_str("</a>");
    html_body.push_str("</nav>");
    html_body.push_str("<main");
    html_body.push_str(" style=\"flex: 1; padding: 2rem;\"");
    html_body.push('>');
    html_body.push_str("<div");
    html_body.push_str(" style=\"padding: 2rem; color: #4caf50;\"");
    html_body.push('>');
    html_body.push_str("<h1");
    html_body.push('>');
    html_body.push_str("✅ Action Successful");
    html_body.push_str("</h1>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/action_test\"");
    html_body.push('>');
    html_body.push_str("Back");
    html_body.push_str("</a>");
    html_body.push_str("</div>");
    html_body.push_str("</main>");
    html_body.push_str("<footer");
    html_body.push_str(" style=\"border-top: 1px solid #333; padding: 1rem; text-align: center; color: #666; font-size: 0.8rem;\"");
    html_body.push('>');
    html_body.push_str("Built with Nucleus V3\n        ");
    html_body.push_str("</footer>");
    html_body.push_str("</div>");
    html_body.push_str("</body></html>");

    axum::response::Html(html_body).into_response()
}

#[allow(non_snake_case, unreachable_code, unused_variables)]
async fn handle_action_test(
    headers: axum::http::HeaderMap,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    println!("LOADER");

    let mut html_body = String::new();
    html_body.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Recipes</title><meta name=\"description\" content=\"Built with Nucleus\"></head><body>");
    html_body.push_str("<div");
    html_body.push_str(" class=\"app-layout\"");
    html_body.push_str(" style=\"display: flex; flex-direction: column; min-height: 100vh; font-family: system-ui, sans-serif; background: #111; color: white;\"");
    html_body.push('>');
    html_body.push_str("<nav");
    html_body.push_str(" style=\"border-bottom: 1px solid #333; padding: 1rem; display: flex; gap: 1rem; align-items: center;\"");
    html_body.push('>');
    html_body.push_str("<span");
    html_body.push_str(" style=\"font-weight: bold; font-size: 1.2rem;\"");
    html_body.push('>');
    html_body.push_str("🍳 Nucleus Recipes");
    html_body.push_str("</span>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Home");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/todo\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Todo");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/counter\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Counter");
    html_body.push_str("</a>");
    html_body.push_str("</nav>");
    html_body.push_str("<main");
    html_body.push_str(" style=\"flex: 1; padding: 2rem;\"");
    html_body.push('>');
    html_body.push_str("<div");
    html_body.push_str(" style=\"padding: 2rem;\"");
    html_body.push('>');
    html_body.push_str("<h1");
    html_body.push('>');
    html_body.push_str("Action Test");
    html_body.push_str("</h1>");
    html_body.push_str("<form");
    html_body.push_str(" method=\"POST\"");
    html_body.push_str(" action=\"/action_test\"");
    html_body.push('>');
    html_body.push_str("<input");
    html_body.push_str(" type=\"text\"");
    html_body.push_str(" name=\"username\"");
    html_body.push_str(" placeholder=\"Enter username\"");
    html_body.push('>');
    html_body.push_str("</input>");
    html_body.push_str("<button");
    html_body.push_str(" type=\"submit\"");
    html_body.push('>');
    html_body.push_str("Submit");
    html_body.push_str("</button>");
    html_body.push_str("</form>");
    html_body.push_str("</div>");
    html_body.push_str("</main>");
    html_body.push_str("<footer");
    html_body.push_str(" style=\"border-top: 1px solid #333; padding: 1rem; text-align: center; color: #666; font-size: 0.8rem;\"");
    html_body.push('>');
    html_body.push_str("Built with Nucleus V3\n        ");
    html_body.push_str("</footer>");
    html_body.push_str("</div>");
    html_body.push_str("</body></html>");

    axum::response::Html(html_body).into_response()
}

#[allow(non_snake_case, unreachable_code, unused_variables)]
async fn handle_action_action_test(
    headers: axum::http::HeaderMap,
    Form(params): Form<std::collections::HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    println!("ACTION");
    println!("Params: {:?}", params);
    return axum::response::Redirect::to("/action_success").into_response();

    axum::response::Html("Action Completed").into_response()
}

#[allow(non_snake_case, unreachable_code, unused_variables)]
async fn handle_hello(
    headers: axum::http::HeaderMap,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl axum::response::IntoResponse {
    let mut html_body = String::new();
    html_body.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Recipes</title><meta name=\"description\" content=\"Built with Nucleus\"></head><body>");
    html_body.push_str("<div");
    html_body.push_str(" class=\"app-layout\"");
    html_body.push_str(" style=\"display: flex; flex-direction: column; min-height: 100vh; font-family: system-ui, sans-serif; background: #111; color: white;\"");
    html_body.push('>');
    html_body.push_str("<nav");
    html_body.push_str(" style=\"border-bottom: 1px solid #333; padding: 1rem; display: flex; gap: 1rem; align-items: center;\"");
    html_body.push('>');
    html_body.push_str("<span");
    html_body.push_str(" style=\"font-weight: bold; font-size: 1.2rem;\"");
    html_body.push('>');
    html_body.push_str("🍳 Nucleus Recipes");
    html_body.push_str("</span>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Home");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/todo\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Todo");
    html_body.push_str("</a>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/counter\"");
    html_body.push_str(" style=\"color: #aaa; text-decoration: none;\"");
    html_body.push('>');
    html_body.push_str("Counter");
    html_body.push_str("</a>");
    html_body.push_str("</nav>");
    html_body.push_str("<main");
    html_body.push_str(" style=\"flex: 1; padding: 2rem;\"");
    html_body.push('>');
    html_body.push_str("<div");
    html_body.push_str(" style=\"text-align: center; margin-top: 5rem;\"");
    html_body.push('>');
    html_body.push_str("<h1");
    html_body.push('>');
    html_body.push_str("👋 Hello World!");
    html_body.push_str("</h1>");
    html_body.push_str("<p");
    html_body.push('>');
    html_body.push_str("This page was rendered on the server with 0ms latency.");
    html_body.push_str("</p>");
    html_body.push_str("<a");
    html_body.push_str(" href=\"/\"");
    html_body.push_str(" style=\"color: #4facfe;\"");
    html_body.push('>');
    html_body.push_str("&larr; Back to Recipes");
    html_body.push_str("</a>");
    html_body.push_str("</div>");
    html_body.push_str("</main>");
    html_body.push_str("<footer");
    html_body.push_str(" style=\"border-top: 1px solid #333; padding: 1rem; text-align: center; color: #666; font-size: 0.8rem;\"");
    html_body.push('>');
    html_body.push_str("Built with Nucleus V3\n        ");
    html_body.push_str("</footer>");
    html_body.push_str("</div>");
    html_body.push_str("</body></html>");

    axum::response::Html(html_body).into_response()
}

// Middleware Module Support

// Logic Module Support
#[path = "../services/mod.rs"]
pub mod services;

// Models Module Support

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    // Initialize Database
    if std::path::Path::new("nucleus.db").exists() {
        match nucleus_std::photon::init_db("sqlite:nucleus.db").await {
            Ok(_) => println!("✅ Database initialized"),
            Err(e) => eprintln!("❌ Database initialization failed: {}", e),
        }
    }

    // Static Router with Zero-Allocation Assets
    let app = Router::new()
        .route("/counter", get(handle_counter))
        .route("/todo", get(handle_todo))
        .route("/login", get(handle_login))
        .route("/", get(handle_index))
        .route("/island_test", get(handle_island_test))
        .route("/counter_widget", get(handle_counter_widget))
        .route("/ts_test", get(handle_ts_test))
        .route("/action_success", get(handle_action_success))
        .route(
            "/action_test",
            get(handle_action_test).post(handle_action_action_test),
        )
        .route("/hello", get(handle_hello))
        .nest_service("/pkg", ServeDir::new("static/pkg"))
        .layer(CompressionLayer::new().br(true).gzip(true));

    // Auto-Inject Middleware if `src/middleware.rs` exists
    let app = app;

    println!("⚛️  Nucleus Hyperdrive (AOT) running on http://0.0.0.0:3002");
    let listener = TcpListener::bind("0.0.0.0:3002").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
