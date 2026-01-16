<n:view title="Dashboard">
    <h1>Welcome to Nucleus</h1>
    <p>This "Kitchen Sink" demo showcases the power of the Nucleus Framework. Explore the sidebar to see features in action.</p>
    
    <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1.5rem; margin-top: 2rem;">
        <div class="card glass">
            <h2>Server Actions</h2>
            <p>Securely call backend functions from the client.</p>
            <a href="/actions" class="btn">Try it</a>
        </div>
        <div class="card glass">
            <h2>Data Grid</h2>
            <p>Enterprise-ready tables with SQLx.</p>
            <a href="/db" class="btn">View Data</a>
        </div>
        <div class="card glass">
            <h2>Real-time</h2>
            <p>Live data streaming over WebSockets.</p>
            <a href="/realtime" class="btn">Connect</a>
        </div>
    </div>
</n:view>
