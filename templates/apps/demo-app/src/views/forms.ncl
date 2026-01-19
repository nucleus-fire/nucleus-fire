<n:view title="Forms & Validation">
    <h1>Forms & Validation</h1>
    <p>Nucleus forms support automatic model binding and validation. Try the form below.</p>

    <div class="card" style="max-width: 600px">
        <form onsubmit="handleSubmit(event)">
            <div class="form-group">
                <label>Email Address</label>
                <input type="email" name="email" required placeholder="you@example.com">
            </div>
            
            <div class="form-group">
                <label>Password</label>
                <input type="password" name="password" required minlength="8">
            </div>
            
            <div class="form-group">
                <label>Profile Type</label>
                <select name="type" style="width: 100%; padding: 0.75rem; background: var(--bg); color: var(--text); border: 1px solid var(--border); border-radius: 6px;">
                    <option value="user">Standard User</option>
                    <option value="admin">Administrator</option>
                </select>
            </div>

            <button type="submit" class="btn">Create Account</button>
        </form>
    </div>

    <script>
        function handleSubmit(e) {
            e.preventDefault();
            const formData = new FormData(e.target);
            const data = Object.fromEntries(formData);
            
            alert("Form Data Captured:\n" + JSON.stringify(data, null, 2));
            // In a real app: await fetch('/api/submit', { ... })
        }
    </script>
</n:view>
