<n:view title="Nucleus Recipes">
    <div class="container">
        <h1>🍳 Nucleus Recipes</h1>
        <p>A collection of examples demonstrating Nucleus capabilities.</p>
        
        <div class="grid">
            <a href="/hello" class="card">
                <h2>👋 Hello World</h2>
                <p>Basic routing and Title rendering.</p>
            </a>
            <a href="/counter" class="card">
                <h2>🔢 Counter</h2>
                <p>Client-side interactivity (WASM).</p>
            </a>
            <a href="/todo" class="card">
                <h2>✅ Todo App</h2>
                <p>Database (SQLite) and Form handling.</p>
            </a>
            <a href="/auth/login" class="card">
                <h2>🔐 Authentication</h2>
                <p>Guards and Sessions.</p>
            </a>
        </div>
        
        <style>
            body { font-family: system-ui; background: #111; color: #eee; }
            .container { max-width: 800px; margin: 2rem auto; }
            .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1rem; margin-top: 2rem; }
            .card { display: block; padding: 1.5rem; background: #222; border-radius: 8px; text-decoration: none; color: inherit; border: 1px solid #333; transition: transform 0.2s; }
            .card:hover { transform: translateY(-3px); border-color: #555; }
            h2 { margin: 0 0 0.5rem 0; color: #4facfe; }
        </style>
    </div>
</n:view>
