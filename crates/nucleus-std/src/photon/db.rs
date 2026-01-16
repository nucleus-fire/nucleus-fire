//! Multi-Database Support for Photon
//!
//! Provides a unified interface for PostgreSQL, MySQL, and SQLite databases.
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::photon::db::{Database, init_db};
//!
//! // Auto-detect from URL
//! init_db("postgres://user:pass@localhost/mydb").await?;
//! init_db("mysql://user:pass@localhost/mydb").await?;
//! init_db("sqlite://./data.db").await?;
//! ```

use std::sync::OnceLock;

// ═══════════════════════════════════════════════════════════════════════════
// DATABASE TYPE DETECTION
// ═══════════════════════════════════════════════════════════════════════════

/// Supported database backends
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseType {
    Postgres,
    MySql,
    Sqlite,
}

impl DatabaseType {
    /// Detect database type from connection URL
    pub fn from_url(url: &str) -> Option<Self> {
        if url.starts_with("postgres://") || url.starts_with("postgresql://") {
            Some(Self::Postgres)
        } else if url.starts_with("mysql://") || url.starts_with("mariadb://") {
            Some(Self::MySql)
        } else if url.starts_with("sqlite://") || url.starts_with("sqlite:") {
            Some(Self::Sqlite)
        } else {
            None
        }
    }
    
    /// Get the placeholder style for this database
    pub fn placeholder(&self, index: usize) -> String {
        match self {
            Self::Postgres => format!("${}", index),
            Self::MySql | Self::Sqlite => "?".to_string(),
        }
    }
    
    /// Get the display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Postgres => "PostgreSQL",
            Self::MySql => "MySQL",
            Self::Sqlite => "SQLite",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// DATABASE POOL
// ═══════════════════════════════════════════════════════════════════════════

/// A database connection pool supporting multiple backends
#[derive(Clone)]
pub enum DatabasePool {
    Postgres(sqlx::PgPool),
    MySql(sqlx::MySqlPool),
    Sqlite(sqlx::SqlitePool),
}

impl std::fmt::Debug for DatabasePool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Postgres(_) => write!(f, "DatabasePool::Postgres"),
            Self::MySql(_) => write!(f, "DatabasePool::MySql"),
            Self::Sqlite(_) => write!(f, "DatabasePool::Sqlite"),
        }
    }
}

impl DatabasePool {
    /// Connect to a database using the URL
    ///
    /// The database type is auto-detected from the URL prefix:
    /// - `postgres://` or `postgresql://` → PostgreSQL
    /// - `mysql://` or `mariadb://` → MySQL
    /// - `sqlite://` or `sqlite:` → SQLite
    pub async fn connect(url: &str) -> Result<Self, sqlx::Error> {
        let db_type = DatabaseType::from_url(url)
            .ok_or_else(|| sqlx::Error::Configuration(
                format!("Unknown database URL format: {}", url).into()
            ))?;
        
        match db_type {
            DatabaseType::Postgres => {
                let pool = sqlx::PgPool::connect(url).await?;
                Ok(Self::Postgres(pool))
            }
            DatabaseType::MySql => {
                let pool = sqlx::MySqlPool::connect(url).await?;
                Ok(Self::MySql(pool))
            }
            DatabaseType::Sqlite => {
                use sqlx::sqlite::SqliteConnectOptions;
                use std::str::FromStr;
                let options = SqliteConnectOptions::from_str(url)
                    .map_err(|e| sqlx::Error::Configuration(format!("Invalid SQLite URL: {}", e).into()))?
                    .create_if_missing(true);
                let pool = sqlx::SqlitePool::connect_with(options).await?;
                Ok(Self::Sqlite(pool))
            }
        }
    }
    
    /// Get the database type
    pub fn db_type(&self) -> DatabaseType {
        match self {
            Self::Postgres(_) => DatabaseType::Postgres,
            Self::MySql(_) => DatabaseType::MySql,
            Self::Sqlite(_) => DatabaseType::Sqlite,
        }
    }
    
    /// Get placeholder for query parameter
    pub fn placeholder(&self, index: usize) -> String {
        self.db_type().placeholder(index)
    }
    
    /// Get the SQLite pool (if this is a SQLite connection)
    pub fn as_sqlite(&self) -> Option<&sqlx::SqlitePool> {
        match self {
            Self::Sqlite(pool) => Some(pool),
            _ => None,
        }
    }
    
    /// Get the PostgreSQL pool (if this is a Postgres connection)
    pub fn as_postgres(&self) -> Option<&sqlx::PgPool> {
        match self {
            Self::Postgres(pool) => Some(pool),
            _ => None,
        }
    }
    
    /// Get the MySQL pool (if this is a MySQL connection)
    pub fn as_mysql(&self) -> Option<&sqlx::MySqlPool> {
        match self {
            Self::MySql(pool) => Some(pool),
            _ => None,
        }
    }
    
    /// Close the connection pool
    pub async fn close(&self) {
        match self {
            Self::Postgres(pool) => pool.close().await,
            Self::MySql(pool) => pool.close().await,
            Self::Sqlite(pool) => pool.close().await,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// GLOBAL POOL
// ═══════════════════════════════════════════════════════════════════════════

static GLOBAL_DB: OnceLock<DatabasePool> = OnceLock::new();

/// Initialize the global database connection
///
/// # Example
///
/// ```rust,ignore
/// use nucleus_std::photon::db::init_db;
///
/// init_db("sqlite://./data.db").await?;
/// ```
pub async fn init_db(url: &str) -> Result<(), sqlx::Error> {
    let pool = DatabasePool::connect(url).await?;
    GLOBAL_DB.set(pool).map_err(|_| {
        sqlx::Error::Configuration("Database already initialized".into())
    })?;
    Ok(())
}

/// Get reference to the global database pool
///
/// # Panics
///
/// Panics if the database has not been initialized with `init_db()`.
pub fn db() -> &'static DatabasePool {
    GLOBAL_DB.get().expect("Database not initialized. Call init_db() first.")
}

/// Check if the database has been initialized
pub fn is_db_initialized() -> bool {
    GLOBAL_DB.get().is_some()
}

// ═══════════════════════════════════════════════════════════════════════════
// QUERY VALUE (for multi-DB bindings)
// ═══════════════════════════════════════════════════════════════════════════

/// A value that can be bound to a query parameter
#[derive(Debug, Clone)]
pub enum QueryValue {
    Text(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
    Bytes(Vec<u8>),
}

impl From<&str> for QueryValue {
    fn from(v: &str) -> Self { QueryValue::Text(v.to_string()) }
}

impl From<String> for QueryValue {
    fn from(v: String) -> Self { QueryValue::Text(v) }
}

impl From<i64> for QueryValue {
    fn from(v: i64) -> Self { QueryValue::Int(v) }
}

impl From<i32> for QueryValue {
    fn from(v: i32) -> Self { QueryValue::Int(v as i64) }
}

impl From<u32> for QueryValue {
    fn from(v: u32) -> Self { QueryValue::Int(v as i64) }
}

impl From<f64> for QueryValue {
    fn from(v: f64) -> Self { QueryValue::Float(v) }
}

impl From<f32> for QueryValue {
    fn from(v: f32) -> Self { QueryValue::Float(v as f64) }
}

impl From<bool> for QueryValue {
    fn from(v: bool) -> Self { QueryValue::Bool(v) }
}

impl<T> From<Option<T>> for QueryValue
where
    T: Into<QueryValue>,
{
    fn from(v: Option<T>) -> Self {
        match v {
            Some(val) => val.into(),
            None => QueryValue::Null,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_database_type_from_url() {
        assert_eq!(DatabaseType::from_url("postgres://localhost/db"), Some(DatabaseType::Postgres));
        assert_eq!(DatabaseType::from_url("postgresql://localhost/db"), Some(DatabaseType::Postgres));
        assert_eq!(DatabaseType::from_url("mysql://localhost/db"), Some(DatabaseType::MySql));
        assert_eq!(DatabaseType::from_url("mariadb://localhost/db"), Some(DatabaseType::MySql));
        assert_eq!(DatabaseType::from_url("sqlite://./data.db"), Some(DatabaseType::Sqlite));
        assert_eq!(DatabaseType::from_url("sqlite:./data.db"), Some(DatabaseType::Sqlite));
        assert_eq!(DatabaseType::from_url("unknown://localhost"), None);
    }
    
    #[test]
    fn test_placeholder_style() {
        assert_eq!(DatabaseType::Postgres.placeholder(1), "$1");
        assert_eq!(DatabaseType::Postgres.placeholder(5), "$5");
        assert_eq!(DatabaseType::MySql.placeholder(1), "?");
        assert_eq!(DatabaseType::MySql.placeholder(5), "?");
        assert_eq!(DatabaseType::Sqlite.placeholder(1), "?");
    }
    
    #[test]
    fn test_database_type_name() {
        assert_eq!(DatabaseType::Postgres.name(), "PostgreSQL");
        assert_eq!(DatabaseType::MySql.name(), "MySQL");
        assert_eq!(DatabaseType::Sqlite.name(), "SQLite");
    }
    
    #[test]
    fn test_query_value_from() {
        assert!(matches!(QueryValue::from("hello"), QueryValue::Text(_)));
        assert!(matches!(QueryValue::from(42i64), QueryValue::Int(42)));
        assert!(matches!(QueryValue::from(42i32), QueryValue::Int(42)));
        assert!(matches!(QueryValue::from(std::f64::consts::PI), QueryValue::Float(_)));
        assert!(matches!(QueryValue::from(true), QueryValue::Bool(true)));
        assert!(matches!(QueryValue::from(Option::<i64>::None), QueryValue::Null));
        assert!(matches!(QueryValue::from(Some(42i64)), QueryValue::Int(42)));
    }
}
