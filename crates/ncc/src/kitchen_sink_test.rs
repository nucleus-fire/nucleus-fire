use crate::ast::Node;
use crate::codegen::generate_rust;
use crate::css::AtomicCompiler;
use crate::db::generate_sql; // Ensure db is pub
use crate::guardian::Guardian;
use crate::parser::parse_root;
use std::fs;
use std::path::Path;

#[test]
fn test_kitchen_sink_compilation() {
    // Define path to the kitchen-sink file
    // We assume running from workspace root or crate root.
    // Try to find the file from expected locations
    let possible_paths = vec![
        "kitchen-sink/src/views/all_features.ncl",
        "../../kitchen-sink/src/views/all_features.ncl",
        "../kitchen-sink/src/views/all_features.ncl",
    ];

    let mut content = String::new();
    let mut found = false;

    for p in possible_paths {
        if Path::new(p).exists() {
            content = fs::read_to_string(p).expect("Failed to read kitchen sink file");
            found = true;
            break;
        }
    }

    if !found {
        println!("Kitchen sink file not found, skipping test.");
        return;
    }

    // 1. Parse
    let (_, mut nodes) = parse_root(&content).expect("Failed to parse kitchen sink");
    assert!(!nodes.is_empty());

    // 2. Guardian Check
    let guardian = Guardian::new();
    let issues = guardian.validate(&nodes);

    // Fail only if we have actual Errors (High Severity)
    if issues.iter().any(|v| v.is_error()) {
        panic!(
            "Guardian validation failed with critical errors: {:?}",
            issues
        );
    }

    // 3. CSS Compilation
    let mut compiler = AtomicCompiler::new();
    let css = compiler.compile(&mut nodes);
    assert!(css.contains("color: blue"));
    assert!(css.contains(".c-"));

    // 4. DB Generation
    for node in &nodes {
        if let Node::Model(model) = node {
            let sql = generate_sql(model);
            assert!(sql.contains("CREATE TABLE product"));
        }
    }

    // 5. Codegen
    let code = generate_rust(&nodes);
    assert!(code.contains("pub fn app() -> Router"));
    assert!(code.contains("async fn root_handler"));
    assert!(code.contains("Form"));
}
