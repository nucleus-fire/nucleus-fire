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
pub mod migrations;
pub mod query;
pub mod relations;

// Re-export main types
pub use db::{db, init_db, is_db_initialized, DatabasePool, DatabaseType, QueryValue};
pub use migrations::{
    create_migration, migration_status, rollback, run_migrations, MigrationError, MigrationInfo,
};
pub use query::{transaction_mysql, transaction_postgres, transaction_sqlite, Builder, Model, Op};
pub use relations::{BelongsTo, HasMany, HasOne};

// Re-export macro
pub use crate::impl_model;
