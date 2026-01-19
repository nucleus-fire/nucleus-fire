//! Database Migrations
//!
//! Version-controlled schema changes with support for applying, rolling back,
//! and checking migration status.
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::photon::migrations::{run_migrations, migration_status};
//!
//! // Apply all pending migrations
//! let applied = run_migrations("./migrations").await?;
//! println!("Applied {} migrations", applied.len());
//!
//! // Check status
//! let status = migration_status("./migrations").await?;
//! for m in status {
//!     println!("{}: {}", m.name, if m.applied { "✅" } else { "⏳" });
//! }
//! ```

use crate::photon::db::{db, DatabasePool, DatabaseType};
use chrono::{DateTime, Utc};
use std::fs;
use std::path::Path;

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Information about a migration
#[derive(Debug, Clone)]
pub struct MigrationInfo {
    pub name: String,
    pub applied: bool,
    pub applied_at: Option<DateTime<Utc>>,
}

/// Migration error type
#[derive(Debug)]
pub enum MigrationError {
    Io(std::io::Error),
    Sql(sqlx::Error),
    Parse(String),
    NotFound(String),
}

impl std::fmt::Display for MigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::Sql(e) => write!(f, "SQL error: {}", e),
            Self::Parse(msg) => write!(f, "Parse error: {}", msg),
            Self::NotFound(name) => write!(f, "Migration not found: {}", name),
        }
    }
}

impl std::error::Error for MigrationError {}

impl From<std::io::Error> for MigrationError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<sqlx::Error> for MigrationError {
    fn from(e: sqlx::Error) -> Self {
        Self::Sql(e)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MIGRATIONS TABLE
// ═══════════════════════════════════════════════════════════════════════════

const MIGRATIONS_TABLE_SQLITE: &str = r#"
CREATE TABLE IF NOT EXISTS _migrations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
)
"#;

const MIGRATIONS_TABLE_POSTGRES: &str = r#"
CREATE TABLE IF NOT EXISTS _migrations (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    applied_at TIMESTAMPTZ DEFAULT NOW()
)
"#;

const MIGRATIONS_TABLE_MYSQL: &str = r#"
CREATE TABLE IF NOT EXISTS _migrations (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)
"#;

async fn ensure_migrations_table() -> Result<(), MigrationError> {
    let pool = db();
    let sql = match pool.db_type() {
        DatabaseType::Sqlite => MIGRATIONS_TABLE_SQLITE,
        DatabaseType::Postgres => MIGRATIONS_TABLE_POSTGRES,
        DatabaseType::MySql => MIGRATIONS_TABLE_MYSQL,
    };

    match pool {
        DatabasePool::Sqlite(p) => {
            sqlx::query(sql).execute(p).await?;
        }
        DatabasePool::Postgres(p) => {
            sqlx::query(sql).execute(p).await?;
        }
        DatabasePool::MySql(p) => {
            sqlx::query(sql).execute(p).await?;
        }
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// CORE FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Run all pending migrations in the specified directory
///
/// Migrations are applied in alphabetical order by filename.
/// Returns a list of migration names that were applied.
pub async fn run_migrations(dir: &str) -> Result<Vec<String>, MigrationError> {
    ensure_migrations_table().await?;

    let pool = db();
    let migrations = list_migration_files(dir)?;
    let applied = get_applied_migrations().await?;

    let mut newly_applied = Vec::new();

    for (name, path) in migrations {
        if applied.contains(&name) {
            continue;
        }

        // Read and execute migration
        let sql = fs::read_to_string(&path)?;

        // Extract UP migration (everything before -- DOWN)
        let up_sql = extract_up_migration(&sql);

        // Execute
        match pool {
            DatabasePool::Sqlite(p) => {
                sqlx::query(&up_sql).execute(p).await?;
            }
            DatabasePool::Postgres(p) => {
                sqlx::query(&up_sql).execute(p).await?;
            }
            DatabasePool::MySql(p) => {
                sqlx::query(&up_sql).execute(p).await?;
            }
        }

        // Record migration
        let record_sql = "INSERT INTO _migrations (name) VALUES (?)";
        match pool {
            DatabasePool::Sqlite(p) => {
                sqlx::query(record_sql).bind(&name).execute(p).await?;
            }
            DatabasePool::Postgres(p) => {
                sqlx::query("INSERT INTO _migrations (name) VALUES ($1)")
                    .bind(&name)
                    .execute(p)
                    .await?;
            }
            DatabasePool::MySql(p) => {
                sqlx::query(record_sql).bind(&name).execute(p).await?;
            }
        }

        newly_applied.push(name);
    }

    Ok(newly_applied)
}

/// Rollback the last N migrations
pub async fn rollback(n: usize) -> Result<Vec<String>, MigrationError> {
    ensure_migrations_table().await?;

    let pool = db();
    let applied = get_applied_migrations_ordered().await?;

    let to_rollback: Vec<_> = applied.into_iter().rev().take(n).collect();
    let mut rolled_back = Vec::new();

    for (name, _applied_at) in to_rollback {
        // Try to find the migration file to get DOWN migration
        // For now, just remove from tracking (users should handle DOWN manually)
        let delete_sql = "DELETE FROM _migrations WHERE name = ?";
        match pool {
            DatabasePool::Sqlite(p) => {
                sqlx::query(delete_sql).bind(&name).execute(p).await?;
            }
            DatabasePool::Postgres(p) => {
                sqlx::query("DELETE FROM _migrations WHERE name = $1")
                    .bind(&name)
                    .execute(p)
                    .await?;
            }
            DatabasePool::MySql(p) => {
                sqlx::query(delete_sql).bind(&name).execute(p).await?;
            }
        }

        rolled_back.push(name);
    }

    Ok(rolled_back)
}

/// Get status of all migrations
pub async fn migration_status(dir: &str) -> Result<Vec<MigrationInfo>, MigrationError> {
    ensure_migrations_table().await?;

    let migrations = list_migration_files(dir)?;
    let applied_map = get_applied_migrations_map().await?;

    let mut status = Vec::new();

    for (name, _path) in migrations {
        let info = MigrationInfo {
            name: name.clone(),
            applied: applied_map.contains_key(&name),
            applied_at: applied_map.get(&name).cloned(),
        };
        status.push(info);
    }

    Ok(status)
}

/// Create a new migration file
pub fn create_migration(name: &str, dir: &str) -> Result<String, std::io::Error> {
    fs::create_dir_all(dir)?;

    let timestamp = Utc::now().format("%Y%m%d%H%M%S");
    let filename = format!("{}_{}.sql", timestamp, name);
    let filepath = Path::new(dir).join(&filename);

    let template = format!(
        "-- Migration: {}\n-- Created: {}\n\n-- UP\n\n\n-- DOWN\n\n",
        name,
        Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );

    fs::write(&filepath, template)?;

    Ok(filename)
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════════════

fn list_migration_files(dir: &str) -> Result<Vec<(String, std::path::PathBuf)>, MigrationError> {
    let path = Path::new(dir);
    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut migrations = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map(|e| e == "sql").unwrap_or(false) {
            if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                migrations.push((name.to_string(), path));
            }
        }
    }

    migrations.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(migrations)
}

fn extract_up_migration(sql: &str) -> String {
    // Find "-- DOWN" marker and take everything before it
    if let Some(pos) = sql.find("-- DOWN") {
        sql[..pos].trim().to_string()
    } else {
        sql.trim().to_string()
    }
}

async fn get_applied_migrations() -> Result<Vec<String>, MigrationError> {
    let pool = db();
    let sql = "SELECT name FROM _migrations ORDER BY name";

    let names: Vec<(String,)> = match pool {
        DatabasePool::Sqlite(p) => sqlx::query_as(sql).fetch_all(p).await?,
        DatabasePool::Postgres(p) => sqlx::query_as(sql).fetch_all(p).await?,
        DatabasePool::MySql(p) => sqlx::query_as(sql).fetch_all(p).await?,
    };

    Ok(names.into_iter().map(|(n,)| n).collect())
}

async fn get_applied_migrations_ordered() -> Result<Vec<(String, DateTime<Utc>)>, MigrationError> {
    let pool = db();
    let sql = "SELECT name, applied_at FROM _migrations ORDER BY applied_at DESC";

    #[derive(sqlx::FromRow)]
    struct Row {
        name: String,
        applied_at: chrono::DateTime<Utc>,
    }

    let rows: Vec<Row> = match pool {
        DatabasePool::Sqlite(p) => sqlx::query_as(sql).fetch_all(p).await?,
        DatabasePool::Postgres(p) => sqlx::query_as(sql).fetch_all(p).await?,
        DatabasePool::MySql(p) => sqlx::query_as(sql).fetch_all(p).await?,
    };

    Ok(rows.into_iter().map(|r| (r.name, r.applied_at)).collect())
}

async fn get_applied_migrations_map(
) -> Result<std::collections::HashMap<String, DateTime<Utc>>, MigrationError> {
    let applied = get_applied_migrations_ordered().await?;
    Ok(applied.into_iter().collect())
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_up_migration() {
        let sql = "CREATE TABLE users (id INT);\n\n-- DOWN\nDROP TABLE users;";
        assert_eq!(extract_up_migration(sql), "CREATE TABLE users (id INT);");
    }

    #[test]
    fn test_extract_up_migration_no_down() {
        let sql = "CREATE TABLE users (id INT);";
        assert_eq!(extract_up_migration(sql), "CREATE TABLE users (id INT);");
    }

    #[test]
    fn test_extract_up_migration_with_up_marker() {
        let sql = "-- UP\nCREATE TABLE users (id INT);\n\n-- DOWN\nDROP TABLE users;";
        let up = extract_up_migration(sql);
        assert!(up.contains("CREATE TABLE"));
        assert!(!up.contains("DROP TABLE"));
    }

    #[test]
    fn test_extract_up_migration_whitespace() {
        let sql = "   CREATE TABLE foo (id INT);   \n\n-- DOWN\nDROP TABLE foo;";
        let up = extract_up_migration(sql);
        assert_eq!(up, "CREATE TABLE foo (id INT);");
    }

    #[test]
    fn test_create_migration_template() {
        let dir = "/tmp/test_migrations";
        let _ = fs::remove_dir_all(dir);

        let result = create_migration("create_users", dir);
        assert!(result.is_ok());

        let filename = result.unwrap();
        assert!(filename.ends_with("_create_users.sql"));

        // Verify file content
        let filepath = Path::new(dir).join(&filename);
        let content = fs::read_to_string(&filepath).unwrap();
        assert!(content.contains("-- Migration: create_users"));
        assert!(content.contains("-- UP"));
        assert!(content.contains("-- DOWN"));

        // Cleanup
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_migration_error_display() {
        let err = MigrationError::NotFound("test".to_string());
        assert!(err.to_string().contains("test"));
        assert!(err.to_string().contains("not found"));

        let err = MigrationError::Parse("parse error".to_string());
        assert!(err.to_string().contains("parse error"));
        assert!(err.to_string().contains("Parse"));
    }

    #[test]
    fn test_migration_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let mig_err = MigrationError::from(io_err);
        match mig_err {
            MigrationError::Io(_) => { /* pass */ }
            _ => panic!("Expected Io variant"),
        }
    }

    #[test]
    fn test_migration_info_struct() {
        let info = MigrationInfo {
            name: "20240101_create_users".to_string(),
            applied: true,
            applied_at: Some(Utc::now()),
        };

        assert_eq!(info.name, "20240101_create_users");
        assert!(info.applied);
        assert!(info.applied_at.is_some());
    }

    #[test]
    fn test_migration_info_not_applied() {
        let info = MigrationInfo {
            name: "20240102_add_posts".to_string(),
            applied: false,
            applied_at: None,
        };

        assert!(!info.applied);
        assert!(info.applied_at.is_none());
    }

    #[test]
    fn test_migrations_table_sql_sqlite() {
        assert!(MIGRATIONS_TABLE_SQLITE.contains("INTEGER PRIMARY KEY AUTOINCREMENT"));
        assert!(MIGRATIONS_TABLE_SQLITE.contains("_migrations"));
        assert!(MIGRATIONS_TABLE_SQLITE.contains("name TEXT"));
    }

    #[test]
    fn test_migrations_table_sql_postgres() {
        assert!(MIGRATIONS_TABLE_POSTGRES.contains("SERIAL PRIMARY KEY"));
        assert!(MIGRATIONS_TABLE_POSTGRES.contains("TIMESTAMPTZ"));
        assert!(MIGRATIONS_TABLE_POSTGRES.contains("_migrations"));
    }

    #[test]
    fn test_migrations_table_sql_mysql() {
        assert!(MIGRATIONS_TABLE_MYSQL.contains("INT AUTO_INCREMENT PRIMARY KEY"));
        assert!(MIGRATIONS_TABLE_MYSQL.contains("VARCHAR(255)"));
        assert!(MIGRATIONS_TABLE_MYSQL.contains("_migrations"));
    }

    #[test]
    fn test_list_migration_files_empty_dir() {
        let result = list_migration_files("/nonexistent/path");
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_list_migration_files_with_files() {
        let dir = "/tmp/test_mig_list";
        let _ = fs::remove_dir_all(dir);
        fs::create_dir_all(dir).unwrap();

        fs::write(format!("{}/001_first.sql", dir), "-- test").unwrap();
        fs::write(format!("{}/002_second.sql", dir), "-- test").unwrap();
        fs::write(format!("{}/readme.txt", dir), "ignore").unwrap(); // Not .sql

        let result = list_migration_files(dir).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, "001_first");
        assert_eq!(result[1].0, "002_second");

        // Cleanup
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_migration_files_sorted_alphabetically() {
        let dir = "/tmp/test_mig_sort";
        let _ = fs::remove_dir_all(dir);
        fs::create_dir_all(dir).unwrap();

        fs::write(format!("{}/c_migration.sql", dir), "-- c").unwrap();
        fs::write(format!("{}/a_migration.sql", dir), "-- a").unwrap();
        fs::write(format!("{}/b_migration.sql", dir), "-- b").unwrap();

        let result = list_migration_files(dir).unwrap();
        assert_eq!(result[0].0, "a_migration");
        assert_eq!(result[1].0, "b_migration");
        assert_eq!(result[2].0, "c_migration");

        // Cleanup
        let _ = fs::remove_dir_all(dir);
    }
}
