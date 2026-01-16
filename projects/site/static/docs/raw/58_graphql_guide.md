# GraphQL Guide

Build GraphQL APIs with Nucleus using async-graphql integration, DataLoader for N+1 prevention, and built-in authentication.

## Quick Start

```rust
use nucleus_std::graph::{
    GraphQL, Schema, Object, SimpleObject, Context,
    EmptyMutation, EmptySubscription
};

#[derive(SimpleObject)]
struct User {
    id: i32,
    name: String,
    email: String,
}

struct Query;

#[Object]
impl Query {
    async fn user(&self, id: i32) -> Option<User> {
        // Fetch from database
        Some(User {
            id,
            name: "Alice".into(),
            email: "alice@example.com".into(),
        })
    }
    
    async fn users(&self) -> Vec<User> {
        vec![]
    }
}

// Build schema
let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

// Mount to router
router.route("/graphql", post(graphql_handler));
router.route("/graphql/playground", get(GraphQL::playground()));
```

## Schema Types

### Simple Objects (Output Types)

```rust
use nucleus_std::graph::SimpleObject;

#[derive(SimpleObject)]
struct Product {
    id: ID,
    name: String,
    price: f64,
    #[graphql(skip)]  // Skip this field
    internal_code: String,
}
```

### Input Objects

```rust
use nucleus_std::graph::InputObject;

#[derive(InputObject)]
struct CreateUserInput {
    name: String,
    email: String,
    #[graphql(default)]
    role: Option<String>,
}
```

### Resolvers

```rust
use nucleus_std::graph::Object;

struct Query;

#[Object]
impl Query {
    // Simple resolver
    async fn hello(&self) -> String {
        "Hello, World!".into()
    }
    
    // With arguments
    async fn user(&self, id: i32) -> Option<User> {
        db::users().find(id).await
    }
    
    // With context
    async fn me(&self, ctx: &Context<'_>) -> Option<User> {
        let auth = ctx.data::<AuthUser>()?;
        db::users().find(auth.id).await
    }
}
```

### Mutations

```rust
struct Mutation;

#[Object]
impl Mutation {
    async fn create_user(&self, input: CreateUserInput) -> Result<User, GraphError> {
        let user = db::users()
            .insert(&input)
            .await?;
        Ok(user)
    }
    
    async fn update_user(&self, id: i32, input: UpdateUserInput) -> Result<User, GraphError> {
        let user = db::users()
            .find(id)
            .update(&input)
            .await?;
        Ok(user)
    }
    
    async fn delete_user(&self, id: i32) -> Result<bool, GraphError> {
        db::users().delete(id).await?;
        Ok(true)
    }
}
```

## GraphQL Playground

### Built-in Playground

```rust
async fn playground() -> Html<String> {
    GraphQL::playground()
}

// In router
router.route("/graphql/playground", get(playground));
```

### Custom Playground URL

```rust
let html = GraphQL::playground_html("/api/graphql");
```

### GraphiQL Alternative

```rust
let html = GraphQL::graphiql_html("/api/graphql");
```

## DataLoader (N+1 Prevention)

### Basic Usage

```rust
use nucleus_std::graph::DataLoader;

let user_loader: DataLoader<i32, User> = DataLoader::new();

// Load single item
let user = user_loader.load(1, |id| async move {
    db::users().find(id).await
}).await;

// Second call uses cache
let user_again = user_loader.load(1, |_| async { None }).await;
// Returns cached value without calling loader
```

### Batch Loading

```rust
let user_loader: DataLoader<i32, User> = DataLoader::new();

// Load multiple items efficiently
let users = user_loader.load_many(
    vec![1, 2, 3, 4, 5],
    |ids| async move {
        // Single query for all IDs
        db::users()
            .where_in("id", &ids)
            .get()
            .await
            .into_iter()
            .map(|u| (u.id, u))
            .collect()
    }
).await;
```

### In Resolvers

```rust
struct Query;

#[Object]
impl Query {
    async fn posts(&self, ctx: &Context<'_>) -> Vec<Post> {
        let posts = db::posts().all().await;
        posts
    }
}

#[ComplexObject]
impl Post {
    async fn author(&self, ctx: &Context<'_>) -> Option<User> {
        let loader = ctx.data::<DataLoader<i32, User>>()?;
        loader.load(self.author_id, |id| async move {
            db::users().find(id).await
        }).await
    }
}
```

## Authentication

### Auth Guard

```rust
use nucleus_std::graph::AuthGuard;

#[Object]
impl Query {
    async fn profile(&self, ctx: &Context<'_>) -> Result<User, GraphError> {
        let user = ctx.data::<Option<AuthUser>>()?;
        let auth = AuthGuard::require_auth(user)?;
        
        db::users().find(auth.id).await
            .ok_or(GraphError::NotFound("User".into()))
    }
}
```

### Role-Based Access

```rust
#[Object]
impl Mutation {
    async fn delete_user(&self, ctx: &Context<'_>, id: i32) -> Result<bool, GraphError> {
        let user = ctx.data::<Option<AuthUser>>()?;
        
        // Require admin role
        AuthGuard::require_role(user, "admin", |u| &u.role)?;
        
        db::users().delete(id).await?;
        Ok(true)
    }
    
    async fn edit_post(&self, ctx: &Context<'_>, id: i32) -> Result<Post, GraphError> {
        let user = ctx.data::<Option<AuthUser>>()?;
        
        // Allow admin or editor
        AuthGuard::require_any_role(user, &["admin", "editor"], |u| &u.role)?;
        
        // ...
    }
}
```

## Pagination

### Cursor-Based Pagination

```rust
use nucleus_std::graph::{Connection, Edge, PageInfo, PaginationInput};

#[Object]
impl Query {
    async fn users(
        &self,
        pagination: Option<PaginationInput>,
    ) -> Connection<User> {
        let pagination = pagination.unwrap_or_default();
        let limit = pagination.limit();
        
        let users = db::users()
            .limit(limit + 1)
            .get()
            .await;
        
        let has_next = users.len() > limit as usize;
        let users: Vec<_> = users.into_iter().take(limit as usize).collect();
        
        Connection::new(
            users,
            PageInfo {
                has_next_page: has_next,
                has_previous_page: pagination.after.is_some(),
                start_cursor: None,
                end_cursor: None,
            },
            100, // total count
        )
    }
}
```

## Error Handling

### GraphQL Errors

```rust
use nucleus_std::graph::{GraphError, GraphQLError};

#[Object]
impl Mutation {
    async fn create_user(&self, input: CreateUserInput) -> Result<User, GraphError> {
        // Validation
        if input.email.is_empty() {
            return Err(GraphError::ValidationError("Email is required".into()));
        }
        
        // Check permissions
        if !can_create_users() {
            return Err(GraphError::PermissionDenied("Cannot create users".into()));
        }
        
        // Not found
        let org = db::orgs().find(input.org_id).await
            .ok_or(GraphError::NotFound("Organization".into()))?;
        
        // Internal error
        db::users().insert(&input).await
            .map_err(|e| GraphError::InternalError(e.to_string()))
    }
}
```

### Custom Error Extensions

```rust
let error = GraphQLError::new("User not found")
    .with_code("USER_NOT_FOUND");
```

## Complete Example

```rust
use nucleus_std::graph::{
    GraphQL, Schema, Object, SimpleObject, InputObject, Context,
    EmptySubscription, DataLoader, AuthGuard, GraphError,
    Connection, PageInfo, PaginationInput
};

// Types
#[derive(SimpleObject, Clone)]
struct User {
    id: i32,
    name: String,
    email: String,
    role: String,
}

#[derive(SimpleObject)]
struct Post {
    id: i32,
    title: String,
    content: String,
    author_id: i32,
}

#[derive(InputObject)]
struct CreatePostInput {
    title: String,
    content: String,
}

// Query
struct Query;

#[Object]
impl Query {
    async fn me(&self, ctx: &Context<'_>) -> Result<User, GraphError> {
        let user = ctx.data::<Option<AuthUser>>()?;
        let auth = AuthGuard::require_auth(user)?;
        db::users().find(auth.id).await
            .ok_or(GraphError::NotFound("User".into()))
    }
    
    async fn posts(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
    ) -> Result<Connection<Post>, GraphError> {
        let pagination = pagination.unwrap_or_default();
        let posts = db::posts().limit(pagination.limit()).get().await;
        
        Ok(Connection::new(posts, PageInfo {
            has_next_page: false,
            has_previous_page: false,
            start_cursor: None,
            end_cursor: None,
        }, 0))
    }
}

// Mutation
struct Mutation;

#[Object]
impl Mutation {
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        input: CreatePostInput,
    ) -> Result<Post, GraphError> {
        let user = ctx.data::<Option<AuthUser>>()?;
        let auth = AuthGuard::require_auth(user)?;
        
        db::posts().insert(&input, auth.id).await
            .map_err(|e| GraphError::InternalError(e.to_string()))
    }
}

// Setup
async fn setup_graphql() -> Router {
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(DataLoader::<i32, User>::new())
        .finish();
    
    Router::new()
        .route("/graphql", post(|
            State(schema): State<Schema<Query, Mutation, EmptySubscription>>,
            Json(req): Json<GraphQLRequest>,
        | async move {
            // Execute query
        }))
        .route("/graphql/playground", get(GraphQL::playground()))
        .with_state(schema)
}
```
