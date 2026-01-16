#![allow(unused_imports)]
        use axum::{response::{Html, IntoResponse}, routing::get, extract::{Query, Form}, Router};
        use tower_http::services::ServeDir;
        use tokio::net::TcpListener;
        use serde::{Serialize, Deserialize};
        
        // --- Static Assets (Zero-Allocation) ---
        #[allow(non_snake_case, unreachable_code, unused_variables)]
async fn handle_index(headers: axum::http::HeaderMap, Query(params): Query<std::collections::HashMap<String, String>>) -> impl axum::response::IntoResponse {

    
    let mut html_body = String::new();
html_body.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Nucleus Media Center</title><meta name=\"description\" content=\"Built with Nucleus\"></head><body>");
html_body.push_str("<link");
html_body.push_str(" rel=\"stylesheet\"");
html_body.push_str(" href=\"/static/css/style.css\"");
html_body.push('>');
html_body.push_str("</link>");
html_body.push_str("<style");
html_body.push_str(" critical=\"true\"");
html_body.push('>');
html_body.push_str("\n        /* Example Critical CSS (Above-the-Fold Optimization) */\n        :root { --bg: #141414; --primary: #e50914; }\n        body { background-color: var(--bg); margin: 0; padding: 0; min-height: 100vh; }\n        .hero { min-height: 85vh; background: #000; }\n    ");
html_body.push_str("</style>");
html_body.push_str("<!-- Begin Include: src/views/components/navbar.ncl -->");
html_body.push_str("<div");
html_body.push_str(" class=\"navbar\"");
html_body.push_str(" id=\"navbar\"");
html_body.push('>');
html_body.push_str("<div");
html_body.push_str(" class=\"logo\"");
html_body.push('>');
html_body.push_str("NUCLEUS PLUS");
html_body.push_str("</div>");
html_body.push_str("<div");
html_body.push_str(" class=\"nav-links\"");
html_body.push('>');
html_body.push_str("<div");
html_body.push_str(" class=\"nav-link active\"");
html_body.push('>');
html_body.push_str("Home");
html_body.push_str("</div>");
html_body.push_str("<div");
html_body.push_str(" class=\"nav-link\"");
html_body.push('>');
html_body.push_str("Series");
html_body.push_str("</div>");
html_body.push_str("<div");
html_body.push_str(" class=\"nav-link\"");
html_body.push('>');
html_body.push_str("Films");
html_body.push_str("</div>");
html_body.push_str("<div");
html_body.push_str(" class=\"nav-link\"");
html_body.push('>');
html_body.push_str("New & Popular");
html_body.push_str("</div>");
html_body.push_str("<div");
html_body.push_str(" class=\"nav-link\"");
html_body.push('>');
html_body.push_str("My List");
html_body.push_str("</div>");
html_body.push_str("</div>");
html_body.push_str("</div>");
html_body.push_str("<!-- End Include: src/views/components/navbar.ncl -->");
html_body.push_str("<!-- Hero -->");
html_body.push_str("<div");
html_body.push_str(" class=\"hero\"");
html_body.push_str(" id=\"hero\"");
html_body.push('>');
html_body.push_str("<div");
html_body.push_str(" class=\"hero-content\"");
html_body.push('>');
html_body.push_str("<h1");
html_body.push_str(" class=\"hero-title\"");
html_body.push_str(" id=\"hero-title\"");
html_body.push('>');
html_body.push_str("Loading...");
html_body.push_str("</h1>");
html_body.push_str("<div");
html_body.push_str(" class=\"hero-meta\"");
html_body.push('>');
html_body.push_str("<span");
html_body.push_str(" class=\"match-score\"");
html_body.push('>');
html_body.push_str("98% Match");
html_body.push_str("</span>");
html_body.push_str("<span");
html_body.push_str(" id=\"hero-year\"");
html_body.push('>');
html_body.push_str("2024");
html_body.push_str("</span>");
html_body.push_str("<span");
html_body.push_str(" style=\"border: 1px solid #fff; padding: 0 5px; font-size: 0.8rem;\"");
html_body.push('>');
html_body.push_str("HD");
html_body.push_str("</span>");
html_body.push_str("</div>");
html_body.push_str("<p");
html_body.push_str(" class=\"hero-desc\"");
html_body.push_str(" id=\"hero-desc\"");
html_body.push('>');
html_body.push_str("Discover the amazing world of Nucleus media.");
html_body.push_str("</p>");
html_body.push_str("<div");
html_body.push_str(" style=\"display: flex;\"");
html_body.push('>');
html_body.push_str("<button");
html_body.push_str(" class=\"btn-hero btn-play\"");
html_body.push_str(" onclick=\"playHero()\"");
html_body.push('>');
html_body.push_str("▶ Play");
html_body.push_str("</button>");
html_body.push_str("<button");
html_body.push_str(" class=\"btn-hero btn-info\"");
html_body.push_str(" onclick=\"openModalHero()\"");
html_body.push('>');
html_body.push_str("ℹ More Info");
html_body.push_str("</button>");
html_body.push_str("<div");
html_body.push_str(" style=\"margin-left: 1rem;\"");
html_body.push('>');
html_body.push_str("<!-- Begin Include: src/views/components/LikeButton.ncl -->");
html_body.push_str("<n:view");
html_body.push_str(" title=\"Like Button\"");
html_body.push('>');
html_body.push_str("<div");
html_body.push_str(" id=\"like-hero-btn\"");
html_body.push_str(" class=\"like-button\"");
html_body.push('>');
html_body.push_str("<button");
html_body.push_str(" id=\"btn-hero-like\"");
html_body.push('>');
html_body.push_str("❤\u{fe0f} ");
html_body.push_str("<span");
html_body.push_str(" id=\"count-hero-like\"");
html_body.push('>');
html_body.push('0');
html_body.push_str("</span>");
html_body.push_str("Likes\n        ");
html_body.push_str("</button>");
html_body.push_str("</div>");
html_body.push_str("<!-- Interactive Island Logic (Client-Side Rust) -->");
html_body.push_str("<style");
html_body.push('>');
html_body.push_str("\n        .like-button button {\n            background: rgba(255,255,255,0.1);\n            border: 1px solid rgba(255,255,255,0.2);\n            color: white;\n            padding: 0.5rem 1rem;\n            border-radius: 20px;\n            cursor: pointer;\n            transition: all 0.2s;\n            display: flex;\n            align-items: center;\n            gap: 0.5rem;\n        }\n        .like-button button:hover {\n            background: rgba(255,255,255,0.2);\n            transform: scale(1.05);\n        }\n    ");
html_body.push_str("</style>");
html_body.push_str("</n:view>");
html_body.push_str("<!-- End Include: src/views/components/LikeButton.ncl -->");
html_body.push_str("</div>");
html_body.push_str("</div>");
html_body.push_str("</div>");
html_body.push_str("</div>");
html_body.push_str("<div");
html_body.push_str(" class=\"content-container\"");
html_body.push_str(" id=\"rows-container\"");
html_body.push('>');
html_body.push_str("<!-- Rows injected by JS -->");
html_body.push_str("</div>");
html_body.push_str("<!-- Detail Modal -->");
html_body.push_str("<div");
html_body.push_str(" class=\"modal-overlay\"");
html_body.push_str(" id=\"detail-modal\"");
html_body.push_str(" onclick=\"closeModal(event)\"");
html_body.push('>');
html_body.push_str("<div");
html_body.push_str(" class=\"modal\"");
html_body.push_str(" id=\"modal-box\"");
html_body.push('>');
html_body.push_str("<button");
html_body.push_str(" class=\"modal-close\"");
html_body.push_str(" onclick=\"closeModalForce()\"");
html_body.push('>');
html_body.push('✕');
html_body.push_str("</button>");
html_body.push_str("<div");
html_body.push_str(" class=\"modal-hero\"");
html_body.push_str(" id=\"modal-hero-img\"");
html_body.push('>');
html_body.push_str("</div>");
html_body.push_str("<div");
html_body.push_str(" class=\"modal-content\"");
html_body.push('>');
html_body.push_str("<h2");
html_body.push_str(" id=\"modal-title\"");
html_body.push_str(" style=\"font-size: 2rem; margin-bottom: 0.5rem;\"");
html_body.push('>');
html_body.push_str("Title");
html_body.push_str("</h2>");
html_body.push_str("<div");
html_body.push_str(" class=\"hero-meta\"");
html_body.push_str(" style=\"margin-bottom: 1rem;\"");
html_body.push('>');
html_body.push_str("<span");
html_body.push_str(" class=\"match-score\"");
html_body.push('>');
html_body.push_str("New");
html_body.push_str("</span>");
html_body.push_str("<span");
html_body.push_str(" id=\"modal-year\"");
html_body.push('>');
html_body.push_str("2024");
html_body.push_str("</span>");
html_body.push_str("<span");
html_body.push_str(" id=\"modal-duration\"");
html_body.push('>');
html_body.push_str("2h 10m");
html_body.push_str("</span>");
html_body.push_str("</div>");
html_body.push_str("<p");
html_body.push_str(" id=\"modal-plot\"");
html_body.push_str(" style=\"font-size: 1.1rem; line-height: 1.5; color: #ccc;\"");
html_body.push('>');
html_body.push_str("Plot details...");
html_body.push_str("</p>");
html_body.push_str("<button");
html_body.push_str(" class=\"btn-hero btn-play\"");
html_body.push_str(" style=\"margin-top: 2rem;\"");
html_body.push_str(" onclick=\"playModal()\"");
html_body.push('>');
html_body.push_str("▶ Play");
html_body.push_str("</button>");
html_body.push_str("</div>");
html_body.push_str("</div>");
html_body.push_str("</div>");
html_body.push_str("<!-- Fullscreen Video -->");
html_body.push_str("<div");
html_body.push_str(" id=\"video-fs\"");
html_body.push('>');
html_body.push_str("<div");
html_body.push_str(" class=\"close-fs\"");
html_body.push_str(" onclick=\"closeVideo()\"");
html_body.push('>');
html_body.push('✕');
html_body.push_str("</div>");
html_body.push_str("<video");
html_body.push_str(" id=\"fs-player\"");
html_body.push_str(" controls=\"true\"");
html_body.push('>');
html_body.push_str("</video>");
html_body.push_str("</div>");
html_body.push_str("<script");
html_body.push_str(" src=\"/static/js/app.js\"");
html_body.push('>');
html_body.push_str("");
html_body.push_str("</script>");
html_body.push_str("<script");
html_body.push_str(" src=\"/static/js/script-f9f74d930f99c12e.js\"");
html_body.push_str(" defer=\"true\"");
html_body.push('>');
html_body.push_str("</script>");
html_body.push_str("<script");
html_body.push_str(" src=\"/static/js/index-ce6c66ed5734cb9d.js\"");
html_body.push_str(" type=\"module\"");
html_body.push_str(" defer=\"true\"");
html_body.push('>');
html_body.push_str("</script>");
html_body.push_str("</body></html>");

    axum::response::Html(html_body).into_response()
}

async fn handle_navbar() -> impl IntoResponse { Html("No view found in src/views/components/navbar.ncl") }#[allow(non_snake_case, unreachable_code, unused_variables)]
async fn handle_LikeButton(headers: axum::http::HeaderMap, Query(params): Query<std::collections::HashMap<String, String>>) -> impl axum::response::IntoResponse {

    
    let mut html_body = String::new();
html_body.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Like Button</title><meta name=\"description\" content=\"Built with Nucleus\"></head><body>");
html_body.push_str("<div");
html_body.push_str(" id=\"like-hero-btn\"");
html_body.push_str(" class=\"like-button\"");
html_body.push('>');
html_body.push_str("<button");
html_body.push_str(" id=\"btn-hero-like\"");
html_body.push('>');
html_body.push_str("❤\u{fe0f} ");
html_body.push_str("<span");
html_body.push_str(" id=\"count-hero-like\"");
html_body.push('>');
html_body.push('0');
html_body.push_str("</span>");
html_body.push_str("Likes\n        ");
html_body.push_str("</button>");
html_body.push_str("</div>");
html_body.push_str("<!-- Interactive Island Logic (Client-Side Rust) -->");
html_body.push_str("");
html_body.push_str("<link");
html_body.push_str(" rel=\"stylesheet\"");
html_body.push_str(" href=\"/static/css/style-519a4e2423c2bd48.css\"");
html_body.push('>');
html_body.push_str("</link>");
html_body.push_str("</body></html>");

    axum::response::Html(html_body).into_response()
}



        // Middleware Module Support
        

        // Logic Module Support
        #[path = "../services/mod.rs"] pub mod services;

        // Models Module Support
        #[path = "../models/mod.rs"] pub mod models;

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
                .route("/", get(handle_index)).route("/navbar", get(handle_navbar)).route("/LikeButton", get(handle_LikeButton))
                .nest_service("/pkg", ServeDir::new("static/pkg"))
                .merge(services::api::routes());
                
            // Auto-Inject Middleware if `src/middleware.rs` exists
            let app = app;
            

            println!("⚛️  Nucleus Hyperdrive (AOT) running on http://0.0.0.0:3002");
            let listener = TcpListener::bind("0.0.0.0:3002").await.unwrap();
            axum::serve(listener, app).await.unwrap();
        }
        