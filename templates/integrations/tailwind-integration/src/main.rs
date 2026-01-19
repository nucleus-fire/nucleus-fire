use axum::{response::Html, routing::get, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    println!("🎨 Tailwind Demo running on http://0.0.0.0:3000");

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Nucleus + Tailwind</title>
    <link href="/static/output.css" rel="stylesheet">
</head>
<body class="bg-gray-900 text-white flex items-center justify-center min-h-screen">
    <div class="max-w-md w-full bg-gray-800 rounded-xl shadow-2xl overflow-hidden transform hover:scale-105 transition-transform duration-300">
        <div class="h-2 bg-gradient-to-r from-blue-500 to-purple-600"></div>
        <div class="p-8">
            <div class="uppercase tracking-wide text-sm text-blue-400 font-semibold">Integration Demo</div>
            <h1 class="block mt-1 text-2xl leading-tight font-bold text-white">Nucleus + Tailwind CSS</h1>
            <p class="mt-2 text-gray-400">
                This card is styled with utility classes directly in the Rust source code.
                The Tailwind CLI watches your Rust files and generates the CSS.
            </p>
            <div class="mt-6">
                <button class="bg-blue-600 hover:bg-blue-500 text-white font-bold py-2 px-4 rounded-full transition-colors duration-200">
                    It Works!
                </button>
            </div>
        </div>
    </div>
</body>
</html>
    "#,
    )
}
