//! Database service layer
//!
//! Common database operations and helpers.

use nucleus_std::photon::Db;

/// Get a database connection
pub fn db() -> Db {
    Db::default()
}
