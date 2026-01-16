# Developer Tools Suite

Nucleus V3.5 introduces a powerful suite of developer tools designed to accelerate your workflow, debug faster, and gain deeper insights into your application.

## 1. Nucleus Console (REPL)

The **Nucleus Console** is an interactive Read-Eval-Print Loop (REPL) that runs within your application's context.

### Features
- **Async Runtime**: Execute `await` code directly.
- **Database Access**: Run queries using `db()` or standard ORM methods.
- **Auto-Imports**: All `nucleus_std` preludes are available.

### Usage
```bash
nucleus console
```

### Examples
```rust
// Fetch users
> db().fetch_all("SELECT * FROM users").await

// Find specific user
> User::find(1).await

// Send test email
> Postman::send("test@example.com", "Hello", "World").await
```

## 2. Database Studio

**Database Studio** is a visual interface for managing your database, viewing schemas, and running SQL queries.

### Features
- **Table Browser**: View and filter table data.
- **Schema Inspector**: Visualize column types and constraints.
- **SQL Scratchpad**: Run arbitrary SQL with results displayed in a table.
- **Dark Mode**: Sleek, modern interface.

### Usage
```bash
nucleus studio
```
The studio will launch at `http://localhost:4000`.

## 3. Request Profiler

The **Request Profiler** middleware automatically instrument requests in development mode, providing real-time performance insights.

### Features
- **Request Timing**: Track total duration of every request.
- **Slow Query Detection**: warnings for requests taking >100ms.
- **Status Codes**: Color-coded output for 2xx, 4xx, 5xx responses.

### Configuration
Enabled by default in `dev` mode. Configure in `app.rs`:

```rust
use atom::middleware::profiler::{ProfilerConfig, LogFormat};

let config = ProfilerConfig::default()
    .with_slow_threshold_ms(200)
    .with_format(LogFormat::Json);
```

### Output
```text
  GET /api/users 200 45.2ms
  POST /api/login 200 120.5ms ⚠️ SLOW
```

## 4. AI Error Assistant

The **AI Error Assistant** intercepts server errors (5xx) and uses an LLM (if configured) to suggest fixes.

### Features
- **Smart Analysis**: Pattern matching for common errors (missing columns, auth).
- **AI Suggestions**: Deep analysis using OpenAI-compatible APIs.
- **Actionable Fixes**: Suggests CLI commands (e.g., `nucleus db migrate`).

### Configuration
Set your API key in `.env`:
```bash
OPENAI_API_KEY=sk-...
```

Or configure manually in middleware:
```rust
use atom::middleware::ai_assist::AiAssistConfig;

let config = AiAssistConfig::default().with_model("gpt-4");
```
