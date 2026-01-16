use nucleus_std::server;
use nucleus_std::errors::{Result, NucleusError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub secret_token: String,
}

/// This function demonstrates a Server Action.
/// On the client (WASM), the body is stripped and replaced with an RPC call.
/// On the server, the full logic runs, including access to secrets.
#[server]
pub async fn get_secure_user(id: i64) -> Result<User> {
    // 🛡️ Guard Clause illustrating NucleusError
    if id < 1 {
        return Err(NucleusError::ValidationError("User ID must be positive".into()));
    }

    // 🔒 Server-only logic (Environment Variable access)
    // This string literal will NOT exist in the client bundle
    let api_secret = std::env::var("STRIPE_KEY").unwrap_or("dev_secret".to_string());
    
    // Simulate DB fetch
    Ok(User {
        id,
        name: format!("User_{}", id),
        secret_token: api_secret, // We can return secrets explicitly if we want to
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_secure_user_success() {
        let user = get_secure_user(1).await.unwrap();
        assert_eq!(user.name, "User_1");
        assert_eq!(user.secret_token, "dev_secret"); // Default fallback
    }

    #[tokio::test]
    async fn test_get_secure_user_validation() {
        let err = get_secure_user(0).await.unwrap_err();
        match err {
            NucleusError::ValidationError(msg) => assert_eq!(msg, "User ID must be positive"),
            _ => panic!("Expected ValidationError, got {:?}", err),
        }
    }
}
