<n:view title="Playground | Nucleus Framework">
    <n:layout name="clean_layout">
        <link href="/assets/playground.css?v=6" rel="stylesheet">
        <link href="/assets/playground_sidebar.css?v=6" rel="stylesheet">

        
        <!-- Playground Container -->
        <div class="playground-container" role="main" id="main-content">
            <!-- Header Bar -->
            <header class="playground-header">
                <div class="playground-header-left">
                    <n:link href="/" class="playground-logo" aria-label="Nucleus Home">
                        <span class="logo-icon">⚡</span>
                        <span class="logo-text">Nucleus</span>
                    </n:link>
                    <span class="playground-separator">/</span>
                    <h1 class="playground-title">Playground</h1>
                </div>
                
                <div class="playground-header-center">
                    <!-- Example selector moved to sidebar -->
                </div>
                
                <div class="playground-header-right">
                    <button id="run-btn" class="playground-btn playground-btn-primary" aria-label="Run code">
                        <span class="btn-icon">▶</span>
                        <span class="btn-text">Run</span>
                    </button>
                    <button id="share-btn" class="playground-btn" aria-label="Share code">
                        <span class="btn-icon">🔗</span>
                        <span class="btn-text">Share</span>
                    </button>
                    <button id="download-btn" class="playground-btn" aria-label="Download HTML">
                        <span class="btn-icon">⬇</span>
                        <span class="btn-text">Download</span>
                    </button>
                    <n:link href="/docs" class="playground-btn" aria-label="Documentation">
                        <span class="btn-icon">📚</span>
                        <span class="btn-text">Docs</span>
                    </n:link>
                </div>
            </header>
            
            <!-- Main Playground Area -->
            <div class="playground-main">
                <!-- Sidebar -->
                <aside class="playground-sidebar">
                    <!-- Project Header -->
                    <div class="sidebar-project-header">
                        <button class="project-dropdown-btn" id="project-dropdown-btn" aria-haspopup="true">
                            <span class="project-icon">📁</span>
                            <span class="project-name" id="current-project-name">My Project</span>
                            <span class="dropdown-arrow">▾</span>
                        </button>
                        <div class="project-dropdown hidden" id="project-dropdown">
                            <div class="dropdown-section">
                                <div class="dropdown-header">Projects</div>
                                <div id="projects-list"></div>
                            </div>
                            <div class="dropdown-divider"></div>
                            <button class="dropdown-item" onclick="createNewProject()">
                                <span>➕</span> New Project
                            </button>
                        </div>
                    </div>

                    <!-- Search -->
                    <div class="sidebar-search">
                        <span class="search-icon">🔍</span>
                        <input type="text" id="file-search" placeholder="Search files..." class="search-input">
                    </div>

                    <!-- View Tabs -->
                    <div class="sidebar-tabs" role="tablist">
                        <button class="sidebar-tab active" data-view="files" role="tab" aria-selected="true">Files</button>
                        <button class="sidebar-tab" data-view="favorites" role="tab" aria-selected="false">★</button>
                        <button class="sidebar-tab" data-view="recents" role="tab" aria-selected="false">⏱</button>
                    </div>

                <!-- Dynamic Content Area (SCROLLABLE) -->
                    <div class="sidebar-content" id="sidebar-content">
                        <!-- Files View (default) -->
                        <div class="sidebar-view active" id="files-view">
                            <div class="file-tree-header">
                                <span class="tree-title">Workspace</span>
                                <button class="tree-action-btn" onclick="openModal('new-file-dialog')" title="New File">+</button>
                            </div>
                            <div class="file-tree" id="file-tree">
                                <!-- Dynamic file tree -->
                            </div>
                            
                            <!-- Divider -->
                            <div class="sidebar-divider"></div>

                            <!-- Templates Section (inside files view so it scrolls) -->
                            <div class="sidebar-section collapsible">
                                <button class="section-header" onclick="toggleSection('templates')">
                                    <span>📚 Templates</span>
                                    <span class="collapse-icon" id="templates-collapse-icon">▾</span>
                                </button>
                                <div class="section-content" id="templates-section">
                                    <div class="template-list">
                                        <div class="template-group">
                                            <div class="template-group-title">Starters</div>
                                            <button class="template-item" data-template="hello">⚡ Hello World</button>
                                            <button class="template-item" data-template="landing">🚀 Landing Page</button>
                                        </div>
                                        <div class="template-group">
                                            <div class="template-group-title">Components</div>
                                            <button class="template-item" data-template="card">🎴 Card Component</button>
                                            <button class="template-item" data-template="button">🔘 Button System</button>
                                            <button class="template-item" data-template="form">📝 Forms & Inputs</button>
                                            <button class="template-item" data-template="wizard">📋 Multi-Step Form</button>
                                        </div>
                                        <div class="template-group">
                                            <div class="template-group-title">Full Apps</div>
                                            <button class="template-item" data-template="dashboard">📊 Dashboard</button>
                                            <button class="template-item" data-template="auth">🔐 Auth Pages</button>
                                        </div>
                                        <div class="template-group">
                                            <div class="template-group-title">State Management</div>
                                            <button class="template-item" data-template="state_client">⚡ Client (Simple)</button>
                                            <button class="template-item" data-template="state_form">📝 Form (Server)</button>
                                            <button class="template-item" data-template="state_island">⚛️ Neutron (Complex)</button>
                                        </div>
                                        <div class="template-group">
                                            <div class="template-group-title">Advanced Patterns</div>
                                            <button class="template-item" data-template="imports">📦 Imports</button>
                                            <button class="template-item" data-template="data">📊 Data Binding</button>
                                            <button class="template-item" data-template="counter">🔢 Counter</button>
                                            <button class="template-item" data-template="todo">✅ Todo App</button>
                                            <button class="template-item" data-template="blog">📰 Blog Layout</button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <!-- Favorites View -->
                        <div class="sidebar-view hidden" id="favorites-view">
                            <div class="empty-state" id="favorites-empty">
                                <span class="empty-icon">⭐</span>
                                <p>No favorites yet</p>
                                <p class="empty-hint">Click ★ on a project to add it here</p>
                            </div>
                            <div class="project-cards" id="favorites-list"></div>
                        </div>

                        <!-- Recents View -->
                        <div class="sidebar-view hidden" id="recents-view">
                            <div class="empty-state" id="recents-empty">
                                <span class="empty-icon">⏱</span>
                                <p>No recent projects</p>
                            </div>
                            <div class="project-cards" id="recents-list"></div>
                        </div>
                    </div>

                    <!-- Settings Footer (FIXED) -->
                    <div class="sidebar-section sidebar-footer">
                        <label class="sidebar-checkbox">
                            <input type="checkbox" id="use-tailwind" checked>
                            <span class="checkbox-label">
                                <span class="icon">🌊</span> Tailwind CSS
                            </span>
                        </label>
                        <div class="sidebar-resources">
                            <button class="resource-toggle-btn" onclick="toggleSection('resources')">
                                <span>📦 Resources</span>
                                <span class="collapse-icon" id="resources-collapse-icon">▸</span>
                            </button>
                            <div class="section-content hidden" id="resources-section">
                                <div class="resource-group">
                                    <label class="resource-label">CSS</label>
                                    <div id="css-resources"></div>
                                    <button class="sidebar-btn-sm" onclick="addResource('css')">+ Add</button>
                                </div>
                                <div class="resource-group">
                                    <label class="resource-label">JS</label>
                                    <div id="js-resources"></div>
                                    <button class="sidebar-btn-sm" onclick="addResource('js')">+ Add</button>
                                </div>
                            </div>
                        </div>
                    </div>
                </aside>

                <!-- Editor & Preview Wrapper -->
                <div class="playground-workspace">
                    <!-- Editor Panel -->
                    <div class="playground-panel playground-editor-panel" id="editor-panel">
                        <div class="panel-header">
                            <div class="panel-tabs" role="tablist" id="editor-tabs">
                                <!-- Dynamic Tabs -->
                            </div>
                            <div class="panel-actions">
                                <button id="format-btn" class="panel-action-btn" aria-label="Format code" title="Format (Ctrl+Shift+F)">
                                    ✨
                                </button>
                                <button id="copy-btn" class="panel-action-btn" aria-label="Copy code" title="Copy">
                                    📋
                                </button>
                            </div>
                        </div>
                        <div class="editor-container" id="editor-container">
                            <div id="monaco-container" style="width: 100%; height: 100%; position: absolute; top: 0; left: 0;"></div>
                            <textarea id="code-editor" class="code-editor hidden" spellcheck="false" placeholder="Write your Nucleus template here..."></textarea>
                            <textarea id="css-editor" class="code-editor hidden" spellcheck="false" placeholder="Add custom styles..."></textarea>
                        </div>
                    </div>
                    
                    <!-- Resize Handle -->
                    <div class="playground-resizer" id="resizer" role="separator" aria-valuenow="50" tabindex="0" aria-label="Resize panels"></div>
                    
                    <!-- Preview Panel -->
                    <div class="playground-panel playground-preview-panel" id="preview-panel">
                        <div class="panel-header">
                            <div class="panel-tabs">
                                <button class="panel-tab active" role="tab" data-preview="render">
                                    <span class="tab-icon">👁</span>
                                    Preview
                                </button>
                                <button class="panel-tab" role="tab" data-preview="html">
                                    <span class="tab-icon">🔧</span>
                                    HTML Output
                                </button>
                            </div>
                            <div class="panel-status">
                                <span id="status-indicator" class="status-indicator status-ready" aria-live="polite">Ready</span>
                            </div>
                        </div>
                        <div class="preview-container">
                            <iframe id="preview-frame" class="preview-frame" title="Live preview" sandbox="allow-scripts allow-modals allow-forms"></iframe>
                            <pre id="html-output" class="html-output hidden" aria-label="Generated HTML output"><code></code></pre>
                        </div>
                    </div>
                </div>
            </div>
            
            <!-- Footer Status Bar -->
            <footer class="playground-footer">
                <div class="footer-left">
                    <span class="status-item">
                        <span class="status-dot"></span>
                        <span id="compile-status">Ready</span>
                    </span>
                </div>
                <div class="footer-center">
                    <kbd>Ctrl</kbd> + <kbd>Enter</kbd> to run
                    <span class="footer-separator">|</span>
                    <kbd>Ctrl</kbd> + <kbd>S</kbd> to save
                </div>
                <div class="footer-right">
                    <span class="footer-hint">Powered by Nucleus</span>
                </div>
            </footer>
        </div>
        
        <!-- Share Modal -->
        <div id="share-modal" class="modal hidden" role="dialog" aria-labelledby="share-modal-title" aria-modal="true">
            <div class="modal-backdrop"></div>
            <div class="modal-content">
                <header class="modal-header">
                    <h2 id="share-modal-title">Share Your Code</h2>
                    <button class="modal-close" onclick="closeModal('share-modal')" aria-label="Close modal">&times;</button>
                </header>
                <div class="modal-body">
                    <p>Copy this URL to share your code:</p>
                    <div class="share-url-container">
                        <input type="text" id="share-url" class="share-url-input" readonly aria-label="Shareable URL">
                        <button id="copy-share-btn" class="playground-btn playground-btn-primary">Copy</button>
                    </div>
                </div>
            </div>
        </div>

        <!-- Confirm Dialog (Semantic) -->
        <dialog id="confirm-dialog" class="playground-dialog">
            <form method="dialog">
                <header class="dialog-header">
                    <h2 class="dialog-title">Switch Template?</h2>
                </header>
                <div class="dialog-body">
                    <p>This will overwrite your current code.</p>
                    <p id="version-warning" class="hidden text-warning">You have a saved version of this template. Which one would you like to load?</p>
                </div>
                <div class="dialog-footer">
                    <button value="cancel" formnovalidate type="button" class="playground-btn" onclick="document.getElementById('confirm-dialog').close()">Cancel</button>
                    <div class="dialog-actions">
                        <button id="btn-load-saved" value="saved" class="playground-btn playground-btn-secondary hidden">Load Saved</button>
                        <button value="confirm" class="playground-btn playground-btn-primary">Load Original</button>
                    </div>
                </div>
            </form>
        </dialog>

        <!-- Resource Dialog -->
        <dialog id="resource-dialog" class="playground-dialog">
            <form method="dialog">
                <header class="dialog-header">
                    <h2 class="dialog-title" id="resource-dialog-title">Add Resource</h2>
                </header>
                <div class="dialog-body">
                    <label class="sidebar-label" style="margin-bottom: 0.5rem; display: block;">Resource URL</label>
                    <input type="url" id="resource-input" class="dialog-input" placeholder="https://..." required>
                    <p class="text-sm text-muted">Supports CDN links (jsdelivr, unpkg, etc.)</p>
                </div>
                <div class="dialog-footer">
                    <button value="cancel" formnovalidate type="button" class="playground-btn" onclick="document.getElementById('resource-dialog').close()">Cancel</button>
                    <button id="btn-add-resource" value="add" class="playground-btn playground-btn-primary">Add Resource</button>
                </div>
            </form>

        </dialog>

        <!-- New File Dialog -->
        <dialog id="new-file-dialog" class="playground-dialog">
            <form method="dialog">
                <header class="dialog-header">
                    <h2 class="dialog-title">New File</h2>
                </header>
                <div class="dialog-body">
                    <label class="sidebar-label" style="margin-bottom: 0.5rem; display: block;">Filename</label>
                    <input type="text" id="new-filename" class="dialog-input" placeholder="component.ncl" required pattern="^[a-zA-Z0-9._-]+$">
                    <p class="text-sm text-muted">Supported: .ncl, .css, .js</p>
                </div>
                <div class="dialog-footer">
                    <button value="cancel" formnovalidate type="button" class="playground-btn" onclick="document.getElementById('new-file-dialog').close()">Cancel</button>
                    <button id="btn-create-file" type="button" class="playground-btn playground-btn-primary">Create</button>
                </div>
            </form>
        </dialog>
        
        <!-- Monaco Editor CDN (loaded async) -->
        <script src="https://cdn.jsdelivr.net/npm/monaco-editor@0.45.0/min/vs/loader.js"></script>
        <script src="/assets/playground.js?v=55"></script>
    </n:layout>
</n:view>
