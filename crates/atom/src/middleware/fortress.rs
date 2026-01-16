use axum::{
    extract::Request,
    middleware::Next,
    response::IntoResponse,
};
use std::panic::AssertUnwindSafe;
use futures::FutureExt;

use std::sync::OnceLock;
use axum::http::{HeaderName, HeaderValue};

static SECURITY_HEADERS: OnceLock<Vec<(HeaderName, HeaderValue)>> = OnceLock::new();

fn get_security_headers() -> &'static Vec<(HeaderName, HeaderValue)> {
    SECURITY_HEADERS.get_or_init(|| {
        use nucleus_std::fortress::{Fortress, CspConfig};
        // FUTURE: Support loading CSP from nucleus.config
        // Currently uses safe defaults.
        let csp = CspConfig::default();
        let headers = Fortress::security_headers(&csp);
        
        headers.into_iter().filter_map(|(k, v)| {
            let name = HeaderName::from_bytes(k.as_bytes()).ok()?;
            let value = HeaderValue::from_str(&v).ok()?;
            Some((name, value))
        }).collect()
    })
}

pub async fn fortress(req: Request, next: Next) -> impl IntoResponse {
    let result = AssertUnwindSafe(next.run(req)).catch_unwind().await;

    match result {
        Ok(mut response) => {
            // Security Hardening: Inject Pre-Computed Headers (Zero Allocation)
            let security_headers = get_security_headers();
            let headers = response.headers_mut();
            
            for (k, v) in security_headers {
                headers.insert(k.clone(), v.clone());
            }

            // Framework Signature (X-Powered-By)
            // Unless explicitly omitted in config, we proudly sign our work.
            if !nucleus_std::GLOBAL_CONFIG.server.omit_signature {
                if let Ok(val) = HeaderValue::from_str("Nucleus") {
                    headers.insert(HeaderName::from_static("x-powered-by"), val);
                }
            }

            // Web Standards Compliance (Global)
            // 1. Date Header (RFC 7231)
            if !headers.contains_key(axum::http::header::DATE) {
                 // Fast Path: Calculate on demand if cached isn't available easily here without complexity.
                 // Actually, let's use the cached one if we can access it, but middleware is separate.
                 // For now, simple chrono is fast enough for middleware or we can use lazy_static
                 let now = chrono::Utc::now().to_rfc2822().replace("+0000", "GMT");
                 if let Ok(val) = HeaderValue::from_str(&now) {
                     headers.insert(axum::http::header::DATE, val);
                 }
            }

            // 2. Content-Type (Sniffing prevention)
            // If body is present but no content-type, default to application/octet-stream or specific text
            // Benchmarks need text/plain but that's handled in the handler. 
            // We just ensure we don't send without *some* type if possible, or let browser sniff (bad).
            // Actually, for this task, the handler level fix is better for content-type.
            
            response
        },
        Err(err) => {
            let error_msg = if let Some(s) = err.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = err.downcast_ref::<String>() {
                s.clone()
            } else {
                format!("{:?}", err)
            };

            eprintln!("💥 ORBITAL BREACH DETECTED (Panic): {}", error_msg);
            
            // Render interactive error overlay
            let template = include_str!("../error_overlay.html");
            let html = template.replace("{{error_message}}", &error_msg);

            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                [("Content-Type", "text/html")],
                html,
            )
                .into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, routing::get, Router};
    use tower::ServiceExt; // for oneshot
    use axum::http::Request;

    #[tokio::test]
    async fn test_security_headers_injection() {
        let app = Router::new()
            .route("/", get(|| async { "Hello" }))
            .layer(axum::middleware::from_fn(fortress));

        let res = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let headers = res.headers();
        
        // Assert Headers Exist
        assert!(headers.contains_key("content-security-policy"));
        assert!(headers.contains_key("x-frame-options"));
        
        // Assert Values
        assert_eq!(headers["x-frame-options"], "DENY");
        assert!(headers["content-security-policy"].to_str().unwrap().contains("script-src 'self'"));
        
        // Assert Framework Signature (Default: Present)
        assert_eq!(headers["x-powered-by"], "Nucleus");
    }

    #[tokio::test]
    async fn test_panic_overlay() {
        let app = Router::new()
            .route("/panic", get(|| async { panic!("Simulated meltdown"); #[allow(unreachable_code)] "Unreachable" }))
            .layer(axum::middleware::from_fn(fortress));

        let res = app
            .oneshot(Request::builder().uri("/panic").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(res.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        
        let headers = res.headers();
        assert_eq!(headers["content-type"], "text/html");
        
        let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        
        // Assert it's our overlay
        assert!(body_str.contains("Orbital Hull Breach"));
        assert!(body_str.contains("Simulated meltdown"));
        assert!(body_str.contains("Nucleus Runtime Panic"));
    }
}
