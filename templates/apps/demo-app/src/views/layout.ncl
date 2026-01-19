<n:layout>
    <head>
        <title>Nucleus Kitchen Sink</title>
        <link rel="stylesheet" href="/assets/styles.css">
        <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;800&family=JetBrains+Mono&display=swap" rel="stylesheet">
    </head>
    <body>
        <nav class="sidebar">
            <div class="brand">⚛️ Nucleus</div>
            <a href="/" class="nav-item">Dashboard</a>
            <a href="/forms" class="nav-item">Forms & Validation</a>
            <a href="/actions" class="nav-item">Server Actions</a>
            <a href="/db" class="nav-item">Data Grid</a>
            <a href="/auth" class="nav-item">Authentication</a>
            <a href="/realtime" class="nav-item">Real-time</a>
            <a href="/payments" class="nav-item">Payments</a>
            <a href="/chain" class="nav-item">Blockchain</a>
        </nav>

        <main class="main-content">
            <!-- Page Content Injected Here -->
            <slot />
        </main>

        <!-- Source Code Toggle -->
        <div class="fab" onclick="toggleSource()" title="View Source">
            &lt;/&gt;
        </div>

        <div id="source-panel" class="glass">
            <div class="panel-header">
                <strong>Source: <span id="source-path">src/views/index.ncl</span></strong>
                <button class="btn" onclick="toggleSource()" style="padding: 4px 12px;">Close</button>
            </div>
            <pre><code id="source-code">Loading...</code></pre>
        </div>

        <script>
            let isOpen = false;
            
            // Map current URL to source file
            const routeMap = {
                '/': 'src/views/index.ncl',
                '/forms': 'src/views/forms.ncl',
                '/actions': 'src/views/actions.ncl',
                '/db': 'src/views/db.ncl',
                '/auth': 'src/views/auth.ncl',
                '/realtime': 'src/views/realtime.ncl',
                '/payments': 'src/views/payments.ncl',
                '/chain': 'src/views/chain.ncl'
            };

            async function toggleSource() {
                const panel = document.getElementById('source-panel');
                isOpen = !isOpen;
                
                if (isOpen) {
                    panel.classList.add('open');
                    await loadSource();
                } else {
                    panel.classList.remove('open');
                }
            }

            async function loadSource() {
                const path = routeMap[window.location.pathname] || 'src/views/index.ncl';
                document.getElementById('source-path').innerText = path;
                
                try {
                    // Call the Server Action via RPC
                    const res = await fetch('/_rpc/get_source', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify(path)
                    });
                    
                    const code = await res.json();
                    document.getElementById('source-code').innerText = code;
                } catch (e) {
                    document.getElementById('source-code').innerText = "// Error loading source: " + e;
                }
            }
            
            // Mark active nav
            document.querySelectorAll('.nav-item').forEach(el => {
                if(el.getAttribute('href') === window.location.pathname) {
                    el.classList.add('active');
                }
            });
        </script>
    </body>
</n:layout>
