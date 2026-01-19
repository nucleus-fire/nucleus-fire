use crate::runtime::{AppState, NucleusRuntime};
use ahash::AHashMap;
use arc_swap::ArcSwap;
#[cfg(test)]
use axum::{body::Body, http::Request};
use nucleus_std::stream::StreamHub;
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn test_runtime_404_compliance() {
    // Setup State
    let now = chrono::Utc::now().to_rfc2822().replace("+0000", "GMT");
    let state = AppState {
        routes: Arc::new(ArcSwap::from_pointee(AHashMap::new())),
        stream_handler: None,
        stream_hub: Arc::new(StreamHub::new()),
        tx: tokio::sync::broadcast::channel(1).0,
        is_dev: false,
        cached_date: Arc::new(ArcSwap::from_pointee(now)),
    };

    // Create Router (using extracted method)
    let app = NucleusRuntime::make_router(state);
    // Note: Real fortress middleware is tested in fortress.rs,
    // effectively this test covers the routing logic itself.

    let res = app
        .oneshot(
            Request::builder()
                .uri("/non-existent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), axum::http::StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_runtime_benchmark_route() {
    // Test "Hello World" benchmark path logic
    let mut routes = AHashMap::new();
    routes.insert(
        "plaintext".to_string(),
        axum::body::Bytes::from("Hello World"),
    );

    let now = chrono::Utc::now().to_rfc2822().replace("+0000", "GMT");
    let state = AppState {
        routes: Arc::new(ArcSwap::from_pointee(routes)),
        stream_handler: None,
        stream_hub: Arc::new(StreamHub::new()),
        tx: tokio::sync::broadcast::channel(1).0,
        is_dev: false,
        cached_date: Arc::new(ArcSwap::from_pointee(now)),
    };

    let app = NucleusRuntime::make_router(state);

    let res = app
        .oneshot(
            Request::builder()
                .uri("/plaintext")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), axum::http::StatusCode::OK);

    // Check Content-Type (Set explicitly in handler)
    let headers = res.headers();
    assert_eq!(headers["content-type"], "text/plain; charset=utf-8");

    // Read body
    let body = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .unwrap();
    assert_eq!(body, "Hello, World!");
}

#[tokio::test]
async fn test_runtime_mime_types() {
    // Test CSS Content-Type
    let mut routes = AHashMap::new();
    routes.insert(
        "styles.css".to_string(),
        axum::body::Bytes::from("body { color: red; }"),
    );

    let now = chrono::Utc::now().to_rfc2822().replace("+0000", "GMT");
    let state = AppState {
        routes: Arc::new(ArcSwap::from_pointee(routes)),
        stream_handler: None,
        stream_hub: Arc::new(StreamHub::new()),
        tx: tokio::sync::broadcast::channel(1).0,
        is_dev: true, // Verification in Dev Mode too
        cached_date: Arc::new(ArcSwap::from_pointee(now)),
    };

    let app = NucleusRuntime::make_router(state);

    let res = app
        .oneshot(
            Request::builder()
                .uri("/styles.css")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), axum::http::StatusCode::OK);
    let headers = res.headers();
    assert_eq!(headers["content-type"], "text/css");
}
