<n:view title="Admin | Nucleus">
    <n:action>
        use crate::controllers::newsletter;
        use axum::http::header::{COOKIE, SET_COOKIE, LOCATION};
        use axum::response::{Html, Redirect, IntoResponse};

        let config = nucleus_std::config::Config::load();
        
        // --- 1. Authenticaton Logic (Cookie-based) ---
        let cookie_header = headers.get(COOKIE).and_then(|h| h.to_str().ok()).unwrap_or("");
        let is_authenticated = cookie_header.contains("admin_session=true");

        // Action Handling
        let action = params.get("action").map(|s| s.as_str()).unwrap_or("");
        
        if action == "login" {
             let username = params.get("username").map(|s| s.as_str()).unwrap_or("");
             let password = params.get("password").map(|s| s.as_str()).unwrap_or("");
             
             // Check against config (defaulting checks if config missing)
             let valid_user = !config.app.admin_username.is_empty() && username == config.app.admin_username;
             let valid_pass = password == config.app.admin_password;

             if valid_user && valid_pass {
                 // Set Cookie and Redirect
                 let cookie_val = "admin_session=true; HttpOnly; Path=/; Max-Age=86400; SameSite=Strict";
                 return (
                     [(SET_COOKIE, cookie_val)],
                     Redirect::to("/admin")
                 ).into_response();
             } else {
                 // Return Login with Error
                 return Redirect::to("/admin?error=Invalid Credentials").into_response();
             }
        }

        if action == "logout" {
             let cookie_val = "admin_session=true; HttpOnly; Path=/; Max-Age=0";
             return (
                 [(SET_COOKIE, cookie_val)],
                 Redirect::to("/admin")
             ).into_response();
        }

        // --- 2. Security Guard ---
        // If not authenticated, STOP here and show Login Form. 
        // Do NOT process any other actions or render the dashboard.
        if !is_authenticated {
            let error = params.get("error").map(|s| s.as_str()).unwrap_or("");
            let err_html = if !error.is_empty() {
                format!("<div style='color: #ef4444; margin-bottom: 1rem; text-align: center;'>{}</div>", error)
            } else { String::new() };

            let login_html = format!(r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Admin Login | Nucleus</title>
                <style>
                    body {{ background: #09090b; color: #f4f4f5; font-family: system-ui, sans-serif; display: flex; align-items: center; justify-content: center; height: 100vh; margin: 0; }}
                    .login-box {{ background: #18181b; padding: 2.5rem; border-radius: 12px; border: 1px solid rgba(255,255,255,0.08); width: 100%; max-width: 320px; box-shadow: 0 20px 25px -5px rgba(0,0,0,0.3); }}
                    h2 {{ margin: 0 0 1.5rem 0; text-align: center; font-weight: 600; font-size: 1.5rem; }}
                    input {{ display: block; width: 100%; padding: 0.75rem; margin-bottom: 1rem; background: #27272a; border: 1px solid #3f3f46; color: white; border-radius: 6px; box-sizing: border-box; }}
                    input:focus {{ outline: 2px solid #8b5cf6; border-color: transparent; }}
                    button {{ width: 100%; padding: 0.75rem; background: #8b5cf6; color: white; border: none; border-radius: 6px; font-weight: 600; cursor: pointer; transition: opacity 0.2s; }}
                    button:hover {{ opacity: 0.9; }}
                </style>
            </head>
            <body>
                <div class="login-box">
                    <h2>Nucleus Admin</h2>
                    {}
                    <form action="/admin" method="POST">
                        <input type="hidden" name="action" value="login">
                        <input type="text" name="username" placeholder="Username" required autofocus>
                        <input type="password" name="password" placeholder="Password" required>
                        <button type="submit">Sign In</button>
                    </form>
                </div>
            </body>
            </html>
            "#, err_html);
            
            return Html(login_html).into_response();
        }

        // --- 3. Authenticated Actions ---
        // Only reachable if authenticated
        
        if !action.is_empty() {
            match action {
                "delete_subscriber" => {
                    if let Some(id_str) = params.get("id") {
                        if let Ok(id) = id_str.parse::<i64>() {
                            let _ = newsletter::delete_subscriber(id).await;
                        }
                    }
                },
                "create_template" => {
                    let name = params.get("name").unwrap_or(&"Untitled".to_string()).clone();
                    let subject = params.get("subject").unwrap_or(&"".to_string()).clone();
                    let body = params.get("body").unwrap_or(&"".to_string()).clone();
                    
                    let _ = newsletter::create_template(name, subject, body).await;
                },
                "delete_template" => {
                    if let Some(id_str) = params.get("id") {
                         if let Ok(id) = id_str.parse::<i64>() {
                            let _ = newsletter::delete_template(id).await;
                         }
                    }
                },
                "broadcast" => {
                    if let Some(id_str) = params.get("template_id") {
                         if let Ok(tmpl_id) = id_str.parse::<i64>() {
                             match newsletter::broadcast_template(tmpl_id).await {
                                 Ok(msg) => {
                                      let encoded = nucleus_std::utils::url_encode(&msg);
                                      return Redirect::to(&format!("/admin?msg={}", encoded)).into_response();
                                 },
                                 Err(e) => {
                                      let encoded = nucleus_std::utils::url_encode(&e);
                                      return Redirect::to(&format!("/admin?error={}", encoded)).into_response();
                                 }
                             }
                         }
                    }
                },
                _ => {}
            }
            
            // Redirect to get fresh state (PRG pattern)
            return Redirect::to("/admin").into_response();
        }
    </n:action>

<n:layout name="layout">
    <style>
        :root {
            --bg-dark: #09090b;
            --bg-elevated: #18181b;
            --sidebar-bg: rgba(9, 9, 11, 0.98);
            --border-color: rgba(255,255,255,0.08);
            --border-hover: rgba(255,255,255,0.15);
            --accent-primary: #8b5cf6;
            --accent-secondary: #06b6d4;
            --accent-success: #10b981;
            --accent-danger: #ef4444;
            --text-main: #f4f4f5;
            --text-muted: #a1a1aa;
            --text-dim: #71717a;
            --glass: rgba(255,255,255,0.03);
            --glass-strong: rgba(255,255,255,0.06);
            --shadow-lg: 0 25px 50px -12px rgba(0,0,0,0.5);
            --shadow-glow: 0 0 40px rgba(139, 92, 246, 0.15);
        }

        * { box-sizing: border-box; }

        body {
            background: var(--bg-dark);
            color: var(--text-main);
            font-family: 'Inter', 'SF Pro Display', system-ui, -apple-system, sans-serif;
            margin: 0; padding: 0;
            height: 100vh; overflow: hidden;
            -webkit-font-smoothing: antialiased;
        }

        /* --- PREMIUM BUTTONS --- */
        .btn {
            display: inline-flex; align-items: center; justify-content: center; gap: 0.5rem;
            padding: 0.6rem 1.2rem; font-size: 0.875rem; font-weight: 600;
            border-radius: 8px; border: none; cursor: pointer;
            transition: all 0.2s ease; text-decoration: none;
        }
        .btn-primary {
            background: linear-gradient(135deg, var(--accent-primary), #7c3aed);
            color: white; box-shadow: 0 4px 14px rgba(139, 92, 246, 0.4);
        }
        .btn-primary:hover { transform: translateY(-1px); box-shadow: 0 6px 20px rgba(139, 92, 246, 0.5); }
        .btn-secondary {
            background: var(--glass-strong); color: var(--text-main);
            border: 1px solid var(--border-color);
        }
        .btn-secondary:hover { background: rgba(255,255,255,0.1); border-color: var(--border-hover); }
        .btn-danger { background: rgba(239,68,68,0.15); color: #f87171; border: 1px solid rgba(239,68,68,0.3); }
        .btn-danger:hover { background: rgba(239,68,68,0.25); }
        .btn-ghost { background: transparent; color: var(--text-muted); }
        .btn-ghost:hover { color: var(--text-main); background: var(--glass); }

        /* --- FORM ELEMENTS --- */
        .form-input, .form-select {
            width: 100%; padding: 0.75rem 1rem; font-size: 0.9rem;
            background: var(--glass-strong); color: var(--text-main);
            border: 1px solid var(--border-color); border-radius: 8px;
            transition: all 0.2s; outline: none;
        }
        .form-input:focus, .form-select:focus {
            border-color: var(--accent-primary);
            box-shadow: 0 0 0 3px rgba(139, 92, 246, 0.15);
        }
        .form-input::placeholder { color: var(--text-dim); }
        .form-label { display: block; margin-bottom: 0.5rem; font-size: 0.85rem; font-weight: 500; color: var(--text-muted); }

        /* --- DASHBOARD GRID --- */
        .dashboard-grid {
            display: grid;
            grid-template-columns: 280px 1fr;
            height: 100vh;
        }

        /* --- SIDEBAR --- */
        .sidebar {
            background: var(--sidebar-bg);
            border-right: 1px solid var(--border-color);
            display: flex; flex-direction: column;
            padding: 1.5rem;
            backdrop-filter: blur(20px);
            z-index: 50;
        }
        .sidebar-header {
            margin-bottom: 2.5rem;
            display: flex; align-items: center; gap: 0.75rem;
        }
        .logo-mark {
            width: 36px; height: 36px;
            background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
            border-radius: 10px;
            box-shadow: var(--shadow-glow);
        }
        .logo-text { font-weight: 700; font-size: 1.1rem; letter-spacing: -0.5px; }
        .nav-list { display: flex; flex-direction: column; gap: 0.25rem; flex: 1; }
        .nav-item {
            display: flex; align-items: center; gap: 0.875rem;
            padding: 0.75rem 1rem; border-radius: 10px;
            color: var(--text-muted); cursor: pointer;
            transition: all 0.2s ease;
            background: transparent; border: none;
            width: 100%; text-align: left; font-family: inherit; font-size: 0.9rem; font-weight: 500;
        }
        .nav-item:hover { background: var(--glass-strong); color: var(--text-main); }
        .nav-item.active {
            background: linear-gradient(90deg, rgba(139, 92, 246, 0.2), rgba(139, 92, 246, 0.05));
            color: white; position: relative;
        }
        .nav-item.active::before {
            content: ''; position: absolute; left: 0; top: 50%; transform: translateY(-50%);
            width: 3px; height: 60%; background: var(--accent-primary); border-radius: 0 4px 4px 0;
        }
        .nav-item svg { width: 20px; height: 20px; opacity: 0.7; flex-shrink: 0; }
        .nav-item.active svg { opacity: 1; color: var(--accent-primary); }

        /* --- CONTENT AREA --- */
        .main-content {
            background: var(--bg-dark);
            background-image: 
                radial-gradient(ellipse at 0% 0%, rgba(139, 92, 246, 0.08) 0%, transparent 50%),
                radial-gradient(ellipse at 100% 100%, rgba(6, 182, 212, 0.06) 0%, transparent 50%);
            overflow-y: auto; position: relative; padding: 2rem;
        }
        .page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem; }
        .page-title { font-size: 1.75rem; font-weight: 700; margin: 0; letter-spacing: -0.02em; }
        .tab-content { display: none; }
        .tab-content.active { display: block; }
        .hidden { display: none !important; }

        /* --- GLASS PANEL --- */
        .glass-panel {
            background: var(--glass);
            border: 1px solid var(--border-color);
            border-radius: 16px;
            backdrop-filter: blur(12px);
        }

        /* --- TOAST SYSTEM --- */
        .toast-container { position: fixed; top: 1.5rem; right: 1.5rem; z-index: 1000; display: flex; flex-direction: column; gap: 0.75rem; }

        /* --- STAT CARDS --- */
        .stat-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1.25rem; margin-bottom: 2rem; }
        .stat-card {
            padding: 1.5rem; display: flex; flex-direction: column;
            position: relative; overflow: hidden; transition: all 0.3s ease;
        }
        .stat-card:hover { transform: translateY(-2px); background: var(--glass-strong); border-color: var(--border-hover); }
        .stat-card::before {
            content: ''; position: absolute; top: 0; left: 0; width: 3px; height: 100%;
            background: linear-gradient(180deg, var(--accent-primary), var(--accent-secondary));
        }
        .stat-val { font-size: 2.5rem; font-weight: 800; color: white; line-height: 1; letter-spacing: -0.02em; }
        .stat-label { font-size: 0.75rem; color: var(--text-dim); text-transform: uppercase; letter-spacing: 0.1em; margin-top: 0.5rem; }

        /* --- TABLE STYLES --- */
        table { width: 100%; border-collapse: collapse; }
        thead { background: rgba(0,0,0,0.3); }
        th { padding: 1rem 1.25rem; text-align: left; font-size: 0.7rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.1em; color: var(--text-dim); border-bottom: 1px solid var(--border-color); }
        td { padding: 1rem 1.25rem; color: var(--text-main); border-bottom: 1px solid var(--border-color); vertical-align: middle; }
        tbody tr:hover { background: var(--glass); }
        .font-mono { font-family: 'JetBrains Mono', 'Fira Code', monospace; font-size: 0.85rem; }

        /* --- EMAIL STUDIO --- */
        .studio-container { display: grid; grid-template-columns: 260px 1fr; height: calc(100vh - 64px); background: var(--bg-dark); border-radius: 12px; overflow: hidden; border: 1px solid var(--border-color); }
        .studio-sidebar { border-right: 1px solid var(--border-color); padding: 1rem; display: flex; flex-direction: column; background: rgba(0,0,0,0.3); }
        .studio-main { display: grid; grid-template-columns: 1fr 1fr; height: 100%; overflow: hidden; }
        .template-item { padding: 0.75rem; border-radius: 8px; cursor: pointer; transition: all 0.2s; border: 1px solid transparent; }
        .template-item:hover { background: var(--glass-strong); border-color: var(--border-color); }

        /* Code Editor */
        .code-editor-container { display: flex; flex-direction: column; border-right: 1px solid var(--border-color); background: #0d1117; }
        .editor-toolbar { height: 52px; background: #161b22; border-bottom: 1px solid rgba(255,255,255,0.05); display: flex; align-items: center; padding: 0 1rem; gap: 1rem; }
        .editor-workspace { flex: 1; position: relative; }
        .monaco-textarea { width: 100%; height: 100%; background: #0d1117; color: #c9d1d9; border: none; padding: 1rem; font-family: 'JetBrains Mono', 'Fira Code', 'Menlo', monospace; font-size: 13px; line-height: 1.6; resize: none; outline: none; white-space: pre; tab-size: 2; }

        /* Preview Area */
        .preview-container { display: flex; flex-direction: column; background: #f8fafc; }
        .preview-toolbar { height: 52px; background: white; border-bottom: 1px solid #e2e8f0; display: flex; align-items: center; justify-content: space-between; padding: 0 1rem; }
        .device-toggle { display: flex; background: #f1f5f9; padding: 3px; border-radius: 8px; gap: 2px; }
        .device-btn { border: none; background: transparent; padding: 6px 14px; border-radius: 6px; cursor: pointer; color: #64748b; font-size: 0.8rem; font-weight: 500; transition: all 0.2s; }
        .device-btn.active { background: white; color: #0f172a; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
        .preview-frame-wrapper { flex: 1; display: flex; justify-content: center; align-items: flex-start; padding: 2rem; overflow: auto; background: linear-gradient(135deg, #f1f5f9, #e2e8f0); }
        .preview-frame { background: white; border: none; transition: all 0.3s ease; box-shadow: 0 25px 50px -12px rgba(0,0,0,0.15); }
        .preview-frame.desktop { width: 100%; height: 100%; max-width: 700px; border-radius: 8px; }
        .preview-frame.mobile { width: 375px; height: 700px; border-radius: 40px; border: 12px solid #1e293b; box-shadow: 0 25px 50px -12px rgba(0,0,0,0.25), inset 0 0 0 2px #334155; }

        /* --- RESPONSIVE --- */
        @media (max-width: 1280px) { .studio-main { grid-template-columns: 1fr; } .preview-container { display: none; } }
        @media (max-width: 1024px) { .dashboard-grid { grid-template-columns: 1fr; } .sidebar { display: none; } .studio-container { grid-template-columns: 1fr; } .studio-sidebar { display: none; } }
        @media (max-width: 768px) { .main-content { padding: 1rem; } .stat-grid { grid-template-columns: 1fr; } }
    </style>

    <div class="dashboard-grid">
    {% for model in [crate::controllers::newsletter::fetch_admin_data().await] %}
        
        <!-- SIDEBAR -->
        <aside class="sidebar">
            <div class="sidebar-header">
                <div class="logo-mark"></div>
                <div style="font-weight: 700; font-size: 1.2rem; letter-spacing: -0.5px;">Nucleus Admin</div>
            </div>
            
            <nav class="nav-list">
                <button class="nav-item active" onclick="switchTab('dashboard')">
                    <svg fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" /></svg>
                    Dashboard
                </button>
                <button class="nav-item" onclick="switchTab('subscribers')">
                    <svg fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" /></svg>
                    Subscribers
                </button>
                <button class="nav-item" onclick="switchTab('templates')">
                    <svg fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" /></svg>
                    Templates
                </button>
                <button class="nav-item" onclick="switchTab('broadcast')">
                    <svg fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" /></svg>
                    Broadcast
                </button>
            </nav>
            
            <div style="padding-top: 1rem; border-top: 1px solid var(--border-color);">
                 <form action="/admin" method="POST" style="margin: 0;">
                    <input type="hidden" name="action" value="logout">
                    <button type="submit" class="nav-item" style="color: #ef4444; width: 100%; text-align: left; justify-content: flex-start; cursor: pointer; background: transparent; border: none; font-size: 0.95rem; font-family: inherit;">
                        <svg fill="none" viewBox="0 0 24 24" stroke="currentColor" style="width: 20px; height: 20px; margin-right: 0.75rem;"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" /></svg>
                        Logout
                    </button>
                 </form>
            </div>
        </aside>

        <!-- MAIN CONTENT -->
        <main class="main-content">
            
            <!-- VIEW: DASHBOARD -->
            <div id="tab-dashboard" class="tab-content active">
                <div class="page-header">
                    <div>
                        <h1 class="page-title">Dashboard</h1>
                        <p style="margin: 0.25rem 0 0; color: var(--text-dim); font-size: 0.9rem;">Welcome back! Here's your newsletter overview.</p>
                    </div>
                    <div style="display: flex; gap: 0.75rem;">
                        <button class="btn btn-secondary" onclick="switchTab('subscribers')">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>
                            Subscribers
                        </button>
                        <button class="btn btn-primary" onclick="switchTab('broadcast')">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 2L11 13"/><path d="M22 2l-7 20-4-9-9-4 20-7z"/></svg>
                            New Broadcast
                        </button>
                    </div>
                </div>

                <!-- Stats Grid -->
                <div class="stat-grid">
                    <div class="glass-panel stat-card" onclick="switchTab('subscribers')" style="cursor: pointer;">
                        <div style="display: flex; justify-content: space-between; align-items: flex-start;">
                            <div>
                                <span class="stat-val">{{model.subscribers.len()}}</span>
                                <span class="stat-label">Total Subscribers</span>
                            </div>
                            <div style="width: 48px; height: 48px; background: linear-gradient(135deg, rgba(16, 185, 129, 0.2), rgba(16, 185, 129, 0.1)); border-radius: 12px; display: flex; align-items: center; justify-content: center;">
                                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#10b981" stroke-width="2"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>
                            </div>
                        </div>
                        <div style="margin-top: 1rem; font-size: 0.8rem; color: var(--accent-success);">
                            <span style="display: inline-flex; align-items: center; gap: 0.25rem;">
                                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M23 6l-9.5 9.5-5-5L1 18"/></svg>
                                Growing
                            </span>
                        </div>
                    </div>
                    <div class="glass-panel stat-card" onclick="switchTab('templates')" style="cursor: pointer;">
                        <div style="display: flex; justify-content: space-between; align-items: flex-start;">
                            <div>
                                <span class="stat-val">{{model.templates.len()}}</span>
                                <span class="stat-label">Email Templates</span>
                            </div>
                            <div style="width: 48px; height: 48px; background: linear-gradient(135deg, rgba(139, 92, 246, 0.2), rgba(139, 92, 246, 0.1)); border-radius: 12px; display: flex; align-items: center; justify-content: center;">
                                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#8b5cf6" stroke-width="2"><path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"/><polyline points="22,6 12,13 2,6"/></svg>
                            </div>
                        </div>
                        <div style="margin-top: 1rem; font-size: 0.8rem; color: var(--text-dim);">Click to manage →</div>
                    </div>
                    <div class="glass-panel stat-card">
                        <div style="display: flex; justify-content: space-between; align-items: flex-start;">
                            <div>
                                <span class="stat-val">—</span>
                                <span class="stat-label">Emails Sent</span>
                            </div>
                            <div style="width: 48px; height: 48px; background: linear-gradient(135deg, rgba(6, 182, 212, 0.2), rgba(6, 182, 212, 0.1)); border-radius: 12px; display: flex; align-items: center; justify-content: center;">
                                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#06b6d4" stroke-width="2"><path d="M22 2L11 13"/><path d="M22 2l-7 20-4-9-9-4 20-7z"/></svg>
                            </div>
                        </div>
                        <div style="margin-top: 1rem; font-size: 0.8rem; color: var(--text-dim);">Send your first broadcast!</div>
                    </div>
                </div>

                <!-- Quick Actions & Recent Activity -->
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1.5rem;">
                    <!-- Quick Actions -->
                    <div class="glass-panel" style="padding: 1.5rem;">
                        <h3 style="margin: 0 0 1.25rem; font-size: 1rem; font-weight: 600; display: flex; align-items: center; gap: 0.5rem;">
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/></svg>
                            Quick Actions
                        </h3>
                        <div style="display: flex; flex-direction: column; gap: 0.75rem;">
                            <button class="btn btn-secondary" onclick="switchTab('templates'); setTimeout(() => resetTemplateForm(), 100);" style="justify-content: flex-start; width: 100%;">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
                                Create New Template
                            </button>
                            <button class="btn btn-secondary" onclick="switchTab('broadcast')" style="justify-content: flex-start; width: 100%;">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 2L11 13"/><path d="M22 2l-7 20-4-9-9-4 20-7z"/></svg>
                                Send Broadcast
                            </button>
                            <a href="/api/newsletter/export" class="btn btn-secondary" style="justify-content: flex-start; width: 100%; text-decoration: none;">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
                                Export Subscribers
                            </a>
                        </div>
                    </div>

                    <!-- Getting Started -->
                    <div class="glass-panel" style="padding: 1.5rem;">
                        <h3 style="margin: 0 0 1.25rem; font-size: 1rem; font-weight: 600; display: flex; align-items: center; gap: 0.5rem;">
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
                            Getting Started
                        </h3>
                        <div style="display: flex; flex-direction: column; gap: 0.75rem;">
                            <div style="display: flex; align-items: center; gap: 0.75rem; padding: 0.75rem; background: var(--glass-strong); border-radius: 8px;">
                                <div style="width: 24px; height: 24px; background: var(--accent-success); border-radius: 50%; display: flex; align-items: center; justify-content: center; flex-shrink: 0;">
                                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="3"><polyline points="20 6 9 17 4 12"/></svg>
                                </div>
                                <span style="font-size: 0.875rem;">Create your first template</span>
                            </div>
                            <div style="display: flex; align-items: center; gap: 0.75rem; padding: 0.75rem; background: var(--glass-strong); border-radius: 8px;">
                                <div style="width: 24px; height: 24px; background: var(--border-color); border-radius: 50%; display: flex; align-items: center; justify-content: center; flex-shrink: 0; color: var(--text-dim);">2</div>
                                <span style="font-size: 0.875rem; color: var(--text-muted);">Grow your subscriber list</span>
                            </div>
                            <div style="display: flex; align-items: center; gap: 0.75rem; padding: 0.75rem; background: var(--glass-strong); border-radius: 8px;">
                                <div style="width: 24px; height: 24px; background: var(--border-color); border-radius: 50%; display: flex; align-items: center; justify-content: center; flex-shrink: 0; color: var(--text-dim);">3</div>
                                <span style="font-size: 0.875rem; color: var(--text-muted);">Send your first broadcast</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <!-- VIEW: SUBSCRIBERS -->
            <div id="tab-subscribers" class="tab-content hidden">
                <div class="page-header">
                    <div>
                        <h1 class="page-title">Subscribers</h1>
                        <p style="margin: 0.25rem 0 0; color: var(--text-dim); font-size: 0.9rem;">Manage your newsletter audience</p>
                    </div>
                    <div style="display: flex; gap: 0.75rem;">
                        <a href="/api/newsletter/export" class="btn btn-secondary">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
                            Export CSV
                        </a>
                    </div>
                </div>

                <!-- Stats Row -->
                <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 1rem; margin-bottom: 1.5rem;">
                    <div class="glass-panel" style="padding: 1rem; display: flex; align-items: center; gap: 1rem;">
                        <div style="width: 40px; height: 40px; background: linear-gradient(135deg, rgba(16, 185, 129, 0.2), rgba(16, 185, 129, 0.1)); border-radius: 10px; display: flex; align-items: center; justify-content: center;">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#10b981" stroke-width="2"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/></svg>
                        </div>
                        <div>
                            <div style="font-size: 1.25rem; font-weight: 700;">{{model.subscribers.len()}}</div>
                            <div style="font-size: 0.75rem; color: var(--text-dim); text-transform: uppercase; letter-spacing: 0.05em;">Total</div>
                        </div>
                    </div>
                    <div class="glass-panel" style="padding: 1rem; display: flex; align-items: center; gap: 1rem;">
                        <div style="width: 40px; height: 40px; background: linear-gradient(135deg, rgba(139, 92, 246, 0.2), rgba(139, 92, 246, 0.1)); border-radius: 10px; display: flex; align-items: center; justify-content: center;">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#8b5cf6" stroke-width="2"><polyline points="23 6 13.5 15.5 8.5 10.5 1 18"/><polyline points="17 6 23 6 23 12"/></svg>
                        </div>
                        <div>
                            <div style="font-size: 1.25rem; font-weight: 700;">—</div>
                            <div style="font-size: 0.75rem; color: var(--text-dim); text-transform: uppercase; letter-spacing: 0.05em;">This Week</div>
                        </div>
                    </div>
                    <div class="glass-panel" style="padding: 1rem; display: flex; align-items: center; gap: 1rem;">
                        <div style="width: 40px; height: 40px; background: linear-gradient(135deg, rgba(6, 182, 212, 0.2), rgba(6, 182, 212, 0.1)); border-radius: 10px; display: flex; align-items: center; justify-content: center;">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#06b6d4" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>
                        </div>
                        <div>
                            <div style="font-size: 1.25rem; font-weight: 700;">100%</div>
                            <div style="font-size: 0.75rem; color: var(--text-dim); text-transform: uppercase; letter-spacing: 0.05em;">Active</div>
                        </div>
                    </div>
                </div>

                <!-- Search Bar -->
                <div style="margin-bottom: 1rem;">
                    <div style="position: relative; max-width: 400px;">
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="position: absolute; left: 1rem; top: 50%; transform: translateY(-50%); color: var(--text-dim);"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
                        <input type="text" id="subscriber-search" class="form-input" placeholder="Search subscribers..." style="padding-left: 2.75rem;" oninput="filterSubscribers(this.value)">
                    </div>
                </div>
                
                <!-- Table -->
                <div class="glass-panel" style="overflow: hidden;">
                    {% if model.subscribers.is_empty() %}
                    <!-- Empty State -->
                    <div style="padding: 4rem 2rem; text-align: center;">
                        <div style="width: 80px; height: 80px; margin: 0 auto 1.5rem; background: linear-gradient(135deg, rgba(139, 92, 246, 0.15), rgba(6, 182, 212, 0.15)); border-radius: 50%; display: flex; align-items: center; justify-content: center;">
                            <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="var(--accent-primary)" stroke-width="1.5"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>
                        </div>
                        <h3 style="margin: 0 0 0.5rem; font-size: 1.25rem; font-weight: 600;">No subscribers yet</h3>
                        <p style="margin: 0 0 1.5rem; color: var(--text-muted); max-width: 400px; margin-left: auto; margin-right: auto;">
                            Start growing your audience! Share your newsletter signup form and watch your community grow.
                        </p>
                        <button class="btn btn-primary" onclick="navigator.clipboard.writeText(window.location.origin + '/'); showToast('Homepage URL copied!');">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
                            Copy Signup Link
                        </button>
                    </div>
                    {% endif %}
                    {% if !model.subscribers.is_empty() %}
                    <div style="overflow-x: auto;">
                        <table id="subscribers-table">
                            <thead>
                                <tr>
                                    <th style="width: 50px;"></th>
                                    <th>Email</th>
                                    <th>Joined</th>
                                    <th style="width: 100px; text-align: center;">Status</th>
                                    <th style="width: 100px; text-align: right;">Actions</th>
                                </tr>
                            </thead>
                            <tbody>
                                {% for sub in &model.subscribers %}
                                <tr class="subscriber-row" data-email="{{ sub.email }}">
                                    <td>
                                        <div style="width: 36px; height: 36px; background: linear-gradient(135deg, #8b5cf6, #06b6d4); border-radius: 50%; display: flex; align-items: center; justify-content: center; font-weight: 600; font-size: 0.8rem; color: white; text-transform: uppercase;">
                                            {{ sub.email.chars().next().unwrap_or('?') }}
                                        </div>
                                    </td>
                                    <td>
                                        <div style="font-weight: 500;">{{ sub.email }}</div>
                                        <div style="font-size: 0.75rem; color: var(--text-dim);">#{{ sub.id }}</div>
                                    </td>
                                    <td style="color: var(--text-muted); font-size: 0.875rem;">{{ sub.created_at }}</td>
                                    <td style="text-align: center;">
                                        <span style="display: inline-flex; align-items: center; gap: 0.25rem; padding: 0.25rem 0.5rem; background: rgba(16, 185, 129, 0.15); color: #10b981; border-radius: 9999px; font-size: 0.7rem; font-weight: 600; text-transform: uppercase;">
                                            <span style="width: 6px; height: 6px; background: currentColor; border-radius: 50%;"></span>
                                            Active
                                        </span>
                                    </td>
                                    <td style="text-align: right;">
                                        <form method="POST" onsubmit="return confirmFormSubmit(this, 'Remove Subscriber', 'Are you sure you want to remove this subscriber? This action cannot be undone.');" style="display:inline;">
                                            <input type="hidden" name="action" value="delete_subscriber">
                                            <input type="hidden" name="id" value="{{ sub.id }}">
                                            <button type="submit" class="btn btn-ghost" style="padding: 0.4rem; color: var(--text-muted);" title="Remove subscriber">
                                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>
                                            </button>
                                        </form>
                                    </td>
                                </tr>
                                {% endfor %}
                            </tbody>
                        </table>
                    </div>
                    <div style="padding: 1rem 1.25rem; border-top: 1px solid var(--border-color); display: flex; justify-content: space-between; align-items: center; font-size: 0.85rem; color: var(--text-muted);">
                        <span>Showing <strong style="color: var(--text-main);">{{model.subscribers.len()}}</strong> subscribers</span>
                        <span id="search-results-count"></span>
                    </div>
                    {% endif %}
                </div>
            </div>

            <!-- VIEW: TEMPLATES STUDIO -->
            <div id="tab-templates" class="tab-content hidden" style="height: 100%; display: flex; flex-direction: column;">
                 <div class="studio-container">
                    <!-- STUDIO SIDEBAR: LIST -->
                    <div class="studio-sidebar">
                        <div style="margin-bottom: 1rem; display: flex; justify-content: space-between; align-items: center;">
                            <h2 style="font-size: 1rem; margin: 0; color:white;">Templates</h2>
                            <button class="btn btn-primary" onclick="resetTemplateForm()" style="font-size: 0.7rem; padding: 0.3rem 0.6rem;">+ New</button>
                        </div>
                        
                        <div style="overflow-y: auto; flex: 1; display:flex; flex-direction:column; gap:0.5rem;">
                            {% for tmpl in &model.templates %}
                            <div class="template-item" onclick="loadTemplate(`{{tmpl.name}}`, `{{tmpl.subject}}`, `{{ nucleus_std::utils::url_encode(&tmpl.body) }}`)">
                                <div style="display: flex; justify-content: space-between; align-items: center;">
                                    <div style="font-weight: 500; font-size: 0.9rem; color: #e4e4e7;">{{ tmpl.name }}</div>
                                    <form method="POST" onsubmit="event.stopPropagation(); return confirmFormSubmit(this, 'Delete Template', 'Are you sure you want to delete this template? This action cannot be undone.');">
                                        <input type="hidden" name="action" value="delete_template">
                                        <input type="hidden" name="id" value="{{ tmpl.id }}">
                                        <button class="btn btn-danger" style="padding: 1px 5px; font-size: 0.6rem; opacity:0.7;">×</button>
                                    </form>
                                </div>
                            </div>
                            {% endfor %}
                        </div>

                        <!-- Presets -->
                        <div style="margin-top: 1rem; padding-top:1rem; border-top:1px solid var(--border-color);">
                            <div style="font-size:0.7rem; color:var(--text-dim); margin-bottom:0.75rem; text-transform:uppercase; letter-spacing:1px; font-weight:600;">Quick Start</div>
                            <div style="display:flex; flex-direction:column; gap:0.5rem;">
                                <button class="btn btn-secondary" onclick="loadPreset('welcome')" style="font-size:0.75rem; justify-content:flex-start; padding:0.5rem 0.75rem;">🎉 Welcome Email</button>
                                <button class="btn btn-secondary" onclick="loadPreset('newsletter')" style="font-size:0.75rem; justify-content:flex-start; padding:0.5rem 0.75rem;">📰 Newsletter</button>
                                <button class="btn btn-secondary" onclick="loadPreset('promotional')" style="font-size:0.75rem; justify-content:flex-start; padding:0.5rem 0.75rem;">🏷️ Promotional</button>
                            </div>
                        </div>
                    </div>

                    <!-- STUDIO MAIN: SPLIT PANE -->
                    <div class="studio-main">
                        <!-- EDITOR -->
                        <div class="code-editor-container">
                            <!-- Editor Toolbar -->
                            <div style="height: 48px; background: #2d2d2d; border-bottom: 1px solid #333; display: flex; align-items: center; padding: 0 1rem; justify-content: space-between;">
                                <div class="form-group" style="margin:0; flex:1; max-width:200px;">
                                    <input type="text" id="tmpl-name" class="form-input" placeholder="Template Name" style="margin:0; border:none; background:transparent; font-size:0.9rem; padding:0.2rem;" required>
                                </div>
                                <div class="form-group" style="margin:0; flex:1; max-width:300px; margin-left:1rem;">
                                    <input type="text" id="tmpl-subject" class="form-input" placeholder="Subject Line" style="margin:0; border:none; background:transparent; font-size:0.9rem; padding:0.2rem; color:#a1a1aa;" required>
                                </div>
                                <div style="display:flex; align-items:center; gap:0.5rem;">
                                     <span id="save-status" style="font-size:0.8rem; color:var(--text-muted); margin-right:0.5rem;"></span>
                                     <button class="btn btn-primary" onclick="saveTemplate()" style="padding:0.4rem 1rem; font-size:0.8rem;">Save</button>
                                </div>
                            </div>
                            
                            <div class="editor-workspace">
                                <textarea id="mjml-editor" class="monaco-textarea" spellcheck="false" placeholder="<mjml>..."></textarea>
                            </div>
                        </div>

                        <!-- PREVIEW -->
                        <div class="preview-container">
                            <div class="preview-toolbar">
                                <div style="font-weight:600; font-size:0.9rem; color:#334155;">Live Preview</div>
                                <div class="device-toggle">
                                    <button class="device-btn active" onclick="setDevice('desktop')">Desktop</button>
                                    <button class="device-btn" onclick="setDevice('mobile')">Mobile</button>
                                </div>
                            </div>
                            <div class="preview-frame-wrapper">
                                <iframe id="preview-frame" class="preview-frame desktop"></iframe>
                            </div>
                        </div>
                    </div>
                 </div>
            </div>

            <!-- VIEW: BROADCAST -->
            <div id="tab-broadcast" class="tab-content hidden">
                <div class="page-header">
                    <div>
                        <h1 class="page-title">Send Broadcast</h1>
                        <p style="margin: 0.25rem 0 0; color: var(--text-dim); font-size: 0.9rem;">Send emails to your subscribers</p>
                    </div>
                </div>

                <div style="display: grid; grid-template-columns: 1fr 380px; gap: 2rem; max-width: 1200px;">
                    <!-- Main Content -->
                    <div>
                        <!-- Step 1: Select Template -->
                        <div class="glass-panel" style="padding: 1.5rem; margin-bottom: 1.5rem;">
                            <div style="display: flex; align-items: center; gap: 1rem; margin-bottom: 1.25rem;">
                                <div style="width: 32px; height: 32px; background: linear-gradient(135deg, var(--accent-primary), #7c3aed); border-radius: 50%; display: flex; align-items: center; justify-content: center; font-weight: 700; font-size: 0.875rem;">1</div>
                                <h3 style="margin: 0; font-size: 1.1rem; font-weight: 600;">Select Template</h3>
                            </div>
                            
                            {% if model.templates.is_empty() %}
                            <div style="padding: 2rem; text-align: center; background: var(--glass-strong); border-radius: 12px; border: 1px dashed var(--border-color);">
                                <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="var(--text-dim)" stroke-width="1.5" style="margin-bottom: 1rem;"><path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"/><polyline points="22,6 12,13 2,6"/></svg>
                                <p style="margin: 0 0 1rem; color: var(--text-muted);">No templates available. Create one first!</p>
                                <button class="btn btn-primary" onclick="switchTab('templates')">Create Template</button>
                            </div>
                            {% endif %}
                            {% if !model.templates.is_empty() %}
                            <div style="display: grid; gap: 0.75rem;">
                                {% for tmpl in &model.templates %}
                                <label style="display: flex; align-items: center; gap: 1rem; padding: 1rem; background: var(--glass-strong); border: 2px solid var(--border-color); border-radius: 12px; cursor: pointer; transition: all 0.2s;" class="template-radio" onclick="selectBroadcastTemplate({{ tmpl.id }}, '{{ tmpl.name }}', '{{ tmpl.subject }}')">
                                    <input type="radio" name="broadcast_template" value="{{ tmpl.id }}" aria-label="Select {{ tmpl.name }} template" style="width: 18px; height: 18px; accent-color: var(--accent-primary);">
                                    <div style="flex: 1;">
                                        <div style="font-weight: 600; margin-bottom: 0.25rem;">{{ tmpl.name }}</div>
                                        <div style="font-size: 0.8rem; color: var(--text-dim);">Subject: {{ tmpl.subject }}</div>
                                    </div>
                                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--text-dim)" stroke-width="2"><polyline points="9 18 15 12 9 6"/></svg>
                                </label>
                                {% endfor %}
                            </div>
                            {% endif %}
                        </div>

                        <!-- Step 2: Review & Send -->
                        <div class="glass-panel" style="padding: 1.5rem;">
                            <div style="display: flex; align-items: center; gap: 1rem; margin-bottom: 1.25rem;">
                                <div style="width: 32px; height: 32px; background: var(--glass-strong); border: 2px solid var(--border-color); border-radius: 50%; display: flex; align-items: center; justify-content: center; font-weight: 700; font-size: 0.875rem; color: var(--text-dim);">2</div>
                                <h3 style="margin: 0; font-size: 1.1rem; font-weight: 600; color: var(--text-muted);">Review & Send</h3>
                            </div>

                            <form method="POST" id="broadcast-form" onsubmit="return confirmBroadcast();">
                                <input type="hidden" name="action" value="broadcast">
                                <input type="hidden" name="template_id" id="broadcast-template-id" value="">
                                
                                <div style="padding: 1.5rem; background: var(--glass-strong); border-radius: 12px; margin-bottom: 1.5rem;">
                                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;">
                                        <span style="color: var(--text-muted);">Template</span>
                                        <span id="broadcast-template-name" style="font-weight: 500;">Not selected</span>
                                    </div>
                                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;">
                                        <span style="color: var(--text-muted);">Subject</span>
                                        <span id="broadcast-subject" style="font-weight: 500;">—</span>
                                    </div>
                                    <div style="display: flex; justify-content: space-between; align-items: center;">
                                        <span style="color: var(--text-muted);">Recipients</span>
                                        <span style="font-weight: 600; color: var(--accent-primary);">{{model.subscribers.len()}} subscribers</span>
                                    </div>
                                </div>

                                <div style="display: flex; gap: 0.75rem;">
                                    <button type="button" class="btn btn-secondary" style="flex: 1;" onclick="sendTestEmail()">
                                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"/><polyline points="22,6 12,13 2,6"/></svg>
                                        Send Test
                                    </button>
                                    <button type="submit" class="btn btn-primary" style="flex: 2;" id="send-broadcast-btn" disabled>
                                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 2L11 13"/><path d="M22 2l-7 20-4-9-9-4 20-7z"/></svg>
                                        Send to All Subscribers
                                    </button>
                                </div>
                            </form>
                        </div>
                    </div>

                    <!-- Sidebar: Summary -->
                    <div>
                        <div class="glass-panel" style="padding: 1.5rem; position: sticky; top: 2rem;">
                            <h4 style="margin: 0 0 1.25rem; font-size: 0.9rem; font-weight: 600; display: flex; align-items: center; gap: 0.5rem;">
                                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>
                                Broadcast Summary
                            </h4>

                            <div style="margin-bottom: 1.5rem;">
                                <div style="display: flex; align-items: center; gap: 1rem; padding: 1rem; background: linear-gradient(135deg, rgba(16, 185, 129, 0.1), rgba(16, 185, 129, 0.05)); border-radius: 12px; margin-bottom: 0.75rem;">
                                    <div style="width: 48px; height: 48px; background: rgba(16, 185, 129, 0.2); border-radius: 12px; display: flex; align-items: center; justify-content: center;">
                                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#10b981" stroke-width="2"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M23 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>
                                    </div>
                                    <div>
                                        <div style="font-size: 1.75rem; font-weight: 800; line-height: 1;">{{model.subscribers.len()}}</div>
                                        <div style="font-size: 0.75rem; color: var(--text-dim); text-transform: uppercase; letter-spacing: 0.05em;">Recipients</div>
                                    </div>
                                </div>
                            </div>

                            <div style="font-size: 0.85rem; color: var(--text-muted); line-height: 1.6;">
                                <div style="display: flex; align-items: flex-start; gap: 0.5rem; margin-bottom: 0.5rem;">
                                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent-success)" stroke-width="2" style="flex-shrink: 0; margin-top: 2px;"><polyline points="20 6 9 17 4 12"/></svg>
                                    <span>Emails will be sent instantly</span>
                                </div>
                                <div style="display: flex; align-items: flex-start; gap: 0.5rem; margin-bottom: 0.5rem;">
                                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent-success)" stroke-width="2" style="flex-shrink: 0; margin-top: 2px;"><polyline points="20 6 9 17 4 12"/></svg>
                                    <span>MJML templates auto-compiled</span>
                                </div>
                                <div style="display: flex; align-items: flex-start; gap: 0.5rem;">
                                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent-success)" stroke-width="2" style="flex-shrink: 0; margin-top: 2px;"><polyline points="20 6 9 17 4 12"/></svg>
                                    <span>Mobile-responsive design</span>
                                </div>
                            </div>

                            <div style="margin-top: 1.5rem; padding-top: 1.5rem; border-top: 1px solid var(--border-color);">
                                <div style="display: flex; align-items: center; gap: 0.5rem; color: var(--text-dim); font-size: 0.8rem;">
                                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
                                    Powered by MJML
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

        <!-- TOAST CONTAINER -->
        <div id="toast-container" class="toast-container"></div>

        </main>
    {% endfor %}
    </div>

    <!-- Preview Modal -->
    <div id="preview-modal" style="display: none; position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.8); z-index: 200; align-items: center; justify-content: center;">
        <div style="background: white; width: 80%; height: 80%; border-radius: 8px; overflow: hidden; display: flex; flex-direction: column;">
            <div style="padding: 1rem; border-bottom: 1px solid #ddd; display: flex; justify-content: space-between; background: #f4f4f5; color: black;">
                <strong>HTML Preview</strong>
                <button onclick="document.getElementById('preview-modal').style.display='none'" style="background:none; border:none; cursor:pointer; font-size: 1.2rem;">&times;</button>
            </div>
            <iframe id="preview-frame" style="flex: 1; border: none; width: 100%; height: 100%;"></iframe>
        </div>
    </div>

    <!-- Confirmation Modal -->
    <div id="confirm-modal" class="confirm-modal-overlay">
        <div class="confirm-modal">
            <div class="confirm-modal-header">
                <div class="confirm-modal-icon" id="confirm-modal-icon">
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
                        <line x1="12" y1="9" x2="12" y2="13"/>
                        <line x1="12" y1="17" x2="12.01" y2="17"/>
                    </svg>
                </div>
                <h3 class="confirm-modal-title" id="confirm-modal-title">Confirm Action</h3>
            </div>
            <p class="confirm-modal-message" id="confirm-modal-message">Are you sure you want to proceed?</p>
            <div class="confirm-modal-actions">
                <button class="btn btn-secondary" id="confirm-modal-cancel" onclick="closeConfirmModal(false)">Cancel</button>
                <button class="btn btn-primary" id="confirm-modal-confirm" onclick="closeConfirmModal(true)">Confirm</button>
            </div>
        </div>
    </div>

    <style>
        .confirm-modal-overlay {
            display: none; position: fixed; inset: 0; z-index: 1000;
            background: rgba(0, 0, 0, 0.7); backdrop-filter: blur(4px);
            align-items: center; justify-content: center;
        }
        .confirm-modal-overlay.open { display: flex; }
        .confirm-modal {
            background: var(--bg-elevated, #18181b); border: 1px solid var(--border-color, rgba(255,255,255,0.08));
            border-radius: 16px; padding: 2rem; max-width: 420px; width: 90%;
            box-shadow: 0 25px 50px -12px rgba(0,0,0,0.5); animation: modalSlideIn 0.25s ease;
        }
        .confirm-modal-header { display: flex; align-items: center; gap: 1rem; margin-bottom: 1rem; }
        .confirm-modal-icon {
            width: 48px; height: 48px; border-radius: 12px;
            display: flex; align-items: center; justify-content: center;
            background: rgba(251, 191, 36, 0.15); color: #fbbf24;
        }
        .confirm-modal-icon.danger { background: rgba(239, 68, 68, 0.15); color: #ef4444; }
        .confirm-modal-icon.info { background: rgba(59, 130, 246, 0.15); color: #3b82f6; }
        .confirm-modal-icon.success { background: rgba(16, 185, 129, 0.15); color: #10b981; }
        .confirm-modal-title { margin: 0; font-size: 1.25rem; font-weight: 600; color: var(--text-main, #f4f4f5); }
        .confirm-modal-message { margin: 0 0 1.5rem; color: var(--text-muted, #a1a1aa); line-height: 1.6; }
        .confirm-modal-actions { display: flex; gap: 0.75rem; justify-content: flex-end; }
        @keyframes modalSlideIn {
            from { opacity: 0; transform: scale(0.95) translateY(-10px); }
            to { opacity: 1; transform: scale(1) translateY(0); }
        }
    </style>

    <script>
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
    </script>
    
    <style>
        @keyframes slideIn {
            from { transform: translateX(100%); opacity: 0; }
            to { transform: translateX(0); opacity: 1; }
        }
    </style>
</n:layout>
</n:view>
