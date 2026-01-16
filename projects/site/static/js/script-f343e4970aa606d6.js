
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

        // --- TEMPLATE MANAGEMENT ---
        function loadTemplate(name, subject, encodedBody) {
            document.querySelector('input[name="name"]').value = name;
            document.querySelector('input[name="subject"]').value = subject;
            document.querySelector('textarea[name="body"]').value = decodeURIComponent(encodedBody.replace(/\+/g, ' '));
            updateQualityIndicator(); // Check quality on load
        }

        function resetTemplateForm() {
            document.querySelector('input[name="name"]').value = '';
            document.querySelector('input[name="subject"]').value = '';
            document.querySelector('textarea[name="body"]').value = '';
            updateQualityIndicator();
        }

        function previewHtml() {
            const content = document.querySelector('textarea[name="body"]').value;
            // Add the wrapper preview locally if possible, or just raw content for now
            // To be accurate to backend, we might want to simulate wrapper, but raw is fine for checking layout
            const modal = document.getElementById('preview-modal');
            const frame = document.getElementById('preview-frame');
            frame.srcdoc = content;
            modal.style.display = 'flex';
        }

        // --- LIVE QUALITY CHECK ---
        const qualityIndicator = document.getElementById('quality-indicator');
        const qualityText = document.getElementById('quality-text');

        function updateQualityIndicator() {
            const content = document.querySelector('textarea[name="body"]').value || "";
            let score = 100;
            let issues = 0;

            if(content.length < 50) { score -= 20; issues++; }
            if(!content.includes("insert_unsubscribe_link") && !content.includes("{{unsubscribe_link}}")) { score -= 30; issues++; }
            if(content.includes("$$$") || content.toLowerCase().includes("buy now")) { score -= 10; issues++; }

            const indicator = document.getElementById('quality-indicator');
            const text = document.getElementById('quality-text');
            
            if (!indicator || !text) return;

            text.innerText = `Quality: ${score}/100`;
            
            // Color Logic
            indicator.style.background = score > 80 ? '#10b981' : (score > 50 ? '#f59e0b' : '#ef4444');
            indicator.style.boxShadow = score > 80 ? '0 0 10px #10b981' : 'none';
        }

        function checkQuality() {
            // Manual check trigger (still useful for specific alerts)
             const content = document.querySelector('textarea[name="body"]').value;
             // Reuse logic or just alert
             updateQualityIndicator();
             alert("Quality check updated. See indicator.");
        }

        function insertAtCursor(before, after) {
            const textarea = document.getElementById('template-body');
            const start = textarea.selectionStart;
            const end = textarea.selectionEnd;
            const text = textarea.value;
            
            const selectedText = text.substring(start, end);
            const replacement = before + selectedText + after;
            
            textarea.value = text.substring(0, start) + replacement + text.substring(end);
            textarea.selectionStart = start + before.length;
            textarea.selectionEnd = start + before.length + selectedText.length;
            textarea.focus();
            updateQualityIndicator(); // Update on insert
        }

        // Attach Listener
        document.addEventListener('DOMContentLoaded', () => {
             const textarea = document.getElementById('template-body');
             if(textarea) {
                 textarea.addEventListener('input', updateQualityIndicator);
             }
        });
    