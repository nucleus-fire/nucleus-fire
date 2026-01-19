<n:view title="Server Actions">
    <h1>Server Actions</h1>
    <p>Clicking the button below calls a Rust function marked with <code>#[server]</code>. The backend logic (accessing env vars) is stripped from the client bundle.</p>
    
    <div class="card">
        <h3>User Lookup</h3>
        <div class="form-group">
            <label>User ID</label>
            <input type="number" id="user-id" value="1" style="max-width: 200px">
        </div>
        <button class="btn" onclick="lookupUser()">Fetch Secure Data</button>
        
        <div id="result" class="glass" style="margin-top: 1rem; padding: 1rem; border-radius: 6px; display: none;">
            <pre><code id="json-output"></code></pre>
        </div>
    </div>

    <script>
        async function lookupUser() {
            const id = parseInt(document.getElementById('user-id').value);
            const btn = document.querySelector('button');
            
            btn.innerText = "Loading...";
            
            try {
                // Determine RPC URL (assuming default mapping)
                const res = await fetch('/_rpc/get_secure_user', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(id)
                });
                
                const data = await res.json();
                
                const out = document.getElementById('result');
                out.style.display = 'block';
                document.getElementById('json-output').innerText = JSON.stringify(data, null, 2);
            } catch (e) {
                alert("Error: " + e);
            }
            
            btn.innerText = "Fetch Secure Data";
        }
    </script>
</n:view>
