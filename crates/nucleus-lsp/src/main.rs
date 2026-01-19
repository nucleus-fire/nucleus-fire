use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

// ═══════════════════════════════════════════════════════════════════════════
// TAG DEFINITIONS - Complete NCL Tag Reference
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone)]
#[allow(dead_code)]
struct TagDefinition {
    name: &'static str,
    description: &'static str,
    docs: &'static str,
    attributes: Vec<AttributeDefinition>,
    snippet: &'static str,
    self_closing: bool,
}

#[derive(Clone)]
#[allow(dead_code)]
struct AttributeDefinition {
    name: &'static str,
    description: &'static str,
    required: bool,
    values: Option<Vec<&'static str>>, // Enum values if applicable
}

fn get_all_tags() -> Vec<TagDefinition> {
    vec![
        // ─────────────────────────────────────────────────────────────────────
        // STRUCTURAL TAGS
        // ─────────────────────────────────────────────────────────────────────
        TagDefinition {
            name: "n:view",
            description: "Define a page/view",
            docs: "The root element for a Nucleus page. Defines metadata like title and description for SEO.\n\n## Example\n```html\n<n:view title=\"Home\" description=\"Welcome page\">\n    <h1>Hello World</h1>\n</n:view>\n```",
            attributes: vec![
                AttributeDefinition { name: "title", description: "Page title for <title> tag", required: true, values: None },
                AttributeDefinition { name: "description", description: "Meta description for SEO", required: false, values: None },
                AttributeDefinition { name: "layout", description: "Layout file to use", required: false, values: None },
            ],
            snippet: "n:view title=\"${1:Page Title}\">\n\t$0\n</n:view>",
            self_closing: false,
        },
        TagDefinition {
            name: "n:layout",
            description: "Wrap content with a layout",
            docs: "Wraps the view content with a reusable layout template. The layout file should define `<n:slot>` for content insertion.\n\n## Example\n```html\n<n:layout name=\"main\">\n    <h1>Page Content</h1>\n</n:layout>\n```",
            attributes: vec![
                AttributeDefinition { name: "name", description: "Layout file name (without .ncl)", required: true, values: None },
            ],
            snippet: "n:layout name=\"${1:layout}\">\n\t$0\n</n:layout>",
            self_closing: false,
        },
        TagDefinition {
            name: "n:slot",
            description: "Content insertion point in layouts",
            docs: "Defines where child content should be inserted in a layout. Use named slots for multiple insertion points.\n\n## Example\n```html\n<!-- In layout.ncl -->\n<main>\n    <n:slot name=\"content\" />\n</main>\n<footer>\n    <n:slot name=\"footer\" />\n</footer>\n```",
            attributes: vec![
                AttributeDefinition { name: "name", description: "Slot name (default: 'default')", required: false, values: None },
            ],
            snippet: "n:slot name=\"${1:content}\" />",
            self_closing: true,
        },
        TagDefinition {
            name: "n:component",
            description: "Define a reusable component",
            docs: "Creates a reusable UI component that can be imported and used across views.\n\n## Example\n```html\n<n:component name=\"Button\">\n    <button class=\"btn {class}\">\n        <n:slot />\n    </button>\n</n:component>\n```",
            attributes: vec![
                AttributeDefinition { name: "name", description: "Component name (PascalCase)", required: true, values: None },
            ],
            snippet: "n:component name=\"${1:ComponentName}\">\n\t$0\n</n:component>",
            self_closing: false,
        },
        TagDefinition {
            name: "n:include",
            description: "Include another NCL file",
            docs: "Includes the content of another NCL file at this location. Useful for partials and shared content.\n\n## Example\n```html\n<n:include path=\"components/header.ncl\" />\n```",
            attributes: vec![
                AttributeDefinition { name: "path", description: "Relative path to NCL file", required: true, values: None },
            ],
            snippet: "n:include path=\"${1:path/to/file.ncl}\" />",
            self_closing: true,
        },

        // ─────────────────────────────────────────────────────────────────────
        // CONTROL FLOW TAGS
        // ─────────────────────────────────────────────────────────────────────
        TagDefinition {
            name: "n:for",
            description: "Loop over a collection",
            docs: "Iterates over a collection, rendering the content for each item.\n\n## Example\n```html\n<n:for item=\"user\" in=\"users\">\n    <div>{user.name}</div>\n</n:for>\n```\n\n## With Index\n```html\n<n:for item=\"item\" index=\"i\" in=\"items\">\n    <div>{i}: {item}</div>\n</n:for>\n```",
            attributes: vec![
                AttributeDefinition { name: "item", description: "Variable name for each item", required: true, values: None },
                AttributeDefinition { name: "in", description: "Collection to iterate over", required: true, values: None },
                AttributeDefinition { name: "index", description: "Optional index variable", required: false, values: None },
            ],
            snippet: "n:for item=\"${1:item}\" in=\"${2:collection}\">\n\t$0\n</n:for>",
            self_closing: false,
        },
        TagDefinition {
            name: "n:if",
            description: "Conditional rendering",
            docs: "Renders content only if the condition is true.\n\n## Example\n```html\n<n:if condition=\"user.is_admin\">\n    <button>Admin Panel</button>\n</n:if>\n```",
            attributes: vec![
                AttributeDefinition { name: "condition", description: "Rust expression that evaluates to bool", required: true, values: None },
            ],
            snippet: "n:if condition=\"${1:condition}\">\n\t$0\n</n:if>",
            self_closing: false,
        },
        TagDefinition {
            name: "n:else",
            description: "Else branch for n:if",
            docs: "Renders content when the preceding `<n:if>` condition is false.\n\n## Example\n```html\n<n:if condition=\"user.is_logged_in\">\n    <span>Welcome, {user.name}</span>\n</n:if>\n<n:else>\n    <a href=\"/login\">Login</a>\n</n:else>\n```",
            attributes: vec![],
            snippet: "n:else>\n\t$0\n</n:else>",
            self_closing: false,
        },

        // ─────────────────────────────────────────────────────────────────────
        // DATA TAGS
        // ─────────────────────────────────────────────────────────────────────
        TagDefinition {
            name: "n:model",
            description: "Define a data model struct",
            docs: "Defines a Rust struct for data modeling. The struct is auto-generated with Serialize/Deserialize derives.\n\n## Example\n```html\n<n:model name=\"User\">\n    id: i64\n    name: String\n    email: String\n</n:model>\n```",
            attributes: vec![
                AttributeDefinition { name: "name", description: "Model/struct name (PascalCase)", required: true, values: None },
            ],
            snippet: "n:model name=\"${1:ModelName}\">\n\t${2:field}: ${3:Type}\n</n:model>",
            self_closing: false,
        },
        TagDefinition {
            name: "n:action",
            description: "Server-side action block",
            docs: "Defines server-side Rust code that runs on form submission or page load. Has access to `params` HashMap.\n\n## Example\n```html\n<n:action>\n    use nucleus_std::photon::query::query;\n    \n    let email = params.get(\"email\").unwrap_or(&\"\".to_string()).clone();\n    if !email.is_empty() {\n        query(\"subscribers\").insert().value(\"email\", email).execute().await.ok();\n    }\n</n:action>\n```",
            attributes: vec![],
            snippet: "n:action>\n\t$0\n</n:action>",
            self_closing: false,
        },
        TagDefinition {
            name: "n:load",
            description: "Data loading block",
            docs: "Loads data before rendering the view. Variables defined here are available in the template.\n\n## Example\n```html\n<n:load>\n    let users = User::query().all().await.unwrap_or_default();\n</n:load>\n```",
            attributes: vec![],
            snippet: "n:load>\n\t$0\n</n:load>",
            self_closing: false,
        },

        // ─────────────────────────────────────────────────────────────────────
        // CONTENT TAGS
        // ─────────────────────────────────────────────────────────────────────
        TagDefinition {
            name: "n:text",
            description: "Localized text from content.deck",
            docs: "Renders localized text from the `content.deck` file. Supports interpolation.\n\n## Example\n```html\n<n:text key=\"welcome.message\" />\n```\n\n## content.deck\n```\nwelcome.message = Welcome to our site!\nwelcome.message:es = ¡Bienvenido a nuestro sitio!\n```",
            attributes: vec![
                AttributeDefinition { name: "key", description: "Content key from content.deck", required: true, values: None },
            ],
            snippet: "n:text key=\"${1:content.key}\" />",
            self_closing: true,
        },
        TagDefinition {
            name: "n:outlet",
            description: "Router outlet for nested routes",
            docs: "Placeholder where nested route content will be rendered. Used in layout files for routing.\n\n## Example\n```html\n<main>\n    <n:outlet />\n</main>\n```",
            attributes: vec![],
            snippet: "n:outlet />",
            self_closing: true,
        },

        // ─────────────────────────────────────────────────────────────────────
        // CLIENT-SIDE TAGS
        // ─────────────────────────────────────────────────────────────────────
        TagDefinition {
            name: "n:hydrate",
            description: "Client-side hydration block",
            docs: "Marks a component for client-side hydration. The Rust code compiles to WASM and runs in the browser.\n\n## Example\n```html\n<n:hydrate>\n    let count = use_state(|| 0);\n    <button onclick={|_| count.set(*count + 1)}>\n        Count: {count}\n    </button>\n</n:hydrate>\n```",
            attributes: vec![],
            snippet: "n:hydrate>\n\t$0\n</n:hydrate>",
            self_closing: false,
        },
        TagDefinition {
            name: "n:script",
            description: "Inline JavaScript",
            docs: "Embeds JavaScript code that runs on the client. Prefer `<n:hydrate>` for interactive Rust code.\n\n## Example\n```html\n<n:script>\n    console.log('Page loaded');\n</n:script>\n```",
            attributes: vec![
                AttributeDefinition { name: "src", description: "External script URL", required: false, values: None },
            ],
            snippet: "n:script>\n\t$0\n</n:script>",
            self_closing: false,
        },

        // ─────────────────────────────────────────────────────────────────────
        // FORM TAGS
        // ─────────────────────────────────────────────────────────────────────
        TagDefinition {
            name: "n:form",
            description: "Enhanced form with validation",
            docs: "A form with built-in CSRF protection and validation support.\n\n## Example\n```html\n<n:form action=\"/subscribe\" method=\"POST\">\n    <input name=\"email\" type=\"email\" required />\n    <button type=\"submit\">Subscribe</button>\n</n:form>\n```",
            attributes: vec![
                AttributeDefinition { name: "action", description: "Form submission URL", required: false, values: None },
                AttributeDefinition { name: "method", description: "HTTP method", required: false, values: Some(vec!["GET", "POST"]) },
            ],
            snippet: "n:form action=\"${1:/action}\" method=\"${2:POST}\">\n\t$0\n</n:form>",
            self_closing: false,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════
// LSP BACKEND
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug)]
struct Backend {
    client: Client,
    root_path: Mutex<Option<String>>,
    documents: Mutex<HashMap<Url, String>>,
}

impl Backend {
    fn get_tag_at_position(&self, content: &str, position: Position) -> Option<String> {
        let lines: Vec<&str> = content.lines().collect();
        if position.line as usize >= lines.len() {
            return None;
        }

        let line = lines[position.line as usize];
        let col = position.character as usize;

        // Find the tag name before the cursor
        let before = &line[..col.min(line.len())];

        // Look for <n:tagname pattern
        if let Some(tag_start) = before.rfind("<n:") {
            let tag_part = &before[tag_start + 3..];
            let tag_name = tag_part
                .split(|c: char| c.is_whitespace() || c == '>' || c == '/')
                .next()
                .unwrap_or("");
            if !tag_name.is_empty() {
                return Some(format!("n:{}", tag_name));
            }
        }

        None
    }

    fn get_completion_context(&self, content: &str, position: Position) -> CompletionContext {
        let lines: Vec<&str> = content.lines().collect();
        if position.line as usize >= lines.len() {
            return CompletionContext::Unknown;
        }

        let line = lines[position.line as usize];
        let col = position.character as usize;
        let before = &line[..col.min(line.len())];

        // Check if we're inside a tag (after < but before >)
        let last_open = before.rfind('<');
        let last_close = before.rfind('>');

        match (last_open, last_close) {
            (Some(open), Some(close)) if open > close => {
                // We're inside a tag
                let tag_content = &before[open..];

                // Check if we're after a space (attribute position)
                if tag_content.contains(' ')
                    && !tag_content.ends_with('=')
                    && !tag_content.ends_with('"')
                {
                    if let Some(tag_start) = tag_content.find("<n:") {
                        let tag_part = &tag_content[tag_start + 3..];
                        let tag_name = tag_part
                            .split(|c: char| c.is_whitespace())
                            .next()
                            .unwrap_or("");
                        return CompletionContext::Attribute(format!("n:{}", tag_name));
                    }
                }

                // Check if we just typed <n: or similar
                if tag_content.ends_with("<n:") || tag_content.contains("<n:") {
                    return CompletionContext::TagName;
                }
            }
            (Some(_), None) => {
                // We're inside the first tag
                if before.ends_with("<n:") || before.ends_with("<") {
                    return CompletionContext::TagName;
                }
            }
            _ => {}
        }

        // Check if we're after a < at the end
        if before.trim_end().ends_with('<') {
            return CompletionContext::TagName;
        }

        CompletionContext::Unknown
    }

    fn validate_document(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let tags = get_all_tags();
        let tag_names: Vec<&str> = tags.iter().map(|t| t.name).collect();

        for (line_num, line) in content.lines().enumerate() {
            // Check for invalid n: tags
            let mut search_start = 0;
            while let Some(tag_start) = line[search_start..].find("<n:") {
                let abs_start = search_start + tag_start;
                let after_prefix = &line[abs_start + 3..];

                // Extract tag name
                let tag_end = after_prefix
                    .find(|c: char| c.is_whitespace() || c == '>' || c == '/')
                    .unwrap_or(after_prefix.len());
                let tag_name = &after_prefix[..tag_end];
                let full_tag = format!("n:{}", tag_name);

                if !tag_names.contains(&full_tag.as_str()) && !tag_name.is_empty() {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position { line: line_num as u32, character: abs_start as u32 },
                            end: Position { line: line_num as u32, character: (abs_start + 3 + tag_end) as u32 },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: Some(NumberOrString::String("invalid-tag".to_string())),
                        source: Some("nucleus".to_string()),
                        message: format!("Unknown Nucleus tag: <{}>. Did you mean one of: n:view, n:for, n:if, n:model, n:action?", full_tag),
                        ..Default::default()
                    });
                }

                search_start = abs_start + 3 + tag_end;
            }

            // Check for unclosed braces in interpolations
            let open_braces = line.matches('{').count();
            let close_braces = line.matches('}').count();
            if open_braces != close_braces {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position {
                            line: line_num as u32,
                            character: 0,
                        },
                        end: Position {
                            line: line_num as u32,
                            character: line.len() as u32,
                        },
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    code: Some(NumberOrString::String("unbalanced-braces".to_string())),
                    source: Some("nucleus".to_string()),
                    message: "Unbalanced braces in expression interpolation".to_string(),
                    ..Default::default()
                });
            }
        }

        // Check for required attributes
        for (line_num, line) in content.lines().enumerate() {
            for tag in &tags {
                let pattern = format!("<{}", tag.name);
                if let Some(tag_start) = line.find(&pattern) {
                    // Find the end of this tag
                    let after = &line[tag_start..];
                    let tag_end = after.find('>').unwrap_or(after.len());
                    let tag_content = &after[..tag_end];

                    for attr in &tag.attributes {
                        if attr.required {
                            let attr_pattern = format!("{}=", attr.name);
                            if !tag_content.contains(&attr_pattern) {
                                diagnostics.push(Diagnostic {
                                    range: Range {
                                        start: Position {
                                            line: line_num as u32,
                                            character: tag_start as u32,
                                        },
                                        end: Position {
                                            line: line_num as u32,
                                            character: (tag_start + tag_end) as u32,
                                        },
                                    },
                                    severity: Some(DiagnosticSeverity::ERROR),
                                    code: Some(NumberOrString::String(
                                        "missing-attribute".to_string(),
                                    )),
                                    source: Some("nucleus".to_string()),
                                    message: format!(
                                        "<{}> requires attribute: {}",
                                        tag.name, attr.name
                                    ),
                                    ..Default::default()
                                });
                            }
                        }
                    }
                }
            }
        }

        diagnostics
    }

    async fn find_definition(&self, content: &str, position: Position) -> Option<Location> {
        let lines: Vec<&str> = content.lines().collect();
        if position.line as usize >= lines.len() {
            return None;
        }

        let line = lines[position.line as usize];
        let col = position.character as usize;

        // Check for layout reference: name="layout_name"
        if let Some(layout_match) = self.find_quoted_value_at(line, col, "name") {
            let root = self.root_path.lock().unwrap().clone()?;
            let layout_path = PathBuf::from(&root)
                .join("src/views")
                .join(format!("{}.ncl", layout_match));

            if layout_path.exists() {
                return Some(Location {
                    uri: Url::from_file_path(&layout_path).ok()?,
                    range: Range::default(),
                });
            }
        }

        // Check for include reference: path="some/file.ncl"
        if let Some(path_match) = self.find_quoted_value_at(line, col, "path") {
            let root = self.root_path.lock().unwrap().clone()?;
            let include_path = PathBuf::from(&root).join(&path_match);

            if include_path.exists() {
                return Some(Location {
                    uri: Url::from_file_path(&include_path).ok()?,
                    range: Range::default(),
                });
            }
        }

        None
    }

    fn find_quoted_value_at(&self, line: &str, col: usize, attr_name: &str) -> Option<String> {
        let pattern = format!("{}=\"", attr_name);
        if let Some(attr_start) = line.find(&pattern) {
            let value_start = attr_start + pattern.len();
            if let Some(value_end) = line[value_start..].find('"') {
                let value = &line[value_start..value_start + value_end];
                // Check if cursor is within this value
                if col >= value_start && col <= value_start + value_end {
                    return Some(value.to_string());
                }
            }
        }
        None
    }

    fn discover_components(&self) -> Vec<CompletionItem> {
        let mut items = Vec::new();

        let root_opt = self.root_path.lock().unwrap().clone();
        if let Some(root) = root_opt {
            // Scan src/components/
            let components_dir = PathBuf::from(&root).join("src/components");
            if let Ok(entries) = std::fs::read_dir(&components_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map(|e| e == "ncl").unwrap_or(false) {
                        if let Some(stem) = path.file_stem() {
                            let name = stem.to_string_lossy().to_string();
                            items.push(CompletionItem {
                                label: format!("n:{}", name),
                                kind: Some(CompletionItemKind::CLASS),
                                detail: Some("Custom Component".to_string()),
                                documentation: Some(Documentation::String(format!(
                                    "Component from src/components/{}.ncl",
                                    name
                                ))),
                                insert_text: Some(format!("n:{} $1 />", name)),
                                insert_text_format: Some(InsertTextFormat::SNIPPET),
                                ..Default::default()
                            });
                        }
                    }
                }
            }

            // Scan src/views/ for layouts
            let views_dir = PathBuf::from(&root).join("src/views");
            if let Ok(entries) = std::fs::read_dir(&views_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map(|e| e == "ncl").unwrap_or(false) {
                        if let Some(stem) = path.file_stem() {
                            let name = stem.to_string_lossy().to_string();
                            if name != "index" {
                                items.push(CompletionItem {
                                    label: name.clone(),
                                    kind: Some(CompletionItemKind::FILE),
                                    detail: Some("Layout file".to_string()),
                                    documentation: Some(Documentation::String(format!(
                                        "Layout from src/views/{}.ncl",
                                        name
                                    ))),
                                    ..Default::default()
                                });
                            }
                        }
                    }
                }
            }
        }

        items
    }
}

#[derive(Debug, Clone)]
enum CompletionContext {
    TagName,
    Attribute(String), // The tag we're in
    Unknown,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        {
            let mut root = self.root_path.lock().unwrap();
            if let Some(uri) = params.root_uri {
                if let Ok(path) = uri.to_file_path() {
                    *root = Some(path.to_string_lossy().to_string());
                }
            }
        }

        self.client
            .log_message(
                MessageType::INFO,
                "Nucleus LSP Initialized with full feature support!",
            )
            .await;

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(true),
                    trigger_characters: Some(vec![
                        ".".to_string(),
                        ":".to_string(),
                        "<".to_string(),
                        "\"".to_string(),
                        " ".to_string(),
                    ]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(
                MessageType::INFO,
                "Nucleus LSP ready with completions, hover, diagnostics, and go-to-definition!",
            )
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let content = params.text_document.text.clone();

        // Store document
        self.documents
            .lock()
            .unwrap()
            .insert(uri.clone(), content.clone());

        // Publish diagnostics
        let diagnostics = self.validate_document(&content);
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        if let Some(change) = params.content_changes.first() {
            let content = change.text.clone();

            // Update stored document
            self.documents
                .lock()
                .unwrap()
                .insert(uri.clone(), content.clone());

            // Publish diagnostics
            let diagnostics = self.validate_document(&content);
            self.client
                .publish_diagnostics(uri, diagnostics, None)
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.documents.lock().unwrap().remove(&uri);
        // Clear diagnostics
        self.client.publish_diagnostics(uri, vec![], None).await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let content = self
            .documents
            .lock()
            .unwrap()
            .get(&uri)
            .cloned()
            .unwrap_or_default();
        let context = self.get_completion_context(&content, position);

        let tags = get_all_tags();
        let mut items: Vec<CompletionItem> = Vec::new();

        match context {
            CompletionContext::TagName => {
                // Provide all tag completions
                for tag in &tags {
                    items.push(CompletionItem {
                        label: tag.name.to_string(),
                        kind: Some(CompletionItemKind::KEYWORD),
                        detail: Some(tag.description.to_string()),
                        documentation: Some(Documentation::MarkupContent(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: tag.docs.to_string(),
                        })),
                        insert_text: Some(tag.snippet.to_string()),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        ..Default::default()
                    });
                }

                // Add discovered components
                items.extend(self.discover_components());
            }
            CompletionContext::Attribute(tag_name) => {
                // Provide attribute completions for the specific tag
                if let Some(tag) = tags.iter().find(|t| t.name == tag_name) {
                    for attr in &tag.attributes {
                        let mut label = attr.name.to_string();
                        if attr.required {
                            label.push_str(" (required)");
                        }

                        items.push(CompletionItem {
                            label: attr.name.to_string(),
                            kind: Some(CompletionItemKind::PROPERTY),
                            detail: Some(attr.description.to_string()),
                            insert_text: Some(format!("{}=\"$1\"", attr.name)),
                            insert_text_format: Some(InsertTextFormat::SNIPPET),
                            ..Default::default()
                        });
                    }
                }
            }
            CompletionContext::Unknown => {
                // Provide general completions
                for tag in &tags {
                    items.push(CompletionItem {
                        label: format!("<{}>", tag.name),
                        kind: Some(CompletionItemKind::SNIPPET),
                        detail: Some(tag.description.to_string()),
                        insert_text: Some(format!("<{}", tag.snippet)),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        ..Default::default()
                    });
                }
            }
        }

        // Add content.deck keys
        let root_opt = self.root_path.lock().unwrap().clone();
        if let Some(root) = root_opt {
            let path = std::path::Path::new(&root).join("content.deck");
            if path.exists() {
                if let Ok(content) = tokio::fs::read_to_string(path).await {
                    for line in content.lines() {
                        if let Some((key, _)) = line.split_once("=") {
                            let key = key.trim();
                            let key = key.split(':').next().unwrap_or(key);

                            items.push(CompletionItem {
                                label: key.to_string(),
                                kind: Some(CompletionItemKind::TEXT),
                                detail: Some("Content Deck Key".to_string()),
                                insert_text: Some(key.to_string()),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let content = self
            .documents
            .lock()
            .unwrap()
            .get(&uri)
            .cloned()
            .unwrap_or_default();

        if let Some(tag_name) = self.get_tag_at_position(&content, position) {
            let tags = get_all_tags();
            if let Some(tag) = tags.iter().find(|t| t.name == tag_name) {
                return Ok(Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!(
                            "## `<{}>`\n\n{}\n\n{}",
                            tag.name, tag.description, tag.docs
                        ),
                    }),
                    range: None,
                }));
            }
        }

        Ok(None)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let content = self
            .documents
            .lock()
            .unwrap()
            .get(&uri)
            .cloned()
            .unwrap_or_default();

        if let Some(location) = self.find_definition(&content, position).await {
            return Ok(Some(GotoDefinitionResponse::Scalar(location)));
        }

        Ok(None)
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;
        let content = self
            .documents
            .lock()
            .unwrap()
            .get(&uri)
            .cloned()
            .unwrap_or_default();

        let mut symbols = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            // Find n:view
            if let Some(start) = line.find("<n:view") {
                if let Some(title_start) = line.find("title=\"") {
                    let after = &line[title_start + 7..];
                    if let Some(title_end) = after.find('"') {
                        let title = &after[..title_end];
                        symbols.push(DocumentSymbol {
                            name: format!("View: {}", title),
                            kind: SymbolKind::CLASS,
                            range: Range {
                                start: Position {
                                    line: line_num as u32,
                                    character: start as u32,
                                },
                                end: Position {
                                    line: line_num as u32,
                                    character: line.len() as u32,
                                },
                            },
                            selection_range: Range {
                                start: Position {
                                    line: line_num as u32,
                                    character: start as u32,
                                },
                                end: Position {
                                    line: line_num as u32,
                                    character: line.len() as u32,
                                },
                            },
                            detail: None,
                            tags: None,
                            children: None,
                            #[allow(deprecated)]
                            deprecated: None,
                        });
                    }
                }
            }

            // Find n:model
            if let Some(start) = line.find("<n:model") {
                if let Some(name_start) = line.find("name=\"") {
                    let after = &line[name_start + 6..];
                    if let Some(name_end) = after.find('"') {
                        let name = &after[..name_end];
                        symbols.push(DocumentSymbol {
                            name: format!("Model: {}", name),
                            kind: SymbolKind::STRUCT,
                            range: Range {
                                start: Position {
                                    line: line_num as u32,
                                    character: start as u32,
                                },
                                end: Position {
                                    line: line_num as u32,
                                    character: line.len() as u32,
                                },
                            },
                            selection_range: Range {
                                start: Position {
                                    line: line_num as u32,
                                    character: start as u32,
                                },
                                end: Position {
                                    line: line_num as u32,
                                    character: line.len() as u32,
                                },
                            },
                            detail: None,
                            tags: None,
                            children: None,
                            #[allow(deprecated)]
                            deprecated: None,
                        });
                    }
                }
            }

            // Find n:action
            if line.contains("<n:action") {
                symbols.push(DocumentSymbol {
                    name: "Action".to_string(),
                    kind: SymbolKind::FUNCTION,
                    range: Range {
                        start: Position {
                            line: line_num as u32,
                            character: 0,
                        },
                        end: Position {
                            line: line_num as u32,
                            character: line.len() as u32,
                        },
                    },
                    selection_range: Range {
                        start: Position {
                            line: line_num as u32,
                            character: 0,
                        },
                        end: Position {
                            line: line_num as u32,
                            character: line.len() as u32,
                        },
                    },
                    detail: None,
                    tags: None,
                    children: None,
                    #[allow(deprecated)]
                    deprecated: None,
                });
            }

            // Find n:component
            if let Some(start) = line.find("<n:component") {
                if let Some(name_start) = line.find("name=\"") {
                    let after = &line[name_start + 6..];
                    if let Some(name_end) = after.find('"') {
                        let name = &after[..name_end];
                        symbols.push(DocumentSymbol {
                            name: format!("Component: {}", name),
                            kind: SymbolKind::CLASS,
                            range: Range {
                                start: Position {
                                    line: line_num as u32,
                                    character: start as u32,
                                },
                                end: Position {
                                    line: line_num as u32,
                                    character: line.len() as u32,
                                },
                            },
                            selection_range: Range {
                                start: Position {
                                    line: line_num as u32,
                                    character: start as u32,
                                },
                                end: Position {
                                    line: line_num as u32,
                                    character: line.len() as u32,
                                },
                            },
                            detail: None,
                            tags: None,
                            children: None,
                            #[allow(deprecated)]
                            deprecated: None,
                        });
                    }
                }
            }
        }

        Ok(Some(DocumentSymbolResponse::Nested(symbols)))
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        root_path: Mutex::new(None),
        documents: Mutex::new(HashMap::new()),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_tags() {
        let tags = get_all_tags();
        assert!(tags.len() >= 15);

        // Check required tags exist
        let tag_names: Vec<&str> = tags.iter().map(|t| t.name).collect();
        assert!(tag_names.contains(&"n:view"));
        assert!(tag_names.contains(&"n:for"));
        assert!(tag_names.contains(&"n:if"));
        assert!(tag_names.contains(&"n:model"));
        assert!(tag_names.contains(&"n:action"));
        assert!(tag_names.contains(&"n:layout"));
        assert!(tag_names.contains(&"n:slot"));
        assert!(tag_names.contains(&"n:component"));
    }

    #[test]
    fn test_tag_has_required_attributes() {
        let tags = get_all_tags();

        let view_tag = tags.iter().find(|t| t.name == "n:view").unwrap();
        assert!(view_tag
            .attributes
            .iter()
            .any(|a| a.name == "title" && a.required));

        let for_tag = tags.iter().find(|t| t.name == "n:for").unwrap();
        assert!(for_tag
            .attributes
            .iter()
            .any(|a| a.name == "item" && a.required));
        assert!(for_tag
            .attributes
            .iter()
            .any(|a| a.name == "in" && a.required));
    }
}
