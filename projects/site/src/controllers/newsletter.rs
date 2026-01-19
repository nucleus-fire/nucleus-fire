use axum::{
    extract::Json,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use nucleus_std::{photon::Model, sqlx::Row};
use regex::Regex;
use serde::{Deserialize, Serialize};

// 1. Data Model: Subscriber
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subscriber {
    pub id: i64,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct PreviewRequest {
    pub mjml: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdminData {
    pub subscribers: Vec<Subscriber>,
    pub templates: Vec<EmailTemplate>,
}

pub async fn fetch_admin_data() -> AdminData {
    let subscribers = list_subscribers().await;
    let templates = list_templates().await;
    AdminData {
        subscribers,
        templates,
    }
}

// Manual implementation to avoid macro/crate version potential conflicts
impl<'r> nucleus_std::sqlx::FromRow<'r, nucleus_std::sqlx::sqlite::SqliteRow> for Subscriber {
    fn from_row(
        row: &'r nucleus_std::sqlx::sqlite::SqliteRow,
    ) -> Result<Self, nucleus_std::sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            email: row.try_get("email")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

// Implement Photon Model
nucleus_std::impl_model!(Subscriber, "subscribers");

// 2. Data Model: EmailTemplate
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailTemplate {
    pub id: i64,
    pub name: String,
    pub subject: String,
    pub body: String, // HTML content
    pub created_at: String,
}

impl<'r> nucleus_std::sqlx::FromRow<'r, nucleus_std::sqlx::sqlite::SqliteRow> for EmailTemplate {
    fn from_row(
        row: &'r nucleus_std::sqlx::sqlite::SqliteRow,
    ) -> Result<Self, nucleus_std::sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            subject: row.try_get("subject")?,
            body: row.try_get("body")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

nucleus_std::impl_model!(EmailTemplate, "email_templates");

// 3. Request Payload
#[derive(Debug, Deserialize)]
pub struct SubscribeRequest {
    pub email: String,
}

// 4. Controller Logic
pub async fn subscribe(Json(payload): Json<SubscribeRequest>) -> impl IntoResponse {
    let email = payload.email.trim();

    // Regex Validation
    let email_regex = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();

    if !email_regex.is_match(email) {
        return (StatusCode::BAD_REQUEST, "Invalid email address").into_response();
    }

    // Check for Duplicate
    let existing = Subscriber::query()
        .r#where("email", email)
        .first::<Subscriber>()
        .await;

    match existing {
        Ok(Some(_)) => return (StatusCode::CONFLICT, "Email already subscribed").into_response(),
        Ok(None) => {} // Not found, proceed
        Err(e) => {
            eprintln!("Database check error: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database check failed").into_response();
        }
    }

    // Insert
    let insert_result = Subscriber::create().value("email", email).execute().await;

    match insert_result {
        Ok(_) => (StatusCode::OK, "Subscribed successfully").into_response(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

// 5. Router
pub fn router() -> Router {
    Router::new()
        .route("/api/newsletter", post(subscribe))
        .route("/api/newsletter/export", get(export_subscribers))
        .route("/api/newsletter/preview", post(preview_mjml))
}

pub async fn preview_mjml(Json(payload): Json<PreviewRequest>) -> impl IntoResponse {
    match compile_mjml(&payload.mjml) {
        Ok(html) => (StatusCode::OK, html).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

// 6. Admin Helpers (logic moved here for testing)
pub async fn list_subscribers() -> Vec<Subscriber> {
    Subscriber::query()
        .order_by("created_at", "DESC")
        .all::<Subscriber>()
        .await
        .unwrap_or_default()
}

pub async fn list_templates() -> Vec<EmailTemplate> {
    EmailTemplate::query()
        .order_by("created_at", "DESC")
        .all::<EmailTemplate>()
        .await
        .unwrap_or_default()
}

pub async fn delete_subscriber(id: i64) -> Result<(), String> {
    // Model::delete_by_id is available if Model trait is in scope
    use nucleus_std::photon::Model;
    Subscriber::delete_by_id(id)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

pub async fn create_template(name: String, subject: String, body: String) -> Result<(), String> {
    EmailTemplate::create()
        .value("name", name)
        .value("subject", subject)
        .value("body", body)
        .execute()
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

pub async fn delete_template(id: i64) -> Result<(), String> {
    use nucleus_std::photon::Model;
    EmailTemplate::delete_by_id(id)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

pub async fn export_subscribers() -> impl IntoResponse {
    let subscribers = list_subscribers().await;
    let mut wtr = csv::Writer::from_writer(vec![]);

    // Header
    let _ = wtr.write_record(["ID", "Email", "Joined At"]);

    for sub in subscribers {
        let _ = wtr.write_record(&[sub.id.to_string(), sub.email, sub.created_at]);
    }

    let data = wtr.into_inner().unwrap_or_default();
    let file_name = format!(
        "subscribers_{}.csv",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/csv".parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{}\"", file_name)
            .parse()
            .unwrap(),
    );

    (headers, data)
}

// Helper to compile MJML to HTML using mrml
fn compile_mjml(content: &str) -> Result<String, String> {
    // Parse
    let root = mrml::parse(content).map_err(|e| e.to_string())?;
    // Options
    let opts = mrml::prelude::render::RenderOptions::default();
    // Render (accessing the inner element as suggested by compiler)
    root.element.render(&opts).map_err(|e| e.to_string())
}

pub async fn broadcast_template(template_id: i64) -> Result<String, String> {
    let tmpl: Result<Option<EmailTemplate>, nucleus_std::sqlx::Error> = EmailTemplate::query()
        .r#where("id", template_id)
        .first::<EmailTemplate>()
        .await;

    match tmpl {
        Ok(Some(t)) => {
            let subscribers = list_subscribers().await;
            let sub_count = subscribers.len();

            let sub_list = subscribers;

            // Compile MJML before sending!
            // If the template is raw HTML, we might need a fallback or assume MJML entirely now.
            // Requirement says "integrate this or something similar", implying migration.
            // For safety, let's try compile, if it fails (not mjml), use as is?
            // Better to enforce MJML for "Exemplary UX".
            let tmpl_body = match compile_mjml(&t.body) {
                Ok(html) => html,
                Err(e) => return Err(format!("MJML Compilation Failed: {}", e)),
            };

            let tmpl_name = t.name.clone();
            let tmpl_subject = t.subject.clone();

            tokio::spawn(async move {
                let mut pm = nucleus_std::postman::Postman::from_env();
                pm.register_template(&tmpl_name, &tmpl_body);

                let mut sent = 0;
                let mut failed = 0;

                for sub in sub_list {
                    let mut vars = std::collections::HashMap::new();
                    vars.insert("email".to_string(), sub.email.clone());
                    // Support unsubscribe link var replacement in MJML too if needed,
                    // usually standard handlebar syntax `{{email}}` passes through MJML fine if not stripped.

                    match pm
                        .send_template(&sub.email, &tmpl_subject, &tmpl_name, &vars)
                        .await
                    {
                        Ok(_) => sent += 1,
                        Err(e) => {
                            eprintln!("Failed to send to {}: {}", sub.email, e);
                            failed += 1;
                        }
                    }
                }
                println!(
                    "Background Broadcast Complete: {} sent, {} failed.",
                    sent, failed
                );
            });

            Ok(format!("Broadcast started for {} subscribers.", sub_count))
        }
        Ok(None) => Err("Template not found".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

// 7. Backend Tests
#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use nucleus_std::photon;
    use tower::ServiceExt;

    async fn setup_test_db() {
        if !photon::is_db_initialized() {
            let _ = photon::init_db("sqlite::memory:").await;
        }

        let pool = photon::db();
        if let Some(sqlite) = pool.as_sqlite() {
            let sql_subs = r#"
            CREATE TABLE IF NOT EXISTS subscribers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT NOT NULL UNIQUE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            "#;
            let _ = nucleus_std::sqlx::query(sql_subs).execute(sqlite).await;

            let sql_tmpl = r#"
            CREATE TABLE IF NOT EXISTS email_templates (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                subject TEXT NOT NULL,
                body TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            "#;
            let _ = nucleus_std::sqlx::query(sql_tmpl).execute(sqlite).await;
        }
    }

    #[tokio::test]
    async fn test_subscribe_flow() {
        setup_test_db().await;
        let app = router();

        // 1. Success
        let email = format!(
            "flow_{}@test.com",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros()
        );
        let body = format!(r#"{{"email": "{}"}}"#, email);

        let res = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/newsletter")
                    .header("content-type", "application/json")
                    .body(Body::from(body.clone()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // 2. Duplicate
        let res_dup = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/newsletter")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res_dup.status(), StatusCode::CONFLICT);

        // 3. Invalid
        let res_inv = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/newsletter")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"email": "not-an-email"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res_inv.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_admin_helpers() {
        setup_test_db().await;

        // Templates - Use MJML now!
        let mjml = r#"<mjml><mj-body><mj-section><mj-column><mj-text>Hello</mj-text></mj-column></mj-section></mj-body></mjml>"#;
        create_template("T1".into(), "S1".into(), mjml.into())
            .await
            .unwrap();
        let templates = list_templates().await;
        assert!(!templates.is_empty());
        let t1 = &templates[0];
        assert_eq!(t1.name, "T1");

        // Broadcast
        let _ = Subscriber::create()
            .value("email", "admin_helper_test@test.com")
            .execute()
            .await;

        let count = broadcast_template(t1.id).await.unwrap();
        assert!(count.contains("Broadcast started"));

        // Delete
        delete_template(t1.id).await.unwrap();
        let _ = delete_subscriber(1).await;
    }

    #[tokio::test]
    async fn test_fetch_admin_data() {
        setup_test_db().await;
        let data = super::fetch_admin_data().await;
        // Just verify we got a result struct back
        println!(
            "Fetched {} subscribers and {} templates",
            data.subscribers.len(),
            data.templates.len()
        );
    }

    #[test]
    fn test_mjml_compilation() {
        let content = r#"<mjml><mj-body><mj-section><mj-column><mj-text>Hello World</mj-text></mj-column></mj-section></mj-body></mjml>"#;
        let compiled = super::compile_mjml(content);
        assert!(compiled.is_ok());
        let html = compiled.unwrap();
        assert!(html.contains("<!doctype html>"));
        assert!(html.contains("Hello World"));
    }
}
