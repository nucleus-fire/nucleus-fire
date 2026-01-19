use crate::ast::{Element, Node};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub struct AtomicCompiler {
    pub rules: HashMap<String, String>, // key: "color:red", value: "c-12345"
}

impl AtomicCompiler {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }
}

impl Default for AtomicCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl AtomicCompiler {
    pub fn compile(&mut self, nodes: &mut [Node]) -> String {
        self.scan_nodes(nodes);
        self.generate_css()
    }

    fn scan_nodes(&mut self, nodes: &mut [Node]) {
        for node in nodes {
            if let Node::Element(el) = node {
                self.scan_element(el);
            }
        }
    }

    fn scan_element(&mut self, el: &mut Element) {
        // Extract style attribute
        if let Some(pos) = el.attributes.iter().position(|(k, _)| k == "style") {
            let (_, style_val) = el.attributes.remove(pos);
            let class_name = self.register_rule(&style_val);

            // Add or append to class attribute
            if let Some(class_pos) = el.attributes.iter().position(|(k, _)| k == "class") {
                el.attributes[class_pos].1.push(' ');
                el.attributes[class_pos].1.push_str(&class_name);
            } else {
                el.attributes.push(("class".to_string(), class_name));
            }
        }

        for child in &mut el.children {
            if let Node::Element(child_el) = child {
                self.scan_element(child_el);
            }
        }
    }

    fn register_rule(&mut self, property: &str) -> String {
        // Simple hash for class name
        if let Some(existing_class) = self.rules.get(property) {
            return existing_class.clone();
        }

        let mut hasher = DefaultHasher::new();
        property.hash(&mut hasher);
        let hash = hasher.finish();
        let class_name = format!("c-{:x}", hash);

        self.rules.insert(property.to_string(), class_name.clone());
        class_name
    }

    fn generate_css(&self) -> String {
        let mut css = String::new();
        for (prop, class_name) in &self.rules {
            css.push_str(&format!(".{} {{ {} }}\n", class_name, prop));
        }
        css
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Element, Node};

    #[test]
    fn test_compile_atomic_css() {
        let mut compiler = AtomicCompiler::new();
        let mut nodes = vec![Node::Element(Element {
            tag_name: "div".to_string(),
            attributes: vec![("style".to_string(), "color: red".to_string())],
            children: vec![],
        })];

        let css = compiler.compile(&mut nodes);

        // Check CSS generation
        assert!(css.contains("color: red"));
        assert!(css.contains(".c-"));

        // Check node modification
        if let Node::Element(el) = &nodes[0] {
            // Style should be removed
            assert!(!el.attributes.iter().any(|(k, _)| k == "style"));
            // Class should be added
            let class_attr = el
                .attributes
                .iter()
                .find(|(k, _)| k == "class")
                .expect("Class attribute missing");
            assert!(class_attr.1.starts_with("c-"));
        } else {
            panic!("Node compiled incorrectly");
        }
    }

    #[test]
    fn test_deduplication() {
        let mut compiler = AtomicCompiler::new();
        let mut nodes = vec![
            Node::Element(Element {
                tag_name: "div".to_string(),
                attributes: vec![("style".to_string(), "color: blue".to_string())],
                children: vec![],
            }),
            Node::Element(Element {
                tag_name: "span".to_string(),
                attributes: vec![("style".to_string(), "color: blue".to_string())],
                children: vec![],
            }),
        ];

        compiler.compile(&mut nodes);

        // Should only have 1 rule
        assert_eq!(compiler.rules.len(), 1);

        // Both elements should have the same class
        if let (Node::Element(el1), Node::Element(el2)) = (&nodes[0], &nodes[1]) {
            let c1 = &el1.attributes.iter().find(|(k, _)| k == "class").unwrap().1;
            let c2 = &el2.attributes.iter().find(|(k, _)| k == "class").unwrap().1;
            assert_eq!(c1, c2);
        }
    }
}
