<n:view title="Documentation | Nucleus Framework">
<n:layout name="layout">

<!-- Enhanced Navigation for Docs -->
<nav class="docs-nav-bar">
    <div class="docs-nav-container">
        <div class="docs-nav-left">
            <n:link href="/" class="docs-nav-logo">
                <span class="logo-icon">⚡</span>
                <span class="logo-text">Nucleus</span>
            </n:link>
            <span class="docs-nav-separator">/</span>
            <span class="docs-nav-section-label">Documentation</span>
        </div>
        <div class="docs-nav-right">
            <n:link href="/" class="docs-nav-item">Home</n:link>
            <n:link href="/#features" class="docs-nav-item">Features</n:link>
            <n:link href="/#benchmarks" class="docs-nav-item">Benchmarks</n:link>
            <n:link href="/playground" class="docs-nav-item">Playground</n:link>
            <a href="https://github.com/nucleus-fire/nucleus-fire" target="_blank" class="docs-nav-item docs-nav-github">
                <svg width="18" height="18" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0C5.374 0 0 5.373 0 12c0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23A11.509 11.509 0 0112 5.803c1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z"/></svg>
                <span>GitHub</span>
            </a>
        </div>
    </div>
</nav>

<!-- External CSS for docs -->
<link href="/assets/docs.css?v=8" rel="stylesheet">

<div class="docs-layout">
    <!-- Sidebar -->
    <aside class="docs-sidebar">
        <div class="docs-search">
            <input type="text" id="doc-search" placeholder="Search docs..." class="docs-search-input">
        </div>
        
        <div class="docs-nav" id="docs-nav">
            <!-- Navigation will be populated dynamically from manifest -->
            <div class="docs-nav-section">
                <h4 class="docs-nav-title">Loading...</h4>
            </div>
        </div>
    </aside>

    <!-- Main Content -->
    <main class="docs-content">
        <div class="docs-content-inner">
            <!-- Breadcrumbs -->
            <div id="docs-breadcrumbs" class="docs-breadcrumbs"></div>

            <div id="docs-body" class="docs-body">
                <!-- Landing Page (shown when no doc selected) -->
                <section class="docs-landing" id="docs-landing">
                    <div class="docs-hero-modern">
                        <div class="docs-hero-content">
                            <span class="hero-badge">Nucleus v1.0.0</span>
                            <h1>Build faster, scale better.</h1>
                            <p>The high-performance framework for modern web applications. Zero-compromise speed, Developer Experience, and Type Safety.</p>
                            <div class="hero-actions">
                                <a href="#24_quick_start_tutorial" class="btn-docs-primary" onclick="loadDoc('24_quick_start_tutorial'); return false;">Get Started →</a>
                                <a href="#02_core_concepts" class="btn-docs-secondary" onclick="loadDoc('02_core_concepts'); return false;">Core Concepts</a>
                            </div>
                        </div>
                    </div>

                    <div class="docs-grid-modern">
                        <div class="docs-card">
                            <div class="card-icon">📚</div>
                            <h3>Guides</h3>
                            <p>In-depth guides for every feature, from Database to Auth.</p>
                            <a href="#20_database_guide" class="docs-card-link" onclick="loadDoc('20_database_guide'); return false;">Explore Guides →</a>
                        </div>
                        <div class="docs-card">
                            <div class="card-icon">⚡</div>
                            <h3>API Reference</h3>
                            <p>Detailed references for the Standard Library, CLI, and Configuration.</p>
                            <a href="#04_stdlib_reference" class="docs-card-link" onclick="loadDoc('04_stdlib_reference'); return false;">View Reference →</a>
                        </div>
                        <div class="docs-card">
                            <div class="card-icon">🛠️</div>
                            <h3>Tooling</h3>
                            <p>Master the Console, Studio, Profiler, and AI Assistant.</p>
                            <a href="#51_developer_tools_guide" class="docs-card-link" onclick="loadDoc('51_developer_tools_guide'); return false;">Learn Tooling →</a>
                        </div>
                    </div>
                    
                    <div class="docs-features-list">
                        <h3>Popular Topics</h3>
                        <div class="topic-tags">
                            <a href="#21_authentication_guide" onclick="loadDoc('21_authentication_guide'); return false;">Authentication</a>
                            <a href="#30_websocket_guide" onclick="loadDoc('30_websocket_guide'); return false;">WebSockets</a>
                            <a href="#07_forms_and_validation" onclick="loadDoc('07_forms_and_validation'); return false;">Forms</a>
                            <a href="#23_deployment_guide" onclick="loadDoc('23_deployment_guide'); return false;">Deployment</a>
                            <a href="#32_middleware_guide" onclick="loadDoc('32_middleware_guide'); return false;">Middleware</a>
                        </div>
                    </div>
                </section>
            </div>

            <!-- Article Navigation -->
            <div id="docs-article-nav" class="docs-article-nav"></div>
        </div>
    </main>

    <!-- Right Sidebar for TOC -->
    <aside class="docs-toc-sidebar">
        <div class="docs-toc-header">On this page</div>
        <div id="docs-toc" class="docs-toc"></div>
    </aside>
</div>

<!-- External script: marked.js (docs.js is loaded via layout.ncl) -->
<script src="https://cdn.jsdelivr.net/npm/marked/marked.min.js"></script>

</n:layout>
</n:view>
