//! Newsletter service
//!
//! Handles newsletter subscription logic.

use nucleus_std::photon::query::query;

/// Subscribe an email to the newsletter
pub async fn subscribe(email: &str) -> Result<(), String> {
    if email.is_empty() || !email.contains('@') {
        return Err("Invalid email address".to_string());
    }
    
    query("subscribers")
        .insert()
        .value("email", email)
        .execute()
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}
