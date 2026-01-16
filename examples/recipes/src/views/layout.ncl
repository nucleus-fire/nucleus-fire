<n:view title="Recipes">
    <div class="app-layout" style="display: flex; flex-direction: column; min-height: 100vh; font-family: system-ui, sans-serif; background: #111; color: white;">
        <nav style="border-bottom: 1px solid #333; padding: 1rem; display: flex; gap: 1rem; align-items: center;">
             <span style="font-weight: bold; font-size: 1.2rem;">🍳 Nucleus Recipes</span>
             <a href="/" style="color: #aaa; text-decoration: none;">Home</a>
             <a href="/todo" style="color: #aaa; text-decoration: none;">Todo</a>
             <a href="/counter" style="color: #aaa; text-decoration: none;">Counter</a>
        </nav>
        <main style="flex: 1; padding: 2rem;">
            <n:outlet />
        </main>
        <footer style="border-top: 1px solid #333; padding: 1rem; text-align: center; color: #666; font-size: 0.8rem;">
            Built with Nucleus V3
        </footer>
    </div>
</n:view>
