use clap::Subcommand;
use std::fs;
use crate::animations::{build_step, colors};
use std::path::Path;
use miette::{IntoDiagnostic, Result};

#[derive(Subcommand, Debug)]
pub enum GenerateCommands {
    /// Generates a full scaffold (Model, View, Migration)
    Scaffold {
        /// Name of the resource (singular, e.g. "User")
        name: String,
        /// Fields in format name:type (e.g. title:string count:int)
        #[arg(trailing_var_arg = true)]
        fields: Vec<String>,
    },
    /// Generates just a Model
    Model {
        name: String,
        fields: Vec<String>,
    },
    /// Generates Payment components (Stripe + Crypto)
    Payments {
        /// Include subscription pricing table
        #[arg(long)]
        subscription: bool,
    },
}

pub fn handle_generate(command: &GenerateCommands) -> Result<()> {
    match command {
        GenerateCommands::Scaffold { name, fields } => {
            println!("\n{}🏗️  Scaffolding resource: {}{}", colors::CYAN, name, colors::RESET);
            
            build_step("📦", &format!("Generating Model: {}", name));
            generate_model(name, fields)?;
            
            build_step("🗄️", "Creating Migration...");
            generate_migration(name, fields)?;
            
            build_step("🎮", "Generating Controllers & Views...");
            generate_controllers(name, fields)?;
            
            build_step("🔗", "Linking logic...");
            update_lib_rs(name)?;
            
            println!("\n{}✅ Scaffold complete for '{}'{}\n", colors::GREEN, name, colors::RESET);
        }
        GenerateCommands::Model { name, fields } => {
             println!("\n{}🏗️  Generating model: {}{}", colors::CYAN, name, colors::RESET);
             generate_model(name, fields)?;
             println!("\n{}✅ Model complete for '{}'{}\n", colors::GREEN, name, colors::RESET);
        }
        GenerateCommands::Payments { subscription } => {
            println!("\n{}💳 Generating Payment Components...{}", colors::CYAN, colors::RESET);
            generate_payments(*subscription)?;
            println!("\n{}✅ Payments initialized!{}", colors::GREEN, colors::RESET);
        }
    }
    Ok(())
}

fn generate_model(name: &str, fields: &[String]) -> Result<()> {
    let lower = name.to_lowercase();
    let content = generate_model_content(name, fields);

    // Write
    let path = format!("src/logic/{}.rs", lower);
    fs::create_dir_all("src/logic").into_diagnostic()?;
    fs::write(&path, content).into_diagnostic()?;
    build_step("  +", &format!("Created {}", path));
    Ok(())
}

fn generate_model_content(name: &str, fields: &[String]) -> String {
    let cap = capitalize(name);
    let lower = name.to_lowercase();
    let table = format!("{}s", lower);

    // 1. Struct Fields
    let struct_fields = fields.iter().map(|f| {
        let (fname, ftype, _) = parse_field(f);
        format!("    pub {}: {},", fname, ftype)
    }).collect::<Vec<_>>().join("\n");

    // 2. DTO Fields (No ID)
    let dto_fields = fields.iter().map(|f| {
        let (fname, ftype, _) = parse_field(f);
        format!("    pub {}: {},", fname, ftype)
    }).collect::<Vec<_>>().join("\n");

    // 3. Insert Values
    let insert_values = fields.iter().map(|f| {
        let (fname, _, _) = parse_field(f);
        format!("            .value(\"{}\", payload.{})", fname, fname)
    }).collect::<Vec<_>>().join("\n");

    // 4. Update Values
    let update_values = insert_values.clone();

    format!(r#"use serde::{{Deserialize, Serialize}};
use nucleus_std::photon::query::{{Model, Builder}};
use nucleus_std::models; // Assuming impl_model is here or re-exported
use nucleus_std::server;
use nucleus_std::errors::Result;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct {cap} {{
    pub id: i64,
{struct_fields}
}}

#[derive(Debug, Deserialize, Serialize)]
pub struct Create{cap} {{
{dto_fields}
}}

#[derive(Debug, Deserialize, Serialize)]
pub struct Update{cap} {{
{dto_fields}
}}

// Active Record Implementation
nucleus_std::impl_model!({cap}, "{table}");

impl {cap} {{
    #[server]
    pub async fn create(payload: Create{cap}) -> Result<()> {{
        Self::create()
{insert_values}
            .execute()
            .await?;
        Ok(())
    }}

    #[server]
    pub async fn update(id: i64, payload: Update{cap}) -> Result<()> {{
        Self::query()
            .update()
{update_values}
            .r#where("id", id)
            .execute()
            .await?;
        Ok(())
    }}
}}
"#, 
    cap = cap,
    table = table,
    struct_fields = struct_fields,
    dto_fields = dto_fields,
    insert_values = insert_values,
    update_values = update_values
    )
}

fn generate_payments(subscription: bool) -> Result<()> {
    let view_dir = "src/views/payments";
    fs::create_dir_all(view_dir).into_diagnostic()?;
    fs::create_dir_all("src/logic").into_diagnostic()?;

    // 1. Checkout View (Stripe Elements)
    let checkout_content = r##"<n:view title="Secure Checkout">
    <script src="https://js.stripe.com/v3/"></script>
    <div class="checkout-container">
        <h1>Checkout</h1>
        <form id="payment-form">
            <div id="payment-element">
                <!-- Stripe Elements will inject here -->
            </div>
            <button id="submit" class="btn-primary">
                <span id="button-text">Pay Now</span>
                <span id="spinner" class="hidden">Processing...</span>
            </button>
            <div id="payment-message" class="hidden"></div>
        </form>
    </div>

    <script>
        const stripe = Stripe('pk_test_YOUR_KEY');
        let elements;

        initialize();

        async function initialize() {
            const { clientSecret } = await fetch("/api/billing/create-intent", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ items: [{ id: "prod_1", quantity: 1 }] }),
            }).then((r) => r.json());

            elements = stripe.elements({ clientSecret });
            const paymentElement = elements.create("payment");
            paymentElement.mount("#payment-element");
        }

        document.querySelector("#payment-form").addEventListener("submit", async (e) => {
            e.preventDefault();
            setLoading(true);

            const { error } = await stripe.confirmPayment({
                elements,
                confirmParams: {
                    return_url: window.location.origin + "/payments/success",
                },
            });

            if (error) {
                showMessage(error.message);
            }
            setLoading(false);
        });

        function showMessage(msg) {
            const msgDiv = document.querySelector("#payment-message");
            msgDiv.classList.remove("hidden");
            msgDiv.innerText = msg;
        }

        function setLoading(isLoading) {
            if (isLoading) {
                document.querySelector("#submit").disabled = true;
                document.querySelector("#spinner").classList.remove("hidden");
                document.querySelector("#button-text").classList.add("hidden");
            } else {
                document.querySelector("#submit").disabled = false;
                document.querySelector("#spinner").classList.add("hidden");
                document.querySelector("#button-text").classList.remove("hidden");
            }
        }
    </script>
    
    <style>
        .checkout-container { max-width: 500px; margin: 4rem auto; padding: 2rem; border: 1px solid #333; border-radius: 8px; background: #1a1a1a; }
        #payment-element { margin-bottom: 24px; }
        .hidden { display: none; }
        .btn-primary { wudth: 100%; padding: 12px; background: #5469d4; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 16px; font-weight: 600; }
        .btn-primary:disabled { opacity: 0.5; cursor: default; }
    </style>
</n:view>"##;
    fs::write(format!("{}/checkout.ncl", view_dir), checkout_content).into_diagnostic()?;
    build_step("  +", &format!("Created {}/checkout.ncl", view_dir));

    // 2. Crypto View (EVM)
    let crypto_content = r##"<n:view title="Connect Wallet">
    <div class="wallet-container">
        <h1>Connect Wallet</h1>
        <button id="connect-btn" class="btn-wallet">
            <img src="https://upload.wikimedia.org/wikipedia/commons/3/36/MetaMask_Fox.svg" width="24" />
            Connect MetaMask
        </button>
        <div id="account-display" class="hidden">
            <p>Connected: <span id="address"></span></p>
            <button id="sign-btn" class="btn-sign">Sign In With Ethereum</button>
        </div>
    </div>

    <script>
        const connectBtn = document.getElementById('connect-btn');
        const signBtn = document.getElementById('sign-btn');
        let account;

        if (typeof window.ethereum !== 'undefined') {
            connectBtn.addEventListener('click', async () => {
                const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
                account = accounts[0];
                document.getElementById('address').innerText = account;
                document.getElementById('account-display').classList.remove('hidden');
                connectBtn.classList.add('hidden');
            });

            signBtn.addEventListener('click', async () => {
                const nonce = Date.now().toString(); // In prod, fetch from server
                const msg = `Sign in to Nucleus\nNonce: ${nonce}`;
                const sig = await window.ethereum.request({
                    method: 'personal_sign',
                    params: [msg, account],
                });
                
                // Verify on backend
                const res = await fetch('/api/billing/verify-signature', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ address: account, signature: sig, message: msg })
                });
                
                if (res.ok) alert('Successfully authenticated via Blockchain!');
                else alert('Signature verification failed.');
            });
        } else {
            connectBtn.innerText = "MetaMask Not Installed";
            connectBtn.disabled = true;
        }
    </script>

    <style>
        .wallet-container { max-width: 400px; margin: 4rem auto; text-align: center; }
        .btn-wallet { display: flex; align-items: center; justify-content: center; gap: 12px; width: 100%; padding: 16px; background: #fff; color: #000; border: none; border-radius: 32px; font-weight: bold; cursor: pointer; transition: transform 0.1s; }
        .btn-wallet:hover { transform: scale(1.02); }
        .btn-sign { margin-top: 1rem; padding: 12px 24px; background: #4caf50; color: white; border: none; border-radius: 4px; cursor: pointer; }
    </style>
</n:view>"##;
    fs::write(format!("{}/crypto.ncl", view_dir), crypto_content).into_diagnostic()?;
    build_step("  +", &format!("Created {}/crypto.ncl", view_dir));

    // 3. Pricing View (Optional)
    if subscription {
        let pricing_content = r##"<n:view title="Pricing">
    <div class="pricing-grid">
        <div class="plan">
            <h2>Starter</h2>
            <div class="price">$10<span>/mo</span></div>
            <ul>
                <li>Basic Features</li>
                <li>1 User</li>
            </ul>
            <button onclick="subscribe('price_starter_id')">Choose Starter</button>
        </div>
        <div class="plan featured">
            <h2>Pro</h2>
            <div class="price">$30<span>/mo</span></div>
            <ul>
                <li>Pro Features</li>
                <li>5 Users</li>
            </ul>
            <button onclick="subscribe('price_pro_id')">Choose Pro</button>
        </div>
    </div>

    <script>
        async function subscribe(priceId) {
            const res = await fetch('/api/billing/create-subscription', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ priceId })
            });
            const data = await res.json();
            // Redirect to Stripe Checkout
            window.location.href = data.url; 
        }
    </script>
    
    <style>
        .pricing-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 2rem; max-width: 900px; margin: 4rem auto; }
        .plan { background: #222; padding: 2rem; border-radius: 8px; text-align: center; border: 1px solid #444; }
        .plan.featured { border-color: #4facfe; transform: scale(1.05); }
        .price { font-size: 3rem; font-weight: bold; margin: 1rem 0; }
        .price span { font-size: 1rem; color: #888; }
        ul { list-style: none; padding: 0; margin-bottom: 2rem; }
        li { margin: 0.5rem 0; color: #ccc; }
        button { background: #4facfe; color: white; border: none; padding: 1rem 2rem; border-radius: 4px; width: 100%; cursor: pointer; font-weight: bold; }
    </style>
</n:view>"##;
        fs::write(format!("{}/pricing.ncl", view_dir), pricing_content).into_diagnostic()?;
        build_step("  +", &format!("Created {}/pricing.ncl", view_dir));
    }

    // 4. Backend Logic
    let backend_content = r#"use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use nucleus_std::payments::Stripe;
use nucleus_std::chain::Chain;
use nucleus_std::errors::Result;

pub fn routes() -> Router {
    Router::new()
        .route("/api/billing/create-intent", post(create_intent))
        .route("/api/billing/verify-signature", post(verify_signature))
        .route("/api/billing/create-subscription", post(create_subscription))
}

#[derive(Deserialize)]
struct CreateIntentPayload {
    items: Vec<Value>,
}

async fn create_intent(_payload: Json<CreateIntentPayload>) -> Result<Json<Value>> {
    // In a real app, calculate amount from items on server-side to prevent tampering
    // For demo, we assume a static amount
    
    let config = &nucleus_std::config::GLOBAL_CONFIG.server;
    let base = format!("http://{}:{}/payments", if config.host == "0.0.0.0" { "localhost" } else { &config.host }, config.port);
    
    let url = Stripe::checkout(
        &format!("{}/success", base),
        &format!("{}/cancel", base),
        vec![], // Line items
        "payment",
        None,
        None
    ).await?; // NucleusError auto-conversion

    Ok(Json(json!({ "clientSecret": "mock_secret_for_demo", "url": url })))
}

#[derive(Deserialize)]
struct VerifySigPayload {
    address: String,
    signature: String,
    message: String,
}

async fn verify_signature(Json(payload): Json<VerifySigPayload>) -> Result<Json<Value>> {
    let valid = Chain::verify_signature(&payload.message, &payload.signature, &payload.address)?;
    
    if valid {
        Ok(Json(json!({ "status": "success" })))
    } else {
        // Return a business error that maps to 400
        Err(nucleus_std::errors::NucleusError::ValidationError("Invalid Signature".into()))
    }
}

#[derive(Deserialize)]
struct CreateSubPayload {
    priceId: String,
}

async fn create_subscription(Json(payload): Json<CreateSubPayload>) -> Result<Json<Value>> {
    // 1. Get or Create Customer (Mocking ID here, normally get from Auth)
    let customer_id = "cus_mock_123"; 
    
    // For Checkout Flow:
    let config = &nucleus_std::config::GLOBAL_CONFIG.server;
    let base = format!("http://{}:{}", if config.host == "0.0.0.0" { "localhost" } else { &config.host }, config.port);

    let url = Stripe::checkout(
        &format!("{}/success", base),
        &format!("{}/cancel", base),
        vec![], // Item with priceId
        "subscription", 
        None, 
        None
    ).await?;

    Ok(Json(json!({ "url": url })))
}
"#;
    
    fs::write("src/logic/billing.rs", backend_content).into_diagnostic()?;
    build_step("  +", "Created src/logic/billing.rs");

    // Register logic
    update_lib_rs("billing")?;

    Ok(())
}

fn parse_field(f: &str) -> (String, String, String) {
    let parts: Vec<&str> = f.split(':').collect();
    let fname = parts[0].to_string();
    let (rust_type, sql_type) = match parts.get(1).unwrap_or(&"string") {
        &"string" | &"text" => ("String", "TEXT NOT NULL"),
        &"int" | &"i32" => ("i32", "INTEGER NOT NULL"),
        &"bigint" | &"i64" => ("i64", "BIGINT NOT NULL"),
        &"bool" | &"boolean" => ("bool", "BOOLEAN NOT NULL"),
        &"float" => ("f64", "REAL"),
        _ => ("String", "TEXT"),
    };
    (fname, rust_type.to_string(), sql_type.to_string())
}

fn generate_migration(name: &str, fields: &[String]) -> Result<()> {
    // 1. Parse SQL
    let sql_fields = fields.iter().map(|f| {
        let (fname, _, sql_type) = parse_field(f);
        format!("    {} {}", fname, sql_type)
    }).collect::<Vec<_>>().join(",\n");

    let table = format!("{}s", name.to_lowercase());
    
    let content = format!(r#"CREATE TABLE IF NOT EXISTS {table} (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
{fields}
);
"#, table = table, fields = sql_fields);

    // 2. Filename
    let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S");
    let filename = format!("migrations/{}_create_{}.sql", timestamp, table);
    
    fs::create_dir_all("migrations").into_diagnostic()?;
    fs::write(&filename, content).into_diagnostic()?;
    build_step("  +", &format!("Created {}", filename));
    Ok(())
}

fn generate_controllers(name: &str, fields: &[String]) -> Result<()> {
    let lower = name.to_lowercase();
    let cap = capitalize(name);
    let view_dir = format!("src/views/{}", lower);
    fs::create_dir_all(&view_dir).into_diagnostic()?;

    // 1. Index View
    let index_content = format!(r#"<n:model list="crate::logic::{modname}::{cap}::query().all().await.unwrap_or_default()">
<div class="container">
    <div class="header">
        <h1>All {cap}s</h1>
        <a href="/{lower}s/new" class="btn">New {cap}</a>
    </div>
    <ul>
        <n:for item="{lower}" in="list">
            <li>
                <a href="/{lower}s/{{ {lower}.id }}">
                    <strong>#{{ {lower}.id }}</strong>
                    <!-- Add first field preview here -->
                </a>
            </li>
        </n:for>
    </ul>
</div>
</n:model>"#, modname = lower, cap = cap, lower = lower);
    
    fs::write(format!("{}/index.ncl", view_dir), index_content).into_diagnostic()?;
    build_step("  +", &format!("Created {}/index.ncl", view_dir));

    // 2. Show View ([id].ncl)
    let display_fields = fields.iter().map(|f| {
        let (fname, _, _) = parse_field(f);
        format!("<p><strong>{}:</strong> {{ item.{} }}</p>", capitalize(&fname), fname)
    }).collect::<Vec<_>>().join("\n    ");

    let show_content = format!(r#"<n:model item="crate::logic::{modname}::{cap}::find(params.id.parse().unwrap_or_default()).await.unwrap_or(None)">
<div class="container">
    <h1>{cap} #{{ item.id }}</h1>
    <div class="card">
    {display_fields}
    </div>
    <div class="actions">
        <a href="/{lower}s/{{ item.id }}/edit" class="btn">Edit</a>
        <a href="/{lower}s" class="btn secondary">Back</a>
    </div>
</div>
</n:model>"#, modname = lower, cap = cap, lower = lower, display_fields = display_fields);

    fs::write(format!("{}/[id].ncl", view_dir), show_content).into_diagnostic()?;
    build_step("  +", &format!("Created {}/[id].ncl", view_dir));

    // 3. New View (new.ncl)
    let form_fields = fields.iter().map(|f| {
        let (fname, _, _) = parse_field(f);
        format!(r#"<div class="field">
        <label for="{name}">{label}</label>
        <input type="text" name="{name}" id="{name}" required />
    </div>"#, name = fname, label = capitalize(&fname))
    }).collect::<Vec<_>>().join("\n    ");

    let new_content = format!(r#"<n:view title="New {cap}">
<div class="container">
    <h1>New {cap}</h1>
    <form method="POST">
    {form_fields}
        <button type="submit" class="btn">Create {cap}</button>
    </form>
    <!-- Action Mapping: Implicitly calls crate::logic::{modname}::{cap}::create if mapped via ncc? 
         For now we rely on manual binding or convention. -->
</div>
</n:view>"#, cap = cap, modname = lower, form_fields = form_fields);

    fs::write(format!("{}/new.ncl", view_dir), new_content).into_diagnostic()?;
    build_step("  +", &format!("Created {}/new.ncl", view_dir));

    // 4. Edit View ([id]/edit.ncl)
    let edit_form_fields = fields.iter().map(|f| {
        let (fname, _, _) = parse_field(f);
        format!(r#"<div class="field">
        <label for="{name}">{label}</label>
        <input type="text" name="{name}" id="{name}" value="{{ item.{name} }}" required />
    </div>"#, name = fname, label = capitalize(&fname))
    }).collect::<Vec<_>>().join("\n    ");

    let edit_content = format!(r#"<n:model item="crate::logic::{modname}::{cap}::find(params.id.parse().unwrap_or_default()).await.unwrap_or(None)">
<div class="container">
    <h1>Edit {cap}</h1>
    <form method="POST">
    {form_fields}
        <button type="submit" class="btn">Update {cap}</button>
    </form>
</div>
</n:model>"#, modname = lower, cap = cap, form_fields = edit_form_fields);

    fs::create_dir_all(format!("{}/[id]", view_dir)).into_diagnostic()?;
    fs::write(format!("{}/[id]/edit.ncl", view_dir), edit_content).into_diagnostic()?;
    build_step("  +", &format!("Created {}/[id]/edit.ncl", view_dir));

    Ok(())
}

fn update_lib_rs(name: &str) -> Result<()> {
    let lib_path = "src/lib.rs";
    if !Path::new(lib_path).exists() {
        return Ok(()); // Skip if no lib.rs
    }

    let content = fs::read_to_string(lib_path).into_diagnostic()?;
    let mod_line = format!("pub mod {};", name.to_lowercase());
    
    // Naive check
    if !content.contains(&mod_line) {
        // Find "pub mod logic {" or similar and insert
        // For now, just append to src/logic/mod.rs if it exists, roughly.
        // Actually, Nucleus structure usually has `pub mod logic { pub mod x; }`.
        
        // Let's assume standard structure: src/lib.rs has `pub mod logic;` and `src/logic/mod.rs` has modules
        
        // Check src/logic/mod.rs
        let logic_mod_path = "src/logic/mod.rs";
        if Path::new(logic_mod_path).exists() {
             let mut logic_content = fs::read_to_string(logic_mod_path).into_diagnostic()?;
             if !logic_content.contains(&format!("pub mod {};", name.to_lowercase())) {
                 logic_content.push_str(&format!("\npub mod {};\n", name.to_lowercase()));
                 fs::write(logic_mod_path, logic_content).into_diagnostic()?;
                 build_step("  +", &format!("Updated {}", logic_mod_path));
             }
        } else {
             // Maybe create it?
             println!("{}⚠️  Could not automatically register module in logic/mod.rs. Please add 'pub mod {};' manually.{}", colors::YELLOW, name.to_lowercase(), colors::RESET);
        }
    }
    
    Ok(())
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_field() {
        assert_eq!(parse_field("title:string"), ("title".to_string(), "String".to_string(), "TEXT NOT NULL".to_string()));
        assert_eq!(parse_field("count:int"), ("count".to_string(), "i32".to_string(), "INTEGER NOT NULL".to_string()));
        assert_eq!(parse_field("is_active:bool"), ("is_active".to_string(), "bool".to_string(), "BOOLEAN NOT NULL".to_string()));
        assert_eq!(parse_field("price:float"), ("price".to_string(), "f64".to_string(), "REAL".to_string()));
        assert_eq!(parse_field("desc"), ("desc".to_string(), "String".to_string(), "TEXT NOT NULL".to_string()));
        assert_eq!(parse_field("custom:unknown"), ("custom".to_string(), "String".to_string(), "TEXT".to_string()));
    }

    #[test]
    fn test_generate_model_content() {
        let fields = vec!["title:string".to_string(), "views:int".to_string()];
        let content = generate_model_content("Post", &fields);
        
        assert!(content.contains("pub struct Post"));
        assert!(content.contains("pub title: String"));
        assert!(content.contains("pub views: i32"));
        assert!(content.contains("pub struct CreatePost"));
        assert!(content.contains("pub struct UpdatePost"));
        assert!(content.contains(".value(\"title\", payload.title)"));
        assert!(content.contains(".value(\"views\", payload.views)"));
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("user"), "User");
        assert_eq!(capitalize("post"), "Post");
        assert_eq!(capitalize("User"), "User"); // Already capitalized
        assert_eq!(capitalize(""), ""); // Empty string
        assert_eq!(capitalize("a"), "A"); // Single character
    }

    #[test]
    fn test_parse_field_all_types() {
        // Test all supported type mappings
        assert_eq!(parse_field("name:text").1, "String");
        assert_eq!(parse_field("age:i32").1, "i32");
        assert_eq!(parse_field("id:bigint").1, "i64");
        assert_eq!(parse_field("id:i64").1, "i64");
        assert_eq!(parse_field("active:boolean").1, "bool");
    }

    #[test]
    fn test_generate_model_content_table_name() {
        let fields = vec!["name:string".to_string()];
        let content = generate_model_content("User", &fields);
        
        // Table name should be pluralized lowercase
        assert!(content.contains("\"users\""));
    }

    #[test]
    fn test_generate_model_content_empty_fields() {
        let fields: Vec<String> = vec![];
        let content = generate_model_content("Empty", &fields);
        
        // Should still generate valid struct with ID
        assert!(content.contains("pub struct Empty"));
        assert!(content.contains("pub id: i64"));
    }

    #[test]
    fn test_generate_model_content_multiple_fields() {
        let fields = vec![
            "title:string".to_string(),
            "content:text".to_string(),
            "views:int".to_string(),
            "rating:float".to_string(),
            "published:bool".to_string(),
        ];
        let content = generate_model_content("Article", &fields);
        
        assert!(content.contains("pub title: String"));
        assert!(content.contains("pub content: String"));
        assert!(content.contains("pub views: i32"));
        assert!(content.contains("pub rating: f64"));
        assert!(content.contains("pub published: bool"));
    }
}

