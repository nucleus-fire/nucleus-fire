# End-to-End Type Safety

Nucleus provides a unique "Contract-First" development experience where your frontend and backend share the same Rust type definitions. This eliminates an entire class of runtime errors.

---

## The Core Principle

Because Nucleus compiles your Views (`.ncl`) and your Logic (`.rs`) together into a single application (even if parts run on the client via WASM), the Rust compiler acts as your source of truth.

---

## Compile-Time Guarantees

### 1. Database Schema Validation

When you define a model, the compiler ensures all queries are valid:

```rust
// src/models/user.rs
#[derive(Model)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub name: Option<String>,
    // pub age: i32, // Field removed
}
```

```html
<!-- src/views/user.ncl -->
<p>Age: <n:text value={user.age} /></p>
<!-- ❌ Compile Error: no field `age` on type `User` -->
```

The build **fails immediately** before you can deploy broken code.

### 2. Props Type Checking

Component props are validated at compile time:

```html
<!-- src/components/profile.ncl -->
<n:component>
    <n:props>
        user: User,        <!-- Required -->
        show_email: bool = true
    </n:props>
    ...
</n:component>
```

```html
<!-- Usage -->
<n:profile user={current_user} />           <!-- ✅ OK -->
<n:profile user="invalid" />                <!-- ❌ Type mismatch -->
<n:profile />                               <!-- ❌ Missing required prop -->
```

### 3. Server Action Return Types

When you define a Server Action, the return type is checked on both sides:

```rust
#[server]
pub async fn get_user(id: i32) -> Result<User, NucleusError> { 
    // ...
}
```

If you change `User` on the server, the client code calling `get_user` must handle the new `User` shape, or it won't compile.

### 4. Form Validation Types

Forms derive their validation from Rust types:

```rust
#[derive(Form, Validate)]
struct CreateUser {
    #[validate(email)]
    email: String,
    
    #[validate(length(min = 8))]
    password: String,
    
    #[validate(range(min = 18, max = 120))]
    age: i32,
}
```

The NCL form automatically gets the right input types and validation rules.

---

## The Awareness Loop

Here's how type safety flows through your application:

1. **Database Change**: You update a struct in `src/models.rs`
   ```rust
   pub struct User {
       pub username: String,
       // pub age: i32, // Removed field
   }
   ```

2. **Frontend Usage**: Your view attempts to access the removed field
   ```html
   <p>Age: <n:text value={user.age} /></p>
   ```

3. **Compiler Check**: When you run `nucleus run`, the compiler validates
   ```text
   error[E0609]: no field `age` on type `models::User`
   --> src/views/user.rs:14:22
   ```

4. **Instant Feedback**: Build fails before deployment

---

## API Response Contracts

### Typed JSON Responses

```rust
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub meta: ResponseMeta,
}

// Handler
async fn get_users() -> Json<ApiResponse<Vec<User>>> {
    // Return type is enforced
}
```

### Client-Side Consumption

```rust
// In WASM client code
let response: ApiResponse<Vec<User>> = fetch("/api/users").await?;
// Type is known, autocomplete works
for user in response.data {
    println!("{}", user.email);
}
```

---

## Migration Type Safety

Nucleus validates migrations against your models:

```sql
-- migrations/20250101_add_bio.sql
ALTER TABLE users ADD COLUMN bio TEXT;
```

```rust
#[derive(Model)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub bio: Option<String>,  // Must match migration
}
```

If you forget to update the model after adding a column, the ORM will warn about schema mismatches.

---

## Signal & Store Type Safety

Neutron signals are fully typed:

```rust
let count: Signal<i32> = Signal::new(0);
count.set("invalid");  // ❌ Compile error: expected i32

#[derive(Store)]
struct AppState {
    user: Option<User>,
    theme: Theme,
}

let store = Store::new(AppState::default());
store.user = "invalid";  // ❌ Compile error
```

---

## Error Type Propagation

Errors flow through the type system:

```rust
// Define typed errors
#[derive(Error, Diagnostic)]
pub enum AppError {
    #[error("User not found")]
    NotFound,
    
    #[error("Permission denied")]
    Forbidden,
}

// Handler returns typed result
async fn get_user(id: i64) -> Result<User, AppError> {
    User::find(id).await.ok_or(AppError::NotFound)
}

// View must handle the error
<n:action>
    match get_user(id).await {
        Ok(user) => { /* render */ }
        Err(e) => { /* handle specific error */ }
    }
</n:action>
```

---

## Benefits

| Aspect | Without Type Safety | With Nucleus |
|--------|---------------------|--------------|
| API changes | Runtime 500 errors | Compile-time failure |
| Typos in field names | Silent undefined | Compiler error |
| Missing props | Runtime crash | Build fails |
| Form validation | Manual checking | Auto-derived |
| Refactoring | Scary, error-prone | Safe, compiler-guided |

---

## Best Practices

1. **Define models first**: Start with your data structures in Rust
2. **Use derive macros**: `#[derive(Model, Serialize, Validate)]`
3. **Prefer `Result<T, E>`**: Always return typed errors
4. **Avoid `unwrap()`**: Use `?` for error propagation
5. **Run `nucleus check`**: Quick type validation without full build
