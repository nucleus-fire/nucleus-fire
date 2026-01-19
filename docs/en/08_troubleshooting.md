# Troubleshooting & Common Issues

## Build Issues

### "Database initialization failed: unable to open database file"
**Cause**: SQLite database file doesn't exist and auto-creation was disabled.
**Fix**: Nucleus auto-creates SQLite databases. Ensure your config uses:
```toml
[database]
url = "sqlite:data.db"
```

### "Build failed due to missing tests" (Guardian Error)
**Cause**: The Guardian enforces TDD. You wrote an `<n:script>` function but no test.
**Fix**: Add a test block:
```html
<n:test>
fn test_my_logic() {
    assert_eq!(calculate_total(10, 5), 15);
}
</n:test>
```

### "unresolved import" Errors
**Cause**: Missing dependency or feature flag.
**Fix**: Check your `Cargo.toml`:
```toml
tower-http = { version = "0.5", features = ["compression-full", "fs", "set-header"] }
```

---

## Runtime Issues

### "Server panic: Invalid route - conflict with previously registered route"
**Cause**: Two routes overlap (e.g., `/docs` and `/docs/*`).
**Fix**: Use specific routes for static files:
```rust
.route_service("/docs/manifest.json", ServeFile::new("static/docs/manifest.json"))
```

### "Port 3000 already in use"
**Cause**: Another process is using the port.
**Fix**:
```bash
# Find and kill the process
lsof -i :3000 | awk 'NR!=1 {print $2}' | xargs kill

# Or change the port in nucleus.config
[server]
port = 3001
```

### "Borrow checker errors in async closures"
**Cause**: Async closures capture variables by reference.
**Fix**: Use `move` to take ownership:
```rust
Pulse::enqueue(move || async move {
    // Variables are moved here
    send_email(&email).await
})
```

---

## View & CSS Issues

### "Styles aren't applying"
**Cause**: Nucleus uses Atomic CSS. Only `c-` prefixed classes are processed.
**Fix**: Use utility classes:
```html
<!-- ✅ Works -->
<div class="c-p-4 c-bg-blue-500">Content</div>

<!-- ❌ Won't work -->
<div class="my-custom-class">Content</div>
```

Or define custom styles:
```html
<n:style>
.my-custom-class {
    padding: 1rem;
    background: blue;
}
</n:style>
```

### "Form not submitting"
**Cause**: Missing `name` attributes or `method`.
**Fix**:
```html
<form method="POST">
    <input type="email" name="email" required>
    <button type="submit">Submit</button>
</form>
```

### "Form validation not working"
**Cause**: Using native HTML instead of `<n:form>`.
**Fix**: Use Nucleus form tags for built-in validation:
```html
<n:form>
    <n:input name="email" type="email" required="true" />
</n:form>
```

---

## Database Issues

### "Migration not running"
**Cause**: Migration file not in `migrations/` folder or wrong naming.
**Fix**: Ensure correct structure:
```
migrations/
  20251227000000_create_users.sql
  20251228000000_add_indexes.sql
```

### "Query returns empty results"
**Cause**: Database not seeded or wrong table name.
**Debug**:
```bash
sqlite3 data.db ".tables"
sqlite3 data.db "SELECT * FROM users LIMIT 5;"
```

---

## Performance Issues

### "Slow response times"
**Causes & Fixes**:

1. **Blocking I/O**: Use `async/await` for all database and HTTP calls
2. **Missing indexes**: Add indexes on frequently queried columns
3. **No caching**: Implement caching for expensive queries:
```rust
Cache::remember("key", 300, || async { expensive_query().await }).await
```

### "High memory usage"
**Fix**: Nucleus uses arena allocation per request. If memory is high:
- Check for memory leaks in long-running background jobs
- Reduce batch sizes in bulk operations
- Stream large files instead of loading into memory

---

## Getting Help

1. **Check the logs**: Server logs show detailed error messages
2. **Enable verbose mode**: `RUST_LOG=debug cargo run`
3. **Search issues**: Check GitHub issues
4. **Community**: Join our Discord for help
