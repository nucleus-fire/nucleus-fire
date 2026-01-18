<!-- Main Layout Template -->
<!-- 
    This layout wraps all pages that specify layout="main"
    Demonstrates the recommended layout structure.
-->
<n:layout>
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>{title}</title>
        <meta name="description" content="{description}" />
        <link rel="stylesheet" href="/static/css/main.css" />
    </head>
    <body>
        <header class="site-header">
            <nav class="nav">
                <n:link href="/" class="nav-logo">⚡ Nucleus</n:link>
                <div class="nav-links">
                    <n:link href="/docs">Docs</n:link>
                    <n:link href="/playground">Playground</n:link>
                </div>
            </nav>
        </header>
        
        <main>
            <!-- Page content is injected here -->
            <n:slot />
        </main>
        
        <footer class="site-footer">
            <p>© 2026 Nucleus Framework. MIT Licensed.</p>
        </footer>
    </body>
    </html>
</n:layout>
