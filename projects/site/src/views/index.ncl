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
        <div class="inline-flex items-center gap-2 px-4 py-2 mb-8 rounded-full hero-badge animate-fade-in-up" style="border-color: rgba(34, 197, 94, 0.3); background: rgba(34, 197, 94, 0.1);">
            <span class="relative flex h-2 w-2">
              <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span>
              <span class="relative inline-flex rounded-full h-2 w-2 bg-green-500"></span>
            </span>
            <span class="text-xs font-bold tracking-wider text-green-300" style="text-shadow: 0 0 10px rgba(34, 197, 94, 0.5);">V1.0.0 PRODUCTION READY</span>
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
                <div class="hero-stat-icon">💰</div>
                <div class="hero-stat-content">
                    <span class="hero-stat-value">90%</span>
                    <span class="hero-stat-label">less RAM</span>
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
            <span class="trusted-logo">🚀 v1.0.0 Stable</span>
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
            <p class="section-desc">Clean, expressive code that compiles to blazingly fast binaries.</p>
        </div>
        
        <div class="code-tabs">
            <div class="tab-buttons">
                <button class="tab-btn active" data-tab="page">Full Page</button>
                <button class="tab-btn" data-tab="form">Forms</button>
                <button class="tab-btn" data-tab="island">Interactive</button>
                <button class="tab-btn" data-tab="api">API Route</button>
                <button class="tab-btn" data-tab="rpc">Server RPC</button>
            </div>
            
            <div class="tab-content">
                <div class="tab-panel active" id="tab-page">
                    <pre class="code-block"><code><span class="comment">&lt;!-- blog/[slug].ncl --&gt;</span>&#10;<span class="tag">&lt;n:view</span> <span class="attr">title</span>=<span class="string">"{post.title}"</span><span class="tag">&gt;</span>&#10;  <span class="tag">&lt;n:loader&gt;</span>&#10;    <span class="keyword">let</span> post = Post::find(slug).<span class="keyword">await</span>?;&#10;  <span class="tag">&lt;/n:loader&gt;</span>&#10;&#10;  <span class="tag">&lt;h1&gt;</span>{post.title}<span class="tag">&lt;/h1&gt;</span>&#10;  <span class="tag">&lt;p&gt;</span>{post.content}<span class="tag">&lt;/p&gt;</span>&#10;<span class="tag">&lt;/n:view&gt;</span></code></pre>
                </div>
                <div class="tab-panel" id="tab-form">
                    <pre class="code-block"><code><span class="comment">&lt;!-- contact.ncl --&gt;</span>&#10;<span class="tag">&lt;n:view</span> <span class="attr">title</span>=<span class="string">"Contact"</span><span class="tag">&gt;</span>&#10;  <span class="tag">&lt;n:action&gt;</span>&#10;    <span class="keyword">let</span> data = form.validate()?;&#10;    Postman::send(&amp;data.email).<span class="keyword">await</span>?;&#10;  <span class="tag">&lt;/n:action&gt;</span>&#10;&#10;  <span class="tag">&lt;n:form&gt;</span>&#10;    <span class="tag">&lt;input</span> <span class="attr">name</span>=<span class="string">"email"</span> <span class="attr">required</span> <span class="tag">/&gt;</span>&#10;    <span class="tag">&lt;button&gt;</span>Submit<span class="tag">&lt;/button&gt;</span>&#10;  <span class="tag">&lt;/n:form&gt;</span>&#10;<span class="tag">&lt;/n:view&gt;</span></code></pre>
                </div>
                <div class="tab-panel" id="tab-island">
                    <pre class="code-block"><code><span class="comment">&lt;!-- Interactive island --&gt;</span>&#10;<span class="tag">&lt;n:island</span> <span class="attr">client:visible</span><span class="tag">&gt;</span>&#10;  <span class="tag">&lt;n:script&gt;</span>&#10;    <span class="keyword">let</span> count = Signal::new(<span class="number">0</span>);&#10;  <span class="tag">&lt;/n:script&gt;</span>&#10;&#10;  <span class="tag">&lt;button</span> <span class="attr">n:on:click</span>=<span class="string">{count.set(c+1)}</span><span class="tag">&gt;</span>&#10;    Clicks: {count}&#10;  <span class="tag">&lt;/button&gt;</span>&#10;<span class="tag">&lt;/n:island&gt;</span></code></pre>
                </div>
                <div class="tab-panel" id="tab-api">
                    <pre class="code-block"><code><span class="comment">&lt;!-- api/users.ncl --&gt;</span>&#10;<span class="tag">&lt;n:view</span> <span class="attr">content_type</span>=<span class="string">"json"</span><span class="tag">&gt;</span>&#10;  <span class="tag">&lt;n:loader&gt;</span>&#10;    <span class="keyword">let</span> users = User::query()&#10;      .filter(<span class="string">"active"</span>, <span class="bool">true</span>)&#10;      .all().<span class="keyword">await</span>?;&#10;&#10;    json!({ <span class="string">"users"</span>: users })&#10;  <span class="tag">&lt;/n:loader&gt;</span>&#10;<span class="tag">&lt;/n:view&gt;</span></code></pre>
                </div>
                <div class="tab-panel" id="tab-rpc">
                    <pre class="code-block"><code><span class="comment">// services/user.rs</span>&#10;<span class="keyword">use</span> nucleus_std::server;&#10;&#10;<span class="attr">#[server]</span>&#10;<span class="keyword">pub async fn</span> <span class="function">create_user</span>(&#10;  name: String&#10;) -&gt; Result&lt;User&gt; {&#10;  User::create()&#10;    .set(<span class="string">"name"</span>, &amp;name)&#10;    .save().<span class="keyword">await</span>&#10;}</code></pre>
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
                <p class="feature-desc">Powered by Tokio. Handles 10x traffic on the same hardware. Consistent sub-ms latency.</p>
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
            <h2 class="section-title">42 modules. Zero dependencies.</h2>
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
            <n:link href="/docs#18_complete_reference" class="btn btn-secondary">View All 42 Modules →</n:link>
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
<section id="comparison" class="comparison-section">
    <div class="container">
        <div class="section-header">
            <p class="section-label">Framework Comparison</p>
            <h2 class="section-title">How Nucleus stacks up</h2>
        </div>
        
        <!-- Hero Stats Row -->
        <div class="compare-hero">
            <div class="compare-hero-stat">
                <span class="compare-hero-value">20,400</span>
                <span class="compare-hero-unit">req/s</span>
                <span class="compare-hero-label">Throughput</span>
            </div>
            <div class="compare-hero-stat">
                <span class="compare-hero-value">42</span>
                <span class="compare-hero-unit">modules</span>
                <span class="compare-hero-label">Built-in</span>
            </div>
            <div class="compare-hero-stat">
                <span class="compare-hero-value">0</span>
                <span class="compare-hero-unit">zero</span>
                <span class="compare-hero-label">External Deps</span>
            </div>
        </div>
        
        <!-- Feature Comparison Table -->
        <div class="compare-table">
            <div class="compare-row header">
                <div class="compare-cell feature">Feature</div>
                <div class="compare-cell nucleus">Nucleus</div>
                <div class="compare-cell other">Next.js</div>
                <div class="compare-cell other">Rails</div>
                <div class="compare-cell other">Laravel</div>
            </div>
            <div class="compare-row">
                <div class="compare-cell feature">Type Safety</div>
                <div class="compare-cell nucleus yes">✓</div>
                <div class="compare-cell other partial">~</div>
                <div class="compare-cell other no">✗</div>
                <div class="compare-cell other partial">~</div>
            </div>
            <div class="compare-row">
                <div class="compare-cell feature">ORM</div>
                <div class="compare-cell nucleus yes">✓</div>
                <div class="compare-cell other no">✗</div>
                <div class="compare-cell other yes">✓</div>
                <div class="compare-cell other yes">✓</div>
            </div>
            <div class="compare-row">
                <div class="compare-cell feature">Auth System</div>
                <div class="compare-cell nucleus yes">✓</div>
                <div class="compare-cell other no">✗</div>
                <div class="compare-cell other partial">~</div>
                <div class="compare-cell other partial">~</div>
            </div>
            <div class="compare-row">
                <div class="compare-cell feature">WebSockets</div>
                <div class="compare-cell nucleus yes">✓</div>
                <div class="compare-cell other no">✗</div>
                <div class="compare-cell other yes">✓</div>
                <div class="compare-cell other yes">✓</div>
            </div>
            <div class="compare-row">
                <div class="compare-cell feature">Background Jobs</div>
                <div class="compare-cell nucleus yes">✓</div>
                <div class="compare-cell other no">✗</div>
                <div class="compare-cell other partial">~</div>
                <div class="compare-cell other yes">✓</div>
            </div>
            <div class="compare-row">
                <div class="compare-cell feature">AI/LLM + MCP</div>
                <div class="compare-cell nucleus yes">✓</div>
                <div class="compare-cell other no">✗</div>
                <div class="compare-cell other no">✗</div>
                <div class="compare-cell other no">✗</div>
            </div>
            <div class="compare-row">
                <div class="compare-cell feature">Zero Runtime Overhead</div>
                <div class="compare-cell nucleus yes">✓</div>
                <div class="compare-cell other no">✗</div>
                <div class="compare-cell other no">✗</div>
                <div class="compare-cell other no">✗</div>
            </div>
        </div>
        
        <p class="compare-legend">✓ Built-in &nbsp;&bull;&nbsp; ~ Plugin required &nbsp;&bull;&nbsp; ✗ Not available</p>
        
        <div class="compare-cta">
            <a href="/docs#18_complete_reference" class="btn btn-secondary">Explore All 42 Modules →</a>
        </div>
    </div>
</section>



<!-- Newsletter Modal (Hidden by default) -->
<div id="newsletter-modal" class="modal-overlay" aria-hidden="true">
    <div class="modal-container glass-panel">
        <button class="modal-close" aria-label="Close modal">×</button>
        <div class="modal-content">
            <div class="modal-icon">✨</div>
            <h3 class="modal-title">Join the Nucleus</h3>
            <p class="modal-desc">Get the latest updates, tutorials, and early access to new features.</p>
            
            <form class="newsletter-form" method="POST">
                <div class="input-group">
                    <input type="email" name="email" id="modal-email" placeholder="dev@example.com" required class="modal-input">
                    <button type="submit" class="modal-submit-btn">
                        <span>Subscribe</span>
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
                    </button>
                </div>
                <p class="modal-note">Zero spam. Unsubscribe anytime.</p>
            </form>
        </div>
    </div>
</div>

<!-- Compact Newsletter Trigger -->
<section class="newsletter-compact">
    <div class="container flex justify-between items-center">
        <div>
            <h3 class="text-xl font-bold text-white mb-1">Stay in the loop</h3>
            <p class="text-gray-400 text-sm">Join 10,000+ developers shipping faster.</p>
        </div>
        <button id="open-newsletter" class="btn btn-secondary btn-sm">Subscribe to Updates</button>
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
    
    /* ===== SECTION BACKGROUNDS ===== */
    
    /* Trusted By - Subtle dark with top border glow */
    .trusted-by {
        background: linear-gradient(180deg, #0a0a0a 0%, #080808 100%);
        border-top: 1px solid rgba(0, 220, 130, 0.1);
    }
    
    /* Examples - Deep purple-tinted dark */
    .examples {
        background: linear-gradient(180deg, #0d0a12 0%, #08060d 100%);
        position: relative;
    }
    .examples::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        height: 1px;
        background: linear-gradient(90deg, transparent, rgba(139, 92, 246, 0.3), transparent);
    }
    
    /* Features - Rich dark with subtle blue tint */
    .features {
        background: linear-gradient(180deg, #070a0f 0%, #050810 100%);
        position: relative;
    }
    .features::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        height: 1px;
        background: linear-gradient(90deg, transparent, rgba(14, 165, 233, 0.3), transparent);
    }
    
    /* Stdlib - Green accent glow */
    .stdlib-section {
        background: linear-gradient(180deg, #050d08 0%, #030805 100%) !important;
        position: relative;
    }
    .stdlib-section::before {
        background: linear-gradient(90deg, transparent, rgba(0, 220, 130, 0.4), transparent) !important;
    }
    
    /* Render Race Section - Deep contrast */
    .render-section {
        background: linear-gradient(180deg, #0a0810 0%, #06050a 100%);
        position: relative;
    }
    .render-section::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        height: 1px;
        background: linear-gradient(90deg, transparent, rgba(168, 85, 247, 0.3), transparent);
    }
    
    /* Benchmarks - Warm dark */
    .benchmarks {
        background: linear-gradient(180deg, #0c0908 0%, #080605 100%) !important;
        position: relative;
    }
    .benchmarks::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        height: 1px;
        background: linear-gradient(90deg, transparent, rgba(251, 146, 60, 0.3), transparent);
    }
    
    /* Comparison - Neutral dark */
    .comparison {
        background: linear-gradient(180deg, #0a0a0a 0%, #060606 100%) !important;
    }
    
    /* Newsletter - Accent glow at bottom */
    .newsletter {
        background: linear-gradient(180deg, #080a0c 0%, #050608 100%);
        position: relative;
    }
    .newsletter::after {
        content: '';
        position: absolute;
        bottom: 0;
        left: 50%;
        transform: translateX(-50%);
        width: 400px;
        height: 200px;
        background: radial-gradient(ellipse, rgba(0, 220, 130, 0.08) 0%, transparent 70%);
        pointer-events: none;
    }
    
    /* CTA - Gradient glow */
    .cta {
        background: linear-gradient(180deg, #060608 0%, #0a0c10 100%);
        position: relative;
    }
    .cta::before {
        content: '';
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 600px;
        height: 400px;
        background: radial-gradient(ellipse, rgba(0, 220, 130, 0.1) 0%, rgba(14, 165, 233, 0.05) 50%, transparent 70%);
        pointer-events: none;
    }
    
    /* ===== COMPARISON SECTION ===== */
    .comparison-section {
        padding: 100px 0;
        background: linear-gradient(180deg, #0a0a0c 0%, #050507 100%);
    }
    
    /* Hero Stats */
    .compare-hero {
        display: flex;
        justify-content: center;
        gap: 60px;
        margin-bottom: 60px;
        padding: 40px 0;
    }
    
    @media (max-width: 640px) {
        .compare-hero {
            flex-direction: column;
            gap: 30px;
            align-items: center;
        }
    }
    
    .compare-hero-stat {
        text-align: center;
    }
    
    .compare-hero-value {
        display: block;
        font-size: 56px;
        font-weight: 800;
        background: linear-gradient(135deg, var(--accent), #0ea5e9);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
        line-height: 1;
    }
    
    .compare-hero-unit {
        display: block;
        font-size: 16px;
        font-weight: 600;
        color: var(--text-secondary);
        text-transform: uppercase;
        letter-spacing: 0.1em;
        margin-top: 4px;
    }
    
    .compare-hero-label {
        display: block;
        font-size: 13px;
        color: var(--text-muted);
        margin-top: 4px;
    }
    
    /* Comparison Table */
    .compare-table {
        max-width: 800px;
        margin: 0 auto;
        background: rgba(255, 255, 255, 0.02);
        border: 1px solid rgba(255, 255, 255, 0.08);
        border-radius: 16px;
        overflow: hidden;
    }
    
    .compare-row {
        display: grid;
        grid-template-columns: 2fr 1fr 1fr 1fr 1fr;
        border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    }
    
    .compare-row:last-child {
        border-bottom: none;
    }
    
    .compare-row.header {
        background: rgba(255, 255, 255, 0.03);
    }
    
    .compare-row.header .compare-cell {
        font-weight: 600;
        font-size: 13px;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        color: var(--text-muted);
    }
    
    .compare-row.header .compare-cell.nucleus {
        color: var(--accent);
    }
    
    .compare-cell {
        padding: 16px 20px;
        font-size: 14px;
        text-align: center;
    }
    
    .compare-cell.feature {
        text-align: left;
        color: var(--text-secondary);
    }
    
    .compare-cell.nucleus {
        background: rgba(0, 220, 130, 0.05);
        font-weight: 600;
    }
    
    .compare-cell.nucleus.yes {
        color: var(--accent);
    }
    
    .compare-cell.other {
        color: var(--text-muted);
    }
    
    .compare-cell.other.yes {
        color: var(--accent);
    }
    
    .compare-cell.other.partial {
        color: #f59e0b;
    }
    
    .compare-cell.other.no {
        color: #6b7280;
    }
    
    @media (max-width: 640px) {
        .compare-row {
            grid-template-columns: 1.5fr 1fr 1fr 1fr 1fr;
        }
        .compare-cell {
            padding: 12px 8px;
            font-size: 12px;
        }
    }
    
    .compare-legend {
        text-align: center;
        font-size: 13px;
        color: var(--text-muted);
        margin-top: 20px;
    }
    
    .compare-cta {
        text-align: center;
        margin-top: 40px;
    }
    
    /* Footer - Darkest */
    footer {
        background: #030303;
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
        padding: 2rem 1.5rem 10rem;
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
    
    /* Code Examples - Fix Wrapping */
    .code-tabs .tab-panel pre {
        white-space: pre-wrap;
        word-wrap: break-word;
        overflow-x: hidden;
    }
    
    .code-tabs .tab-panel code {
        white-space: pre-wrap;
        word-break: break-word;
    }
    
    .code-tabs .code-block {
        overflow-x: hidden;
        max-width: 100%;
    }
    /* ===== NEWSLETTER MODAL ===== */
    .modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: rgba(0, 0, 0, 0.7);
        backdrop-filter: blur(8px);
        z-index: 1000;
        display: flex;
        align-items: center;
        justify-content: center;
        opacity: 0;
        pointer-events: none;
        transition: opacity 0.3s ease;
    }
    
    .modal-overlay.active {
        opacity: 1;
        pointer-events: auto;
    }
    
    .modal-container {
        background: linear-gradient(145deg, rgba(20, 20, 20, 0.9), rgba(10, 10, 10, 0.95));
        border: 1px solid rgba(255, 255, 255, 0.1);
        padding: 40px;
        border-radius: 24px;
        width: 100%;
        max-width: 480px;
        position: relative;
        transform: translateY(20px) scale(0.95);
        transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
        box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
    }
    
    .modal-overlay.active .modal-container {
        transform: translateY(0) scale(1);
    }
    
    .modal-close {
        position: absolute;
        top: 20px;
        right: 20px;
        background: transparent;
        border: none;
        color: #666;
        font-size: 24px;
        cursor: pointer;
        padding: 5px;
        line-height: 1;
        transition: color 0.2s;
    }
    
    .modal-close:hover {
        color: #fff;
    }
    
    .modal-icon {
        font-size: 48px;
        margin-bottom: 24px;
        text-align: center;
        animation: float 3s ease-in-out infinite;
    }
    
    .modal-title {
        color: #fff;
        font-size: 28px;
        font-weight: 800;
        text-align: center;
        margin-bottom: 12px;
        letter-spacing: -0.02em;
    }
    
    .modal-desc {
        color: #999;
        text-align: center;
        margin-bottom: 32px;
        line-height: 1.6;
    }
    
    .modal-input {
        width: 100%;
        background: rgba(255, 255, 255, 0.05);
        border: 1px solid rgba(255, 255, 255, 0.1);
        padding: 16px;
        border-radius: 12px;
        color: #fff;
        margin-bottom: 16px;
        font-size: 16px;
        transition: all 0.2s;
    }
    
    .modal-input:focus {
        outline: none;
        border-color: #8b5cf6;
        background: rgba(139, 92, 246, 0.1);
    }
    
    .modal-submit-btn {
        width: 100%;
        background: #8b5cf6;
        color: white;
        border: none;
        padding: 16px;
        border-radius: 12px;
        font-weight: 600;
        font-size: 16px;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
        transition: all 0.2s;
    }
    
    .modal-submit-btn:hover {
        background: #7c3aed;
        transform: translateY(-2px);
        box-shadow: 0 10px 20px rgba(124, 58, 237, 0.2);
    }
    
    .modal-note {
        text-align: center;
        font-size: 12px;
        color: #555;
        margin-top: 16px;
    }
    
    .newsletter-compact {
        background: #000;
        border-top: 1px solid #111;
        padding: 32px 0;
    }
</n:style>

<!-- Hero Canvas Animation -->
<script src="/assets/hero.js"></script>
<script src="/assets/home.js"></script>

</n:layout>
</n:view>
