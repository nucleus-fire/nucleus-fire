<n:view title="Nucleus — The Hyper-Optimized Framework">
    <n:model name="Subscriber">
        email: String
    </n:model>
    
    <n:action>
        use nucleus_std::photon::query::query;
        
        let email = params.get("email").unwrap_or(&"".to_string()).clone();
        if !email.is_empty() && email.contains('@') {
            // Use Photon query builder instead of raw SQL
            match query("subscribers")
                .insert()
                .value("email", email.clone())
                .execute()
                .await
            {
                Ok(_) => println!("✅ New Subscriber: {}", email),
                Err(e) => {
                    // Handle duplicate (UNIQUE constraint) gracefully
                    if e.to_string().contains("UNIQUE") {
                        println!("ℹ️  Subscriber already exists: {}", email);
                    } else {
                        eprintln!("❌ Failed to save subscriber: {}", e);
                    }
                }
            }
        }
    </n:action>
<n:layout name="layout">

<!-- Desktop Navigation (hidden on mobile) -->
<nav class="nav desktop-nav">
    <div class="container nav-inner">
        <n:link href="/" class="nav-logo">⚡ Nucleus</n:link>
        <div class="nav-links">
            <a href="#features" class="nav-link">Features</a>
            <a href="#comparison" class="nav-link">Speed</a>
            <a href="#benchmarks" class="nav-link">Benchmarks</a>
            <n:link href="/docs" class="nav-link">Docs</n:link>
            <n:link href="/playground" class="nav-link">Playground</n:link>
            <a href="https://github.com/nucleus-fire/nucleus-fire" target="_blank" class="nav-link github-link">
                <svg height="20" viewBox="0 0 16 16" width="20" fill="currentColor"><path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path></svg>
            </a>
            <n:link href="/docs" class="btn btn-gradient">Get Started</n:link>
        </div>
    </div>
</nav>

<!-- Mobile Bottom Navigation (visible on mobile only) -->
<nav class="bottom-nav mobile-nav">
    <a href="#features" class="bottom-nav-item">
        <span class="bottom-nav-icon">✨</span>
        <span class="bottom-nav-label">Features</span>
    </a>
    <a href="#comparison" class="bottom-nav-item">
        <span class="bottom-nav-icon">⚡</span>
        <span class="bottom-nav-label">Speed</span>
    </a>
    <n:link href="/docs" class="bottom-nav-item">
        <span class="bottom-nav-icon">📚</span>
        <span class="bottom-nav-label">Docs</span>
    </n:link>
    <n:link href="/playground" class="bottom-nav-item">
        <span class="bottom-nav-icon">🎮</span>
        <span class="bottom-nav-label">Playground</span>
    </n:link>
    <a href="https://github.com/nucleus-fire/nucleus-fire" target="_blank" class="bottom-nav-item">
        <span class="bottom-nav-icon">
            <svg height="20" viewBox="0 0 16 16" width="20" fill="currentColor"><path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path></svg>
        </span>
        <span class="bottom-nav-label">GitHub</span>
    </a>
</nav>

<!-- Premium Hero Section - Inspired by Linear/Vercel -->
<section class="hero-premium relative min-h-screen overflow-hidden" id="hero">
    <!-- Animated Gradient Orb Background -->
    <div class="hero-orb"></div>
    <div class="hero-orb-glow"></div>
    
    <!-- Grid Overlay -->
    <div class="hero-grid"></div>
    
    <!-- Floating Code Snippets (decorative) -->
    <div class="floating-code floating-code-1">
        <span class="code-bracket">{</span> <span class="code-key">type</span>: <span class="code-string">"safe"</span> <span class="code-bracket">}</span>
    </div>
    <div class="floating-code floating-code-2">
        <span class="code-fn">async</span> <span class="code-key">fn</span> <span class="code-fn">handle</span>()
    </div>
    <div class="floating-code floating-code-3">
        <span class="code-comment">// Zero runtime</span>
    </div>
    <div class="floating-code floating-code-4">
        <span class="code-bracket">&lt;/&gt;</span>
    </div>
    <div class="floating-code floating-code-5">
        <span class="code-key">$_</span>
    </div>
    <div class="floating-code floating-code-6">
        <span class="code-bracket">&gt;&gt;</span>
    </div>
    
    <!-- Main Content (Centered) - With padding for stats bar -->
    <div class="hero-content-wrapper">
        <!-- Release Badge -->
        <div class="inline-flex items-center gap-2 px-4 py-2 mb-8 rounded-full hero-badge animate-fade-in-up" style="border-color: rgba(168, 85, 247, 0.3); background: rgba(168, 85, 247, 0.1);">
            <span class="relative flex h-2 w-2">
              <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-purple-400 opacity-75"></span>
              <span class="relative inline-flex rounded-full h-2 w-2 bg-purple-500"></span>
            </span>
            <span class="text-xs font-bold tracking-wider text-purple-300" style="text-shadow: 0 0 10px rgba(168, 85, 247, 0.5);">NEW: AI AGENTS & MCP SUPPORT</span>
        </div>

        <!-- Main Headline -->
        <h1 class="hero-headline animate-fade-in-up delay-100">
            <span class="block text-white/90">The</span>
            <span class="block hero-gradient-text">Genetic Code</span>
            <span class="block text-white/90">of Modern Web</span>
        </h1>
        
        <!-- Tagline -->
        <p class="hero-tagline animate-fade-in-up delay-200">
            Build high-performance, type-safe web applications with a <span class="text-white font-medium">zero-runtime framework</span> designed for the future.
        </p>
        
        <!-- CTA Buttons -->
        <div class="hero-cta-wrapper animate-fade-in-up delay-300">
            <n:link href="/docs" class="hero-btn-primary group">
                <span>Get Started</span>
                <svg width="18" height="18" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="transition-transform group-hover:translate-x-1">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
                </svg>
            </n:link>
            <n:link href="/docs" class="hero-btn-secondary group">
                <span>View Docs</span>
                <svg width="16" height="16" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="opacity-50 group-hover:opacity-100 transition-opacity">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
                </svg>
            </n:link>
        </div>
    </div>
    
    <!-- Bottom Stats Bar (Fixed at bottom) -->
    <div class="hero-stats-container">
        <div class="hero-stats-bar">
            <div class="hero-stat-card">
                <div class="hero-stat-icon">⚡</div>
                <div class="hero-stat-content">
                    <span class="hero-stat-value">20,400</span>
                    <span class="hero-stat-label">req/s</span>
                </div>
            </div>
            <div class="hero-stat-card">
                <div class="hero-stat-icon">🚀</div>
                <div class="hero-stat-content">
                    <span class="hero-stat-value">0.8ms</span>
                    <span class="hero-stat-label">latency</span>
                </div>
            </div>
            <div class="hero-stat-card">
                <div class="hero-stat-icon">🛡️</div>
                <div class="hero-stat-content">
                    <span class="hero-stat-value">100%</span>
                    <span class="hero-stat-label">type-safe</span>
                </div>
            </div>
            <div class="hero-stat-card">
                <div class="hero-stat-icon">✨</div>
                <div class="hero-stat-content">
                    <span class="hero-stat-value">Zero</span>
                    <span class="hero-stat-label">runtime</span>
                </div>
            </div>
        </div>
    </div>
</section>

<!-- Trusted By Section -->
<section class="trusted-by">
    <div class="container">
        <p class="trusted-label">Powering the next generation of web apps</p>
        <div class="trusted-logos">
            <span class="trusted-logo">🚀 10k+ Downloads</span>
            <span class="trusted-logo">⭐ 2.4k GitHub Stars</span>
            <span class="trusted-logo">👥 Growing Community</span>
            <span class="trusted-logo">🔒 Production Ready</span>
        </div>
    </div>
</section>


<!-- Code Examples Section -->
<section id="examples" class="examples">
    <div class="container">
        <div class="section-header">
            <p class="section-label">See It In Action</p>
            <h2 class="section-title">Write less. Do more.</h2>
        </div>
        
        <div class="code-tabs">
            <div class="tab-buttons">
                <button class="tab-btn active" data-tab="api">API Endpoint</button>
                <button class="tab-btn" data-tab="db">Database</button>
                <button class="tab-btn" data-tab="auth">Authentication</button>
                <button class="tab-btn" data-tab="ws">WebSocket</button>
            </div>
            
            <div class="tab-content">
                <div class="tab-panel active" id="tab-api">
                    <pre class="code-block"><code><span class="keyword">use</span> nucleus::prelude::*;

<span class="attr">#[server]</span>
<span class="keyword">async fn</span> <span class="function">get_users</span>() -> Result&lt;Vec&lt;User&gt;&gt; {
    User::query()
        .filter(<span class="string">"active"</span>, <span class="bool">true</span>)
        .limit(<span class="number">10</span>)
        .fetch()
        .<span class="keyword">await</span>
}</code></pre>
                </div>
                <div class="tab-panel" id="tab-db">
                    <pre class="code-block"><code><span class="comment">// Type-safe SQL with Photon ORM</span>
<span class="keyword">let</span> posts = Post::query()
    .filter(<span class="string">"author_id"</span>, user.id)
    .order_by(<span class="string">"created_at"</span>, Desc)
    .include(<span class="string">"comments"</span>)
    .fetch()
    .<span class="keyword">await</span>?;</code></pre>
                </div>
                <div class="tab-panel" id="tab-auth">
                    <pre class="code-block"><code><span class="comment">// Built-in auth with Fortress</span>
<span class="keyword">let</span> hash = Fortress::hash_password(<span class="string">"secret"</span>)?;
<span class="keyword">let</span> token = Fortress::generate_token(&user.id)?;

<span class="comment">// Role-based access control</span>
<span class="keyword">if</span> user.has_role(Role::Admin) {
    <span class="comment">// Admin only code</span>
}</code></pre>
                </div>
                <div class="tab-panel" id="tab-ws">
                    <pre class="code-block"><code><span class="comment">// Real-time WebSocket channels</span>
<span class="keyword">impl</span> StreamHandler <span class="keyword">for</span> ChatHandler {
    <span class="keyword">async fn</span> <span class="function">on_message</span>(&self, msg: Value) {
        Channel::new(<span class="string">"chat"</span>)
            .broadcast(<span class="string">"new_message"</span>, &msg)
            .<span class="keyword">await</span>?;
    }
}</code></pre>
                </div>
            </div>
        </div>
    </div>
</section>

<!-- Features Section -->
<section id="features" class="features">
    <div class="container">
        <div class="section-header">
            <p class="section-label">Why Nucleus?</p>
            <h2 class="section-title">Everything you need.<br>Nothing you don't.</h2>
            <p class="section-desc">Built for teams who want to move fast without breaking things.</p>
        </div>
        
        <div class="features-grid">
            <div class="feature-card">
                <div class="feature-icon">⚛️</div>
                <h3 class="feature-title">Neutron State</h3>
                <p class="feature-desc">Fine-grained reactivity with signals. Shared state across server and client boundaries.</p>
            </div>

            <div class="feature-card">
                <div class="feature-icon">⚡</div>
                <h3 class="feature-title">Hyper Speed</h3>
                <p class="feature-desc">Powered by Tokio and Hyper. No garbage collection. Consistent sub-millisecond latency.</p>
            </div>
            
            <div class="feature-card">
                <div class="feature-icon">🛡️</div>
                <h3 class="feature-title">End-to-End Type Safety</h3>
                <p class="feature-desc">Change your database schema. Get compile errors in your frontend. Ship with confidence.</p>
            </div>
            
            <div class="feature-card">
                <div class="feature-icon">🔥</div>
                <h3 class="feature-title">Instant Hot Reload</h3>
                <p class="feature-desc">State-preserving HMR that updates in milliseconds. Your flow, uninterrupted.</p>
            </div>
            
            <div class="feature-card">
                <div class="feature-icon">🔐</div>
                <h3 class="feature-title">Security First</h3>
                <p class="feature-desc">CSP headers, HSTS, and XSS protection enabled by default. Fortress middleware built-in.</p>
            </div>
            
            <div class="feature-card">
                <div class="feature-icon">📡</div>
                <h3 class="feature-title">Real-time Ready</h3>
                <p class="feature-desc">WebSocket engine with automatic reconnection. Build live dashboards in minutes.</p>
            </div>
            
            <div class="feature-card">
                <div class="feature-icon">🎯</div>
                <h3 class="feature-title">100 Lighthouse Score</h3>
                <p class="feature-desc">RFC 7231 compliant headers, automatic sitemaps, and optimized asset delivery.</p>
            </div>
            
            <div class="feature-card">
                <div class="feature-icon">📦</div>
                <h3 class="feature-title">Static Export</h3>
                <p class="feature-desc">One-command SSG with incremental builds, WebP conversion, and multi-platform publish.</p>
            </div>
            
            <div class="feature-card">
                <div class="feature-icon">🚀</div>
                <h3 class="feature-title">Deploy Anywhere</h3>
                <p class="feature-desc">Unified publish to Netlify, Vercel, Cloudflare, and GitHub Pages. No config needed.</p>
            </div>

            <div class="feature-card">
                <div class="feature-icon">💻</div>
                <h3 class="feature-title">Nucleus Console</h3>
                <p class="feature-desc">Interactive async REPL. Query your database, test functions, and debug logic instantly.</p>
            </div>

            <div class="feature-card">
                <div class="feature-icon">🗄️</div>
                <h3 class="feature-title">Database Studio</h3>
                <p class="feature-desc">Built-in GUI for browsing tables, viewing schema, and executing SQL queries.</p>
            </div>

            <div class="feature-card">
                <div class="feature-icon">⏱️</div>
                <h3 class="feature-title">Request Profiler</h3>
                <p class="feature-desc">Visualize query performance, cache hits, and request timing in real-time.</p>
            </div>

            <div class="feature-card">
                <div class="feature-icon">🤖</div>
                <h3 class="feature-title">AI Error Assistant</h3>
                <p class="feature-desc">Intelligent error analysis that suggests fixes for runtime exceptions.</p>
            </div>

             <div class="feature-card">
                <div class="feature-icon">🧠</div>
                <h3 class="feature-title">AI Native</h3>
                <p class="feature-desc">First-class support for MCP and Agents. Build autonomous systems out of the box.</p>
            </div>
        </div>
    </div>
</section>

<!-- Batteries Included Section -->
<section id="stdlib" class="stdlib-section">
    <div class="container">
        <div class="section-header">
            <p class="section-label">Batteries Included</p>
            <h2 class="section-title">24 modules. Zero dependencies.</h2>
            <p class="section-desc">Everything you need to build production apps. No npm install required.</p>
        </div>
        
        <div class="stdlib-grid">
            <a href="/docs#25_email_guide" class="stdlib-card" style="text-decoration: none; color: inherit; display: block;" onclick="window.location.href='/docs#25_email_guide'">
                <div class="stdlib-icon">📧</div>
                <h3 class="stdlib-title">Postman</h3>
                <p class="stdlib-desc">Email via SMTP & AWS SES with templates and HTML support</p>
                <span class="stdlib-tag">Email</span>
            </a>
            
            <a href="/docs#26_image_processing_guide" class="stdlib-card" style="text-decoration: none; color: inherit; display: block;" onclick="window.location.href='/docs#26_image_processing_guide'">
                <div class="stdlib-icon">🖼️</div>
                <h3 class="stdlib-title">Lens</h3>
                <p class="stdlib-desc">Resize, crop, blur, rotate images. WebP, JPEG, PNG conversion</p>
                <span class="stdlib-tag">Media</span>
            </a>
            
            <a href="/docs#29_analytics_guide" class="stdlib-card" style="text-decoration: none; color: inherit; display: block;" onclick="window.location.href='/docs#29_analytics_guide'">
                <div class="stdlib-icon">📊</div>
                <h3 class="stdlib-title">Beacon</h3>
                <p class="stdlib-desc">Privacy-first analytics with Plausible, webhooks, or self-hosted</p>
                <span class="stdlib-tag">Analytics</span>
            </a>
            
            <a href="/docs#27_offline_sync_guide" class="stdlib-card" style="text-decoration: none; color: inherit; display: block;" onclick="window.location.href='/docs#27_offline_sync_guide'">
                <div class="stdlib-icon">🔄</div>
                <h3 class="stdlib-title">Gondola</h3>
                <p class="stdlib-desc">CRDTs for offline-first. LWW registers, counters, sync stores</p>
                <span class="stdlib-tag">Sync</span>
            </a>
            
            <a href="/docs#28_i18n_guide" class="stdlib-card" style="text-decoration: none; color: inherit; display: block;" onclick="window.location.href='/docs#28_i18n_guide'">
                <div class="stdlib-icon">🌍</div>
                <h3 class="stdlib-title">Polyglot</h3>
                <p class="stdlib-desc">Full i18n: translations, plurals, date/number formatting</p>
                <span class="stdlib-tag">i18n</span>
            </a>
            
            <a href="/docs#20_database_guide" class="stdlib-card" style="text-decoration: none; color: inherit; display: block;" onclick="window.location.href='/docs#20_database_guide'">
                <div class="stdlib-icon">💾</div>
                <h3 class="stdlib-title">Photon</h3>
                <p class="stdlib-desc">Type-safe SQL ORM with migrations and relationships</p>
                <span class="stdlib-tag">Database</span>
            </a>
            
            <a href="/docs#21_authentication_guide" class="stdlib-card" style="text-decoration: none; color: inherit; display: block;" onclick="window.location.href='/docs#21_authentication_guide'">
                <div class="stdlib-icon">🔐</div>
                <h3 class="stdlib-title">Fortress</h3>
                <p class="stdlib-desc">Auth, JWT, sessions, password hashing, RBAC built-in</p>
                <span class="stdlib-tag">Security</span>
            </a>
            
            <a href="/docs#30_websocket_guide" class="stdlib-card" style="text-decoration: none; color: inherit; display: block;" onclick="window.location.href='/docs#30_websocket_guide'">
                <div class="stdlib-icon">📡</div>
                <h3 class="stdlib-title">Stream</h3>
                <p class="stdlib-desc">WebSocket channels with auto-reconnect and rooms</p>
                <span class="stdlib-tag">Real-time</span>
            </a>
            
            <a href="/docs#55_scout_guide" class="stdlib-card" style="text-decoration: none; color: inherit; display: block;" onclick="window.location.href='/docs#55_scout_guide'">
                <div class="stdlib-icon">🔍</div>
                <h3 class="stdlib-title">Scout</h3>
                <p class="stdlib-desc">Full-text search with Meilisearch and Typesense backends</p>
                <span class="stdlib-tag">Search</span>
            </a>
            
            <a href="/docs#56_testing_utilities_guide" class="stdlib-card" style="text-decoration: none; color: inherit; display: block;" onclick="window.location.href='/docs#56_testing_utilities_guide'">
                <div class="stdlib-icon">🧪</div>
                <h3 class="stdlib-title">Testing</h3>
                <p class="stdlib-desc">MockServer, TestClient, and Factory pattern for tests</p>
                <span class="stdlib-tag">Testing</span>
            </a>
            
            <a href="/docs#57_push_notifications_guide" class="stdlib-card" style="text-decoration: none; color: inherit; display: block;" onclick="window.location.href='/docs#57_push_notifications_guide'">
                <div class="stdlib-icon">📱</div>
                <h3 class="stdlib-title">Push</h3>
                <p class="stdlib-desc">Push notifications via Firebase FCM and OneSignal</p>
                <span class="stdlib-tag">Notifications</span>
            </a>
            
            <a href="/docs#58_graphql_guide" class="stdlib-card" style="text-decoration: none; color: inherit; display: block;" onclick="window.location.href='/docs#58_graphql_guide'">
                <div class="stdlib-icon">🔗</div>
                <h3 class="stdlib-title">GraphQL</h3>
                <p class="stdlib-desc">Full GraphQL with schema builder, playground, subscriptions</p>
                <span class="stdlib-tag">API</span>
            </a>

            <a href="/docs#59_ai_agents" class="stdlib-card" style="text-decoration: none; color: inherit; display: block;" onclick="window.location.href='/docs#59_ai_agents'">
                <div class="stdlib-icon">🧠</div>
                <h3 class="stdlib-title">Agents</h3>
                <p class="stdlib-desc">Autonomous AI agents with MCP tool support and ReAct loop</p>
                <span class="stdlib-tag">AI</span>
            </a>
        </div>
        
        <div class="stdlib-cta">
            <n:link href="/docs#04_stdlib_reference" class="btn btn-secondary">View All 24 Modules →</n:link>
        </div>
    </div>
</section>

<!-- Rendering Speed Simulation -->
<section id="comparison" class="render-section">
    <div class="container">
        <div class="section-header">
            <p class="section-label">Performance Core</p>
            <h2 class="section-title">Rendering Throughput</h2>
            <p class="section-desc">Simulating complex component tree updates at 60Hz</p>
        </div>
        
        <!-- Render Dashboard -->
        <div class="render-dashboard">
            <div class="render-controls">
                <button class="race-btn primary" id="start-render">
                    <span class="btn-icon">▶</span> Run Benchmark
                </button>
            </div>

            <div class="cores-grid">
                <!-- Nucleus Core -->
                <div class="framework-core nucleus-core">
                    <div class="core-header">
                        <span class="core-logo">⚡</span>
                        <span class="core-name">Nucleus</span>
                    </div>
                    <div class="core-visual">
                        <div class="reactor-ring ring-1"></div>
                        <div class="reactor-ring ring-2"></div>
                        <div class="reactor-ring ring-3"></div>
                        <div class="reactor-center"></div>
                        <div class="particle-emitter" id="nucleus-emitter"></div>
                    </div>
                    <div class="core-stats">
                        <div class="stat-row">
                            <span class="stat-label">FPS</span>
                            <span class="stat-val highlight" id="nucleus-fps">0</span>
                        </div>
                        <div class="stat-row">
                            <span class="stat-label">Ops/sec</span>
                            <span class="stat-val" id="nucleus-ops">0</span>
                        </div>
                    </div>
                    <div class="core-status">Idle</div>
                </div>

                <!-- Next.js Core -->
                <div class="framework-core">
                    <div class="core-header">
                        <span class="core-logo">▲</span>
                        <span class="core-name">Next.js</span>
                    </div>
                    <div class="core-visual visual-slow">
                        <div class="reactor-ring ring-1"></div>
                        <div class="reactor-ring ring-2"></div>
                        <div class="reactor-center"></div>
                    </div>
                    <div class="core-stats">
                        <div class="stat-row">
                            <span class="stat-label">FPS</span>
                            <span class="stat-val" id="nextjs-fps">0</span>
                        </div>
                        <div class="stat-row">
                            <span class="stat-label">Ops/sec</span>
                            <span class="stat-val" id="nextjs-ops">0</span>
                        </div>
                    </div>
                    <div class="core-status">Idle</div>
                </div>
                
                <!-- Remix Core -->
                <div class="framework-core">
                    <div class="core-header">
                        <span class="core-logo">💿</span>
                        <span class="core-name">Remix</span>
                    </div>
                    <div class="core-visual visual-slow">
                        <div class="reactor-ring ring-1"></div>
                        <div class="reactor-ring ring-2"></div>
                        <div class="reactor-center"></div>
                    </div>
                    <div class="core-stats">
                        <div class="stat-row">
                            <span class="stat-label">FPS</span>
                            <span class="stat-val" id="remix-fps">0</span>
                        </div>
                        <div class="stat-row">
                            <span class="stat-label">Ops/sec</span>
                            <span class="stat-val" id="remix-ops">0</span>
                        </div>
                    </div>
                    <div class="core-status">Idle</div>
                </div>
                
                <!-- FastAPI Core -->
                <div class="framework-core">
                    <div class="core-header">
                        <span class="core-logo">🐍</span>
                        <span class="core-name">FastAPI</span>
                    </div>
                    <div class="core-visual visual-slow">
                        <div class="reactor-ring ring-1"></div>
                        <div class="reactor-ring ring-2"></div>
                        <div class="reactor-center"></div>
                    </div>
                    <div class="core-stats">
                        <div class="stat-row">
                            <span class="stat-label">FPS</span>
                            <span class="stat-val" id="fastapi-fps">0</span>
                        </div>
                        <div class="stat-row">
                            <span class="stat-label">Ops/sec</span>
                            <span class="stat-val" id="fastapi-ops">0</span>
                        </div>
                    </div>
                    <div class="core-status">Idle</div>
                </div>
            </div>
        </div>
    </div>
</section>

<!-- Benchmarks Section -->
<section id="benchmarks" class="benchmarks">
    <div class="container">
        <div class="benchmark-grid">
            <div>
                <p class="section-label">Performance</p>
                <h2 class="section-title">Faster than you think possible.</h2>
                <p class="section-desc" style="margin-bottom: 48px;">
                    Nucleus handles more traffic on a $5 droplet than Node.js handles on a dedicated server.
                </p>
                
                <div class="benchmark-bars">
                    <div class="benchmark-item">
                        <div class="benchmark-label">
                            <span class="font-medium">Nucleus</span>
                            <span class="text-accent font-semibold">20,400 req/s</span>
                        </div>
                        <div class="benchmark-bar">
                            <div class="benchmark-fill nucleus"></div>
                        </div>
                    </div>
                    
                    <div class="benchmark-item">
                        <div class="benchmark-label">
                            <span class="text-secondary">Node.js (Fastify)</span>
                            <span class="text-secondary">8,200 req/s</span>
                        </div>
                        <div class="benchmark-bar">
                            <div class="benchmark-fill node"></div>
                        </div>
                    </div>
                    
                    <div class="benchmark-item">
                        <div class="benchmark-label">
                            <span class="text-secondary">Python (FastAPI)</span>
                            <span class="text-secondary">4,500 req/s</span>
                        </div>
                        <div class="benchmark-bar">
                            <div class="benchmark-fill python"></div>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="hero-code" style="padding: 32px;">
                <div style="margin-bottom: 24px; padding-bottom: 16px; border-bottom: 1px solid rgba(255,255,255,0.1);">
                    <span class="font-semibold">Benchmark Configuration</span>
                </div>
                <div style="display: flex; flex-direction: column; gap: 12px; font-size: 14px;">
                    <div style="display: flex; justify-content: space-between;">
                        <span class="text-secondary">CPU</span>
                        <span>1 vCPU</span>
                    </div>
                    <div style="display: flex; justify-content: space-between;">
                        <span class="text-secondary">Memory</span>
                        <span>512 MB</span>
                    </div>
                    <div style="display: flex; justify-content: space-between;">
                        <span class="text-secondary">Connections</span>
                        <span>256 concurrent</span>
                    </div>
                    <div style="display: flex; justify-content: space-between;">
                        <span class="text-secondary">Latency (p99)</span>
                        <span class="text-accent">0.8ms</span>
                    </div>
                </div>
            </div>
        </div>
    </div>
</section>

<!-- Feature Comparison Section -->
<section id="comparison" class="comparison" style="padding: 100px 0; background: linear-gradient(180deg, var(--bg-secondary) 0%, var(--bg-primary) 100%);">
    <div class="container">
        <div class="section-header" style="text-align: center; margin-bottom: 60px;">
            <p class="section-label">Framework Comparison</p>
            <h2 class="section-title">How Nucleus stacks up</h2>
            <p class="section-desc" style="max-width: 600px; margin: 0 auto;">
                A complete fullstack framework that rivals the best in every category.
            </p>
        </div>
        
        <div class="comparison-table-wrapper" style="overflow-x: auto; border-radius: 20px; background: rgba(255,255,255,0.02); border: 1px solid rgba(255,255,255,0.06);">
            <table class="comparison-table" style="width: 100%; border-collapse: collapse; font-size: 14px; min-width: 800px;">
                <thead>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.1);">
                        <th style="padding: 20px 24px; text-align: left; font-weight: 600;">Feature</th>
                        <th style="padding: 20px 16px; text-align: center; background: rgba(130,100,255,0.1); border-radius: 16px 16px 0 0;">
                            <span style="color: var(--accent-primary);">⚛️ Nucleus</span>
                        </th>
                        <th style="padding: 20px 16px; text-align: center; color: var(--text-secondary);">Next.js</th>
                        <th style="padding: 20px 16px; text-align: center; color: var(--text-secondary);">Rails</th>
                        <th style="padding: 20px 16px; text-align: center; color: var(--text-secondary);">Django</th>
                        <th style="padding: 20px 16px; text-align: center; color: var(--text-secondary);">Laravel</th>
                    </tr>
                </thead>
                <tbody>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">Language</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);"><span style="color: #f97316;">Rust</span></td>
                        <td style="padding: 16px; text-align: center; color: var(--text-secondary);">JS/TS</td>
                        <td style="padding: 16px; text-align: center; color: var(--text-secondary);">Ruby</td>
                        <td style="padding: 16px; text-align: center; color: var(--text-secondary);">Python</td>
                        <td style="padding: 16px; text-align: center; color: var(--text-secondary);">PHP</td>
                    </tr>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">Type Safety</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Full</td>
                        <td style="padding: 16px; text-align: center;">⚠️ Optional</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                        <td style="padding: 16px; text-align: center;">⚠️ Optional</td>
                        <td style="padding: 16px; text-align: center;">⚠️ Optional</td>
                    </tr>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">Memory Safety</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Guaranteed</td>
                        <td style="padding: 16px; text-align: center;">❌ GC</td>
                        <td style="padding: 16px; text-align: center;">❌ GC</td>
                        <td style="padding: 16px; text-align: center;">❌ GC</td>
                        <td style="padding: 16px; text-align: center;">❌ GC</td>
                    </tr>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">Built-in ORM</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Photon</td>
                        <td style="padding: 16px; text-align: center;">❌ Prisma*</td>
                        <td style="padding: 16px; text-align: center;">✅ ActiveRecord</td>
                        <td style="padding: 16px; text-align: center;">✅ Django ORM</td>
                        <td style="padding: 16px; text-align: center;">✅ Eloquent</td>
                    </tr>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">Built-in Auth</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Fortress</td>
                        <td style="padding: 16px; text-align: center;">❌ NextAuth*</td>
                        <td style="padding: 16px; text-align: center;">✅ Devise*</td>
                        <td style="padding: 16px; text-align: center;">✅ Built-in</td>
                        <td style="padding: 16px; text-align: center;">✅ Jetstream*</td>
                    </tr>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">Real-time WebSockets</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Stream</td>
                        <td style="padding: 16px; text-align: center;">❌ Pusher*</td>
                        <td style="padding: 16px; text-align: center;">✅ ActionCable</td>
                        <td style="padding: 16px; text-align: center;">❌ Channels*</td>
                        <td style="padding: 16px; text-align: center;">✅ Broadcasting</td>
                    </tr>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">Background Jobs</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Pulse</td>
                        <td style="padding: 16px; text-align: center;">❌ Bull*</td>
                        <td style="padding: 16px; text-align: center;">✅ Sidekiq*</td>
                        <td style="padding: 16px; text-align: center;">❌ Celery*</td>
                        <td style="padding: 16px; text-align: center;">✅ Queues</td>
                    </tr>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">Cron Scheduler</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Built-in</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                        <td style="padding: 16px; text-align: center;">❌ Whenever*</td>
                        <td style="padding: 16px; text-align: center;">❌ django-cron*</td>
                        <td style="padding: 16px; text-align: center;">✅ Scheduler</td>
                    </tr>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">AI/LLM Integration</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Neural</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                    </tr>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">MCP Support</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Native</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                    </tr>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">Image Processing</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Lens</td>
                        <td style="padding: 16px; text-align: center;">❌ Sharp*</td>
                        <td style="padding: 16px; text-align: center;">❌ ImageMagick*</td>
                        <td style="padding: 16px; text-align: center;">❌ Pillow*</td>
                        <td style="padding: 16px; text-align: center;">❌ Intervention*</td>
                    </tr>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">Full-text Search</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Sonar</td>
                        <td style="padding: 16px; text-align: center;">❌ Algolia*</td>
                        <td style="padding: 16px; text-align: center;">❌ Searchkick*</td>
                        <td style="padding: 16px; text-align: center;">❌ Haystack*</td>
                        <td style="padding: 16px; text-align: center;">❌ Scout*</td>
                    </tr>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">Email Sending</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Postman</td>
                        <td style="padding: 16px; text-align: center;">❌ Nodemailer*</td>
                        <td style="padding: 16px; text-align: center;">✅ ActionMailer</td>
                        <td style="padding: 16px; text-align: center;">✅ Built-in</td>
                        <td style="padding: 16px; text-align: center;">✅ Mailable</td>
                    </tr>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">Rate Limiting</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Fortress</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                        <td style="padding: 16px; text-align: center;">❌ Rack::Attack*</td>
                        <td style="padding: 16px; text-align: center;">❌ django-ratelimit*</td>
                        <td style="padding: 16px; text-align: center;">✅ Throttle</td>
                    </tr>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">Health Checks</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Built-in</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                        <td style="padding: 16px; text-align: center;">❌ health_check*</td>
                        <td style="padding: 16px; text-align: center;">❌ django-health*</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                    </tr>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">Static Export (SSG)</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Unified</td>
                        <td style="padding: 16px; text-align: center;">✅</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                    </tr>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">Zero Runtime</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Yes</td>
                        <td style="padding: 16px; text-align: center;">❌ Node.js</td>
                        <td style="padding: 16px; text-align: center;">❌ Ruby</td>
                        <td style="padding: 16px; text-align: center;">❌ Python</td>
                        <td style="padding: 16px; text-align: center;">❌ PHP</td>
                    </tr>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">Development Tools</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Console + GUI</td>
                        <td style="padding: 16px; text-align: center;">⚠️ Basic</td>
                        <td style="padding: 16px; text-align: center;">✅ Console</td>
                        <td style="padding: 16px; text-align: center;">✅ Shell</td>
                        <td style="padding: 16px; text-align: center;">✅ Tinker</td>
                    </tr>

                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">Interactive REPL</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Console</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                        <td style="padding: 16px; text-align: center;">✅ Console</td>
                        <td style="padding: 16px; text-align: center;">✅ Shell</td>
                        <td style="padding: 16px; text-align: center;">✅ Tinker</td>
                    </tr>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">Database Studio (GUI)</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Studio</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                        <td style="padding: 16px; text-align: center;">✅ Admin</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                    </tr>
                    <tr style="border-bottom: 1px solid rgba(255,255,255,0.05);">
                        <td style="padding: 16px 24px;">Live Profiler</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);">✅ Built-in</td>
                        <td style="padding: 16px; text-align: center;">❌</td>
                        <td style="padding: 16px; text-align: center;">❌ MiniProfiler*</td>
                        <td style="padding: 16px; text-align: center;">❌ DebugTbar*</td>
                        <td style="padding: 16px; text-align: center;">❌ Telescope*</td>
                    </tr>
                    <tr>
                        <td style="padding: 16px 24px; font-weight: 600;">Throughput</td>
                        <td style="padding: 16px; text-align: center; background: rgba(130,100,255,0.05);"><span style="color: #22c55e; font-weight: 600;">20,400 req/s</span></td>
                        <td style="padding: 16px; text-align: center; color: var(--text-secondary);">~3,500</td>
                        <td style="padding: 16px; text-align: center; color: var(--text-secondary);">~2,000</td>
                        <td style="padding: 16px; text-align: center; color: var(--text-secondary);">~1,500</td>
                        <td style="padding: 16px; text-align: center; color: var(--text-secondary);">~2,500</td>
                    </tr>
                </tbody>
            </table>
        </div>
        <p style="text-align: center; margin-top: 20px; font-size: 13px; color: var(--text-tertiary);">
            * Requires third-party package. ✅ = Built-in, ⚠️ = Partial/Optional, ❌ = Requires external package
        </p>
    </div>
</section>



<!-- Newsletter Section -->
<section class="newsletter">
    <div class="container newsletter-content">
        <h2 class="newsletter-title">Stay Updated</h2>
        <p class="newsletter-desc">Get the latest Nucleus news, tutorials, and releases.</p>
        <form class="newsletter-form" method="POST">
            <input type="email" name="email" id="newsletter-email" autocomplete="email" placeholder="you@example.com" class="newsletter-input" required>
            <button type="submit" class="btn btn-gradient">Subscribe</button>
        </form>
        <p class="newsletter-note">No spam. Unsubscribe anytime.</p>
    </div>
</section>

<!-- CTA Section -->
<section class="cta">
    <div class="cta-bg"></div>
    <div class="container cta-content">
        <h2 class="cta-title">Ready to build<br><span class="gradient-text">something amazing?</span></h2>
        <p class="section-desc" style="margin-bottom: 40px;">Join thousands of developers shipping faster with Nucleus.</p>
        <div class="flex gap-4 justify-center">
            <n:link href="/docs" class="btn btn-gradient">Get Started</n:link>
            <a href="https://github.com/nucleus-fire/nucleus-fire" class="btn btn-secondary">Star on GitHub</a>
        </div>
    </div>
</section>

<!-- Footer -->
<footer class="footer">
    <div class="container footer-inner">
        <div class="text-muted text-sm">© 2026 Nucleus Framework. MIT Licensed.</div>
        <div class="footer-links">
            <n:link href="/docs" class="footer-link">Documentation</n:link>
            <a href="https://github.com/nucleus-fire/nucleus-fire" class="footer-link">GitHub</a>
            <a href="https://twitter.com" class="footer-link">Twitter</a>
            <a href="https://discord.com" class="footer-link">Discord</a>
        </div>
    </div>
</footer>

<!-- Scripts -->
<!-- Scoped Animations -->
<n:style>
    /* ===== PREMIUM HERO SECTION ===== */
    .hero-premium {
        background: #030303;
        color: white;
    }
    
    /* Hero Content Layout */
    .hero-content-wrapper {
        position: relative;
        z-index: 20;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        text-align: center;
        min-height: 100vh;
        padding: 6rem 1.5rem 10rem;
        max-width: 64rem;
        margin: 0 auto;
    }
    
    .hero-headline {
        font-size: clamp(2.5rem, 8vw, 5rem);
        font-weight: 900;
        letter-spacing: -0.02em;
        line-height: 1.1;
        margin-bottom: 1.5rem;
    }
    
    .hero-headline .hero-gradient-text {
        font-size: clamp(3rem, 10vw, 7rem);
    }
    
    .hero-tagline {
        font-size: clamp(1rem, 2.5vw, 1.25rem);
        color: rgba(148, 163, 184, 0.9);
        max-width: 36rem;
        margin: 0 auto 2.5rem;
        line-height: 1.7;
    }
    
    .hero-cta-wrapper {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
    }
    
    @media (min-width: 640px) {
        .hero-cta-wrapper {
            flex-direction: row;
            gap: 1.25rem;
        }
    }
    
    /* Stats Container - Fixed at Bottom */
    .hero-stats-container {
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        z-index: 30;
        padding: 1.5rem;
        background: linear-gradient(180deg, transparent 0%, rgba(0, 0, 0, 0.9) 100%);
    }
    
    .hero-stats-bar {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        gap: 0.75rem;
        max-width: 64rem;
        margin: 0 auto;
    }
    
    @media (min-width: 768px) {
        .hero-stats-bar {
            gap: 1.5rem;
        }
    }
    
    /* Animated Gradient Orb */
    .hero-orb {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 500px;
        height: 500px;
        background: radial-gradient(circle at 30% 30%, 
            rgba(96, 165, 250, 0.4) 0%,
            rgba(139, 92, 246, 0.3) 30%,
            rgba(168, 85, 247, 0.2) 50%,
            rgba(59, 130, 246, 0.1) 70%,
            transparent 100%
        );
        border-radius: 50%;
        filter: blur(60px);
        animation: orbPulse 8s ease-in-out infinite, orbFloat 12s ease-in-out infinite;
        z-index: 1;
    }
    
    .hero-orb-glow {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 700px;
        height: 700px;
        background: radial-gradient(circle, 
            rgba(139, 92, 246, 0.15) 0%,
            rgba(59, 130, 246, 0.08) 40%,
            transparent 70%
        );
        border-radius: 50%;
        filter: blur(80px);
        animation: orbPulse 10s ease-in-out infinite reverse;
        z-index: 0;
    }
    
    @keyframes orbPulse {
        0%, 100% { transform: translate(-50%, -50%) scale(1); opacity: 0.8; }
        50% { transform: translate(-50%, -50%) scale(1.15); opacity: 1; }
    }
    
    @keyframes orbFloat {
        0%, 100% { transform: translate(-50%, -50%); }
        25% { transform: translate(-48%, -52%); }
        50% { transform: translate(-52%, -48%); }
        75% { transform: translate(-50%, -50%); }
    }
    
    /* Grid Overlay */
    .hero-grid {
        position: absolute;
        inset: 0;
        background-image: 
            linear-gradient(rgba(255,255,255,0.02) 1px, transparent 1px),
            linear-gradient(90deg, rgba(255,255,255,0.02) 1px, transparent 1px);
        background-size: 60px 60px;
        z-index: 2;
        pointer-events: none;
        mask-image: radial-gradient(ellipse at center, transparent 20%, black 80%);
        -webkit-mask-image: radial-gradient(ellipse at center, transparent 20%, black 80%);
    }
    
    /* Floating Code Snippets */
    .floating-code {
        position: absolute;
        font-family: ui-monospace, SFMono-Regular, monospace;
        font-size: 0.75rem;
        padding: 0.5rem 1rem;
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid rgba(255, 255, 255, 0.06);
        border-radius: 0.5rem;
        backdrop-filter: blur(8px);
        color: rgba(255, 255, 255, 0.4);
        z-index: 15;
        animation: floatCode 20s ease-in-out infinite;
        opacity: 0.6;
    }
    
    .floating-code-1 { top: 15%; left: 8%; animation-delay: 0s; }
    .floating-code-2 { top: 25%; left: 5%; animation-delay: -3s; }
    .floating-code-3 { top: 70%; left: 10%; animation-delay: -6s; }
    .floating-code-4 { top: 20%; right: 8%; animation-delay: -2s; }
    .floating-code-5 { top: 60%; right: 5%; animation-delay: -8s; }
    .floating-code-6 { top: 75%; right: 12%; animation-delay: -4s; }
    
    .code-bracket { color: rgba(96, 165, 250, 0.7); }
    .code-key { color: rgba(168, 85, 247, 0.8); }
    .code-string { color: rgba(52, 211, 153, 0.8); }
    .code-fn { color: rgba(251, 191, 36, 0.8); }
    .code-comment { color: rgba(100, 116, 139, 0.7); font-style: italic; }
    
    @keyframes floatCode {
        0%, 100% { transform: translateY(0) rotate(0deg); }
        25% { transform: translateY(-10px) rotate(1deg); }
        50% { transform: translateY(5px) rotate(-1deg); }
        75% { transform: translateY(-5px) rotate(0.5deg); }
    }
    
    /* Hero Badge */
    .hero-badge {
        background: linear-gradient(135deg, rgba(59, 130, 246, 0.1) 0%, rgba(139, 92, 246, 0.1) 100%);
        border: 1px solid rgba(59, 130, 246, 0.2);
        backdrop-filter: blur(12px);
    }
    
    /* Hero Animated Gradient Text */
    .hero-gradient-text {
        background: linear-gradient(135deg, #60a5fa 0%, #c084fc 25%, #f472b6 50%, #c084fc 75%, #60a5fa 100%);
        background-size: 300% 300%;
        -webkit-background-clip: text;
        background-clip: text;
        -webkit-text-fill-color: transparent;
        animation: gradientShift 6s ease infinite;
        line-height: 1.1;
    }
    
    @keyframes gradientShift {
        0%, 100% { background-position: 0% 50%; }
        50% { background-position: 100% 50%; }
    }
    
    /* Hero Primary Button with Glow */
    .hero-btn-primary {
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.875rem 2rem;
        background: linear-gradient(135deg, #3b82f6 0%, #8b5cf6 100%);
        color: white;
        font-weight: 700;
        border-radius: 0.75rem;
        transition: all 0.3s ease;
        box-shadow: 0 0 30px rgba(139, 92, 246, 0.3);
    }
    
    .hero-btn-primary:hover {
        transform: translateY(-2px);
        box-shadow: 0 0 50px rgba(139, 92, 246, 0.5), 0 10px 40px rgba(59, 130, 246, 0.3);
    }
    
    /* Hero Secondary Button */
    .hero-btn-secondary {
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.875rem 1.75rem;
        background: rgba(255, 255, 255, 0.05);
        border: 1px solid rgba(255, 255, 255, 0.1);
        color: white;
        font-weight: 600;
        border-radius: 0.75rem;
        transition: all 0.3s ease;
        backdrop-filter: blur(12px);
    }
    
    .hero-btn-secondary:hover {
        background: rgba(255, 255, 255, 0.1);
        border-color: rgba(255, 255, 255, 0.2);
        transform: translateY(-2px);
    }
    
    /* Stats Bar (Glassmorphic) */
    .hero-stats-bar {
        background: linear-gradient(180deg, transparent 0%, rgba(0, 0, 0, 0.8) 100%);
        border-top: 1px solid rgba(255, 255, 255, 0.05);
        backdrop-filter: blur(20px);
    }
    
    /* Stat Cards */
    .hero-stat-card {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 0.875rem 1.5rem;
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid rgba(255, 255, 255, 0.06);
        border-radius: 1rem;
        backdrop-filter: blur(12px);
        transition: all 0.3s ease;
    }
    
    .hero-stat-card:hover {
        background: rgba(255, 255, 255, 0.06);
        border-color: rgba(139, 92, 246, 0.3);
        transform: translateY(-4px);
        box-shadow: 0 12px 40px rgba(139, 92, 246, 0.15);
    }
    
    .hero-stat-icon {
        font-size: 1.25rem;
    }
    
    .hero-stat-content {
        display: flex;
        flex-direction: column;
    }
    
    .hero-stat-value {
        font-family: ui-monospace, monospace;
        font-size: 1.25rem;
        font-weight: 800;
        background: linear-gradient(135deg, #60a5fa 0%, #c084fc 100%);
        -webkit-background-clip: text;
        background-clip: text;
        -webkit-text-fill-color: transparent;
    }
    
    .hero-stat-label {
        font-size: 0.7rem;
        color: rgba(148, 163, 184, 0.7);
        text-transform: uppercase;
        letter-spacing: 0.1em;
    }
    
    /* Hero Stats - Glassmorphic Cards with Pulse */
    .hero-stat {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.25rem;
        padding: 0.75rem 1.25rem;
        background: rgba(255, 255, 255, 0.04);
        border: 1px solid rgba(255, 255, 255, 0.08);
        border-radius: 0.75rem;
        backdrop-filter: blur(12px);
        transition: all 0.3s ease;
    }
    
    .hero-stat:hover {
        background: rgba(255, 255, 255, 0.08);
        border-color: rgba(139, 92, 246, 0.3);
        transform: translateY(-2px);
        box-shadow: 0 8px 32px rgba(139, 92, 246, 0.15);
    }
    
    .hero-stat-value {
        font-family: ui-monospace, monospace;
        font-size: 1.5rem;
        font-weight: 800;
        background: linear-gradient(135deg, #60a5fa 0%, #a78bfa 100%);
        -webkit-background-clip: text;
        background-clip: text;
        -webkit-text-fill-color: transparent;
        animation: statPulse 3s ease infinite;
    }
    
    @keyframes statPulse {
        0%, 100% { opacity: 1; }
        50% { opacity: 0.8; }
    }
    
    .hero-stat-label {
        font-size: 0.65rem;
        color: rgba(148, 163, 184, 0.7);
        text-transform: uppercase;
        letter-spacing: 0.1em;
        letter-spacing: 0.05em;
    }
    
    /* Scroll Hint Animation */
    @keyframes scrollHint {
        0%, 100% { transform: translateY(0); opacity: 1; }
        50% { transform: translateY(6px); opacity: 0.5; }
    }
    
    .animate-scroll-hint {
        animation: scrollHint 2s ease infinite;
    }
    
    /* Fade In Up Animation */
    @keyframes fadeInUp {
        from { opacity: 0; transform: translateY(20px); }
        to { opacity: 1; transform: translateY(0); }
    }
    .animate-fade-in-up {
        opacity: 0;
        animation: fadeInUp 0.8s cubic-bezier(0.16, 1, 0.3, 1) forwards;
    }
    .delay-100 { animation-delay: 0.1s; }
    .delay-200 { animation-delay: 0.2s; }
    .delay-300 { animation-delay: 0.3s; }
    .delay-400 { animation-delay: 0.4s; }
    .delay-500 { animation-delay: 0.5s; }
    
    .animate-bounce-slow {
        animation: bounce 3s infinite;
    }
    @keyframes bounce {
        0%, 100% { transform: translate(-50%, 0); }
        50% { transform: translate(-50%, 10px); }
    }
</n:style>

<!-- Hero Canvas Animation -->
<script src="/assets/hero.js"></script>
<script src="/assets/home.js"></script>

</n:layout>
</n:view>
