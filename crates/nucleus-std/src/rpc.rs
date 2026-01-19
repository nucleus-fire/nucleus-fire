use crate::errors::{NucleusError, Result};
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
pub async fn call<T, R>(name: &str, args: T) -> Result<R>
where
    T: Serialize,
    R: for<'de> Deserialize<'de>,
{
    // In a real implementation this would get the configured API base
    // For now we assume relative path /_rpc
    let url = format!("/_rpc/{}", name);

    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .json(&args)
        .send()
        .await
        .map_err(|e| NucleusError::NetworkError(e))?;

    let text = res
        .text()
        .await
        .map_err(|e| NucleusError::NetworkError(e))?;

    // Handle serialized error from server if needed, or parse result
    serde_json::from_str(&text)
        .map_err(|e| NucleusError::ValidationError(format!("RPC Parse Error: {}", e)))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn call<T, R>(_name: &str, _args: T) -> Result<R>
where
    T: Serialize,
    R: for<'de> Deserialize<'de>,
{
    // This should never be reached in server mode if the macro works correctly
    // The server implementation replaces this call with the actual body
    // But trait bounds might require it to exist
    Err(NucleusError::InternalError(
        "RPC call attempted on server side".to_string(),
    ))
}
