use crate::ast::Node;

pub struct Guardian;

impl Guardian {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, nodes: &[Node]) -> Result<(), String> {
        let has_script = nodes.iter().any(|n| matches!(n, Node::Script { .. }));
        let has_spec = nodes
            .iter()
            .any(|n| matches!(n, Node::Spec(_) | Node::Test(_)));

        if has_script && !has_spec {
            return Err("Guardian Error: Found <n:script> logic without corresponding <n:spec> or <n:test>.".to_string());
        }

        self.validate_a11y(nodes)
    }

    fn validate_a11y(&self, nodes: &[Node]) -> Result<(), String> {
        for node in nodes {
            match node {
                Node::Element(el) => {
                    // Rule 1: Images must have alt text
                    if el.tag_name == "img" && !el.attributes.iter().any(|(k, _)| k == "alt") {
                        return Err(format!("Accessibility Violation: <img> tag (near {:?}) missing 'alt' attribute.", el.attributes));
                    }

                    // Rule 2: Buttons must have content or aria-label
                    if el.tag_name == "button"
                        && el.children.is_empty()
                        && !el.attributes.iter().any(|(k, _)| k == "aria-label")
                    {
                        return Err(
                            "Accessibility Violation: <button> empty and missing 'aria-label'."
                                .to_string(),
                        );
                    }

                    // Rule 3: Anchors must have content
                    if el.tag_name == "a"
                        && el.children.is_empty()
                        && !el.attributes.iter().any(|(k, _)| k == "aria-label")
                    {
                        return Err(
                            "Accessibility Violation: <a> link empty and missing 'aria-label'."
                                .to_string(),
                        );
                    }

                    // Rule 4: Inputs must have accessible label or identifier
                    if el.tag_name == "input" {
                        let attrs = &el.attributes;
                        let type_attr = attrs
                            .iter()
                            .find(|(k, _)| k == "type")
                            .map(|(_, v)| v.as_str())
                            .unwrap_or("text");
                        let ignored_types = ["hidden", "submit", "reset", "button", "image"];

                        if !ignored_types.contains(&type_attr) {
                            let has_label = attrs.iter().any(|(k, _)| {
                                [
                                    "aria-label",
                                    "aria-labelledby",
                                    "title",
                                    "id",
                                    "placeholder",
                                ]
                                .contains(&k.as_str())
                            });
                            if !has_label {
                                return Err(format!("Accessibility Violation: <input type='{}'> missing 'aria-label', 'title', 'id', or 'placeholder'.", type_attr));
                            }
                        }
                    }

                    self.validate_a11y(&el.children)?;
                }
                Node::If { children, .. } => self.validate_a11y(children)?,
                Node::For { children, .. } => self.validate_a11y(children)?,
                _ => {}
            }
        }
        Ok(())
    }
}

impl Default for Guardian {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Node;

    #[test]
    fn test_guardian_passes_without_script() {
        let nodes = vec![Node::Text("Hello".to_string())];
        let guardian = Guardian::new();
        assert!(guardian.validate(&nodes).is_ok());
    }

    #[test]
    fn test_guardian_passes_with_script_and_spec() {
        let nodes = vec![
            Node::Script {
                content: "fn main() {}".to_string(),
                attributes: vec![],
            },
            Node::Spec("fn test() {}".to_string()),
        ];
        let guardian = Guardian::new();
        assert!(guardian.validate(&nodes).is_ok());
    }

    #[test]
    fn test_guardian_fails_with_script_no_spec() {
        let nodes = vec![Node::Script {
            content: "fn main() {}".to_string(),
            attributes: vec![],
        }];
        let guardian = Guardian::new();
        assert!(guardian.validate(&nodes).is_err());
    }

    #[test]
    fn test_guardian_fails_img_no_alt() {
        let nodes = vec![Node::Element(crate::ast::Element {
            tag_name: "img".to_string(),
            attributes: vec![("src".to_string(), "foo.jpg".to_string())],
            children: vec![],
        })];
        let guardian = Guardian::new();
        assert!(guardian.validate(&nodes).is_err());
    }

    #[test]
    fn test_guardian_fails_input_no_label() {
        let nodes = vec![Node::Element(crate::ast::Element {
            tag_name: "input".to_string(),
            attributes: vec![("type".to_string(), "text".to_string())], // No label/id/placeholder
            children: vec![],
        })];
        let guardian = Guardian::new();
        assert!(guardian.validate(&nodes).is_err());
    }

    #[test]
    fn test_guardian_passes_accessible_input() {
        let nodes = vec![Node::Element(crate::ast::Element {
            tag_name: "input".to_string(),
            attributes: vec![
                ("type".to_string(), "text".to_string()),
                ("aria-label".to_string(), "Username".to_string()),
            ],
            children: vec![],
        })];
        let guardian = Guardian::new();
        assert!(guardian.validate(&nodes).is_ok());
    }
}
