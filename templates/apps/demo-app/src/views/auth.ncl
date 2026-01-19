<n:view title="Authentication">
    <h1>Authentication</h1>
    <p>Secure login flow with session management.</p>

    <div class="card" style="max-width: 400px; margin: 2rem auto;">
        <h2 style="text-align: center; margin-bottom: 2rem;">Sign In</h2>
        <form>
            <div class="form-group">
                <label>Email</label>
                <input type="email" value="admin@nucleus.dev">
            </div>
            <div class="form-group">
                <label>Password</label>
                <input type="password" value="password123">
            </div>
            <button type="button" class="btn" style="width: 100%" onclick="alert('Authenticated successfully (Mock)! Session Cookie set.')">Sign In</button>
        </form>
        <p style="text-align: center; margin-top: 1rem; font-size: 0.9rem;">
            <a href="#" style="color: var(--primary);">Forgot password?</a>
        </p>
    </div>
</n:view>
