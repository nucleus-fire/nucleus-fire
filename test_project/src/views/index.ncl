<n:view title="Welcome to Nucleus">
<n:layout name="layout">
    <div class="container">
        <header>
            <h1><n:text key="welcome_title" /></h1>
            <p><n:text key="welcome_message" /></p>
        </header>

        <main>
            <div class="card">
                <h2>🚀 You are running on Nucleus V3</h2>
                <p>Edit <code>src/views/index.ncl</code> to see changes instantly (HMR).</p>
            </div>
            
            <div class="features">
                <div class="feature">
                    <h3>⚡️ Fast</h3>
                    <p>Compiled to binary. No GC pauses.</p>
                </div>
                <div class="feature">
                    <h3>🔋 Battery Included</h3>
                    <p>Database, Auth, and WASM in one box.</p>
                </div>
            </div>
        </main>
    </div>
</n:layout>
</n:view>
