use crate::ast::{Model, Node};
use crate::nir::to_nir;
use crate::db::calculate_schema_hash;

#[test]
fn test_nir_generation() {
    let nodes = vec![Node::Text("Hello".to_string())];
    let json = to_nir(&nodes);
    assert!(json.contains("\"version\": \"1.0.0\""));
    assert!(json.contains("Hello"));
}

#[test]
fn test_schema_hashing() {
    let model1 = Model {
        name: "User".to_string(),
        fields: vec![("username".to_string(), "String".to_string())],
        methods: vec![],
        attributes: vec![],
    };
    let model2 = Model {
        name: "User".to_string(),
        fields: vec![("username".to_string(), "String".to_string())],
        methods: vec![],
        attributes: vec![],
    };
    // Different fields
    let model3 = Model {
        name: "User".to_string(),
        fields: vec![("age".to_string(), "Integer".to_string())],
        methods: vec![],
        attributes: vec![],
    };

    let hash1 = calculate_schema_hash(&model1);
    let hash2 = calculate_schema_hash(&model2);
    let hash3 = calculate_schema_hash(&model3);

    assert_eq!(hash1, hash2);
    assert_ne!(hash1, hash3);
}

#[test]
fn test_rosetta_codegen() {
    use crate::ast::Element;
    use crate::rosetta::{generate_swiftui, generate_kotlin};

    let nodes = vec![
        Node::Element(Element {
            tag_name: "n:view".to_string(),
            attributes: vec![],
            children: vec![
                Node::Element(Element {
                    tag_name: "n:text".to_string(),
                    attributes: vec![],
                    children: vec![Node::Text("Hello".to_string())],
                })
            ],
        })
    ];

    let swift = generate_swiftui(&nodes);
    assert!(swift.contains("VStack"));
    assert!(swift.contains("Text(\"Hello\")"));

    let kotlin = generate_kotlin(&nodes);
    assert!(kotlin.contains("Column"));
    assert!(kotlin.contains("Text(\"Hello\")"));
}
