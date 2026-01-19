use nucleus_std::server;
use nucleus_std::errors::Result;

#[server]
pub async fn login(user: String) -> Result<bool> {
    // Simplified Auth Logic (would normally check DB/Hash)
    Ok(!user.is_empty())
}
