//! Photon - Type-safe Database Layer
//!
//! A comprehensive database abstraction for Nucleus supporting PostgreSQL, MySQL, and SQLite.
//!
//! # Features
//!
//! - **Multi-Database**: PostgreSQL, MySQL, SQLite from one API
//! - **Query Builder**: Fluent, type-safe SQL generation
//! - **Transactions**: ACID-compliant with automatic rollback
//! - **Migrations**: Version-controlled schema changes
//! - **Relationships**: HasMany, BelongsTo, HasOne with eager loading
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use nucleus_std::photon::{init_db, Model, Op};
//!
//! // Initialize database
//! init_db("sqlite://./data.db").await?;
//!
//! // Define a model
//! #[derive(sqlx::FromRow)]
//! struct User {
//!     id: i64,
//!     name: String,
//! }
//! impl_model!(User, "users");
//!
//! // Query data
//! let users = User::query()
//!     .filter_op("age", Op::Gte, 18)
//!     .order_by("name", "ASC")
//!     .all::<User>()
//!     .await?;
//! ```

pub mod db;
pub mod query;
pub mod migrations;
pub mod relations;

// Re-export main types
pub use db::{DatabasePool, DatabaseType, QueryValue, init_db, db, is_db_initialized};
#[allow(deprecated)]
pub use query::{Builder, Model, Op, transaction, transaction_sqlite, transaction_postgres, transaction_mysql};
pub use migrations::{run_migrations, rollback, migration_status, create_migration, MigrationInfo, MigrationError};
pub use relations::{HasMany, BelongsTo, HasOne};

// Re-export macro
pub use crate::impl_model;
