use nucleus_std::errors::Result;
use nucleus_std::server;

#[server]
pub async fn login(user: String) -> Result<bool> {
    // Simplified Auth Logic (would normally check DB/Hash)
    Ok(!user.is_empty())
}
