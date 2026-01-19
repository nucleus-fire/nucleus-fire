use axum::{extract::Request, middleware::Next, response::Response};
use bumpalo::Bump;
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Clone)]
pub struct Arena(pub Arc<Mutex<Bump>>);

impl Arena {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(Bump::new())))
    }
}

impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn arena_middleware(mut req: Request, next: Next) -> Response {
    let arena = Arena::new();
    req.extensions_mut().insert(arena);

    // Process request
    // Process request
    // Arena is dropped here automatically
    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_arena_allocator_injection() {
        // Build a dummy service with the middleware
        let app = axum::Router::new()
            .route(
                "/",
                axum::routing::get(|req: Request<Body>| async move {
                    // Check if Arena is present
                    if req.extensions().get::<Arena>().is_some() {
                        StatusCode::OK
                    } else {
                        StatusCode::INTERNAL_SERVER_ERROR
                    }
                }),
            )
            .layer(axum::middleware::from_fn(arena_middleware));

        // Send a request
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
