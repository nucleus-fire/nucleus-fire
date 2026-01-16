# Building Content-Heavy Apps (CMS)

Nucleus is ideal for Content Management Systems due to its strict typing and efficient database access via Photon.

## 1. Modeling Content
Unlike legacy frameworks that use loose schemas, Nucleus enforces structure using Rust structs and the Photon ORM.

**`src/models.rs`:**
```rust
use nucleus_std::impl_model;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct BlogPost {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub content: String, // stored as Markdown or HTML
    pub published: bool,
}

impl_model!(BlogPost, "posts");
```

## 2. The Admin Interface
Use **Islands** for rich text editing and **Actions** for saving data.

**`src/views/admin/edit.ncl`:**
```html
<n:view title="Edit Post" layout="admin">
    <n:form action="update_post">
        <input type="hidden" name="id" value="{post.id}" />
        <input type="text" name="title" value="{post.title}" />
        
        <!-- Interactive Markdown Editor Island -->
        <n:island src="src/components/MarkdownEditor.ncl" 
                  client:load 
                  content="{post.content}" />
                  
        <button type="submit">Save Changes</button>
    </n:form>
</n:view>
```

## 3. Serving Content
Use **Dynamic Routes** to serve pages based on slugs.

**`src/views/blog/[slug].ncl`:**
```html
<n:view title="{post.title}">
    <n:script>
        use crate::models::BlogPost;
        // Fetch post by slug from URL param
        let post = BlogPost::query()
            .where("slug", params.get("slug").unwrap())
            .one()
            .await
            .expect("Post not found");
            
        // Render Markdown to HTML (server-side)
        let html_content = nucleus::markdown::render(&post.content);
    </n:script>

    <article class="prose lg:prose-xl">
        <h1>{post.title}</h1>
        <div class="content">
            <n:text value="html_content" escape="false" />
        </div>
    </article>
</n:view>
```

## Key Benefits
1.  **SEO**: Content is purely server-rendered (SSR).
2.  **Performance**: Static pages are served in milliseconds.
3.  **Safety**: Invalid slugs or missing data return 404s type-safely.
