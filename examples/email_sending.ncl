<n:script>
    use nucleus_std::postman::{Postman, Email, EmailProvider};
    use std::collections::HashMap;

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 1: Basic Email Sending
    // ═══════════════════════════════════════════════════════════════════════════

    async fn send_welcome_email(user_email: &str, user_name: &str) -> Result<(), String> {
        // Initialize from environment (SMTP_* or SES_* vars)
        let postman = Postman::from_env();

        let email = Email::new(
            user_email,
            "Welcome to Our App!",
            &format!("Hi {}, thanks for signing up!", user_name)
        )
        .html(&format!(r#"
            <h1>Welcome, {}!</h1>
            <p>We're excited to have you on board.</p>
            <a href="https://app.example.com/onboarding">Get Started</a>
        "#, user_name));

        let result = postman.send(email).await?;
        println!("Email sent! ID: {}", result.message_id);
        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 2: Templated Transactional Email
    // ═══════════════════════════════════════════════════════════════════════════

    async fn send_order_confirmation(order_id: &str, email: &str, total: &str) -> Result<(), String> {
        let mut postman = Postman::from_env();

        // Register reusable templates
        postman.register_template("order_confirmation", r#"
            Order #{{order_id}} Confirmed!
            
            Total: {{total}}
            
            Thank you for your purchase.
            Track your order: https://shop.example.com/orders/{{order_id}}
        "#);

        let mut vars = HashMap::new();
        vars.insert("order_id".to_string(), order_id.to_string());
        vars.insert("total".to_string(), total.to_string());

        postman.send_template(email, "Order Confirmed", "order_confirmation", &vars).await?;
        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 3: Email with CC/BCC
    // ═══════════════════════════════════════════════════════════════════════════

    async fn send_team_notification(message: &str) -> Result<(), String> {
        let postman = Postman::from_env();

        let email = Email::new("team@example.com", "Team Update", message)
            .cc("manager@example.com")
            .cc("lead@example.com")
            .bcc("archive@example.com")
            .reply_to("no-reply@example.com");

        postman.send(email).await?;
        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 4: Testing with Mock Provider
    // ═══════════════════════════════════════════════════════════════════════════

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn test_email_sending() {
            let postman = Postman::new(EmailProvider::Mock);
            
            let email = Email::new("test@example.com", "Test", "Body");
            let result = postman.send(email).await;
            
            assert!(result.is_ok());
            assert!(result.unwrap().message_id.starts_with("mock_"));
        }
    }
</n:script>
