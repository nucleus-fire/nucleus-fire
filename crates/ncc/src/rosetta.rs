use crate::ast::{Element, Node};

pub fn generate_swiftui(nodes: &[Node]) -> String {
    let mut code = String::new();
    for node in nodes {
        if let Node::Element(el) = node {
            code.push_str(&map_element_swift(el));
        }
    }
    code
}

pub fn generate_kotlin(nodes: &[Node]) -> String {
    let mut code = String::new();
    for node in nodes {
        if let Node::Element(el) = node {
            code.push_str(&map_element_kotlin(el));
        }
    }
    code
}

fn map_element_swift(el: &Element) -> String {
    match el.tag_name.as_str() {
        "n:view" => {
            let children: String = el.children.iter().map(map_node_swift).collect();
            format!("VStack {{\n{}\n}}", children)
        }
        "n:list" => {
            let children: String = el.children.iter().map(map_node_swift).collect();
            format!("List {{\n{}\n}}", children)
        }
        "n:text" | "span" | "h1" => {
            // Simplified text mapping
            if let Some(Node::Text(t)) = el.children.first() {
                format!("Text(\"{}\")", t)
            } else {
                "Text(\"\")".to_string()
            }
        }
        _ => "".to_string(),
    }
}

fn map_node_swift(node: &Node) -> String {
    match node {
        Node::Element(el) => map_element_swift(el),
        Node::Text(t) => format!("Text(\"{}\")", t),
        _ => "".to_string(),
    }
}

fn map_element_kotlin(el: &Element) -> String {
    match el.tag_name.as_str() {
        "n:view" => {
            let children: String = el.children.iter().map(map_node_kotlin).collect();
            format!("Column {{\n{}\n}}", children)
        }
        "n:list" => {
            let children: String = el.children.iter().map(map_node_kotlin).collect();
            format!("LazyColumn {{\n{}\n}}", children)
        }
        "n:text" | "span" | "h1" => {
            if let Some(Node::Text(t)) = el.children.first() {
                format!("Text(\"{}\")", t)
            } else {
                "Text(\"\")".to_string()
            }
        }
        _ => "".to_string(),
    }
}

fn map_node_kotlin(node: &Node) -> String {
    match node {
        Node::Element(el) => map_element_kotlin(el),
        Node::Text(t) => format!("Text(\"{}\")", t),
        _ => "".to_string(),
    }
}
