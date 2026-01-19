<n:script>
    use nucleus_std::pulse::Pulse;
    use nucleus_std::postman::{Postman, Email};

    // Example: Welcome Email on Signup
    // We don't want to block the HTTP response on SMTP, so we use Pulse.
    async fn register_user(email: String) {
        // ... create user logic ...

        // Enqueue background job
        Pulse::enqueue(move || {
            async move {
                let mail = Email {
                    to: email,
                    subject: "Welcome!".to_string(),
                    body: "Thanks for joining.".to_string(),
                };
                
                // Retry logic could go here
                if let Err(e) = Postman::send(mail) {
                    println!("Failed to send mail: {}", e);
                }
            }
        }).await.expect("Queue full");
    }
</n:script>
