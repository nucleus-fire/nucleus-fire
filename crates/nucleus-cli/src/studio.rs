//! Nucleus Studio - Database Browser
//!
//! Web-based database management interface.
//! Run with: `nucleus studio`

#![forbid(unsafe_code)]

use axum::{
    extract::{Path, Query, State},
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use miette::{miette, Result};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// ═══════════════════════════════════════════════════════════════════════════
// STATE
// ═══════════════════════════════════════════════════════════════════════════

/// Shared database connection
pub struct StudioState {
    conn: Mutex<Connection>,
    db_path: String,
}

impl StudioState {
    /// Create new state with database connection
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path).map_err(|e| miette!("Failed to open database: {}", e))?;
        Ok(Self {
            conn: Mutex::new(conn),
            db_path: path.to_string(),
        })
    }
    /// Get database info
    pub fn get_info(&self) -> DbInfo {
        let conn = self.conn.lock().unwrap();
        let version: String = conn
            .query_row("SELECT sqlite_version()", [], |row| row.get(0))
            .unwrap_or_else(|_| "unknown".to_string());
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table'",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);
        DbInfo {
            path: self.db_path.clone(),
            sqlite_version: version,
            table_count: count as usize,
        }
    }

    /// List all tables
    pub fn list_tables(&self) -> Vec<TableInfo> {
        let conn = self.conn.lock().unwrap();
        let mut tables = Vec::new();
        let names: Vec<String> = {
            let mut stmt = conn
                .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
                .unwrap();
            let mut rows = stmt.query([]).unwrap();
            let mut res = Vec::new();
            while let Ok(Some(r)) = rows.next() {
                if let Ok(n) = r.get(0) {
                    res.push(n);
                }
            }
            res
        };
        for name in names {
            let count: i64 = conn
                .query_row(&format!("SELECT COUNT(*) FROM \"{}\"", name), [], |r| {
                    r.get(0)
                })
                .unwrap_or(0);
            tables.push(TableInfo {
                name,
                row_count: count,
            });
        }
        tables
    }

    /// Get table schema
    pub fn get_schema(&self, table: &str) -> Vec<ColumnInfo> {
        let conn = self.conn.lock().unwrap();
        let mut columns = Vec::new();
        let mut stmt = conn
            .prepare(&format!("PRAGMA table_info(\"{}\")", table))
            .unwrap();
        let mut rows = stmt.query([]).unwrap();
        while let Ok(Some(r)) = rows.next() {
            if let (Ok(name), Ok(col_type), Ok(notnull), Ok(pk)) = (
                r.get::<_, String>(1),
                r.get::<_, String>(2),
                r.get::<_, i32>(3),
                r.get::<_, i32>(5),
            ) {
                columns.push(ColumnInfo {
                    name,
                    col_type,
                    nullable: notnull == 0,
                    primary_key: pk > 0,
                });
            }
        }
        columns
    }

    /// Get table data
    pub fn get_data(&self, table: &str, params: PaginationQuery) -> TableData {
        let conn = self.conn.lock().unwrap();
        let offset = params.offset.unwrap_or(0);
        let limit = params.limit.unwrap_or(50);

        let columns: Vec<String> = {
            let mut stmt = conn
                .prepare(&format!("PRAGMA table_info(\"{}\")", table))
                .unwrap();
            let mut rows = stmt.query([]).unwrap();
            let mut res = Vec::new();
            while let Ok(Some(r)) = rows.next() {
                if let Ok(n) = r.get(1) {
                    res.push(n);
                }
            }
            res
        };

        let mut where_clause = String::from("1=1");
        if let (Some(col), Some(op), Some(val)) =
            (params.filter_col, params.filter_op, params.filter_val)
        {
            if columns.contains(&col) {
                let operator = match op.as_str() {
                    "=" => "=",
                    "contains" => "LIKE",
                    ">" => ">",
                    "<" => "<",
                    _ => "=",
                };
                if operator == "LIKE" {
                    where_clause = format!("\"{}\" LIKE '%{}%'", col, val.replace("'", "''"));
                } else {
                    where_clause = format!("\"{}\" {} '{}'", col, operator, val.replace("'", "''"));
                }
            }
        }

        let total: i64 = conn
            .query_row(
                &format!("SELECT COUNT(*) FROM \"{}\" WHERE {}", table, where_clause),
                [],
                |r| r.get(0),
            )
            .unwrap_or(0);

        let mut order_clause = String::new();
        if let Some(sort_col) = params.sort_col {
            if columns.contains(&sort_col) {
                let dir = if params.sort_dir.unwrap_or("ASC".to_string()).to_uppercase() == "DESC" {
                    "DESC"
                } else {
                    "ASC"
                };
                order_clause = format!("ORDER BY \"{}\" {}", sort_col, dir);
            }
        }

        let col_count = columns.len();
        let sql = if !order_clause.is_empty() {
            format!(
                "SELECT * FROM \"{}\" WHERE {} {} LIMIT {} OFFSET {}",
                table, where_clause, order_clause, limit, offset
            )
        } else {
            format!(
                "SELECT * FROM \"{}\" WHERE {} LIMIT {} OFFSET {}",
                table, where_clause, limit, offset
            )
        };

        let mut stmt = conn.prepare(&sql).unwrap();
        let mut db_rows = stmt.query([]).unwrap();
        let mut result_rows = Vec::new();
        while let Ok(Some(row)) = db_rows.next() {
            result_rows.push(extract_row_values(row, col_count));
        }

        TableData {
            columns,
            rows: result_rows,
            total,
        }
    }

    pub fn insert_row(
        &self,
        table: &str,
        row: serde_json::Map<String, serde_json::Value>,
    ) -> QueryResult {
        let conn = self.conn.lock().unwrap();
        let mut cols = Vec::new();
        let mut vals = Vec::new();

        for (key, value) in row {
            cols.push(format!("\"{}\"", key));
            if value.is_null() {
                vals.push("NULL".to_string());
            } else if let Some(s) = value.as_str() {
                vals.push(format!("'{}'", s.replace("'", "''")));
            } else {
                vals.push(value.to_string());
            }
        }

        let sql = format!(
            "INSERT INTO \"{}\" ({}) VALUES ({})",
            table,
            cols.join(", "),
            vals.join(", ")
        );
        match conn.execute(&sql, []) {
            Ok(affected) => QueryResult {
                columns: vec![],
                rows: vec![],
                affected: Some(affected),
                error: None,
            },
            Err(e) => QueryResult {
                columns: vec![],
                rows: vec![],
                affected: None,
                error: Some(e.to_string()),
            },
        }
    }

    pub fn update_row(
        &self,
        table: &str,
        pk_col: &str,
        pk_val: &str,
        row: serde_json::Map<String, serde_json::Value>,
    ) -> QueryResult {
        let conn = self.conn.lock().unwrap();
        let mut set_clauses = Vec::new();
        for (key, value) in row {
            if value.is_null() {
                set_clauses.push(format!("\"{}\" = NULL", key));
            } else if let Some(s) = value.as_str() {
                set_clauses.push(format!("\"{}\" = '{}'", key, s.replace("'", "''")));
            } else {
                set_clauses.push(format!("\"{}\" = {}", key, value));
            }
        }
        let sql = format!(
            "UPDATE \"{}\" SET {} WHERE \"{}\" = '{}'",
            table,
            set_clauses.join(", "),
            pk_col,
            pk_val.replace("'", "''")
        );
        match conn.execute(&sql, []) {
            Ok(affected) => QueryResult {
                columns: vec![],
                rows: vec![],
                affected: Some(affected),
                error: None,
            },
            Err(e) => QueryResult {
                columns: vec![],
                rows: vec![],
                affected: None,
                error: Some(e.to_string()),
            },
        }
    }

    pub fn delete_row(&self, table: &str, pk_col: &str, pk_val: &str) -> QueryResult {
        let conn = self.conn.lock().unwrap();
        let sql = format!(
            "DELETE FROM \"{}\" WHERE \"{}\" = '{}'",
            table,
            pk_col,
            pk_val.replace("'", "''")
        );
        match conn.execute(&sql, []) {
            Ok(affected) => QueryResult {
                columns: vec![],
                rows: vec![],
                affected: Some(affected),
                error: None,
            },
            Err(e) => QueryResult {
                columns: vec![],
                rows: vec![],
                affected: None,
                error: Some(e.to_string()),
            },
        }
    }

    pub fn execute_query(&self, sql: &str) -> QueryResult {
        let conn = self.conn.lock().unwrap();
        let sql = sql.trim();
        let is_select =
            sql.to_uppercase().starts_with("SELECT") || sql.to_uppercase().starts_with("PRAGMA");

        if is_select {
            let mut stmt = match conn.prepare(sql) {
                Ok(s) => s,
                Err(e) => {
                    return QueryResult {
                        columns: vec![],
                        rows: vec![],
                        affected: None,
                        error: Some(e.to_string()),
                    }
                }
            };
            let columns: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();
            let col_count = columns.len();
            let mut rows = match stmt.query([]) {
                Ok(r) => r,
                Err(e) => {
                    return QueryResult {
                        columns: vec![],
                        rows: vec![],
                        affected: None,
                        error: Some(e.to_string()),
                    }
                }
            };
            let mut result_rows = Vec::new();
            while let Ok(Some(row)) = rows.next() {
                result_rows.push(extract_row_values(row, col_count));
            }
            QueryResult {
                columns,
                rows: result_rows,
                affected: None,
                error: None,
            }
        } else {
            match conn.execute(sql, []) {
                Ok(affected) => QueryResult {
                    columns: vec![],
                    rows: vec![],
                    affected: Some(affected),
                    error: None,
                },
                Err(e) => QueryResult {
                    columns: vec![],
                    rows: vec![],
                    affected: None,
                    error: Some(e.to_string()),
                },
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

/// GET / - Serve main UI
async fn index() -> Html<&'static str> {
    Html(include_str!("studio_ui.html"))
}

/// GET /api/info
async fn get_info(State(state): State<Arc<StudioState>>) -> Json<DbInfo> {
    Json(state.get_info())
}

/// GET /api/tables
async fn list_tables(State(state): State<Arc<StudioState>>) -> Json<Vec<TableInfo>> {
    Json(state.list_tables())
}

/// GET /api/tables/:name/schema
async fn get_schema(
    State(state): State<Arc<StudioState>>,
    Path(table): Path<String>,
) -> Json<Vec<ColumnInfo>> {
    Json(state.get_schema(&table))
}

/// GET /api/tables/:name/data
async fn get_data(
    State(state): State<Arc<StudioState>>,
    Path(table): Path<String>,
    Query(params): Query<PaginationQuery>,
) -> Json<TableData> {
    Json(state.get_data(&table, params))
}

/// POST /api/tables/:name/row
async fn insert_row(
    State(state): State<Arc<StudioState>>,
    Path(table): Path<String>,
    Json(payload): Json<InsertRowRequest>,
) -> Json<QueryResult> {
    Json(state.insert_row(&table, payload.row))
}

/// PUT /api/tables/:name/row
async fn update_row(
    State(state): State<Arc<StudioState>>,
    Path(table): Path<String>,
    Json(payload): Json<UpdateRowRequest>,
) -> Json<QueryResult> {
    Json(state.update_row(&table, &payload.pk_col, &payload.pk_val, payload.row))
}

/// DELETE /api/tables/:name/row
async fn delete_row(
    State(state): State<Arc<StudioState>>,
    Path(table): Path<String>,
    Json(payload): Json<DeleteRowRequest>,
) -> Json<QueryResult> {
    Json(state.delete_row(&table, &payload.pk_col, &payload.pk_val))
}

/// POST /api/query
async fn execute_query(
    State(state): State<Arc<StudioState>>,
    Json(query): Json<SqlQuery>,
) -> Json<QueryResult> {
    Json(state.execute_query(&query.sql))
}

// ═══════════════════════════════════════════════════════════════════════════
// API TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Serialize)]
pub struct TableInfo {
    pub name: String,
    pub row_count: i64,
}

#[derive(Serialize)]
pub struct ColumnInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub col_type: String,
    pub nullable: bool,
    pub primary_key: bool,
}

#[derive(Serialize)]
pub struct TableData {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub total: i64,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub sort_col: Option<String>,
    pub sort_dir: Option<String>,
    pub filter_col: Option<String>,
    pub filter_op: Option<String>,
    pub filter_val: Option<String>,
}

#[derive(Deserialize)]
pub struct SqlQuery {
    pub sql: String,
}

#[derive(Serialize)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub affected: Option<usize>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct DbInfo {
    pub path: String,
    pub sqlite_version: String,
    pub table_count: usize,
}

#[derive(Deserialize)]
pub struct InsertRowRequest {
    pub row: serde_json::Map<String, serde_json::Value>,
}

#[derive(Deserialize)]
pub struct UpdateRowRequest {
    pub pk_col: String,
    pub pk_val: String,
    pub row: serde_json::Map<String, serde_json::Value>,
}

#[derive(Deserialize)]
pub struct DeleteRowRequest {
    pub pk_col: String,
    pub pk_val: String, // Treat all PKs as strings for simplicity
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/// Extract row values as strings
fn extract_row_values(row: &rusqlite::Row, col_count: usize) -> Vec<String> {
    let mut values = Vec::new();
    for i in 0..col_count {
        let val: String = if let Ok(s) = row.get::<_, String>(i) {
            s
        } else if let Ok(n) = row.get::<_, i64>(i) {
            n.to_string()
        } else if let Ok(f) = row.get::<_, f64>(i) {
            f.to_string()
        } else if row.get::<_, Option<String>>(i).is_ok() {
            "NULL".to_string()
        } else {
            // Fallback for blobs or other types
            "blob".to_string()
        };
        values.push(val);
    }
    values
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

/// Create the Studio router
pub fn create_router(state: Arc<StudioState>) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/api/info", get(get_info))
        .route("/api/tables", get(list_tables))
        .route("/api/tables/:name/schema", get(get_schema))
        .route("/api/tables/:name/data", get(get_data))
        .route(
            "/api/tables/:name/row",
            post(insert_row).put(update_row).delete(delete_row),
        )
        .route("/api/query", post(execute_query))
        .with_state(state)
}

// ═══════════════════════════════════════════════════════════════════════════
// PUBLIC API
// ═══════════════════════════════════════════════════════════════════════════

/// Run the database studio server
pub async fn run_studio(database_url: Option<String>, port: u16) -> Result<()> {
    let path = if let Some(url) = database_url {
        url.strip_prefix("sqlite:").unwrap_or(&url).to_string()
    } else if let Ok(url) = std::env::var("DATABASE_URL") {
        url.strip_prefix("sqlite:").unwrap_or(&url).to_string()
    } else if std::path::Path::new("nucleus.db").exists() {
        "nucleus.db".to_string()
    } else {
        eprintln!("\n  \x1b[31m✗\x1b[0m No database found.");
        eprintln!("    Set DATABASE_URL or create nucleus.db\n");
        return Ok(());
    };

    let state = Arc::new(StudioState::new(&path)?);
    let app = create_router(state);

    println!("\n  \x1b[1;36m⚛️  Nucleus Studio\x1b[0m");
    println!("  Database: {}", path);
    println!("  Server:   http://localhost:{}\n", port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .map_err(|e| miette!("Failed to bind: {}", e))?;

    let serve_result: std::result::Result<(), std::io::Error> =
        axum::serve(listener, app.into_make_service()).await;
    serve_result.map_err(|e| miette!("Server error: {}", e))?;

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn setup_test_db() -> StudioState {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO users (name, email) VALUES ('Alice', 'alice@example.com')",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO users (name, email) VALUES ('Bob', 'bob@example.com')",
            [],
        )
        .unwrap();
        StudioState {
            conn: Mutex::new(conn),
            db_path: "memory".to_string(),
        }
    }

    #[test]
    fn test_get_info() {
        let state = setup_test_db();
        let info = state.get_info();
        assert_eq!(info.path, "memory");
        assert_eq!(info.table_count, 1);
    }

    #[test]
    fn test_list_tables() {
        let state = setup_test_db();
        let tables = state.list_tables();
        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].name, "users");
        assert_eq!(tables[0].row_count, 2);
    }

    #[test]
    fn test_get_schema() {
        let state = setup_test_db();
        let cols = state.get_schema("users");
        assert_eq!(cols.len(), 3);
        assert_eq!(cols[0].name, "id");
        assert_eq!(cols[0].primary_key, true);
        assert_eq!(cols[1].name, "name");
    }

    #[test]
    fn test_get_data_basic() {
        let state = setup_test_db();
        let data = state.get_data(
            "users",
            PaginationQuery {
                limit: None,
                offset: None,
                sort_col: None,
                sort_dir: None,
                filter_col: None,
                filter_op: None,
                filter_val: None,
            },
        );
        assert_eq!(data.total, 2);
        assert_eq!(data.rows.len(), 2);
    }

    #[test]
    fn test_get_data_filtering() {
        let state = setup_test_db();
        // Test exact match
        let data = state.get_data(
            "users",
            PaginationQuery {
                limit: None,
                offset: None,
                sort_col: None,
                sort_dir: None,
                filter_col: Some("name".into()),
                filter_op: Some("=".into()),
                filter_val: Some("Alice".into()),
            },
        );
        assert_eq!(data.rows.len(), 1);
        assert_eq!(data.rows[0][1], "Alice");

        // Test contains
        let data = state.get_data(
            "users",
            PaginationQuery {
                limit: None,
                offset: None,
                sort_col: None,
                sort_dir: None,
                filter_col: Some("email".into()),
                filter_op: Some("contains".into()),
                filter_val: Some("example".into()),
            },
        );
        assert_eq!(data.rows.len(), 2);
    }

    #[test]
    fn test_get_data_sorting() {
        let state = setup_test_db();
        let data = state.get_data(
            "users",
            PaginationQuery {
                limit: None,
                offset: None,
                sort_col: Some("name".into()),
                sort_dir: Some("DESC".into()),
                filter_col: None,
                filter_op: None,
                filter_val: None,
            },
        );
        assert_eq!(data.rows[0][1], "Bob");
        assert_eq!(data.rows[1][1], "Alice");
    }

    #[test]
    fn test_crud_lifecycle() {
        let state = setup_test_db();

        // INSERT
        let res = state.insert_row(
            "users",
            json!({"name": "Charlie", "email": "charlie@example.com"})
                .as_object()
                .unwrap()
                .clone(),
        );
        assert!(res.error.is_none());
        assert_eq!(res.affected, Some(1));

        let data = state.get_data(
            "users",
            PaginationQuery {
                limit: None,
                offset: None,
                sort_col: None,
                sort_dir: None,
                filter_col: Some("name".into()),
                filter_op: Some("=".into()),
                filter_val: Some("Charlie".into()),
            },
        );
        assert_eq!(data.rows.len(), 1);
        let id = &data.rows[0][0];

        // UPDATE
        let res = state.update_row(
            "users",
            "id",
            id,
            json!({"email": "charlie_new@example.com"})
                .as_object()
                .unwrap()
                .clone(),
        );
        assert!(res.error.is_none());

        // DELETE
        let res = state.delete_row("users", "id", id);
        assert!(res.error.is_none());

        let data_final = state.get_data(
            "users",
            PaginationQuery {
                limit: None,
                offset: None,
                sort_col: None,
                sort_dir: None,
                filter_col: Some("name".into()),
                filter_op: Some("=".into()),
                filter_val: Some("Charlie".into()),
            },
        );
        assert_eq!(data_final.rows.len(), 0);
    }

    #[test]
    fn test_execute_query() {
        let state = setup_test_db();

        let res = state.execute_query("SELECT * FROM users");
        assert_eq!(res.rows.len(), 2);

        let res = state.execute_query("CREATE TABLE test_q (id INT)");
        assert!(res.error.is_none());

        let res = state.execute_query("INVALID SQL");
        assert!(res.error.is_some());
    }

    #[test]
    fn test_data_types() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE types (
            t_text TEXT, t_int INTEGER, t_real REAL, t_blob BLOB, t_null TEXT
        )",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO types VALUES (?, ?, ?, ?, ?)",
            ("hello", 42, 3.14, vec![1, 2, 3], rusqlite::types::Null),
        )
        .unwrap();

        let state = StudioState {
            conn: Mutex::new(conn),
            db_path: "memory".to_string(),
        };
        let data = state.get_data(
            "types",
            PaginationQuery {
                limit: None,
                offset: None,
                sort_col: None,
                sort_dir: None,
                filter_col: None,
                filter_op: None,
                filter_val: None,
            },
        );

        assert_eq!(data.rows[0][0], "hello");
        assert_eq!(data.rows[0][1], "42");
        assert_eq!(data.rows[0][2], "3.14");
        assert_eq!(data.rows[0][3], "blob");
        assert_eq!(data.rows[0][4], "NULL");
    }

    #[test]
    fn test_pagination() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("CREATE TABLE items (id INT)", []).unwrap();
        for i in 1..=10 {
            conn.execute("INSERT INTO items VALUES (?)", [i]).unwrap();
        }
        let state = StudioState {
            conn: Mutex::new(conn),
            db_path: "memory".to_string(),
        };

        let p1 = state.get_data(
            "items",
            PaginationQuery {
                limit: Some(3),
                offset: Some(0),
                sort_col: Some("id".into()),
                sort_dir: Some("ASC".into()),
                filter_col: None,
                filter_op: None,
                filter_val: None,
            },
        );
        assert_eq!(p1.rows.len(), 3);
        assert_eq!(p1.rows[0][0], "1");

        let p2 = state.get_data(
            "items",
            PaginationQuery {
                limit: Some(3),
                offset: Some(3),
                sort_col: Some("id".into()),
                sort_dir: Some("ASC".into()),
                filter_col: None,
                filter_op: None,
                filter_val: None,
            },
        );
        assert_eq!(p2.rows.len(), 3);
        assert_eq!(p2.rows[0][0], "4");

        let p_end = state.get_data(
            "items",
            PaginationQuery {
                limit: Some(5),
                offset: Some(8),
                sort_col: Some("id".into()),
                sort_dir: Some("ASC".into()),
                filter_col: None,
                filter_op: None,
                filter_val: None,
            },
        );
        assert_eq!(p_end.rows.len(), 2);
    }

    #[test]
    fn test_security_escaping() {
        let state = setup_test_db();
        // Insert with quote
        let res = state.insert_row(
            "users",
            json!({"name": "O'Reilly", "email": "test@test.com"})
                .as_object()
                .unwrap()
                .clone(),
        );
        assert!(res.error.is_none());

        // Filter with quote
        let data = state.get_data(
            "users",
            PaginationQuery {
                limit: None,
                offset: None,
                sort_col: None,
                sort_dir: None,
                filter_col: Some("name".into()),
                filter_op: Some("=".into()),
                filter_val: Some("O'Reilly".into()),
            },
        );
        assert_eq!(data.rows.len(), 1);
        assert_eq!(data.rows[0][1], "O'Reilly");

        // Update with quote
        let id = &data.rows[0][0];
        let res = state.update_row(
            "users",
            "id",
            id,
            json!({"name": "New'Name"}).as_object().unwrap().clone(),
        );
        assert!(res.error.is_none());

        let data_updated = state.get_data(
            "users",
            PaginationQuery {
                limit: None,
                offset: None,
                sort_col: None,
                sort_dir: None,
                filter_col: Some("name".into()),
                filter_op: Some("=".into()),
                filter_val: Some("New'Name".into()),
            },
        );
        assert_eq!(data_updated.rows.len(), 1);
    }
}
