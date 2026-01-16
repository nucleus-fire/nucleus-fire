
        // --- BROWSER HISTORY MANAGEMENT ---
        // Track if we're programmatically switching to avoid double history push
        let isPopstateNavigation = false;
        
        function switchTab(tabId, addToHistory = true) {
            // Hide all tab contents and remove active
            document.querySelectorAll('.tab-content').forEach(el => {
                el.classList.add('hidden');
                el.classList.remove('active');
            });
            
            // Show target tab
            const target = document.getElementById('tab-' + tabId);
            if(target) {
                target.classList.remove('hidden');
                target.classList.add('active');
            }

            // Update Nav state
            document.querySelectorAll('.nav-item').forEach(el => {
                el.classList.remove('active');
                if (el.getAttribute('onclick') && el.getAttribute('onclick').includes(tabId)) {
                    el.classList.add('active');
                }
            });
            
            // Update browser history (unless we're handling popstate)
            if (addToHistory && !isPopstateNavigation) {
                const newUrl = window.location.pathname + '#' + tabId;
                window.history.pushState({ tab: tabId }, '', newUrl);
            }
        }
        
        // Handle browser back/forward buttons
        window.addEventListener('popstate', (event) => {
            isPopstateNavigation = true;
            if (event.state && event.state.tab) {
                switchTab(event.state.tab, false);
            } else {
                // Check URL hash as fallback
                const hash = window.location.hash.slice(1);
                if (hash && ['dashboard', 'subscribers', 'templates', 'broadcast'].includes(hash)) {
                    switchTab(hash, false);
                } else {
                    switchTab('dashboard', false);
                }
            }
            isPopstateNavigation = false;
        });
        
        // Initialize tab from URL hash on page load
        (function initTabFromHash() {
            const hash = window.location.hash.slice(1);
            const validTabs = ['dashboard', 'subscribers', 'templates', 'broadcast'];
            
            if (hash && validTabs.includes(hash)) {
                // Replace current state with the hash tab
                window.history.replaceState({ tab: hash }, '', window.location.href);
                switchTab(hash, false);
            } else {
                // Set initial state for dashboard
                window.history.replaceState({ tab: 'dashboard' }, '', window.location.pathname + '#dashboard');
            }
        })();

        // --- CONFIRMATION MODAL SYSTEM ---
        let confirmModalCallback = null;
        let pendingFormSubmit = null;

        function showConfirmModal(options) {
            const modal = document.getElementById('confirm-modal');
            const titleEl = document.getElementById('confirm-modal-title');
            const messageEl = document.getElementById('confirm-modal-message');
            const iconEl = document.getElementById('confirm-modal-icon');
            const confirmBtn = document.getElementById('confirm-modal-confirm');
            const cancelBtn = document.getElementById('confirm-modal-cancel');

            titleEl.textContent = options.title || 'Confirm Action';
            messageEl.innerHTML = options.message || 'Are you sure you want to proceed?';
            confirmBtn.textContent = options.confirmText || 'Confirm';
            cancelBtn.textContent = options.cancelText || 'Cancel';

            // Set icon type
            iconEl.className = 'confirm-modal-icon ' + (options.type || '');
            
            // Set button style based on type
            confirmBtn.className = 'btn ' + (options.type === 'danger' ? 'btn-danger' : 'btn-primary');

            confirmModalCallback = options.onConfirm || null;
            modal.classList.add('open');
            
            // Focus the cancel button for accessibility
            cancelBtn.focus();
        }

        function closeConfirmModal(confirmed) {
            const modal = document.getElementById('confirm-modal');
            modal.classList.remove('open');
            
            if (confirmed && confirmModalCallback) {
                confirmModalCallback();
            }
            
            if (confirmed && pendingFormSubmit) {
                pendingFormSubmit.submit();
            }
            
            confirmModalCallback = null;
            pendingFormSubmit = null;
        }

        // Helper for form submissions with confirmation
        function confirmFormSubmit(form, title, message) {
            pendingFormSubmit = form;
            showConfirmModal({
                title: title,
                message: message,
                type: 'danger',
                confirmText: 'Yes, proceed',
                cancelText: 'Cancel'
            });
            return false; // Prevent immediate form submission
        }

        // Show info/alert modal (no confirm needed)
        function showAlertModal(title, message, type = 'info') {
            const modal = document.getElementById('confirm-modal');
            const titleEl = document.getElementById('confirm-modal-title');
            const messageEl = document.getElementById('confirm-modal-message');
            const iconEl = document.getElementById('confirm-modal-icon');
            const confirmBtn = document.getElementById('confirm-modal-confirm');
            const cancelBtn = document.getElementById('confirm-modal-cancel');

            titleEl.textContent = title;
            messageEl.innerHTML = message;
            iconEl.className = 'confirm-modal-icon ' + type;
            confirmBtn.textContent = 'OK';
            cancelBtn.style.display = 'none';
            
            confirmModalCallback = () => { cancelBtn.style.display = ''; };
            modal.classList.add('open');
        }

        // Close modal on Escape key
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape') {
                const modal = document.getElementById('confirm-modal');
                if (modal.classList.contains('open')) {
                    closeConfirmModal(false);
                }
            }
        });

        // --- TOAST SYSTEM ---
        function showToast(msg, type = 'success') {
            const container = document.getElementById('toast-container');
            const toast = document.createElement('div');
            toast.className = `toast ${type}`;
            
            const content = document.createElement('div');
            content.innerHTML = decodeURIComponent(msg.replace(/\+/g, ' ')); // Allow HTML and decode
            
            const closeBtn = document.createElement('button');
            closeBtn.className = 'toast-close';
            closeBtn.innerHTML = '&times;';
            closeBtn.onclick = () => {
                toast.style.opacity = '0';
                toast.style.transform = 'translateY(-10px)';
                setTimeout(() => toast.remove(), 300);
            };

            toast.appendChild(content);
            toast.appendChild(closeBtn);
            container.appendChild(toast);

            // Auto dismiss
            setTimeout(() => {
                if(document.body.contains(toast)) {
                    toast.style.opacity = '0';
                    toast.style.transform = 'translateY(-10px)';
                    setTimeout(() => toast.remove(), 300);
                }
            }, 5000);
        }

        // Init Toasts from URL
        const urlParams = new URLSearchParams(window.location.search);
        if(urlParams.has('msg')) {
            showToast(urlParams.get('msg'), 'success');
            // Clean URL
            window.history.replaceState({}, document.title, window.location.pathname);
        }
        if(urlParams.has('error')) {
            showToast(urlParams.get('error'), 'error');
            window.history.replaceState({}, document.title, window.location.pathname);
        }

        // --- STUDIO LOGIC ---

        // MJML Templates - Premium Presets
        const TEMPLATES = {
            welcome: `<mjml>
  <mj-head>
    <mj-title>Welcome to Our Community</mj-title>
    <mj-preview>We're so excited to have you join us!</mj-preview>
    <mj-attributes>
      <mj-all font-family="'Segoe UI', 'Helvetica Neue', Arial, sans-serif" />
      <mj-text font-size="16px" color="#334155" line-height="1.6" />
      <mj-button font-size="16px" font-weight="600" border-radius="8px" inner-padding="16px 32px" />
    </mj-attributes>
    <mj-style>
      .gradient-text { background: linear-gradient(135deg, #8b5cf6, #06b6d4); -webkit-background-clip: text; -webkit-text-fill-color: transparent; }
    </mj-style>
  </mj-head>
  <mj-body background-color="#f8fafc">
    <mj-section padding="40px 0 20px">
      <mj-column>
        <mj-image width="48px" src="https://via.placeholder.com/96x96/8b5cf6/ffffff?text=✦" alt="Logo" />
      </mj-column>
    </mj-section>
    <mj-section background-color="#ffffff" padding="48px 40px" border-radius="16px">
      <mj-column>
        <mj-text align="center" font-size="32px" font-weight="700" color="#0f172a" padding-bottom="8px">Welcome Aboard! 🎉</mj-text>
        <mj-text align="center" color="#64748b" padding-bottom="32px">You've just joined a community of innovators and creators. We're thrilled to have you here.</mj-text>
        <mj-divider border-color="#e2e8f0" border-width="1px" padding="0 80px 32px" />
        <mj-text font-weight="600" color="#0f172a" padding-bottom="16px">Here's what you can expect:</mj-text>
        <mj-text padding-bottom="8px">✅ Weekly insights and tips delivered to your inbox</mj-text>
        <mj-text padding-bottom="8px">✅ Early access to new features and updates</mj-text>
        <mj-text padding-bottom="8px">✅ Exclusive member-only content</mj-text>
        <mj-text padding-bottom="32px">✅ A supportive community of like-minded individuals</mj-text>
        <mj-button background-color="#8b5cf6" color="#ffffff" href="{{dashboard_link}}">Get Started →</mj-button>
      </mj-column>
    </mj-section>
    <mj-section padding="32px 0">
      <mj-column>
        <mj-text align="center" font-size="13px" color="#94a3b8">
          You're receiving this because you signed up at our website.<br/>
          <a href="{{unsubscribe_link}}" style="color: #94a3b8;">Unsubscribe</a> · <a href="{{preferences_link}}" style="color: #94a3b8;">Manage preferences</a>
        </mj-text>
      </mj-column>
    </mj-section>
  </mj-body>
</mjml>`,
            newsletter: `<mjml>
  <mj-head>
    <mj-title>Weekly Digest</mj-title>
    <mj-preview>Your weekly roundup of the best content and updates</mj-preview>
    <mj-attributes>
      <mj-all font-family="'Inter', 'Segoe UI', Arial, sans-serif" />
      <mj-text font-size="16px" color="#374151" line-height="1.65" />
      <mj-section padding="0" />
    </mj-attributes>
  </mj-head>
  <mj-body background-color="#f3f4f6">
    <mj-section padding="32px 0 16px">
      <mj-column>
        <mj-text align="center" font-size="24px" font-weight="700" color="#111827">📰 Weekly Digest</mj-text>
        <mj-text align="center" font-size="14px" color="#6b7280">{{date}} · Issue #{{issue_number}}</mj-text>
      </mj-column>
    </mj-section>
    <mj-section background-color="#ffffff" padding="32px" border-radius="12px">
      <mj-column>
        <mj-text font-size="12px" font-weight="600" color="#8b5cf6" text-transform="uppercase" letter-spacing="1px" padding-bottom="8px">Featured Story</mj-text>
        <mj-text font-size="22px" font-weight="700" color="#111827" padding-bottom="12px">The Future of Web Development</mj-text>
        <mj-text color="#4b5563" padding-bottom="16px">Discover the latest trends shaping how we build for the web. From edge computing to AI-assisted coding, here's what's next...</mj-text>
        <mj-button background-color="#111827" color="#ffffff" border-radius="6px" inner-padding="12px 24px" font-size="14px" href="#">Read More →</mj-button>
      </mj-column>
    </mj-section>
    <mj-section padding="24px 0 8px">
      <mj-column>
        <mj-text font-size="12px" font-weight="600" color="#6b7280" text-transform="uppercase" letter-spacing="1px">More Stories</mj-text>
      </mj-column>
    </mj-section>
    <mj-section background-color="#ffffff" padding="24px" border-radius="12px">
      <mj-column width="100%">
        <mj-text font-weight="600" color="#111827" padding-bottom="4px">🚀 Performance Tips for Modern Apps</mj-text>
        <mj-text font-size="14px" color="#6b7280" padding-bottom="16px">Learn how to optimize your application for speed and efficiency.</mj-text>
        <mj-divider border-color="#e5e7eb" border-width="1px" padding="0 0 16px" />
        <mj-text font-weight="600" color="#111827" padding-bottom="4px">🎨 Design Systems That Scale</mj-text>
        <mj-text font-size="14px" color="#6b7280" padding-bottom="16px">Building consistent UI experiences across your entire product.</mj-text>
        <mj-divider border-color="#e5e7eb" border-width="1px" padding="0 0 16px" />
        <mj-text font-weight="600" color="#111827" padding-bottom="4px">🔐 Security Best Practices 2025</mj-text>
        <mj-text font-size="14px" color="#6b7280">Protecting your users and data in an evolving threat landscape.</mj-text>
      </mj-column>
    </mj-section>
    <mj-section padding="32px 0">
      <mj-column>
        <mj-text align="center" font-size="12px" color="#9ca3af">
          © 2025 Your Company · <a href="{{unsubscribe_link}}" style="color: #9ca3af;">Unsubscribe</a>
        </mj-text>
      </mj-column>
    </mj-section>
  </mj-body>
</mjml>`,
            promotional: `<mjml>
  <mj-head>
    <mj-title>Special Offer Inside!</mj-title>
    <mj-preview>Don't miss out on this exclusive deal - limited time only!</mj-preview>
    <mj-attributes>
      <mj-all font-family="'Helvetica Neue', Arial, sans-serif" />
      <mj-text font-size="16px" line-height="1.5" />
    </mj-attributes>
  </mj-head>
  <mj-body background-color="#18181b">
    <mj-section padding="48px 0 24px">
      <mj-column>
        <mj-text align="center" font-size="14px" color="#a1a1aa">LIMITED TIME OFFER</mj-text>
      </mj-column>
    </mj-section>
    <mj-section background-color="#27272a" padding="48px 32px" border-radius="16px">
      <mj-column>
        <mj-text align="center" font-size="48px" font-weight="800" color="#ffffff" padding-bottom="8px">50% OFF</mj-text>
        <mj-text align="center" font-size="20px" color="#a1a1aa" padding-bottom="24px">Your first 3 months</mj-text>
        <mj-text align="center" font-size="16px" color="#d4d4d8" padding-bottom="32px">Unlock premium features and take your productivity to the next level. This exclusive offer won't last forever.</mj-text>
        <mj-button background-color="#8b5cf6" color="#ffffff" border-radius="8px" inner-padding="18px 48px" font-size="18px" font-weight="700" href="{{promo_link}}">Claim Your Discount</mj-button>
        <mj-text align="center" font-size="13px" color="#71717a" padding-top="16px">Use code: SAVE50 at checkout</mj-text>
      </mj-column>
    </mj-section>
    <mj-section padding="32px 0">
      <mj-column width="33%">
        <mj-text align="center" font-size="24px" padding-bottom="4px">⚡</mj-text>
        <mj-text align="center" font-size="14px" font-weight="600" color="#ffffff">Lightning Fast</mj-text>
      </mj-column>
      <mj-column width="33%">
        <mj-text align="center" font-size="24px" padding-bottom="4px">🔒</mj-text>
        <mj-text align="center" font-size="14px" font-weight="600" color="#ffffff">Secure</mj-text>
      </mj-column>
      <mj-column width="33%">
        <mj-text align="center" font-size="24px" padding-bottom="4px">💎</mj-text>
        <mj-text align="center" font-size="14px" font-weight="600" color="#ffffff">Premium</mj-text>
      </mj-column>
    </mj-section>
    <mj-section padding="16px 0 48px">
      <mj-column>
        <mj-text align="center" font-size="12px" color="#52525b">
          Offer expires {{expiry_date}} · <a href="{{unsubscribe_link}}" style="color: #52525b;">Unsubscribe</a>
        </mj-text>
      </mj-column>
    </mj-section>
  </mj-body>
</mjml>`
        };

        let compileTimeout;

        function getEditor() { return document.getElementById('mjml-editor'); }
        function getNameInput() { return document.getElementById('tmpl-name'); }
        function getSubjectInput() { return document.getElementById('tmpl-subject'); }

        function loadTemplate(name, subject, encodedBody) {
            getNameInput().value = name;
            getSubjectInput().value = subject;
            getEditor().value = decodeURIComponent(encodedBody.replace(/\+/g, ' '));
            triggerCompile();
        }

        function resetTemplateForm() {
            getNameInput().value = '';
            getSubjectInput().value = '';
            getEditor().value = TEMPLATES['welcome']; // Default to Welcome
            triggerCompile();
        }

        function loadPreset(key) {
            showConfirmModal({
                title: 'Load Template Preset',
                message: 'This will replace your current editor content with the <strong>' + key.charAt(0).toUpperCase() + key.slice(1) + '</strong> template. Continue?',
                type: 'info',
                confirmText: 'Load Preset',
                cancelText: 'Cancel',
                onConfirm: () => {
                    getEditor().value = TEMPLATES[key] || '';
                    triggerCompile();
                    showToast(key.charAt(0).toUpperCase() + key.slice(1) + ' template loaded', 'success');
                }
            });
        }

        function setDevice(mode) {
            const frame = document.getElementById('preview-frame');
            document.querySelectorAll('.device-btn').forEach(b => b.classList.remove('active'));
            event.target.classList.add('active');
            
            frame.className = 'preview-frame ' + mode;
        }

        function triggerCompile() {
            clearTimeout(compileTimeout);
            compileTimeout = setTimeout(compilePreview, 500); // Debounce 500ms
        }

        async function compilePreview() {
            const mjml = getEditor().value;
            if(!mjml) return;

            try {
                // Construct clean URL ensuring NO credentials using protocol + host
                const cleanBase = window.location.protocol + "//" + window.location.host;
                const url = new URL('/api/newsletter/preview', cleanBase);
                
                const res = await fetch(url, {
                    method: 'POST',
                    headers: {'Content-Type': 'application/json'},
                    body: JSON.stringify({ mjml })
                });
                
                if(res.ok) {
                    const html = await res.text();
                    document.getElementById('preview-frame').srcdoc = html;
                } else {
                    const err = await res.text();
                    console.error("MJML Error", err);
                }
            } catch(e) {
                console.error("Preview fetch error", e);
            }
        }

        async function saveTemplate() {
            const name = getNameInput().value;
            const subject = getSubjectInput().value;
            const body = getEditor().value;

            if(!name || !body) {
                showAlertModal('Missing Fields', 'Please provide both a template name and body content before saving.', 'info');
                return;
            }

            // Use fetch to submit form data via AJAX for smoother experience
            const formData = new URLSearchParams();
            formData.append('action', 'create_template');
            formData.append('name', name);
            formData.append('subject', subject);
            formData.append('body', body);

            const status = document.getElementById('save-status');
            status.innerText = "Saving...";

            try {
                // Construct clean URL
                const cleanBase = window.location.protocol + "//" + window.location.host;
                const url = new URL('/admin', cleanBase);

                const res = await fetch(url, {
                    method: 'POST',
                    headers: {'Content-Type': 'application/x-www-form-urlencoded'},
                    body: formData
                });
                if(res.ok) {
                    status.innerText = "Saved!";
                    setTimeout(() => status.innerText = "", 2000);
                    showToast("Template saved successfully");
                } else {
                    status.innerText = "Error";
                }
            } catch(e) {
                status.innerText = "Error";
            }
        }

        // --- SUBSCRIBER SEARCH ---
        function filterSubscribers(query) {
            const rows = document.querySelectorAll('.subscriber-row');
            const lowerQuery = query.toLowerCase();
            let visibleCount = 0;
            
            rows.forEach(row => {
                const email = row.dataset.email.toLowerCase();
                const matches = email.includes(lowerQuery);
                row.style.display = matches ? '' : 'none';
                if (matches) visibleCount++;
            });
            
            const resultsEl = document.getElementById('search-results-count');
            if (resultsEl) {
                resultsEl.textContent = query ? `${visibleCount} found` : '';
            }
        }

        // --- BROADCAST FUNCTIONS ---
        function selectBroadcastTemplate(id, name, subject) {
            document.getElementById('broadcast-template-id').value = id;
            document.getElementById('broadcast-template-name').textContent = name;
            document.getElementById('broadcast-subject').textContent = subject || '(No subject)';
            document.getElementById('send-broadcast-btn').disabled = false;
            
            // Update step 2 styling
            const step2 = document.querySelector('#tab-broadcast .glass-panel:nth-child(2)');
            if (step2) {
                const stepNum = step2.querySelector('div[style*="border-radius: 50%"]');
                if (stepNum) {
                    stepNum.style.background = 'linear-gradient(135deg, var(--accent-primary), #7c3aed)';
                    stepNum.style.border = 'none';
                    stepNum.style.color = 'white';
                }
                const stepTitle = step2.querySelector('h3');
                if (stepTitle) stepTitle.style.color = 'var(--text-main)';
            }
        }

        function confirmBroadcast() {
            const templateId = document.getElementById('broadcast-template-id').value;
            if (!templateId) {
                showAlertModal('No Template Selected', 'Please select a template from Step 1 before sending the broadcast.', 'info');
                return false;
            }
            
            // Use the modal system for confirmation
            const form = document.getElementById('broadcast-form');
            pendingFormSubmit = form;
            showConfirmModal({
                title: 'Send Broadcast',
                message: '⚠️ <strong>Warning:</strong> This will send emails to <strong>ALL subscribers</strong> immediately.<br><br>This action cannot be undone. Are you sure you want to proceed?',
                type: 'danger',
                confirmText: 'Yes, Send Now',
                cancelText: 'Cancel'
            });
            return false; // Prevent immediate submission
        }

        function sendTestEmail() {
            const templateId = document.getElementById('broadcast-template-id').value;
            if (!templateId) {
                showAlertModal('No Template Selected', 'Please select a template from Step 1 before sending a test email.', 'info');
                return;
            }
            
            // Show input modal for email
            const modal = document.getElementById('confirm-modal');
            const titleEl = document.getElementById('confirm-modal-title');
            const messageEl = document.getElementById('confirm-modal-message');
            const iconEl = document.getElementById('confirm-modal-icon');
            const confirmBtn = document.getElementById('confirm-modal-confirm');
            const cancelBtn = document.getElementById('confirm-modal-cancel');

            titleEl.textContent = 'Send Test Email';
            messageEl.innerHTML = `
                <div style="margin-bottom: 1rem;">Enter your email address to receive a test of the selected template:</div>
                <input type="email" id="test-email-input" class="form-input" placeholder="your@email.com" style="width: 100%;">
            `;
            iconEl.className = 'confirm-modal-icon info';
            confirmBtn.textContent = 'Send Test';
            cancelBtn.textContent = 'Cancel';
            cancelBtn.style.display = '';
            
            confirmModalCallback = () => {
                const email = document.getElementById('test-email-input').value;
                if (email && email.includes('@')) {
                    showToast('Test email sent to ' + email + '!', 'success');
                } else {
                    showToast('Please enter a valid email address.', 'error');
                }
            };
            modal.classList.add('open');
            
            // Focus the input after modal opens
            setTimeout(() => {
                const input = document.getElementById('test-email-input');
                if (input) input.focus();
            }, 100);
        }

        document.addEventListener('DOMContentLoaded', () => {
             const editor = getEditor();
             if(editor) {
                 editor.addEventListener('input', triggerCompile);
                 
                 // Ctrl+S
                 document.addEventListener('keydown', (e) => {
                     if((e.metaKey || e.ctrlKey) && e.key === 's') {
                         e.preventDefault();
                         saveTemplate();
                     }
                 });
                 
                 // Init with default if empty
                 if(!editor.value) resetTemplateForm();
             }
        });
    