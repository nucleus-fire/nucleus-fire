use nucleus_std::axum::{
    routing::get,
    Router,
    response::{Html, IntoResponse},
    extract::Path,
};
use std::fs;
use pulldown_cmark::{Parser, Options, html};

pub fn router() -> Router {
    Router::new()
        .route("/posts/:slug", get(show_post))
        .route("/sitemap.xml", get(sitemap))
}

async fn show_post(Path(slug): Path<String>) -> impl IntoResponse {
    let filepath = format!("content/{}.md", slug);
    
    match fs::read_to_string(&filepath) {
        Ok(markdown_input) => {
            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            let parser = Parser::new_ext(&markdown_input, options);

            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);

            // Wrap in layout (simplified for demo)
            let page = format!(r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>{} | Minimalist Blog</title>
                    <link href="/assets/styles.css" rel="stylesheet">
                    <link rel="preconnect" href="https://fonts.googleapis.com">
                    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
                    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600;700&family=Merriweather:ital,wght@0,300;0,700;1,300&display=swap" rel="stylesheet">
                </head>
                <body class="bg-white font-serif antialiased text-gray-900 selection:bg-gray-100 selection:text-gray-900">
                     <header class="py-10 max-w-4xl mx-auto px-6 flex justify-between items-center border-b border-gray-100">
                        <a href="/" class="text-2xl font-bold font-sans tracking-tight hover:text-gray-600 transition">
                            My Thoughts
                        </a>
                        <nav class="space-x-6 text-sm font-sans font-medium">
                            <a href="/" class="hover:text-gray-600 transition">Home</a>
                            <a href="/about" class="hover:text-gray-600 transition">About</a>
                        </nav>
                    </header>
                    <main class="max-w-4xl mx-auto px-6 py-12 min-h-[60vh]">
                        <article class="prose lg:prose-xl">
                            {}
                        </article>
                    </main>
                     <footer class="py-10 text-center text-sm font-sans text-gray-500 border-t border-gray-100 mt-12">
                        &copy; 2026 Minimalist Blog using Nucleus
                    </footer>
                </body>
                </html>
            "#, slug, html_output);

            Html(page)
        },
        Err(_) => Html("<h1>Post not found</h1>".to_string())
    }
}

async fn sitemap() -> impl IntoResponse {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
   <url>
      <loc>http://127.0.0.1:3000/</loc>
      <changefreq>daily</changefreq>
      <priority>1.0</priority>
   </url>
   <url>
      <loc>http://127.0.0.1:3000/about</loc>
      <priority>0.8</priority>
   </url>
   <url>
      <loc>http://127.0.0.1:3000/posts/hello</loc>
      <changefreq>monthly</changefreq>
      <priority>0.8</priority>
   </url>
</urlset>"#;
    (
        [(nucleus_std::axum::http::header::CONTENT_TYPE, "application/xml")],
        xml
    )
}

