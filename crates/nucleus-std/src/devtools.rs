pub fn get_script() -> &'static str {
    r#"
    (function() {
        if (window.__NUCLEUS_DEVTOOLS_INSTALLED__) return;
        window.__NUCLEUS_DEVTOOLS_INSTALLED__ = true;

        class NucleusDevTools extends HTMLElement {
            constructor() {
                super();
                this.attachShadow({ mode: 'open' });
            }

            connectedCallback() {
                this.shadowRoot.innerHTML = `
                <style>
                    :host {
                        position: fixed;
                        bottom: 20px;
                        right: 20px;
                        z-index: 99999;
                        font-family: system-ui, sans-serif;
                    }
                    .badge {
                        width: 40px;
                        height: 40px;
                        background: #111;
                        border: 1px solid #333;
                        border-radius: 50%;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        cursor: pointer;
                        box-shadow: 0 4px 12px rgba(0,0,0,0.5);
                        font-size: 20px;
                        transition: transform 0.2s;
                    }
                    .badge:hover {
                        transform: scale(1.1);
                        border-color: #4facfe;
                    }
                    .panel {
                        position: absolute;
                        bottom: 50px;
                        right: 0;
                        width: 300px;
                        background: rgba(17, 17, 17, 0.95);
                        backdrop-filter: blur(10px);
                        border: 1px solid #333;
                        border-radius: 8px;
                        padding: 1rem;
                        color: white;
                        display: none;
                        max-height: 400px;
                        overflow-y: auto;
                    }
                    .panel.open {
                        display: block;
                    }
                    h3 { margin: 0 0 0.5rem 0; font-size: 14px; color: #4facfe; }
                    .row { font-size: 12px; margin-bottom: 4px; display: flex; justify-content: space-between; }
                    .key { color: #888; }
                    .val { color: #eee; font-family: monospace; }
                </style>
                <div class="badge">⚛️</div>
                <div class="panel">
                    <h3>Nucleus DevTools</h3>
                    <div id="content"></div>
                </div>
                `;

                this.badge = this.shadowRoot.querySelector('.badge');
                this.panel = this.shadowRoot.querySelector('.panel');
                this.content = this.shadowRoot.querySelector('#content');

                this.badge.addEventListener('click', () => {
                    this.panel.classList.toggle('open');
                    this.render();
                });

                // Periodic refresh
                setInterval(() => {
                    if (this.panel.classList.contains('open')) {
                        this.render();
                    }
                }, 1000);
            }

            render() {
                let html = '';
                
                // Route Info
                html += `<div class="row"><span class="key">Path</span><span class="val">${window.location.pathname}</span></div>`;
                
                // Memory Usage (if available)
                if (window.performance && window.performance.memory) {
                     const mem = Math.round(window.performance.memory.usedJSHeapSize / 1024 / 1024);
                     html += `<div class="row"><span class="key">Memory</span><span class="val">${mem} MB</span></div>`;
                }

                // Check for HMR socket
                const wsStatus = window.__NUCLEUS_HMR_SOCKET__ ? (window.__NUCLEUS_HMR_SOCKET__.readyState === 1 ? 'Connected' : 'Disconnected') : 'Inactive';
                 html += `<div class="row"><span class="key">HMR</span><span class="val">${wsStatus}</span></div>`;

                // Scan for Signals (naive scan of global scope or DOM)
                // For now, just static info
                
                this.content.innerHTML = html;
            }
        }

        customElements.define('nucleus-devtools', NucleusDevTools);
        document.body.appendChild(document.createElement('nucleus-devtools'));
    })();
    "#
}
