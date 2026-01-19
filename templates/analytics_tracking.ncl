<n:script>
    use nucleus_std::beacon::{Beacon, AnalyticsProvider, AnalyticsEvent};
    use std::collections::HashMap;
    use serde_json::json;

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 1: Page Analytics
    // ═══════════════════════════════════════════════════════════════════════════

    async fn track_page_view(path: &str, user_id: Option<&str>) {
        // Initialize from environment or with specific provider
        let beacon = Beacon::from_env();

        // Identify user if logged in
        if let Some(uid) = user_id {
            beacon.identify(uid, HashMap::new());
        }

        // Track page view
        beacon.page_view(path).await;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 2: E-Commerce Events
    // ═══════════════════════════════════════════════════════════════════════════

    async fn track_purchase(product_id: &str, price: f64, currency: &str) {
        let beacon = Beacon::from_env();

        beacon.track("purchase", json!({
            "product_id": product_id,
            "price": price,
            "currency": currency,
            "timestamp": chrono::Utc::now().to_rfc3339()
        })).await;
    }

    async fn track_add_to_cart(product_id: &str, quantity: u32) {
        let beacon = Beacon::from_env();

        beacon.track("add_to_cart", json!({
            "product_id": product_id,
            "quantity": quantity
        })).await;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 3: User Behavior Tracking
    // ═══════════════════════════════════════════════════════════════════════════

    async fn track_signup(user_id: &str, plan: &str, source: &str) {
        let beacon = Beacon::from_env();

        // Identify the new user
        let mut traits = HashMap::new();
        traits.insert("plan".to_string(), json!(plan));
        traits.insert("signup_source".to_string(), json!(source));
        beacon.identify(user_id, traits);

        // Track signup event
        beacon.track("signup", json!({
            "plan": plan,
            "source": source
        })).await;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 4: Error Tracking
    // ═══════════════════════════════════════════════════════════════════════════

    async fn track_error(error_message: &str, error_code: Option<i32>, stack: Option<&str>) {
        let beacon = Beacon::from_env();

        beacon.error(error_message, Some(json!({
            "code": error_code,
            "stack": stack,
            "page": "/current-page"
        }))).await;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 5: A/B Testing Events
    // ═══════════════════════════════════════════════════════════════════════════

    async fn track_experiment(experiment_name: &str, variant: &str, converted: bool) {
        let beacon = Beacon::from_env();

        beacon.track("experiment", json!({
            "name": experiment_name,
            "variant": variant,
            "converted": converted
        })).await;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 6: Middleware Integration
    // ═══════════════════════════════════════════════════════════════════════════

    // Use in Axum/web framework middleware
    async fn analytics_middleware(path: String, user_id: Option<String>) {
        let beacon = Beacon::from_env();
        
        if let Some(uid) = &user_id {
            beacon.identify(uid, HashMap::new());
        }
        
        beacon.page_view(&path).await;
    }
</n:script>
