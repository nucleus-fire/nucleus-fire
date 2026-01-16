# End-to-End Type Safety

Nucleus provides a unique "Contract-First" development experience where your frontend and backend share the same Rust type definitions.

## How it works

Because Nucleus compiles your Views (`.ncl`) and your Logic (`.rs`) together into a single application (even if parts run on the client via WASM), the Rust compiler acts as your source of truth.

### The "Awareness" Loop

1. **Database Change**: You update a struct in `src/models.rs`.
   ```rust
   pub struct User {
       pub username: String,
       // pub age: i32, // Removed field
   }
   ```

2. **Frontend Usage**: Your view `src/views/user.ncl` attempts to access the removed field.
   ```html
   <p>Age: <n:text value={user.age} /></p>
   ```

3. **Compiler Check**: When you run `nucleus run` (or `cargo build`), the compiler attempts to compile the view code against the updated struct.

4. **Instant Feedback**: The build **fails** immediately.
   ```text
   error[E0609]: no field `age` on type `models::User`
   --> src/views/user.rs:14:22
   ```

This ensures your frontend is *always* aware of API and Database changes before you even deploy.

## Server Actions (`#[server]`)

When you define a Server Action, the return type is checked on both sides.

```rust
#[server]
pub async fn get_user(id: i32) -> Result<User, NucleusError> { ... }
```

If you change `User` on the server, the client code calling `get_user` must handle the new `User` shape, or it won't compile.
