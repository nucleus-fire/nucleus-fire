use crate::ast::{Element, Node};

pub fn generate_rust(nodes: &[Node]) -> String {
    // Debug logging removed for production
    // std::fs::write("/tmp/debug_ast.txt", format!("{:#?}", nodes)).ok();
    let mut code = String::new();
    
    // Header
    code.push_str("use axum::{Router, routing::get, extract::{Form, Query}};\n");
    code.push_str("use serde::{Deserialize, Serialize};\n\n");

    // Generate Router
    code.push_str(&generate_router(nodes));

    // Generate Handlers & Types
    for node in nodes {
        match node {
             Node::Element(el) if el.tag_name == "n:view" => {
                 code.push_str(&generate_view_handler_fn(el, "root_handler"));
                 // Check for action and generate handler
                 if el.children.iter().any(|c| matches!(c, Node::Action(_))) {
                     code.push_str(&generate_action_handler_fn(el, "action_handler"));
                 }
             },
             Node::Model(model) => {
                 code.push_str(&generate_model(model));
             },
             _ => {}
        }
    }
    
    code
}

pub fn generate_model(model: &crate::ast::Model) -> String {
    let mut code = String::new();
    
    // Always add allow(dead_code) to suppress warnings for unused models
    code.push_str("#[allow(dead_code)]\n");
    
    // 1. Attributes (Implicit Defaults if empty)
    if model.attributes.is_empty() {
        code.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    } else {
        for attr in &model.attributes {
            code.push_str(attr);
            code.push('\n');
        }
    }
    
    // 2. Struct Definition
    code.push_str(&format!("pub struct {} {{\n", model.name));
    for (name, ty) in &model.fields {
        code.push_str(&format!("    pub {}: {},\n", name, ty));
    }
    code.push_str("}\n\n");
    
    // 3. Impl Block (Methods)
    if !model.methods.is_empty() {
        code.push_str(&format!("impl {} {{\n", model.name));
        for method in &model.methods {
            code.push_str(method);
            code.push('\n');
        }
        code.push_str("}\n\n");
    }
    
    code
}

fn generate_router(nodes: &[Node]) -> String {
    let mut routes = String::new();
    for node in nodes {
        if let Node::Element(el) = node {
            if el.tag_name == "n:view" {
                // Simplified routing: /view_name -> handler
                let path = "/"; 
                routes.push_str(&format!(".route(\"{}\", get(root_handler))", path));
                
                // Add POST handler if Action exists
                if el.children.iter().any(|c| matches!(c, Node::Action(_))) {
                    routes.push_str(&format!(".route(\"{}\", axum::routing::post(action_handler))", path));
                }
                routes.push('\n');
            }
        }
    }

    format!(
        "pub fn app() -> Router {{\n    Router::new()\n        {}\n}}\n\n",
        routes
    )
}

pub fn generate_nodes_handler_body(nodes: &[Node], fn_name: &str) -> String {
    let mut func_body = String::from("let mut html_body = String::new();\n");
    for node in nodes {
        render_node_to_body(node, &mut func_body);
    }
    format!(
        "#[allow(non_snake_case, unreachable_code, unused_variables)]\nasync fn {}(headers: axum::http::HeaderMap, Query(params): Query<std::collections::HashMap<String, String>>) -> impl axum::response::IntoResponse {{\n    {}\n    axum::response::Html(html_body).into_response()\n}}\n\n",
        fn_name, func_body
    )
}

pub fn find_action_recursive(nodes: &[Node]) -> Option<String> {
    for node in nodes {
        match node {
             Node::Action(code) => {
                 return Some(code.clone());
             },
             Node::Element(el) => {
                 if let Some(code) = find_action_recursive(&el.children) {
                     return Some(code);
                 }
             },
             _ => {}
        }
    }
    None
}

pub fn find_loaders_recursive(nodes: &[Node]) -> String {
    let mut code = String::new();
    for node in nodes {
        match node {
             Node::Loader(c) => {
                 code.push_str(c);
                 code.push('\n');
             },
             Node::Element(el) => {
                 code.push_str(&find_loaders_recursive(&el.children));
             },
             _ => {}
        }
    }
    code
}

pub fn render_html(nodes: &[Node]) -> String {
    let mut body = String::new();
    
    // Find root view
    if let Some(Node::Element(el)) = nodes.iter().find(|n| matches!(n, Node::Element(e) if e.tag_name == "n:view")) {
        // Check if the view has an n:layout wrapper
        if let Some(layout_element) = el.children.iter().find_map(|child| {
            if let Node::Element(e) = child {
                if e.tag_name == "n:layout" {
                    return Some(e);
                }
            }
            None
        }) {
            // Get layout name from attribute
            let layout_name = layout_element.attributes.iter()
                .find(|(k, _)| k == "name")
                .map(|(_, v)| v.as_str())
                .unwrap_or("layout");
            
            // Load layout file
            let layout_path = format!("src/views/{}.ncl", layout_name);
            if let Ok(layout_content) = std::fs::read_to_string(&layout_path) {
                if let Ok(layout_nodes) = crate::parser::parse_code(&layout_content) {
                    // Render layout, replacing n:slot with view content
                    render_layout_with_content(&layout_nodes, &layout_element.children, &mut body);
                    // Inject Nucleus Router for client-side navigation
                    body.push_str("<script src=\"/static/js/router.js\" defer></script>");
                    return body;
                }
            }
            
            // Fallback: if layout file not found, render view content directly
            for child in &layout_element.children {
                static_render_node(child, &mut body);
            }
            // Inject Nucleus Router for client-side navigation
            body.push_str("<script src=\"/static/js/router.js\" defer></script>");
            return body;
        }
        
        // No layout - use default HTML shell
        let title = el.attributes.iter()
            .find(|(k, _)| k == "title")
            .map(|(_, v)| v.as_str())
            .unwrap_or("Nucleus App");
            
        let mut description = "Built with Nucleus";
        let mut extra_head = String::new();
            
        if let Some(desc) = el.attributes.iter().find(|(k, _)| k == "description").map(|(_, v)| v.as_str()) {
            description = desc;
        }

        // Pass 1: Collect meta tags and links for head
        for child in &el.children {
            if let Node::Element(e) = child {
                if e.tag_name == "meta" || e.tag_name == "link" {
                    let mut meta_tag = String::new();
                    static_render_element(e, &mut meta_tag);
                    extra_head.push_str(&meta_tag);
                }
            }
        }
            
        // Auto-SEO: OpenGraph & Twitter Cards
        extra_head.push_str(&format!("<meta property=\"og:title\" content=\"{}\">", title));
        extra_head.push_str(&format!("<meta property=\"og:description\" content=\"{}\">", description));
        extra_head.push_str("<meta property=\"og:type\" content=\"website\">");
        extra_head.push_str("<meta name=\"twitter:card\" content=\"summary_large_image\">");
            
        body.push_str(&format!("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>{}</title><meta name=\"description\" content=\"{}\">{}</head><body>", title, description, extra_head));
        
        // Pass 2: Render body (skip meta/link tags we already moved to head)
        for child in &el.children {
            if let Node::Element(child_el) = child {
                if child_el.tag_name == "n:form" {
                    body.push_str(&generate_form_html(child_el));
                    continue;
                }
                if child_el.tag_name == "meta" || child_el.tag_name == "link" {
                    continue;
                }
            }
            static_render_node(child, &mut body);
        }
        // Inject Nucleus Router for client-side navigation
        body.push_str("<script src=\"/static/js/router.js\" defer></script>");
        body.push_str("</body></html>");
    } else {
        // Just render generic nodes (for layout files, partials, etc.)
        for node in nodes {
             static_render_node(node, &mut body);
        }
    }
    
    body
}

/// Render layout nodes, replacing n:slot with the provided content
fn render_layout_with_content(layout_nodes: &[Node], content: &[Node], body: &mut String) {
    for node in layout_nodes {
        render_layout_node(node, content, body);
    }
}

fn render_layout_node(node: &Node, content: &[Node], body: &mut String) {
    match node {
        // Handle Slot - this is where we inject the view content
        Node::Slot { .. } => {
            for child in content {
                static_render_node(child, body);
            }
        },
        Node::Element(el) => {
            // Check for n:slot as element (fallback, shouldn't happen with proper parsing)
            if el.tag_name == "n:slot" {
                for child in content {
                    static_render_node(child, body);
                }
                return;
            }
            
            // Skip n:view wrapper in layouts
            if el.tag_name == "n:view" {
                for child in &el.children {
                    render_layout_node(child, content, body);
                }
                return;
            }
            
            // Regular element - render with layout context
            body.push_str(&format!("<{}", el.tag_name));
            for (k, v) in &el.attributes {
                body.push_str(&format!(" {}=\"{}\"", k, v));
            }
            body.push('>');
            
            for child in &el.children {
                render_layout_node(child, content, body);
            }
            
            body.push_str(&format!("</{}>", el.tag_name));
        },
        Node::Text(t) => body.push_str(t),
        Node::Style(css) => {
            body.push_str("<style>");
            body.push_str(css);
            body.push_str("</style>");
        },
        Node::Script { content: script_content, attributes } => {
            let lang = attributes.iter().find(|(k, _)| k == "lang").map(|(_, v)| v.as_str());
            if lang != Some("rust") {
                body.push_str("<script");
                for (k, v) in attributes {
                    if k != "lang" {
                        body.push_str(&format!(" {}=\"{}\"", k, v));
                    }
                }
                body.push('>');
                body.push_str(script_content);
                body.push_str("</script>");
            }
        },
        _ => {}
    }
}



pub fn generate_view_handler_fn(el: &Element, fn_name: &str) -> String {
    // 1. Extract Scripts (Code Injection)
    let mut injected_code = String::new();
    
    // We build the function body as a String of Rust statements
    let mut func_body = String::from("let mut html_body = String::new();\n");

    // SEO
    let title = el.attributes.iter()
        .find(|(k, _)| k == "title")
        .map(|(_, v)| v.as_str())
        .unwrap_or("Nucleus App");

    // Manual push for DOCTYPE and HTML shell
    func_body.push_str(&format!("html_body.push_str(\"<!DOCTYPE html><html lang=\\\"en\\\"><head><meta charset=\\\"UTF-8\\\"><meta name=\\\"viewport\\\" content=\\\"width=device-width, initial-scale=1.0\\\"><title>{}</title><meta name=\\\"description\\\" content=\\\"Built with Nucleus\\\"></head><body>\");\n", title));

    // 0. Inject Loader Code (Pre-render)
    // 0. Inject Loader Code (Pre-render) - Recursively find loaders (e.g. in n:layout)
    injected_code.push_str(&find_loaders_recursive(&el.children));

    for child in &el.children {
        match child {
             Node::Script { content, attributes } => {
                 let is_rust = attributes.iter().find(|(k, _)| k == "lang").map(|(_, v)| v == "rust").unwrap_or(true);
                 if is_rust {
                     injected_code.push_str(content);
                     injected_code.push('\n');
                 }
             },
             Node::Loader(_) => {}, // Handled above
             Node::Action(_) => {}, // Handled separately (POST handler)
             _ => render_node_to_body(child, &mut func_body) // Appends Rust statements to func_body
        }
    }
    // Inject Nucleus Router for client-side navigation (SPA-like behavior)
    func_body.push_str("html_body.push_str(\"<script src=\\\\\\\"/static/js/router.js\\\\\\\" defer></script>\");\\n");
    func_body.push_str("html_body.push_str(\"</body></html>\");\n");

    // Generate Form Struct if present
    let form_struct = if let Some(form) = find_form(el) {
        generate_form_struct(form)
    } else {
        "".to_string()
    };

    let protected = el.attributes.iter().any(|(k, v)| k == "protected" && v == "true");
    let guard_code = if protected {
        r#"
        let cookie_header = headers.get("cookie").and_then(|h| h.to_str().ok()).unwrap_or("");
        if !cookie_header.contains("session=") {
            return axum::response::Redirect::to("/login").into_response();
        }
        "#
    } else {
        ""
    };

    format!(
        "{}#[allow(non_snake_case, unreachable_code, unused_variables)]\nasync fn {}(headers: axum::http::HeaderMap, Query(params): Query<std::collections::HashMap<String, String>>) -> impl axum::response::IntoResponse {{\n{}\n    {}\n    {}\n    axum::response::Html(html_body).into_response()\n}}\n\n",
        form_struct, fn_name, guard_code, injected_code, func_body
    )
}

pub fn generate_action_handler_fn(el: &Element, fn_name: &str) -> String {
    let mut action_code = String::new();
    let mut has_action = false;
    
    // Find Action Node
    for child in &el.children {
        if let Node::Action(code) = child {
             action_code.push_str(code);
             action_code.push('\n');
             has_action = true;
        }
    }
    
    if !has_action {
        return String::new();
    }

    // Default return if user code doesn't return (though user code usually should redirect)
    // We append a basic OK just in case, but usually action code ends with return.
    format!(
        "#[allow(non_snake_case, unreachable_code, unused_variables)]\nasync fn {}(headers: axum::http::HeaderMap, Form(params): Form<std::collections::HashMap<String, String>>) -> impl axum::response::IntoResponse {{\n    {}\n    axum::response::Html(\"Action Completed\").into_response()\n}}\n\n",
        fn_name, action_code
    )
}

// === STATIC HTML RENDERING (for interpreter mode / hot-swap) ===
// These functions output actual HTML, NOT Rust code generation

fn static_render_element(el: &Element, body: &mut String) {
    // Handle n:layout - find and insert the child view's content at <n:outlet>
    if el.tag_name == "n:layout" {
        // Just render children for now (layout wrapping is handled at parse level)
        for child in &el.children {
            static_render_node(child, body);
        }
        return;
    }

    if el.tag_name == "n:image" {
        let src = el.attributes.iter().find(|(k,_)| k == "src").map(|(_,v)| v.as_str()).unwrap_or("");
        let alt = el.attributes.iter().find(|(k,_)| k == "alt").map(|(_,v)| v.as_str()).unwrap_or("");
        body.push_str(&format!("<img src=\"{}\" alt=\"{}\" loading=\"lazy\" decoding=\"async\" />", src, alt));
        return;
    }
    
    if el.tag_name == "n:link" {
        let href = el.attributes.iter().find(|(k,_)| k == "href").map(|(_,v)| v.as_str()).unwrap_or("#");
        body.push_str(&format!("<a href=\"{}\" data-nucleus-link=\"true\">", href));
        for child in &el.children {
            static_render_node(child, body);
        }
        body.push_str("</a>");
        return;
    }

    // Standard Element
    body.push_str(&format!("<{}", el.tag_name));
    
    // Attributes
    for (k, v) in &el.attributes {
        body.push_str(&format!(" {}=\"{}\"", k, v));
    }
    
    body.push('>');
    
    for child in &el.children {
        static_render_node(child, body);
    }
    
    body.push_str(&format!("</{}>", el.tag_name));
}

fn static_render_node(node: &Node, body: &mut String) {
    match node {
        Node::Element(e) => static_render_element(e, body),
        Node::Text(t) => body.push_str(t),
        Node::Style(css) => {
            body.push_str("<style>");
            body.push_str(css);
            body.push_str("</style>");
        },
        Node::Script { content, attributes } => {
            // Only render client-side scripts (not Rust server code)
            let lang = attributes.iter().find(|(k, _)| k == "lang").map(|(_, v)| v.as_str());
            if lang != Some("rust") {
                body.push_str("<script");
                for (k, v) in attributes {
                    if k != "lang" {
                        body.push_str(&format!(" {}=\"{}\"", k, v));
                    }
                }
                body.push('>');
                body.push_str(content);
                body.push_str("</script>");
            }
        },
        Node::Include { path, attributes } => {
            match std::fs::read_to_string(path) {
                Ok(mut content) => {
                    for (key, val) in attributes {
                        content = content.replace(&format!("{{{{ {} }}}}", key), val);
                        content = content.replace(&format!("{{{{{}}}}}", key), val);
                    }
                    match crate::parser::parse_root(&content) {
                        Ok((_, nodes)) => {
                            for child in nodes {
                                static_render_node(&child, body);
                            }
                        },
                        Err(_) => {
                            body.push_str(&format!("<!-- Error parsing included file: {} -->", path));
                        }
                    }
                },
                Err(_) => {
                    body.push_str(&format!("<!-- Error reading included file: {} -->", path));
                }
            }
        },
        Node::Outlet => {
            body.push_str("<!-- Outlet -->");
        },
        // Skip server-only nodes in static render
        Node::Loader(_) | Node::Action(_) | Node::For { .. } | Node::If { .. } | Node::Interpolation(_) => {},
        _ => {}
    }
}

// === RUST CODE GENERATION (for AOT compilation) ===
// These functions output Rust code that builds HTML at runtime

fn generate_element_html(el: &Element, body: &mut String) {
    if el.tag_name == "n:image" {
        let src = el.attributes.iter().find(|(k,_)| k == "src").map(|(_,v)| v.as_str()).unwrap_or("");
        let alt = el.attributes.iter().find(|(k,_)| k == "alt").map(|(_,v)| v.as_str()).unwrap_or("");
        body.push_str(&format!("html_body.push_str(\"<img src=\\\"{}\\\" alt=\\\"{}\\\" loading=\\\"lazy\\\" decoding=\\\"async\\\" />\");\n", src, alt));
        return;
    }
    
    if el.tag_name == "n:link" {
        let href = el.attributes.iter().find(|(k,_)| k == "href").map(|(_,v)| v.as_str()).unwrap_or("#");
        body.push_str(&format!("html_body.push_str(\"<a href=\\\"{}\\\" data-nucleus-link=\\\"true\\\">\");\n", href));
        for child in &el.children {
             render_node_to_body(child, body);
        }
        body.push_str("html_body.push_str(\"</a>\");\n");
        return;
    }

    // Standard Element
    body.push_str(&format!("html_body.push_str(\"<{}\");\n", el.tag_name));
    
    // Attributes
    for (k, v) in &el.attributes {
        let mut fmt_str = String::new();
        let mut args = Vec::new();
        
        let mut remaining = v.as_str();
        while let Some(start) = remaining.find("{{") {
            // Escape any quotes in the text part for the format string
            fmt_str.push_str(&remaining[..start].replace("\"", "\\\"")); 
            
            if let Some(end) = remaining[start..].find("}}") {
                let expr = &remaining[start+2 .. start+end];
                args.push(expr.trim().to_string());
                fmt_str.push_str("{}");
                remaining = &remaining[start+end+2..];
            } else {
                // Unclosed or broken
                fmt_str.push_str("{{");
                remaining = &remaining[start+2..];
            }
        }
        fmt_str.push_str(&remaining.replace("\"", "\\\""));
        
        if args.is_empty() {
             body.push_str(&format!("html_body.push_str(\" {}=\\\"{}\\\"\");\n", k, fmt_str));
        } else {
             let args_str = args.join(", ");
             body.push_str(&format!("html_body.push_str(&format!(\" {}=\\\"{}\\\"\", {}));\n", k, fmt_str, args_str));
        }
    }
    
    body.push_str("html_body.push_str(\">\");\n");
    
    for child in &el.children {
        render_node_to_body(child, body);
    }
    
    body.push_str(&format!("html_body.push_str(\"</{}>\");\n", el.tag_name));
}

fn render_node_to_body(node: &Node, body: &mut String) {
    match node {
         Node::Element(e) => generate_element_html(e, body),
         Node::Interpolation(expr) => {
             // Generate: html_body.push_str(&(expr).to_string());
             body.push_str(&format!("html_body.push_str(&({}).to_string());\n", expr));
         },
         Node::Text(t) => {
             // Use Debug formatting to generate a valid Rust string literal with correct escaping
             body.push_str(&format!("html_body.push_str({:?});\n", t));
         },
         Node::Style(css) => {
             let safe_css = css.replace("\"", "\\\"");
             body.push_str(&format!("html_body.push_str(\"<style>{}</style>\");\n", safe_css));
         },
         Node::For { variable, iterable, children } => {
             // Generate: for var in iterable { ... }
             body.push_str(&format!("for {} in {} {{\n", variable, iterable));
             for child in children {
                 render_node_to_body(child, body);
             }
             body.push_str("}\n");
         },
         Node::If { condition, children } => {
             // Generate: if condition { ... }
             body.push_str(&format!("if {} {{\n", condition));
             for child in children {
                 render_node_to_body(child, body);
             }
             body.push_str("}\n");
         },
         Node::Model(model) => {
             body.push_str(&generate_model(model));
         },
          Node::Include { path, attributes } => {
             // ... existing include logic ...
             // (omitted for brevity in replacement, essentially keeping logical flow)
             // Actually, I will just append the Outlet match arm after Include
             match std::fs::read_to_string(path) {
                 Ok(mut content) => {
                     for (key, val) in attributes {
                         content = content.replace(&format!("{{{{ {} }}}}", key), val);
                         content = content.replace(&format!("{{{{{}}}}}", key), val);
                     }
                     match crate::parser::parse_root(&content) {
                         Ok((_, nodes)) => {
                             body.push_str(&format!("html_body.push_str(\"<!-- Begin Include: {} -->\");\n", path));
                             for child in nodes {
                                 render_node_to_body(&child, body);
                             }
                             body.push_str(&format!("html_body.push_str(\"<!-- End Include: {} -->\");\n", path));
                         },
                         Err(_e) => {
                              body.push_str(&format!("html_body.push_str(\"<!-- Error parsing included file: {} -->\");\n", path));
                         }
                     }
                 },
                 Err(_) => {
                      body.push_str(&format!("html_body.push_str(\"<!-- Error reading included file: {} -->\");\n", path));
                 }
             }
         },
         Node::Outlet => {
             body.push_str("html_body.push_str(\"<!-- Outlet -->\");\n");
         },
         Node::Island { path: _, directive: _, attributes: _ } => {
             // ... (Keep existing Island logic) ...
             // Re-instating checks to ensure I don't overwrite logic.
             // Actually, I just need to add the new match arms.
             // I will replace the end of the function.
        }, // End Island
        Node::Loader(_) => {},
        Node::Action(_) => {},
        _ => {}
    }
}

fn find_form(el: &Element) -> Option<&Element> {
    for child in &el.children {
        if let Node::Element(child_el) = child {
            if child_el.tag_name == "n:form" {
                return Some(child_el);
            }
        }
    }
    None
}

fn generate_form_html(_el: &Element) -> String {
    "<form>...</form>".to_string()
}

fn generate_form_struct(el: &Element) -> String {
    let mut fields = String::new();
    for child in &el.children {
        if let Node::Element(input) = child {
            if input.tag_name == "n:input" {
                if let Some(name) = input.attributes.iter().find(|(k, _)| k == "name") {
                    fields.push_str(&format!("    pub {}: String,\n", name.1));
                }
            }
        }
    }

    format!(
        "#[derive(Deserialize)]\npub struct FormData {{\n{}\n}}\n\n",
        fields
    )
}

pub fn generate_wasm_header() -> String {
    r#"#![cfg(target_arch = "wasm32")]
#![allow(unused_imports, unused_variables)]
use wasm_bindgen::prelude::*;
use web_sys::{console, WebSocket, MessageEvent, Storage};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    console::log_1(&"Hydrating Nucleus App (Generalized WASM)...".into());
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let storage = window.session_storage()?.expect("session storage");

    // Common HMR Logic (Optional, can be conditionally added)
    // For now we expose these variables to user blocks.
"#
    .to_string()
}

pub fn generate_wasm_footer() -> String {
    r#"
    Ok(())
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Element, Node};

    #[test]
    fn test_router_generation() {
        let nodes = vec![
            Node::Element(Element {
                tag_name: "n:view".to_string(),
                attributes: vec![],
                children: vec![],
            })
        ];
        
        let code = generate_router(&nodes);
        assert!(code.contains("Router::new()"));
        assert!(code.contains(".route(\"/\", get(root_handler))"));
    }

    #[test]
    fn test_form_struct_generation() {
        let form = Element {
            tag_name: "n:form".to_string(),
            attributes: vec![],
            children: vec![
                Node::Element(Element {
                    tag_name: "n:input".to_string(),
                    attributes: vec![("name".to_string(), "email".to_string())],
                    children: vec![],
                })
            ],
        };

        let code = generate_form_struct(&form);
        assert!(code.contains("pub struct FormData"));
        assert!(code.contains("pub email: String"));
        assert!(code.contains("#[derive(Deserialize)]"));
    }
}
