//! Guardian Linter - Validates NCL code for quality, security, accessibility, and performance.
#![allow(unused_assignments)]
#![allow(clippy::useless_format)]

use crate::ast::Node;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, Clone)]
pub enum GuardianRule {
    #[error("Accessibility: {message}")]
    #[diagnostic(code(guardian::a11y), severity(Warning))]
    A11y {
        message: String,
        #[label("Here")]
        span: Option<SourceSpan>,
    },

    #[error("Security Risk: {message}")]
    #[diagnostic(
        code(guardian::security),
        severity(Error),
        help("Ensure you are not exposing the application to XSS or Injection attacks.")
    )]
    Security {
        message: String,
        #[label("Risk Source")]
        span: Option<SourceSpan>,
    },

    #[error("Performance: {message}")]
    #[diagnostic(code(guardian::perf), severity(Warning))]
    Performance {
        message: String,
        #[label("Optimization Opportunity")]
        span: Option<SourceSpan>,
    },

    #[error("Quality: {message}")]
    #[diagnostic(code(guardian::quality), severity(Error))]
    Quality {
        message: String,
        #[label("Required Fix")]
        span: Option<SourceSpan>,
    },
}

pub struct Guardian;

impl Guardian {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, nodes: &[Node]) -> Vec<GuardianRule> {
        let mut violations = Vec::new();

        self.validate_structure(nodes, &mut violations);
        self.validate_nodes(nodes, &mut violations);

        violations
    }

    fn validate_structure(&self, nodes: &[Node], violations: &mut Vec<GuardianRule>) {
        let has_script = nodes.iter().any(|n| matches!(n, Node::Script { .. }));
        let has_spec = nodes
            .iter()
            .any(|n| matches!(n, Node::Spec(_) | Node::Test(_)));

        if has_script && !has_spec {
            violations.push(GuardianRule::Quality {
                message: "Found <n:script> logic without corresponding <n:spec> or <n:test>."
                    .to_string(),
                span: None,
            });
        }
    }

    fn validate_nodes(&self, nodes: &[Node], violations: &mut Vec<GuardianRule>) {
        for node in nodes {
            match node {
                Node::Element(el) => {
                    // A11y: Images must have alt
                    if el.tag_name == "img" && !el.attributes.iter().any(|(k, _)| k == "alt") {
                        violations.push(GuardianRule::A11y {
                            message: "<img> tag missing 'alt' attribute.".to_string(),
                            span: None,
                        });
                    }

                    // A11y: Inputs needs labels
                    if el.tag_name == "input" {
                        let has_label = el.attributes.iter().any(|(k, _)| {
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
                            violations.push(GuardianRule::A11y {
                                message: "<input> missing accessible label (aria-label, id, placeholder, etc).".to_string(),
                                span: None,
                            });
                        }
                    }

                    // Perf: Large Inline Styles
                    if let Some((_, style)) = el.attributes.iter().find(|(k, _)| k == "style") {
                        if style.len() > 150 {
                            violations.push(GuardianRule::Performance {
                                message: "Inline style is too long (>150 chars). Extract to CSS class for better caching.".to_string(),
                                span: None,
                            });
                        }
                    }

                    // Security: iFrames
                    if el.tag_name == "iframe" && !el.attributes.iter().any(|(k, _)| k == "sandbox")
                    {
                        violations.push(GuardianRule::Security {
                            message: "<iframe> detected without 'sandbox' attribute.".to_string(),
                            span: None,
                        });
                    }

                    self.validate_nodes(&el.children, violations);
                }
                Node::If { children, .. } => self.validate_nodes(children, violations),
                Node::For { children, .. } => self.validate_nodes(children, violations),
                _ => {}
            }
        }
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
    fn test_guardian_a11y() {
        let nodes = vec![Node::Element(crate::ast::Element {
            tag_name: "img".to_string(),
            attributes: vec![("src".to_string(), "foo.jpg".to_string())],
            children: vec![],
        })];
        let guardian = Guardian::new();
        let issues = guardian.validate(&nodes);
        assert!(issues
            .iter()
            .any(|i| matches!(i, GuardianRule::A11y { .. })));
    }

    #[test]
    fn test_guardian_perf() {
        let long_style = "color: red;".repeat(20);
        let nodes = vec![Node::Element(crate::ast::Element {
            tag_name: "div".to_string(),
            attributes: vec![("style".to_string(), long_style)],
            children: vec![],
        })];
        let guardian = Guardian::new();
        let issues = guardian.validate(&nodes);
        assert!(issues
            .iter()
            .any(|i| matches!(i, GuardianRule::Performance { .. })));
    }
}
