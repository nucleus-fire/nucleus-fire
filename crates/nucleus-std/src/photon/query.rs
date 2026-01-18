//! Photon Query Builder
//!
//! A fluent, type-safe SQL query builder supporting multiple databases.
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::photon::{Model, Op, Builder};
//!
//! let users = User::query()
//!     .filter_op("age", Op::Gte, 18)
//!     .filter_op("status", Op::In, vec!["active", "pending"])
//!     .or_where("role", "admin")
//!     .order_by("created_at", "DESC")
//!     .limit(10)
//!     .all::<User>()
//!     .await?;
//! ```

use sqlx::{FromRow, Row};
use std::fmt::Write;
use std::future::Future;
use std::pin::Pin;
use crate::photon::db::{db, DatabaseType, QueryValue};

// ═══════════════════════════════════════════════════════════════════════════
// OPERATORS
// ═══════════════════════════════════════════════════════════════════════════

/// SQL comparison operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    /// Equal (=)
    Eq,
    /// Not equal (!=)
    Ne,
    /// Greater than (>)
    Gt,
    /// Greater than or equal (>=)
    Gte,
    /// Less than (<)
    Lt,
    /// Less than or equal (<=)
    Lte,
    /// Pattern match (LIKE)
    Like,
    /// Case-insensitive pattern match (ILIKE) - PostgreSQL only
    ILike,
    /// In a list (IN)
    In,
    /// Not in a list (NOT IN)
    NotIn,
    /// Is NULL
    IsNull,
    /// Is not NULL
    IsNotNull,
}

impl Op {
    /// Convert to SQL string for a specific database
    pub fn to_sql(&self, db_type: DatabaseType) -> &'static str {
        match self {
            Op::Eq => "=",
            Op::Ne => "!=",
            Op::Gt => ">",
            Op::Gte => ">=",
            Op::Lt => "<",
            Op::Lte => "<=",
            Op::Like => "LIKE",
            Op::ILike => match db_type {
                DatabaseType::Postgres => "ILIKE",
                _ => "LIKE", // Fallback for MySQL/SQLite
            },
            Op::In => "IN",
            Op::NotIn => "NOT IN",
            Op::IsNull => "IS NULL",
            Op::IsNotNull => "IS NOT NULL",
        }
    }
    
    /// Check if this operator requires a value
    pub fn requires_value(&self) -> bool {
        !matches!(self, Op::IsNull | Op::IsNotNull)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MODEL TRAIT
// ═══════════════════════════════════════════════════════════════════════════

/// Trait for database models
#[async_trait::async_trait]
pub trait Model: Sized + Send + Unpin {
    /// Get the table name for this model
    fn table_name() -> &'static str;
    
    /// Start a query builder for this model
    fn query() -> Builder<'static> {
        Builder::new(Self::table_name())
    }
    
    /// Start an INSERT query builder
    fn create() -> Builder<'static> {
        Builder::new(Self::table_name()).insert()
    }
    
    /// Find a record by ID
    async fn find<T>(id: i64) -> Result<Option<T>, sqlx::Error>
    where
        T: for<'r> FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
    {
        Self::query().r#where("id", id).first::<T>().await
    }
    
    /// Delete a record by ID
    async fn delete_by_id(id: i64) -> Result<u64, sqlx::Error> {
        let result = Self::query().delete().r#where("id", id).execute().await?;
        Ok(result.rows_affected())
    }
}

/// Macro to implement Model trait for a type
#[macro_export]
macro_rules! impl_model {
    ($type:ty, $table:expr) => {
        impl $crate::photon::query::Model for $type {
            fn table_name() -> &'static str {
                $table
            }
        }
    };
}

// ═══════════════════════════════════════════════════════════════════════════
// QUERY BUILDER
// ═══════════════════════════════════════════════════════════════════════════

/// Operation type for the query
#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    Select,
    Insert,
    Update,
    Delete,
}

/// A WHERE clause
struct WhereClause {
    column: String,
    operator: Op,
    value: QueryValue,
    conjunction: Conjunction,
}

#[derive(Debug, Clone, Copy)]
enum Conjunction {
    And,
    Or,
}

/// A JOIN clause
struct JoinClause {
    table: String,
    on_left: String,
    on_right: String,
    join_type: JoinType,
}

#[derive(Debug, Clone, Copy)]
pub enum JoinType {
    Inner,
    Left,
    Right,
}

impl JoinType {
    fn as_sql(&self) -> &'static str {
        match self {
            JoinType::Inner => "INNER",
            JoinType::Left => "LEFT",
            JoinType::Right => "RIGHT",
        }
    }
}

/// Fluent SQL query builder
pub struct Builder<'a> {
    table: &'a str,
    select: Vec<String>,
    wheres: Vec<WhereClause>,
    joins: Vec<JoinClause>,
    order_by: Vec<(String, String)>,
    limit: Option<i64>,
    offset: Option<i64>,
    operation: Operation,
    values: Vec<(String, QueryValue)>,
    includes: Vec<String>,
}

impl<'a> Builder<'a> {
    /// Create a new query builder for a table
    pub fn new(table: &'a str) -> Self {
        Self {
            table,
            select: vec!["*".to_string()],
            wheres: vec![],
            joins: vec![],
            order_by: vec![],
            limit: None,
            offset: None,
            operation: Operation::Select,
            values: vec![],
            includes: vec![],
        }
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // OPERATION MODES
    // ─────────────────────────────────────────────────────────────────────────
    
    /// Switch to INSERT mode
    pub fn insert(mut self) -> Self {
        self.operation = Operation::Insert;
        self
    }
    
    /// Switch to UPDATE mode
    pub fn update(mut self) -> Self {
        self.operation = Operation::Update;
        self
    }
    
    /// Switch to DELETE mode
    pub fn delete(mut self) -> Self {
        self.operation = Operation::Delete;
        self
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // SELECT CLAUSES
    // ─────────────────────────────────────────────────────────────────────────
    
    /// Set columns to select
    pub fn select(mut self, columns: &[&str]) -> Self {
        self.select = columns.iter().map(|s| s.to_string()).collect();
        self
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // WHERE CLAUSES
    // ─────────────────────────────────────────────────────────────────────────
    
    /// Add a WHERE clause with equality operator
    pub fn r#where<V>(self, column: &str, value: V) -> Self
    where
        QueryValue: From<V>,
    {
        self.filter_op(column, Op::Eq, value)
    }
    
    /// Add a WHERE clause with a custom operator
    pub fn filter_op<V>(mut self, column: &str, op: Op, value: V) -> Self
    where
        QueryValue: From<V>,
    {
        self.wheres.push(WhereClause {
            column: column.to_string(),
            operator: op,
            value: QueryValue::from(value),
            conjunction: Conjunction::And,
        });
        self
    }
    
    
    /// Add an OR WHERE clause
    pub fn or_where<V>(mut self, column: &str, value: V) -> Self
    where
        QueryValue: From<V>,
    {
        self.wheres.push(WhereClause {
            column: column.to_string(),
            operator: Op::Eq,
            value: QueryValue::from(value),
            conjunction: Conjunction::Or,
        });
        self
    }
    
    /// Add an OR WHERE clause with operator
    pub fn or_filter_op<V>(mut self, column: &str, op: Op, value: V) -> Self
    where
        QueryValue: From<V>,
    {
        self.wheres.push(WhereClause {
            column: column.to_string(),
            operator: op,
            value: QueryValue::from(value),
            conjunction: Conjunction::Or,
        });
        self
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // JOIN CLAUSES
    // ─────────────────────────────────────────────────────────────────────────
    
    /// Add an INNER JOIN
    pub fn join(self, table: &str, on_left: &str, on_right: &str) -> Self {
        self.join_type(JoinType::Inner, table, on_left, on_right)
    }
    
    /// Add a LEFT JOIN
    pub fn left_join(self, table: &str, on_left: &str, on_right: &str) -> Self {
        self.join_type(JoinType::Left, table, on_left, on_right)
    }
    
    /// Add a RIGHT JOIN
    pub fn right_join(self, table: &str, on_left: &str, on_right: &str) -> Self {
        self.join_type(JoinType::Right, table, on_left, on_right)
    }
    
    fn join_type(mut self, join_type: JoinType, table: &str, on_left: &str, on_right: &str) -> Self {
        self.joins.push(JoinClause {
            table: table.to_string(),
            on_left: on_left.to_string(),
            on_right: on_right.to_string(),
            join_type,
        });
        self
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // ORDER & PAGINATION
    // ─────────────────────────────────────────────────────────────────────────
    
    /// Add ORDER BY clause
    pub fn order_by(mut self, column: &str, direction: &str) -> Self {
        self.order_by.push((column.to_string(), direction.to_uppercase()));
        self
    }
    
    /// Add LIMIT clause
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    
    /// Add OFFSET clause
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // INSERT/UPDATE VALUES
    // ─────────────────────────────────────────────────────────────────────────
    
    /// Add a value for INSERT/UPDATE
    pub fn value<V>(mut self, column: &str, value: V) -> Self
    where
        QueryValue: From<V>,
    {
        self.values.push((column.to_string(), QueryValue::from(value)));
        self
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // RELATIONSHIPS
    // ─────────────────────────────────────────────────────────────────────────
    
    /// Include a relationship for eager loading
    pub fn include(mut self, relation: &str) -> Self {
        self.includes.push(relation.to_string());
        self
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // SQL GENERATION
    // ─────────────────────────────────────────────────────────────────────────
    
    /// Build SQL and extract values for binding
    pub fn to_sql(&self, db_type: DatabaseType) -> (String, Vec<&QueryValue>) {
        let mut sql = String::new();
        let mut bindings = Vec::new();
        let mut param_index = 1;
        
        match self.operation {
            Operation::Select => {
                write!(sql, "SELECT {} FROM {}", self.select.join(", "), self.table).unwrap();
                
                // Joins
                for join in &self.joins {
                    write!(
                        sql, " {} JOIN {} ON {} = {}",
                        join.join_type.as_sql(),
                        join.table,
                        join.on_left,
                        join.on_right
                    ).unwrap();
                }
                
                // Where
                if !self.wheres.is_empty() {
                    sql.push_str(" WHERE ");
                    for (i, w) in self.wheres.iter().enumerate() {
                        if i > 0 {
                            match w.conjunction {
                                Conjunction::And => sql.push_str(" AND "),
                                Conjunction::Or => sql.push_str(" OR "),
                            }
                        }
                        
                        if w.operator.requires_value() {
                            write!(
                                sql, "{} {} {}",
                                w.column,
                                w.operator.to_sql(db_type),
                                db_type.placeholder(param_index)
                            ).unwrap();
                            bindings.push(&w.value);
                            param_index += 1;
                        } else {
                            write!(sql, "{} {}", w.column, w.operator.to_sql(db_type)).unwrap();
                        }
                    }
                }
                
                // Order
                if !self.order_by.is_empty() {
                    sql.push_str(" ORDER BY ");
                    let orders: Vec<String> = self.order_by.iter()
                        .map(|(col, dir)| format!("{} {}", col, dir))
                        .collect();
                    sql.push_str(&orders.join(", "));
                }
                
                // Limit/Offset
                if let Some(limit) = self.limit {
                    write!(sql, " LIMIT {}", limit).unwrap();
                }
                if let Some(offset) = self.offset {
                    write!(sql, " OFFSET {}", offset).unwrap();
                }
            }
            
            Operation::Insert => {
                let cols: Vec<&String> = self.values.iter().map(|(c, _)| c).collect();
                let placeholders: Vec<String> = (1..=self.values.len())
                    .map(|i| db_type.placeholder(i))
                    .collect();
                
                write!(
                    sql, "INSERT INTO {} ({}) VALUES ({})",
                    self.table,
                    cols.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "),
                    placeholders.join(", ")
                ).unwrap();
                
                for (_, v) in &self.values {
                    bindings.push(v);
                }
            }
            
            Operation::Update => {
                write!(sql, "UPDATE {} SET ", self.table).unwrap();
                
                let sets: Vec<String> = self.values.iter()
                    .enumerate()
                    .map(|(i, (col, _))| format!("{} = {}", col, db_type.placeholder(i + 1)))
                    .collect();
                sql.push_str(&sets.join(", "));
                
                for (_, v) in &self.values {
                    bindings.push(v);
                    param_index += 1;
                }
                
                // Where
                if !self.wheres.is_empty() {
                    sql.push_str(" WHERE ");
                    for (i, w) in self.wheres.iter().enumerate() {
                        if i > 0 {
                            match w.conjunction {
                                Conjunction::And => sql.push_str(" AND "),
                                Conjunction::Or => sql.push_str(" OR "),
                            }
                        }
                        write!(
                            sql, "{} {} {}",
                            w.column,
                            w.operator.to_sql(db_type),
                            db_type.placeholder(param_index)
                        ).unwrap();
                        bindings.push(&w.value);
                        param_index += 1;
                    }
                }
            }
            
            Operation::Delete => {
                write!(sql, "DELETE FROM {}", self.table).unwrap();
                
                if !self.wheres.is_empty() {
                    sql.push_str(" WHERE ");
                    for (i, w) in self.wheres.iter().enumerate() {
                        if i > 0 {
                            match w.conjunction {
                                Conjunction::And => sql.push_str(" AND "),
                                Conjunction::Or => sql.push_str(" OR "),
                            }
                        }
                        write!(
                            sql, "{} {} {}",
                            w.column,
                            w.operator.to_sql(db_type),
                            db_type.placeholder(param_index)
                        ).unwrap();
                        bindings.push(&w.value);
                        param_index += 1;
                    }
                }
            }
        }
        
        (sql, bindings)
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // EXECUTION
    // ─────────────────────────────────────────────────────────────────────────
    
    /// Fetch all matching rows
    pub async fn all<T>(self) -> Result<Vec<T>, sqlx::Error>
    where
        T: for<'r> FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
    {
        let pool = db();
        let (sql, values) = self.to_sql(pool.db_type());
        
        // For SQLite (primary support)
        if let Some(sqlite_pool) = pool.as_sqlite() {
            let mut query = sqlx::query_as::<_, T>(&sql);
            
            for val in values {
                query = match val {
                    QueryValue::Text(v) => query.bind(v),
                    QueryValue::Int(v) => query.bind(v),
                    QueryValue::Float(v) => query.bind(v),
                    QueryValue::Bool(v) => query.bind(v),
                    QueryValue::Null => query.bind(Option::<String>::None),
                    QueryValue::Bytes(v) => query.bind(v),
                };
            }
            
            return query.fetch_all(sqlite_pool).await;
        }
        
        Err(sqlx::Error::Configuration("Unsupported database type for this query".into()))
    }
    
    /// Fetch the first matching row
    pub async fn first<T>(self) -> Result<Option<T>, sqlx::Error>
    where
        T: for<'r> FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
    {
        let results = self.limit(1).all::<T>().await?;
        Ok(results.into_iter().next())
    }
    
    /// Execute an INSERT/UPDATE/DELETE and return query result
    /// 
    /// For INSERT, use `.last_insert_rowid()` to get the ID.
    /// For UPDATE/DELETE, use `.rows_affected()` to get count.
    pub async fn execute(self) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let pool = db();
        let (sql, values) = self.to_sql(pool.db_type());
        
        if let Some(sqlite_pool) = pool.as_sqlite() {
            let mut query = sqlx::query(&sql);
            
            for val in values {
                query = match val {
                    QueryValue::Text(v) => query.bind(v),
                    QueryValue::Int(v) => query.bind(v),
                    QueryValue::Float(v) => query.bind(v),
                    QueryValue::Bool(v) => query.bind(v),
                    QueryValue::Null => query.bind(Option::<String>::None),
                    QueryValue::Bytes(v) => query.bind(v),
                };
            }
            
            return query.execute(sqlite_pool).await;
        }
        
        Err(sqlx::Error::Configuration("Unsupported database type".into()))
    }
    
    /// Execute an INSERT and return the new row's ID
    /// 
    /// Convenience method that calls `execute()` and extracts `last_insert_rowid()`.
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// let id = User::create()
    ///     .value("name", "Alice")
    ///     .insert_get_id()
    ///     .await?;
    /// ```
    pub async fn insert_get_id(self) -> Result<i64, sqlx::Error> {
        let result = self.execute().await?;
        Ok(result.last_insert_rowid())
    }
    
    /// Execute an UPDATE/DELETE and return the number of affected rows
    /// 
    /// Convenience method that calls `execute()` and extracts `rows_affected()`.
    pub async fn run(self) -> Result<u64, sqlx::Error> {
        let result = self.execute().await?;
        Ok(result.rows_affected())
    }
    
    /// Count matching rows
    pub async fn count(self) -> Result<i64, sqlx::Error> {
        let pool = db();
        let mut builder = Builder::new(self.table);
        builder.select = vec!["COUNT(*) as count".to_string()];
        builder.wheres = self.wheres;
        builder.joins = self.joins;
        
        let (sql, values) = builder.to_sql(pool.db_type());
        
        if let Some(sqlite_pool) = pool.as_sqlite() {
            let mut query = sqlx::query(&sql);
            
            for val in values {
                query = match val {
                    QueryValue::Text(v) => query.bind(v),
                    QueryValue::Int(v) => query.bind(v),
                    QueryValue::Float(v) => query.bind(v),
                    QueryValue::Bool(v) => query.bind(v),
                    QueryValue::Null => query.bind(Option::<String>::None),
                    QueryValue::Bytes(v) => query.bind(v),
                };
            }
            
            let row = query.fetch_one(sqlite_pool).await?;
            return Ok(row.get::<i64, _>("count"));
        }
        
        Err(sqlx::Error::Configuration("Unsupported database type".into()))
    }
    
    /// Check if any matching rows exist
    pub async fn exists(self) -> Result<bool, sqlx::Error> {
        let count = self.limit(1).count().await?;
        Ok(count > 0)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TRANSACTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Execute a closure within a SQLite database transaction
///
/// The transaction is automatically committed if the closure returns Ok,
/// or rolled back if it returns Err or panics.
///
/// # Example
///
/// ```rust,ignore
/// use nucleus_std::photon::transaction_sqlite;
///
/// transaction_sqlite(|tx| Box::pin(async move {
///     sqlx::query("INSERT INTO users (name) VALUES (?)")
///         .bind("Alice")
///         .execute(&mut **tx)
///         .await?;
///     Ok(())
/// })).await?;
/// ```
pub async fn transaction_sqlite<F, T>(f: F) -> Result<T, sqlx::Error>
where
    F: for<'c> FnOnce(&'c mut sqlx::Transaction<'_, sqlx::Sqlite>) -> Pin<Box<dyn Future<Output = Result<T, sqlx::Error>> + Send + 'c>>,
{
    let pool = db();
    
    let sqlite_pool = pool.as_sqlite()
        .ok_or_else(|| sqlx::Error::Configuration("Not a SQLite database".into()))?;
    
    let mut tx = sqlite_pool.begin().await?;
    
    match f(&mut tx).await {
        Ok(result) => {
            tx.commit().await?;
            Ok(result)
        }
        Err(e) => {
            tx.rollback().await.ok();
            Err(e)
        }
    }
}

/// Execute a closure within a PostgreSQL database transaction
///
/// # Example
///
/// ```rust,ignore
/// use nucleus_std::photon::transaction_postgres;
///
/// transaction_postgres(|tx| Box::pin(async move {
///     sqlx::query("INSERT INTO users (name) VALUES ($1)")
///         .bind("Alice")
///         .execute(&mut **tx)
///         .await?;
///     Ok(())
/// })).await?;
/// ```
pub async fn transaction_postgres<F, T>(f: F) -> Result<T, sqlx::Error>
where
    F: for<'c> FnOnce(&'c mut sqlx::Transaction<'_, sqlx::Postgres>) -> Pin<Box<dyn Future<Output = Result<T, sqlx::Error>> + Send + 'c>>,
{
    let pool = db();
    
    let pg_pool = pool.as_postgres()
        .ok_or_else(|| sqlx::Error::Configuration("Not a PostgreSQL database".into()))?;
    
    let mut tx = pg_pool.begin().await?;
    
    match f(&mut tx).await {
        Ok(result) => {
            tx.commit().await?;
            Ok(result)
        }
        Err(e) => {
            tx.rollback().await.ok();
            Err(e)
        }
    }
}

/// Execute a closure within a MySQL database transaction
///
/// # Example
///
/// ```rust,ignore
/// use nucleus_std::photon::transaction_mysql;
///
/// transaction_mysql(|tx| Box::pin(async move {
///     sqlx::query("INSERT INTO users (name) VALUES (?)")
///         .bind("Alice")
///         .execute(&mut **tx)
///         .await?;
///     Ok(())
/// })).await?;
/// ```
pub async fn transaction_mysql<F, T>(f: F) -> Result<T, sqlx::Error>
where
    F: for<'c> FnOnce(&'c mut sqlx::Transaction<'_, sqlx::MySql>) -> Pin<Box<dyn Future<Output = Result<T, sqlx::Error>> + Send + 'c>>,
{
    let pool = db();
    
    let mysql_pool = pool.as_mysql()
        .ok_or_else(|| sqlx::Error::Configuration("Not a MySQL database".into()))?;
    
    let mut tx = mysql_pool.begin().await?;
    
    match f(&mut tx).await {
        Ok(result) => {
            tx.commit().await?;
            Ok(result)
        }
        Err(e) => {
            tx.rollback().await.ok();
            Err(e)
        }
    }
}

/// Legacy transaction function - uses SQLite
/// 
/// **Deprecated**: Use `transaction_sqlite`, `transaction_postgres`, or `transaction_mysql` instead.
#[deprecated(since = "0.2.0", note = "Use transaction_sqlite, transaction_postgres, or transaction_mysql")]
pub async fn transaction<F, T>(f: F) -> Result<T, sqlx::Error>
where
    F: for<'c> FnOnce(&'c mut sqlx::Transaction<'_, sqlx::Sqlite>) -> Pin<Box<dyn Future<Output = Result<T, sqlx::Error>> + Send + 'c>>,
{
    transaction_sqlite(f).await
}

// ═══════════════════════════════════════════════════════════════════════════
// SHORTCUT FUNCTION
// ═══════════════════════════════════════════════════════════════════════════

/// Create a query builder for a table
pub fn query(table: &str) -> Builder<'_> {
    Builder::new(table)
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_op_to_sql() {
        assert_eq!(Op::Eq.to_sql(DatabaseType::Sqlite), "=");
        assert_eq!(Op::Ne.to_sql(DatabaseType::Sqlite), "!=");
        assert_eq!(Op::Gt.to_sql(DatabaseType::Sqlite), ">");
        assert_eq!(Op::Gte.to_sql(DatabaseType::Sqlite), ">=");
        assert_eq!(Op::Lt.to_sql(DatabaseType::Sqlite), "<");
        assert_eq!(Op::Lte.to_sql(DatabaseType::Sqlite), "<=");
        assert_eq!(Op::Like.to_sql(DatabaseType::Sqlite), "LIKE");
        assert_eq!(Op::ILike.to_sql(DatabaseType::Postgres), "ILIKE");
        assert_eq!(Op::ILike.to_sql(DatabaseType::Sqlite), "LIKE"); // Fallback
        assert_eq!(Op::IsNull.to_sql(DatabaseType::Sqlite), "IS NULL");
        assert_eq!(Op::IsNotNull.to_sql(DatabaseType::Sqlite), "IS NOT NULL");
    }
    
    #[test]
    fn test_op_requires_value() {
        assert!(Op::Eq.requires_value());
        assert!(Op::Like.requires_value());
        assert!(!Op::IsNull.requires_value());
        assert!(!Op::IsNotNull.requires_value());
    }
    
    #[test]
    fn test_select_sql() {
        let builder = Builder::new("users")
            .select(&["id", "name"])
            .r#where("status", "active")
            .order_by("created_at", "DESC")
            .limit(10);
        
        let (sql, bindings) = builder.to_sql(DatabaseType::Sqlite);
        
        assert!(sql.contains("SELECT id, name FROM users"));
        assert!(sql.contains("WHERE status = ?"));
        assert!(sql.contains("ORDER BY created_at DESC"));
        assert!(sql.contains("LIMIT 10"));
        assert_eq!(bindings.len(), 1);
    }
    
    #[test]
    fn test_postgres_placeholders() {
        let builder = Builder::new("users")
            .r#where("id", 1i64)
            .r#where("status", "active");
        
        let (sql, _) = builder.to_sql(DatabaseType::Postgres);
        
        assert!(sql.contains("$1"));
        assert!(sql.contains("$2"));
    }
    
    #[test]
    fn test_insert_sql() {
        let builder = Builder::new("users")
            .insert()
            .value("name", "Alice")
            .value("email", "alice@example.com");
        
        let (sql, bindings) = builder.to_sql(DatabaseType::Sqlite);
        
        assert!(sql.contains("INSERT INTO users"));
        assert!(sql.contains("name, email"));
        assert!(sql.contains("VALUES (?, ?)"));
        assert_eq!(bindings.len(), 2);
    }
    
    #[test]
    fn test_update_sql() {
        let builder = Builder::new("users")
            .update()
            .value("name", "Bob")
            .r#where("id", 1i64);
        
        let (sql, bindings) = builder.to_sql(DatabaseType::Sqlite);
        
        assert!(sql.contains("UPDATE users SET"));
        assert!(sql.contains("name = ?"));
        assert!(sql.contains("WHERE id = ?"));
        assert_eq!(bindings.len(), 2);
    }
    
    #[test]
    fn test_delete_sql() {
        let builder = Builder::new("users")
            .delete()
            .r#where("id", 1i64);
        
        let (sql, bindings) = builder.to_sql(DatabaseType::Sqlite);
        
        assert!(sql.contains("DELETE FROM users"));
        assert!(sql.contains("WHERE id = ?"));
        assert_eq!(bindings.len(), 1);
    }
    
    #[test]
    fn test_or_where() {
        let builder = Builder::new("users")
            .r#where("status", "active")
            .or_where("role", "admin");
        
        let (sql, bindings) = builder.to_sql(DatabaseType::Sqlite);
        
        assert!(sql.contains("WHERE status = ?"));
        assert!(sql.contains("OR role = ?"));
        assert_eq!(bindings.len(), 2);
    }
    
    #[test]
    fn test_joins() {
        let builder = Builder::new("posts")
            .join("users", "posts.user_id", "users.id")
            .left_join("comments", "posts.id", "comments.post_id");
        
        let (sql, _) = builder.to_sql(DatabaseType::Sqlite);
        
        assert!(sql.contains("INNER JOIN users ON posts.user_id = users.id"));
        assert!(sql.contains("LEFT JOIN comments ON posts.id = comments.post_id"));
    }
    
    #[test]
    fn test_filter_op() {
        let builder = Builder::new("users")
            .filter_op("age", Op::Gte, 18i64)
            .filter_op("deleted_at", Op::IsNull, 0i64); // Value ignored for IsNull
        
        let (sql, _) = builder.to_sql(DatabaseType::Sqlite);
        
        assert!(sql.contains("age >= ?"));
        assert!(sql.contains("deleted_at IS NULL"));
    }
}
