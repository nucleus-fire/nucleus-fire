
        // Init Mobile Nav (if needed in future, currently CSS handled)
        
        function switchTab(tabId) {
            // Hide all tab contents
            document.querySelectorAll('.tab-content').forEach(el => el.classList.add('hidden'));
            
            // Show target tab
            const target = document.getElementById('tab-' + tabId);
            if(target) target.classList.remove('hidden');

            // Update Nav state
            document.querySelectorAll('.nav-item').forEach(el => {
                el.classList.remove('active');
                if (el.getAttribute('onclick') && el.getAttribute('onclick').includes(tabId)) {
                    el.classList.add('active');
                }
            });
            
            // Special case for Dashboard active state reset if click came from elsewhere
            if(tabId === 'dashboard') { /* Handle dashboard logic if needed */ }
        }

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

        // MJML Templates
        const TEMPLATES = {
            welcome: `<mjml>
  <mj-head>
    <mj-title>Welcome to Nucleus</mj-title>
    <mj-attributes>
      <mj-all font-family="'Helvetica', 'Arial', sans-serif"></mj-all>
      <mj-text font-weight="400" font-size="16px" color="#000000" line-height="24px"></mj-text>
    </mj-attributes>
  </mj-head>
  <mj-body background-color="#F4F4F4">
    <mj-section padding="20px 0">
      <mj-column>
        <mj-image width="150px" src="https://via.placeholder.com/150x50?text=Logo" alt="Logo"></mj-image>
      </mj-column>
    </mj-section>
    <mj-section background-color="#ffffff" padding="40px 20px" border-radius="8px">
      <mj-column>
        <mj-text align="center" font-size="24px" font-weight="700">Welcome Aboard!</mj-text>
        <mj-text align="center" color="#555555">We're thrilled to have you with us. Here is what you can expect...</mj-text>
        <mj-button background-color="#8b5cf6" color="white" href="#">Get Started</mj-button>
      </mj-column>
    </mj-section>
    <mj-section>
      <mj-column>
        <mj-text align="center" font-size="12px" color="#999999">
          <a href="{{unsubscribe_link}}" style="color: #999999;">Unsubscribe</a>
        </mj-text>
      </mj-column>
    </mj-section>
  </mj-body>
</mjml>`,
            newsletter: `<mjml>
  <mj-body background-color="#ffffff">
    <mj-section>
      <mj-column>
        <mj-text font-size="20px" color="#8b5cf6" font-weight="bold">Weekly Update</mj-text>
        <mj-divider border-color="#8b5cf6"></mj-divider>
        <mj-text>Here are the top stories for this week...</mj-text>
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
            if(confirm("Overwrite current editor content?")) {
                getEditor().value = TEMPLATES[key] || '';
                triggerCompile();
            }
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

            if(!name || !body) { alert("Name and Body required"); return; }

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
                const url = new URL('/admin', window.location.origin);

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
    