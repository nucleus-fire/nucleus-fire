
        function switchTab(tabId) {
            // Hide all
            document.querySelectorAll('.tab-content').forEach(el => el.classList.remove('active'));
            document.querySelectorAll('.tab-btn').forEach(el => el.classList.remove('active'));
            
            // Show target
            document.getElementById('tab-' + tabId).classList.add('active');
            
            // Activate btn - finding by text is brittle, better usage would be data attrs, but for now loop
            // actually I passed the tabId.
            // Using event.target would be better but I used inline onclick.
            // Let's just rely on re-querying by onclick attr match or simple logic
            const btns = document.querySelectorAll('.tab-btn');
            btns.forEach(btn => {
                if(btn.getAttribute('onclick').includes(tabId)) {
                    btn.classList.add('active');
                }
            });
        }
    