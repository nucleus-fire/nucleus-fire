# Developer Tools Suite

Nucleus includes a comprehensive suite of developer tools designed to accelerate your workflow, debug faster, and gain deeper insights into your application.

---

## 1. Nucleus Console (REPL)

The **Nucleus Console** is an interactive Read-Eval-Print Loop (REPL) that runs within your application's context.

### Features
- **Async Runtime**: Execute `await` code directly
- **Database Access**: Run queries using `db()` or ORM methods
- **Auto-Imports**: All `nucleus_std` preludes are available
- **History**: Command history with up/down arrows
- **Tab Completion**: Auto-complete for methods and variables

### Usage

```bash
nucleus console
```

### Examples

```rust
// Fetch all users
> User::all().await
[User { id: 1, email: "alice@example.com" }, ...]

// Find specific user
> User::find(1).await
Some(User { id: 1, email: "alice@example.com" })

// Create a user
> User::create()
    .set("email", "new@example.com")
    .set("name", "New User")
    .save()
    .await
Ok(User { id: 5, ... })

// Run raw SQL
> db().sqlite().fetch_all("SELECT * FROM users WHERE active = 1").await
[...]

// Send test email
> Postman::send("test@example.com", "Test Subject", "Hello!").await
Ok(())

// Check cache
> Cache::get::<String>("session:123")
Some("user_data")
```

### Configuration

```toml
# nucleus.config
[console]
history_file = ".nucleus_history"
max_history = 1000
prompt = "nucleus> "
```

---

## 2. Database Studio

**Database Studio** is a visual interface for managing your database, viewing schemas, and running SQL queries.

### Features
- **Table Browser**: View, filter, and paginate table data
- **Schema Inspector**: Column types, indexes, and constraints
- **SQL Scratchpad**: Run queries with syntax highlighting
- **Query History**: Track and re-run previous queries
- **Export**: Download results as CSV or JSON
- **Dark Mode**: Modern, eye-friendly interface

### Usage

```bash
nucleus studio
```

Opens at `http://localhost:4000`

### Screenshots

The studio provides:
- Left sidebar with table list
- Main area with data grid
- Bottom panel for SQL queries
- Right panel for column details

### API Access

The studio exposes an API for programmatic access:

```bash
# List tables
curl http://localhost:4000/api/tables

# Run query
curl -X POST http://localhost:4000/api/query \
  -H "Content-Type: application/json" \
  -d '{"sql": "SELECT * FROM users LIMIT 10"}'
```

---

## 3. Request Profiler

The **Request Profiler** automatically instruments requests in development mode.

### Features
- **Request Timing**: Total duration of every request
- **Database Queries**: Count and timing of SQL queries
- **Slow Query Detection**: Warnings for requests >100ms
- **Status Codes**: Color-coded output (2xx green, 4xx yellow, 5xx red)
- **Memory Usage**: Per-request memory allocation

### Output

```text
  GET    /               200   12.3ms  │ 2 queries  │  1.2MB
  GET    /api/users      200   45.2ms  │ 5 queries  │  2.4MB
  POST   /api/login      200  120.5ms  │ 8 queries  │  3.1MB ⚠️ SLOW
  GET    /static/app.js  200    0.8ms  │ 0 queries  │  0.1MB
  POST   /api/upload     500   89.4ms  │ 3 queries  │ 15.2MB ❌ ERROR
```

### Configuration

```rust
// src/middleware.rs
use nucleus_std::middleware::profiler::{ProfilerConfig, LogFormat};

let config = ProfilerConfig::default()
    .with_slow_threshold_ms(200)
    .with_format(LogFormat::Pretty)  // or Json
    .with_memory_tracking(true)
    .with_query_tracking(true);
```

### JSON Output

For structured logging:

```json
{
  "method": "GET",
  "path": "/api/users",
  "status": 200,
  "duration_ms": 45.2,
  "queries": 5,
  "query_time_ms": 32.1,
  "memory_bytes": 2516582
}
```

---

## 4. AI Error Assistant

The **AI Error Assistant** intercepts errors and suggests fixes using AI.

### Features
- **Pattern Matching**: Recognizes common errors
- **AI Analysis**: Deep analysis using LLM
- **Actionable Fixes**: Suggests CLI commands
- **Code Suggestions**: Provides fix snippets

### How It Works

1. Error occurs (5xx response or panic)
2. Assistant captures stack trace and context
3. Pattern matching tries common fixes
4. If configured, AI provides deeper analysis
5. Suggestions are logged to console

### Example Output

```text
❌ Error: Column 'bio' not found in table 'users'

🔍 Analysis:
   The migration for 'bio' column may not have been applied.

💡 Suggested Fix:
   1. Check if migration exists: nucleus db status
   2. Create migration if needed: nucleus db new add_bio_to_users
   3. Run migrations: nucleus db up

📝 Code Fix (if migration exists):
   -- Add to your migration:
   ALTER TABLE users ADD COLUMN bio TEXT;
```

### Configuration

```bash
# .env
OPENAI_API_KEY=sk-...
NUCLEUS_AI_ASSIST=true
```

```rust
// src/middleware.rs
use nucleus_std::middleware::ai_assist::AiAssistConfig;

let config = AiAssistConfig::default()
    .with_model("gpt-4")
    .with_context_lines(10)  // Lines of code to include
    .with_include_schema(true);  // Include DB schema
```

---

## 5. DevTools Panel (Browser)

Nucleus injects a DevTools panel into development builds.

### Features
- **Component Tree**: Visualize component hierarchy
- **State Inspector**: View Neutron signals and stores
- **Network Panel**: RPC calls with timing
- **HMR Status**: Hot reload status and timing
- **Performance**: Hydration and render timing

### Toggle

Press `Ctrl+Shift+D` (or `Cmd+Shift+D` on Mac) to toggle the panel.

### Disable in Development

```toml
# nucleus.config
[dev]
devtools_panel = false
```

---

## 6. Hot Module Replacement (HMR)

Real-time updates without page refresh.

### How It Works

1. You save a `.ncl` file
2. Compiler generates diff
3. WebSocket sends update signal
4. Client patches DOM
5. State is preserved

### HMR Timing

```text
[HMR] File changed: src/views/home.ncl
[HMR] Compiled in 45ms
[HMR] Patch applied in 12ms
[HMR] Total: 57ms
```

### State Preservation

Neutron signals are automatically preserved:

```html
<n:island client:load>
    <n:script>
        let count = Signal::new(0);  // Preserved across HMR
    </n:script>
    <p>Count: {count}</p>
</n:island>
```

---

## 7. Debug Mode

Enable verbose logging:

```bash
# All debug output
NUCLEUS_DEBUG=1 nucleus dev

# Specific modules
NUCLEUS_DEBUG=router,db nucleus dev

# Log level
NUCLEUS_LOG=debug nucleus dev
```

### Log Levels

| Level | Description |
|-------|-------------|
| `error` | Only errors |
| `warn` | Errors and warnings |
| `info` | General info (default) |
| `debug` | Detailed debugging |
| `trace` | Everything |

---

## 8. Performance Profiling

Generate performance profiles:

```bash
# CPU profiling
nucleus run --profile cpu

# Memory profiling  
nucleus run --profile memory

# Generate flamegraph
nucleus run --profile flamegraph
```

### Output

Profiles are saved to:
- `profile/cpu.json` - Chrome DevTools format
- `profile/memory.json` - Allocation tracking
- `profile/flamegraph.svg` - Visual flamegraph

### Viewing Profiles

```bash
# Open in Chrome DevTools
open profile/cpu.json

# View flamegraph in browser
open profile/flamegraph.svg
```

---

## 9. Test Runner

Integrated test runner:

```bash
# Run all tests
nucleus test

# Watch mode
nucleus test --watch

# Specific file
nucleus test tests/api_test.rs

# With coverage
nucleus test --coverage
```

### Coverage Report

```text
╭───────────────────────────────────────────────────────────╮
│                    Coverage Report                         │
├─────────────────────────────────┬──────────┬──────────────┤
│ File                            │ Coverage │ Uncovered    │
├─────────────────────────────────┼──────────┼──────────────┤
│ src/services/auth.rs            │   94.2%  │ Lines 45-48  │
│ src/services/db.rs              │   87.3%  │ Lines 102    │
│ src/models/user.rs              │  100.0%  │              │
├─────────────────────────────────┼──────────┼──────────────┤
│ Total                           │   92.1%  │              │
╰─────────────────────────────────┴──────────┴──────────────╯
```

---

## 10. CLI Quick Reference

| Command | Description |
|---------|-------------|
| `nucleus console` | Interactive REPL |
| `nucleus studio` | Database GUI |
| `nucleus test` | Run tests |
| `nucleus test --watch` | Watch mode |
| `nucleus test --coverage` | With coverage |
| `NUCLEUS_DEBUG=1 nucleus dev` | Debug mode |
