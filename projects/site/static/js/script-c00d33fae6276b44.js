
        function switchTab(tabId) {
            document.querySelectorAll('.tab-content').forEach(el => el.classList.remove('active'));
            document.querySelectorAll('.tab-btn').forEach(el => {
                el.classList.remove('active');
                if (el.getAttribute('onclick') && el.getAttribute('onclick').includes(tabId)) {
                    el.classList.add('active');
                }
            });
            document.getElementById('tab-' + tabId).classList.add('active');
        }

        // Flash Messages
        function showFlash(msg, type = 'success') {
            const container = document.getElementById('flash-container');
            const div = document.createElement('div');
            div.style.padding = '1rem 2rem';
            div.style.marginBottom = '1rem';
            div.style.borderRadius = '8px';
            div.style.color = 'white';
            div.style.background = type === 'error' ? '#ef4444' : '#10b981';
            div.style.boxShadow = '0 4px 12px rgba(0,0,0,0.3)';
            div.style.animation = 'slideIn 0.3s ease-out';
            div.innerText = decodeURIComponent(msg.replace(/\+/g, ' '));
            
            container.appendChild(div);
            
            setTimeout(() => {
                div.style.opacity = '0';
                setTimeout(() => div.remove(), 300);
            }, 5000);
        }

        // Init Flash
        const urlParams = new URLSearchParams(window.location.search);
        if(urlParams.has('msg')) showFlash(urlParams.get('msg'), 'success');
        if(urlParams.has('error')) showFlash(urlParams.get('error'), 'error');

        // Template Loading
        function loadTemplate(name, subject, encodedBody) {
            document.querySelector('input[name="name"]').value = name;
            document.querySelector('input[name="subject"]').value = subject;
            document.querySelector('textarea[name="body"]').value = decodeURIComponent(encodedBody.replace(/\+/g, ' '));
            switchTab('templates');
        }

        function previewHtml() {
            const content = document.querySelector('textarea[name="body"]').value;
            const modal = document.getElementById('preview-modal');
            const frame = document.getElementById('preview-frame');
            
            frame.srcdoc = content;
            modal.style.display = 'flex';
        }

        function checkQuality() {
            const content = document.querySelector('textarea[name="body"]').value;
            let score = 100;
            let issues = [];

            if(content.length < 50) {
                score -= 20;
                issues.push("Content is too short.");
            }
            if(!content.includes("unsubscribe")) {
                score -= 30;
                issues.push("Missing 'unsubscribe' link (Critical).");
            }
            if(content.includes("$$$") || content.toLowerCase().includes("buy now")) {
                score -= 10;
                issues.push("Potential spam trigger words found.");
            }

            let msg = `Quality Score: ${score}/100\n\n`;
            if(issues.length > 0) {
                msg += "Issues:\n- " + issues.join("\n- ");
            } else {
                msg += "Looks good!";
            }
            alert(msg);
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
        }
    