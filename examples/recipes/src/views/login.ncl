<n:view title="Login">
    <n:script>
        // Check for login attempt
        if let Some(user) = params.get("user") {
            if crate::logic::auth::login(user).await {
                // Success: Set Cookie and Redirect
                let cookie = format!("session={}; Path=/; HttpOnly", user);
                let mut headers = axum::http::HeaderMap::new();
                headers.insert("Set-Cookie", cookie.parse().unwrap());
                return (headers, axum::response::Redirect::to("/todo")).into_response();
            }
        }
    </n:script>

    <div style="font-family: system-ui; max-width: 400px; margin: 4rem auto; padding: 2rem; border: 1px solid #333; border-radius: 8px; background: #111; color: white;">
        <h1 style="text-align: center;">🔐 Login</h1>
        <form action="/login" method="get" style="display: flex; flex-direction: column; gap: 1rem;">
            <input name="user" placeholder="Username" required style="padding: 0.8rem; border-radius: 4px; border: 1px solid #444; background: #222; color: white;" />
            <button type="submit" style="padding: 0.8rem; background: #4facfe; color: white; border: none; border-radius: 4px; font-weight: bold; cursor: pointer;">Sign In</button>
        </form>
    </div>
</n:view>
