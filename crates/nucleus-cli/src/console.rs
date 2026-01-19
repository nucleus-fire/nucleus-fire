//! Nucleus Console - Interactive REPL
//!
//! Provides an interactive command-line interface for:
//! - Running database queries
//! - Inspecting application state

#![forbid(unsafe_code)]

use comfy_table::{Cell, Color, Table};
use miette::{miette, Result};
use rusqlite::{types::ValueRef, Connection};
use rustyline::error::ReadlineError;
use rustyline::{Config, DefaultEditor, EditMode};
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════
// CONSOLE CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════

/// Console configuration options
#[derive(Debug, Clone)]
pub struct ConsoleConfig {
    /// Database path (SQLite file)
    pub database_path: Option<String>,
    /// History file path  
    pub history_file: PathBuf,
}

impl Default for ConsoleConfig {
    fn default() -> Self {
        let history_file = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".nucleus_console_history");

        Self {
            database_path: None,
            history_file,
        }
    }
}

impl ConsoleConfig {
    /// Create config with database path
    pub fn with_database(mut self, path: impl Into<String>) -> Self {
        self.database_path = Some(path.into());
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CONSOLE COMMANDS
// ═══════════════════════════════════════════════════════════════════════════

/// Built-in console commands (prefixed with .)
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Help,
    Exit,
    Clear,
    Tables,
    Describe(String),
    Info,
    Sql(String),
}

impl Command {
    /// Parse input into a command
    pub fn parse(input: &str) -> Self {
        let trimmed = input.trim();

        if let Some(stripped) = trimmed.strip_prefix('.') {
            let parts: Vec<&str> = stripped.split_whitespace().collect();
            match parts.first().map(|s| s.to_lowercase()).as_deref() {
                Some("help") | Some("h") | Some("?") => Command::Help,
                Some("exit") | Some("quit") | Some("q") => Command::Exit,
                Some("clear") | Some("cls") => Command::Clear,
                Some("tables") | Some("t") => Command::Tables,
                Some("describe") | Some("desc") | Some("d") => parts
                    .get(1)
                    .map_or(Command::Help, |t| Command::Describe((*t).to_string())),
                Some("info") | Some("i") => Command::Info,
                _ => Command::Sql(trimmed.to_string()),
            }
        } else if !trimmed.is_empty() {
            Command::Sql(trimmed.to_string())
        } else {
            Command::Help
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CONSOLE SESSION
// ═══════════════════════════════════════════════════════════════════════════

/// Interactive console session
pub struct Console {
    config: ConsoleConfig,
    conn: Option<Connection>,
    editor: DefaultEditor,
}

impl Console {
    /// Create a new console
    pub fn new(config: ConsoleConfig) -> Result<Self> {
        let rl_config = Config::builder()
            .edit_mode(EditMode::Emacs)
            .auto_add_history(true)
            .build();

        let editor = DefaultEditor::with_config(rl_config)
            .map_err(|e| miette!("Failed to create editor: {}", e))?;

        Ok(Self {
            config,
            conn: None,
            editor,
        })
    }

    /// Connect to database
    pub fn connect(&mut self) -> Result<()> {
        let path = self
            .config
            .database_path
            .as_ref()
            .ok_or_else(|| miette!("No database path configured"))?;

        println!("  Connecting to {}...", path);

        let conn = Connection::open(path).map_err(|e| miette!("Database error: {}", e))?;

        self.conn = Some(conn);
        println!("  \x1b[32m✓\x1b[0m Connected!\n");
        Ok(())
    }

    /// Run the REPL loop
    pub fn run(&mut self) -> Result<()> {
        self.print_banner();
        let _ = self.editor.load_history(&self.config.history_file);

        loop {
            match self.editor.readline("\x1b[36mnucleus>\x1b[0m ") {
                Ok(line) => match Command::parse(&line) {
                    Command::Exit => {
                        println!("\n  Goodbye! 👋\n");
                        break;
                    }
                    Command::Help => self.print_help(),
                    Command::Clear => self.clear_screen(),
                    Command::Tables => self.list_tables()?,
                    Command::Describe(t) => self.describe_table(&t)?,
                    Command::Info => self.show_info()?,
                    Command::Sql(q) => self.execute_query(&q)?,
                },
                Err(ReadlineError::Interrupted) => println!("\n  (Use .exit or Ctrl+D to quit)"),
                Err(ReadlineError::Eof) => {
                    println!("\n  Goodbye! 👋\n");
                    break;
                }
                Err(e) => eprintln!("  \x1b[31mError:\x1b[0m {:?}", e),
            }
        }

        let _ = self.editor.save_history(&self.config.history_file);
        Ok(())
    }

    fn print_banner(&self) {
        println!("\n  \x1b[1;36m╔══════════════════════════════════════════╗\x1b[0m");
        println!("  \x1b[1;36m║\x1b[0m   ⚛️  \x1b[1mNucleus Console\x1b[0m                    \x1b[1;36m║\x1b[0m");
        println!(
            "  \x1b[1;36m║\x1b[0m   Interactive Database REPL              \x1b[1;36m║\x1b[0m"
        );
        println!("  \x1b[1;36m╚══════════════════════════════════════════╝\x1b[0m");
        println!("\n  Type \x1b[33m.help\x1b[0m for commands, \x1b[33m.exit\x1b[0m to quit\n");
    }

    fn print_help(&self) {
        println!("\n  \x1b[1mCommands:\x1b[0m");
        println!("    \x1b[33m.help\x1b[0m            Show this help");
        println!("    \x1b[33m.exit\x1b[0m            Exit console");
        println!("    \x1b[33m.tables\x1b[0m          List all tables");
        println!("    \x1b[33m.describe TABLE\x1b[0m  Show table structure");
        println!("    \x1b[33m.info\x1b[0m            Database info");
        println!("    \x1b[33m.clear\x1b[0m           Clear screen\n");
        println!("  \x1b[1mSQL:\x1b[0m Enter any SQL query directly\n");
    }

    fn clear_screen(&self) {
        print!("\x1b[2J\x1b[1;1H");
        self.print_banner();
    }

    fn list_tables(&self) -> Result<()> {
        let conn = self
            .conn
            .as_ref()
            .ok_or_else(|| miette!("Not connected to database"))?;

        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .map_err(|e| miette!("Query error: {}", e))?;

        let tables: Vec<String> = stmt
            .query_map([], |row: &rusqlite::Row| row.get(0))
            .map_err(|e| miette!("Query error: {}", e))?
            .filter_map(|r| r.ok())
            .collect();

        println!();
        if tables.is_empty() {
            println!("  \x1b[90mNo tables found\x1b[0m");
        } else {
            println!("  \x1b[1mTables:\x1b[0m");
            for name in tables {
                println!("    • {}", name);
            }
        }
        println!();
        Ok(())
    }

    fn describe_table(&self, table: &str) -> Result<()> {
        let conn = self
            .conn
            .as_ref()
            .ok_or_else(|| miette!("Not connected to database"))?;

        let mut stmt = conn
            .prepare(&format!("PRAGMA table_info({})", table))
            .map_err(|e| miette!("Query error: {}", e))?;

        let mut t = Table::new();
        t.set_header(vec!["Column", "Type", "Nullable", "Default", "PK"]);

        let rows = stmt
            .query_map([], |row: &rusqlite::Row| {
                Ok((
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i32>(3)?,
                    row.get::<_, Option<String>>(4)?,
                    row.get::<_, i32>(5)?,
                ))
            })
            .map_err(|e| miette!("Query error: {}", e))?;

        let mut count = 0;
        for row in rows.flatten() {
            t.add_row(vec![
                Cell::new(&row.0).fg(Color::Cyan),
                Cell::new(&row.1),
                Cell::new(if row.2 == 0 { "YES" } else { "NO" }),
                Cell::new(row.3.as_deref().unwrap_or("-")),
                Cell::new(if row.4 > 0 { "✓" } else { "" }).fg(Color::Yellow),
            ]);
            count += 1;
        }

        println!();
        if count == 0 {
            println!("  \x1b[31mTable '{}' not found\x1b[0m", table);
        } else {
            println!("  \x1b[1mTable:\x1b[0m {}\n{}", table, t);
        }
        println!();
        Ok(())
    }

    fn show_info(&self) -> Result<()> {
        let conn = self
            .conn
            .as_ref()
            .ok_or_else(|| miette!("Not connected to database"))?;

        let version: String = conn
            .query_row("SELECT sqlite_version()", [], |row: &rusqlite::Row| {
                row.get(0)
            })
            .map_err(|e| miette!("Query error: {}", e))?;

        println!("\n  \x1b[1mDatabase Information:\x1b[0m");
        println!("    SQLite Version: {}", version);
        println!(
            "    Path: {}\n",
            self.config.database_path.as_deref().unwrap_or("N/A")
        );
        Ok(())
    }

    fn execute_query(&self, query: &str) -> Result<()> {
        let conn = self
            .conn
            .as_ref()
            .ok_or_else(|| miette!("Not connected to database"))?;

        let start = std::time::Instant::now();
        let upper = query.trim().to_uppercase();
        let is_select = upper.starts_with("SELECT") || upper.starts_with("PRAGMA");

        if is_select {
            let mut stmt = conn
                .prepare(query)
                .map_err(|e| miette!("Query error: {}", e))?;

            let col_count = stmt.column_count();
            let col_names: Vec<String> =
                stmt.column_names().iter().map(|s| s.to_string()).collect();

            let mut table = Table::new();
            table.set_header(&col_names);

            let mut row_count = 0;
            let rows = stmt
                .query_map([], |row: &rusqlite::Row| {
                    let mut values = Vec::new();
                    for i in 0..col_count {
                        let val = match row.get_ref(i).ok() {
                            Some(ValueRef::Null) => "NULL".to_string(),
                            Some(ValueRef::Integer(n)) => n.to_string(),
                            Some(ValueRef::Real(f)) => f.to_string(),
                            Some(ValueRef::Text(t)) => String::from_utf8_lossy(t).to_string(),
                            Some(ValueRef::Blob(b)) => format!("[{} bytes]", b.len()),
                            None => "?".to_string(),
                        };
                        values.push(val);
                    }
                    Ok(values)
                })
                .map_err(|e| miette!("Query error: {}", e))?;

            for row in rows.flatten() {
                table.add_row(row);
                row_count += 1;
            }

            let elapsed = start.elapsed();
            println!();
            if row_count == 0 {
                println!("  \x1b[90mNo rows returned\x1b[0m ({:.2?})", elapsed);
            } else {
                println!("{}", table);
                println!("  \x1b[90m{} row(s) ({:.2?})\x1b[0m", row_count, elapsed);
            }
            println!();
        } else {
            let affected = conn
                .execute(query, [])
                .map_err(|e| miette!("Query error: {}", e))?;
            let elapsed = start.elapsed();
            println!(
                "\n  \x1b[32m✓\x1b[0m {} row(s) affected ({:.2?})\n",
                affected, elapsed
            );
        }
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PUBLIC API
// ═══════════════════════════════════════════════════════════════════════════

/// Run the interactive console
pub async fn run_console(database_url: Option<String>) -> Result<()> {
    let mut config = ConsoleConfig::default();

    if let Some(url) = database_url {
        let path = url.strip_prefix("sqlite:").unwrap_or(&url);
        config = config.with_database(path);
    } else if let Ok(url) = std::env::var("DATABASE_URL") {
        let path = url.strip_prefix("sqlite:").unwrap_or(&url);
        config = config.with_database(path);
    } else if let Ok(content) = std::fs::read_to_string("nucleus.config") {
        if let Some(url) = extract_database_url(&content) {
            let path = url.strip_prefix("sqlite:").unwrap_or(&url);
            config = config.with_database(path);
        }
    }

    if config.database_path.is_none() {
        eprintln!("\n  \x1b[31m✗\x1b[0m No database URL found.");
        eprintln!("    Set DATABASE_URL or add to nucleus.config\n");
        return Ok(());
    }

    let mut console = Console::new(config)?;
    console.connect()?;
    console.run()?;
    Ok(())
}

fn extract_database_url(content: &str) -> Option<String> {
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("url") && line.contains('=') {
            let value = line.split('=').nth(1)?.trim();
            let url = value.trim_matches(|c| c == '"' || c == '\'');
            if !url.starts_with("${") {
                return Some(url.to_string());
            }
        }
    }
    None
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_parse_help() {
        assert_eq!(Command::parse(".help"), Command::Help);
        assert_eq!(Command::parse(".h"), Command::Help);
    }

    #[test]
    fn test_command_parse_exit() {
        assert_eq!(Command::parse(".exit"), Command::Exit);
        assert_eq!(Command::parse(".q"), Command::Exit);
    }

    #[test]
    fn test_command_parse_tables() {
        assert_eq!(Command::parse(".tables"), Command::Tables);
    }

    #[test]
    fn test_command_parse_describe() {
        assert_eq!(
            Command::parse(".describe users"),
            Command::Describe("users".to_string())
        );
    }

    #[test]
    fn test_command_parse_sql() {
        assert_eq!(
            Command::parse("SELECT * FROM users"),
            Command::Sql("SELECT * FROM users".to_string())
        );
    }

    #[test]
    fn test_console_config() {
        let config = ConsoleConfig::default().with_database("test.db");
        assert_eq!(config.database_path, Some("test.db".to_string()));
    }

    #[test]
    fn test_extract_database_url() {
        let content = r#"url = "sqlite:./data.db""#;
        assert_eq!(
            extract_database_url(content),
            Some("sqlite:./data.db".to_string())
        );
    }
}
