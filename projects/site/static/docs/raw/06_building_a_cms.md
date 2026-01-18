# Building a CMS with Nucleus

This tutorial walks you through building a complete Content Management System with Nucleus, covering content modeling, admin interface, frontend rendering, and advanced features.

---

## Overview

We'll build a blog CMS with:
- **Posts**: Title, content (Markdown), slug, status
- **Categories**: Hierarchical organization
- **Tags**: Many-to-many relationships
- **Media**: Image uploads and management
- **Admin Dashboard**: CRUD interface with rich text editing

---

## 1. Project Setup

```bash
# Create new project
nucleus new my-cms

cd my-cms
```

---

## 2. Data Modeling

### Post Model

```rust
// src/models/post.rs
use nucleus_std::photon::Model;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Model)]
#[table("posts")]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub featured_image: Option<String>,
    pub status: PostStatus,
    pub author_id: i64,
    pub category_id: Option<i64>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PostStatus {
    Draft,
    Published,
    Scheduled,
    Archived,
}

impl Post {
    pub fn is_published(&self) -> bool {
        matches!(self.status, PostStatus::Published)
    }
    
    pub fn url(&self) -> String {
        format!("/blog/{}", self.slug)
    }
}
```

### Category Model

```rust
// src/models/category.rs
#[derive(Debug, Clone, Serialize, Deserialize, Model)]
#[table("categories")]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent_id: Option<i64>,  // For hierarchy
    pub sort_order: i32,
}

impl Category {
    pub async fn posts(&self) -> Vec<Post> {
        Post::query()
            .filter("category_id", self.id)
            .filter("status", "Published")
            .order_by("published_at", "DESC")
            .all()
            .await
            .unwrap_or_default()
    }
}
```

### Tag Model (Many-to-Many)

```rust
// src/models/tag.rs
#[derive(Debug, Clone, Serialize, Deserialize, Model)]
#[table("tags")]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub slug: String,
}

// Junction table
#[derive(Debug, Clone, Serialize, Deserialize, Model)]
#[table("post_tags")]
pub struct PostTag {
    pub post_id: i64,
    pub tag_id: i64,
}
```

---

## 3. Database Migrations

```bash
nucleus db new create_cms_tables
```

```sql
-- migrations/20250118_create_cms_tables.sql

-- UP
CREATE TABLE categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    description TEXT,
    parent_id INTEGER REFERENCES categories(id),
    sort_order INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    content TEXT NOT NULL,
    excerpt TEXT,
    featured_image TEXT,
    status TEXT DEFAULT 'Draft',
    author_id INTEGER NOT NULL,
    category_id INTEGER REFERENCES categories(id),
    published_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL
);

CREATE TABLE post_tags (
    post_id INTEGER REFERENCES posts(id) ON DELETE CASCADE,
    tag_id INTEGER REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (post_id, tag_id)
);

CREATE INDEX idx_posts_slug ON posts(slug);
CREATE INDEX idx_posts_status ON posts(status);
CREATE INDEX idx_posts_published ON posts(published_at);

-- DOWN
DROP TABLE post_tags;
DROP TABLE tags;
DROP TABLE posts;
DROP TABLE categories;
```

Apply migrations:

```bash
nucleus db up
```

---

## 4. Admin Interface

### Admin Layout

```html
<!-- src/views/admin/layout.ncl -->
<n:layout>
    <!DOCTYPE html>
    <html>
    <head>
        <title>{title} | CMS Admin</title>
        <link href="/static/css/admin.css" rel="stylesheet">
    </head>
    <body class="admin">
        <aside class="sidebar">
            <div class="logo">📝 My CMS</div>
            <nav>
                <n:link href="/admin">Dashboard</n:link>
                <n:link href="/admin/posts">Posts</n:link>
                <n:link href="/admin/categories">Categories</n:link>
                <n:link href="/admin/tags">Tags</n:link>
                <n:link href="/admin/media">Media</n:link>
            </nav>
        </aside>
        <main class="content">
            <n:slot />
        </main>
        <script src="/static/js/admin.js"></script>
    </body>
    </html>
</n:layout>
```

### Posts List

```html
<!-- src/views/admin/posts/index.ncl -->
<n:view title="Posts">
    <n:layout name="admin/layout">
        <n:action>
            let posts = Post::query()
                .order_by("created_at", "DESC")
                .limit(20)
                .all()
                .await?;
        </n:action>
        
        <div class="page-header">
            <h1>Posts</h1>
            <n:link href="/admin/posts/new" class="btn btn-primary">
                + New Post
            </n:link>
        </div>
        
        <table class="data-table">
            <thead>
                <tr>
                    <th>Title</th>
                    <th>Status</th>
                    <th>Date</th>
                    <th>Actions</th>
                </tr>
            </thead>
            <tbody>
                <n:for item={post} in={posts}>
                    <tr>
                        <td>
                            <n:link href="/admin/posts/{post.id}/edit">
                                {post.title}
                            </n:link>
                        </td>
                        <td>
                            <span class="badge badge-{post.status.to_lowercase()}">
                                {post.status}
                            </span>
                        </td>
                        <td>{post.created_at.format("%Y-%m-%d")}</td>
                        <td>
                            <n:link href="/blog/{post.slug}" target="_blank">View</n:link>
                            <n:link href="/admin/posts/{post.id}/edit">Edit</n:link>
                        </td>
                    </tr>
                </n:for>
            </tbody>
        </table>
    </n:layout>
</n:view>
```

### Post Editor

```html
<!-- src/views/admin/posts/edit.ncl -->
<n:view title="Edit Post">
    <n:layout name="admin/layout">
        <n:action>
            let id = params.get("id").and_then(|s| s.parse::<i64>().ok());
            let post = if let Some(id) = id {
                Post::find(id).await?
            } else {
                Post::default()
            };
            let categories = Category::all().await?;
            let tags = Tag::all().await?;
        </n:action>
        
        <form method="POST" action="/admin/posts/save" class="editor-form">
            <input type="hidden" name="id" value="{post.id}" />
            
            <div class="form-group">
                <label>Title</label>
                <input type="text" name="title" value="{post.title}" 
                       placeholder="Enter post title" required />
            </div>
            
            <div class="form-group">
                <label>Slug</label>
                <input type="text" name="slug" value="{post.slug}" 
                       placeholder="post-url-slug" />
            </div>
            
            <div class="form-group">
                <label>Category</label>
                <select name="category_id">
                    <option value="">No category</option>
                    <n:for item={cat} in={categories}>
                        <option value="{cat.id}" 
                                selected={post.category_id == Some(cat.id)}>
                            {cat.name}
                        </option>
                    </n:for>
                </select>
            </div>
            
            <!-- Rich Text Editor Island -->
            <n:island client:load>
                <n:script>
                    // Initialize markdown editor
                </n:script>
                <div class="form-group">
                    <label>Content</label>
                    <textarea name="content" id="editor" rows="20">
                        {post.content}
                    </textarea>
                </div>
            </n:island>
            
            <div class="form-group">
                <label>Status</label>
                <select name="status">
                    <option value="Draft" selected={post.status == "Draft"}>Draft</option>
                    <option value="Published" selected={post.status == "Published"}>Published</option>
                    <option value="Scheduled" selected={post.status == "Scheduled"}>Scheduled</option>
                </select>
            </div>
            
            <div class="form-actions">
                <button type="submit" class="btn btn-primary">Save Post</button>
                <n:link href="/admin/posts" class="btn">Cancel</n:link>
            </div>
        </form>
    </n:layout>
</n:view>
```

### Save Post Action

```html
<!-- src/views/admin/posts/save.ncl -->
<n:view>
    <n:action>
        let id = form.get("id").and_then(|s| s.parse::<i64>().ok());
        let title = form.get("title").unwrap_or(&"".to_string()).clone();
        let slug = form.get("slug").unwrap_or(&slugify(&title)).clone();
        let content = form.get("content").unwrap_or(&"".to_string()).clone();
        let status = form.get("status").unwrap_or(&"Draft".to_string()).clone();
        let category_id = form.get("category_id").and_then(|s| s.parse::<i64>().ok());
        
        if let Some(id) = id {
            // Update existing
            Post::update(id)
                .set("title", &title)
                .set("slug", &slug)
                .set("content", &content)
                .set("status", &status)
                .set("category_id", category_id)
                .set("updated_at", Utc::now())
                .execute()
                .await?;
        } else {
            // Create new
            Post::create()
                .set("title", &title)
                .set("slug", &slug)
                .set("content", &content)
                .set("status", &status)
                .set("category_id", category_id)
                .set("author_id", auth.id)
                .save()
                .await?;
        }
        
        return redirect("/admin/posts");
    </n:action>
</n:view>
```

---

## 5. Frontend Blog

### Blog Index

```html
<!-- src/views/blog/index.ncl -->
<n:view title="Blog">
    <n:layout name="layout">
        <n:action>
            let posts = Post::query()
                .filter("status", "Published")
                .order_by("published_at", "DESC")
                .limit(10)
                .all()
                .await?;
        </n:action>
        
        <h1>Latest Posts</h1>
        
        <div class="posts-grid">
            <n:for item={post} in={posts}>
                <article class="post-card">
                    <n:if condition={post.featured_image.is_some()}>
                        <n:image src={post.featured_image.unwrap()} 
                                 alt={post.title} 
                                 width="400" />
                    </n:if>
                    <h2>
                        <n:link href="/blog/{post.slug}">{post.title}</n:link>
                    </h2>
                    <p class="excerpt">{post.excerpt.unwrap_or_default()}</p>
                    <time>{post.published_at.unwrap().format("%B %d, %Y")}</time>
                </article>
            </n:for>
        </div>
    </n:layout>
</n:view>
```

### Single Post

```html
<!-- src/views/blog/[slug].ncl -->
<n:view title="{post.title}">
    <n:layout name="layout">
        <n:action>
            let slug = params.get("slug").unwrap();
            let post = Post::query()
                .filter("slug", slug)
                .filter("status", "Published")
                .one()
                .await?
                .ok_or(NucleusError::NotFound)?;
                
            // Render markdown to HTML
            let html_content = nucleus::markdown::render(&post.content);
        </n:action>
        
        <article class="blog-post">
            <header>
                <h1>{post.title}</h1>
                <time>{post.published_at.unwrap().format("%B %d, %Y")}</time>
            </header>
            
            <n:if condition={post.featured_image.is_some()}>
                <n:image src={post.featured_image.unwrap()} 
                         alt={post.title} 
                         class="featured" />
            </n:if>
            
            <div class="content prose">
                <n:text value={html_content} escape={false} />
            </div>
        </article>
    </n:layout>
</n:view>
```

---

## 6. Advanced Features

### SEO Optimization

```html
<!-- In your layout -->
<head>
    <title>{title}</title>
    <meta name="description" content="{description}" />
    <meta property="og:title" content="{title}" />
    <meta property="og:description" content="{description}" />
    <meta property="og:image" content="{featured_image}" />
    <link rel="canonical" href="https://example.com{path}" />
</head>
```

### RSS Feed

```html
<!-- src/views/feed.ncl -->
<n:view content_type="application/rss+xml">
    <n:action>
        let posts = Post::query()
            .filter("status", "Published")
            .order_by("published_at", "DESC")
            .limit(20)
            .all()
            .await?;
    </n:action>
    
    <?xml version="1.0" encoding="UTF-8"?>
    <rss version="2.0">
        <channel>
            <title>My Blog</title>
            <link>https://example.com</link>
            <n:for item={post} in={posts}>
                <item>
                    <title>{post.title}</title>
                    <link>https://example.com/blog/{post.slug}</link>
                    <pubDate>{post.published_at.unwrap().to_rfc2822()}</pubDate>
                </item>
            </n:for>
        </channel>
    </rss>
</n:view>
```

### Full-Text Search

```rust
// Using Scout for search
let results = Scout::search("posts")
    .query(&search_term)
    .limit(10)
    .execute()
    .await?;
```

---

## Key Benefits

1. **SEO**: Content is purely server-rendered (SSR)
2. **Performance**: Pages served in milliseconds
3. **Type Safety**: Invalid slugs return 404s type-safely
4. **Security**: SQL injection impossible with Photon ORM
5. **Flexibility**: Markdown + custom components
