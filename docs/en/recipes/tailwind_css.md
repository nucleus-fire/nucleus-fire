# Recipe: Tailwind CSS Integration

Nucleus pairs perfectly with Tailwind CSS. Since Nucleus uses standard Rust files, you can simply point the Tailwind CLI at your `src/` directory.

> [!TIP]
> This approach uses the official **Standalone CLI**. It adds **zero overhead** to your Rust compile times.

## 1. Setup

Create a `tailwinc.config.js` in your project root:

```js
/** @type {import('tailwindcss').Config} */
module.exports = {
  // Watch all Rust files for class names
  content: ["./src/**/*.rs", "./templates/**/*.html"],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

## 2. Generate CSS

Run the Tailwind CLI in "watch" mode during development:

```bash
# Using npx
npx tailwindcss -i input.css -o static/output.css --watch

# Or using the standalone binary
./tailwindcss -i input.css -o static/output.css --watch
```

## 3. Usage in Rust

Just use the classes in your HTML strings or templates.

```rust
async fn index() -> Html<&'static str> {
    Html(r#"
        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
            Click Me
        </button>
    "#)
}
```

The CLI will detect `bg-blue-500` in your Rust code and regenerate the CSS instantly.
