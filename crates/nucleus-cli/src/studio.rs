//! Nucleus Studio - Database Browser
//!
//! Web-based database management interface.
//! Run with: `nucleus studio`

#![forbid(unsafe_code)]

use axum::{
    Router,
    routing::{get, post},
    response::{Html, Json},
    extract::{State, Path, Query},
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
        let conn = Connection::open(path)
            .map_err(|e| miette!("Failed to open database: {}", e))?;
        Ok(Self {
            conn: Mutex::new(conn),
            db_path: path.to_string(),
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// API TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Serialize)]
struct TableInfo {
    name: String,
    row_count: i64,
}

#[derive(Serialize)]
struct ColumnInfo {
    name: String,
    #[serde(rename = "type")]
    col_type: String,
    nullable: bool,
    primary_key: bool,
}

#[derive(Serialize)]
struct TableData {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
    total: i64,
}

#[derive(Deserialize)]
struct PaginationQuery {
    offset: Option<i64>,
    limit: Option<i64>,
}

#[derive(Deserialize)]
struct SqlQuery {
    sql: String,
}

#[derive(Serialize)]
struct QueryResult {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
    affected: Option<usize>,
    error: Option<String>,
}

#[derive(Serialize)]
struct DbInfo {
    path: String,
    sqlite_version: String,
    table_count: usize,
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
        } else {
            "NULL".to_string()
        };
        values.push(val);
    }
    values
}

/// Create error response
fn error_response(msg: String) -> Json<QueryResult> {
    Json(QueryResult {
        columns: vec![],
        rows: vec![],
        affected: None,
        error: Some(msg),
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

/// GET / - Serve main UI
async fn index() -> Html<&'static str> {
    Html(include_str!("studio_ui.html"))
}

/// GET /api/info - Database info
async fn get_info(State(state): State<Arc<StudioState>>) -> Json<DbInfo> {
    let conn = state.conn.lock().unwrap();
    
    let version: String = conn
        .query_row("SELECT sqlite_version()", [], |row: &rusqlite::Row| row.get(0))
        .unwrap_or_else(|_| "unknown".to_string());
    
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM sqlite_master WHERE type='table'", [], |row: &rusqlite::Row| row.get(0))
        .unwrap_or(0);
    
    Json(DbInfo {
        path: state.db_path.clone(),
        sqlite_version: version,
        table_count: count as usize,
    })
}

/// GET /api/tables - List all tables
async fn list_tables(State(state): State<Arc<StudioState>>) -> Json<Vec<TableInfo>> {
    let conn = state.conn.lock().unwrap();
    let mut tables = Vec::new();
    
    // Get table names
    let names: Vec<String> = {
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name").unwrap();
        let mut rows = stmt.query([]).unwrap();
        let mut result = Vec::new();
        while let Ok(Some(row)) = rows.next() {
            let r: &rusqlite::Row = row;
            if let Ok(name) = r.get::<_, String>(0) {
                result.push(name);
            }
        }
        result
    };
    
    // Get counts for each
    for name in names {
        let count: i64 = conn
            .query_row(&format!("SELECT COUNT(*) FROM \"{}\"", name), [], |r: &rusqlite::Row| r.get(0))
            .unwrap_or(0);
        tables.push(TableInfo { name, row_count: count });
    }
    
    Json(tables)
}

/// GET /api/tables/:name/schema - Get table schema
async fn get_schema(
    State(state): State<Arc<StudioState>>,
    Path(table): Path<String>,
) -> Json<Vec<ColumnInfo>> {
    let conn = state.conn.lock().unwrap();
    let mut columns = Vec::new();
    
    let mut stmt = conn.prepare(&format!("PRAGMA table_info(\"{}\")", table)).unwrap();
    let mut rows = stmt.query([]).unwrap();
    
    while let Ok(Some(row)) = rows.next() {
        let r: &rusqlite::Row = row;
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
    
    Json(columns)
}

/// GET /api/tables/:name/data - Get table data with pagination
async fn get_data(
    State(state): State<Arc<StudioState>>,
    Path(table): Path<String>,
    Query(params): Query<PaginationQuery>,
) -> Json<TableData> {
    let conn = state.conn.lock().unwrap();
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(50);
    
    // Get column names
    let columns: Vec<String> = {
        let mut stmt = conn.prepare(&format!("PRAGMA table_info(\"{}\")", table)).unwrap();
        let mut rows = stmt.query([]).unwrap();
        let mut result = Vec::new();
        while let Ok(Some(row)) = rows.next() {
            let r: &rusqlite::Row = row;
            if let Ok(name) = r.get::<_, String>(1) {
                result.push(name);
            }
        }
        result
    };
    
    // Get total count
    let total: i64 = conn
        .query_row(&format!("SELECT COUNT(*) FROM \"{}\"", table), [], |r: &rusqlite::Row| r.get(0))
        .unwrap_or(0);
    
    // Get rows as strings
    let col_count = columns.len();
    let sql = format!("SELECT * FROM \"{}\" LIMIT {} OFFSET {}", table, limit, offset);
    let mut stmt = conn.prepare(&sql).unwrap();
    let mut db_rows = stmt.query([]).unwrap();
    
    let mut result_rows: Vec<Vec<String>> = Vec::new();
    while let Ok(Some(row)) = db_rows.next() {
        result_rows.push(extract_row_values(row, col_count));
    }
    
    Json(TableData { columns, rows: result_rows, total })
}

/// POST /api/query - Execute SQL query
async fn execute_query(
    State(state): State<Arc<StudioState>>,
    Json(query): Json<SqlQuery>,
) -> Json<QueryResult> {
    let conn = state.conn.lock().unwrap();
    let sql = query.sql.trim();
    let is_select = sql.to_uppercase().starts_with("SELECT") || 
                    sql.to_uppercase().starts_with("PRAGMA");
    
    if is_select {
        let prepare_result = conn.prepare(sql);
        let mut stmt: rusqlite::Statement = match prepare_result {
            Ok(s) => s,
            Err(e) => { let err: rusqlite::Error = e; return error_response(err.to_string()); }
        };
        
        let columns: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();
        let col_count = columns.len();
        
        let query_result = stmt.query([]);
        let mut rows: rusqlite::Rows = match query_result {
            Ok(r) => r,
            Err(e) => { let err: rusqlite::Error = e; return error_response(err.to_string()); }
        };
        
        let mut result_rows: Vec<Vec<String>> = Vec::new();
        while let Ok(Some(row)) = rows.next() {
            result_rows.push(extract_row_values(row, col_count));
        }
        
        Json(QueryResult { columns, rows: result_rows, affected: None, error: None })
    } else {
        let exec_result = conn.execute(sql, []);
        match exec_result {
            Ok(affected) => Json(QueryResult {
                columns: vec![],
                rows: vec![],
                affected: Some(affected),
                error: None,
            }),
            Err(e) => { let err: rusqlite::Error = e; error_response(err.to_string()) }
        }
    }
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
    
    let serve_result: std::result::Result<(), std::io::Error> = axum::serve(listener, app.into_make_service()).await;
    serve_result.map_err(|e| miette!("Server error: {}", e))?;
    
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_studio_state_creation() {
        let file = NamedTempFile::new().unwrap();
        let state = StudioState::new(file.path().to_str().unwrap());
        assert!(state.is_ok());
    }

    #[test]
    fn test_extract_row_values() {
        let conn = Connection::open_in_memory().unwrap();
        
        conn.execute("CREATE TABLE test (
            text_col TEXT,
            int_col INTEGER,
            float_col REAL,
            null_col TEXT
        )", []).unwrap();
        
        conn.execute("INSERT INTO test VALUES (?, ?, ?, ?)", 
            ("hello", 42, 3.14, rusqlite::types::Null)).unwrap();
            
        let mut stmt = conn.prepare("SELECT * FROM test").unwrap();
        let mut rows = stmt.query([]).unwrap();
        
        if let Ok(Some(row)) = rows.next() {
            let values = extract_row_values(row, 4);
            assert_eq!(values[0], "hello");
            assert_eq!(values[1], "42");
            assert_eq!(values[2], "3.14");
            assert_eq!(values[3], "NULL");
        } else {
            panic!("No row found");
        }
    }

    #[test]
    fn test_error_response_format() {
        let err_json = error_response("Test Error".to_string());
        let val = serde_json::to_value(&err_json.0).unwrap();
        
        assert_eq!(val["error"], "Test Error");
        assert!(val["rows"].as_array().unwrap().is_empty());
        assert!(val["columns"].as_array().unwrap().is_empty());
    }
}
