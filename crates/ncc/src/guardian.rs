//! Guardian Linter - Validates NCL code for quality, security, accessibility, and performance.

use crate::ast::Node;
use miette::{Diagnostic, SourceSpan};
use std::fmt;

/// Severity level for Guardian rules
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Warning,
    Error,
}

/// Category of Guardian rule
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCategory {
    A11y,
    Security,
    Performance,
    Quality,
}

impl fmt::Display for RuleCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleCategory::A11y => write!(f, "Accessibility"),
            RuleCategory::Security => write!(f, "Security"),
            RuleCategory::Performance => write!(f, "Performance"),
            RuleCategory::Quality => write!(f, "Quality"),
        }
    }
}

/// A single Guardian rule violation
#[derive(Debug, Clone)]
pub struct GuardianViolation {
    pub category: RuleCategory,
    pub message: String,
    pub span: Option<SourceSpan>,
    pub help: Option<String>,
}

impl GuardianViolation {
    pub fn a11y(message: impl Into<String>) -> Self {
        Self {
            category: RuleCategory::A11y,
            message: message.into(),
            span: None,
            help: None,
        }
    }

    pub fn security(message: impl Into<String>) -> Self {
        Self {
            category: RuleCategory::Security,
            message: message.into(),
            span: None,
            help: Some(
                "Ensure you are not exposing the application to XSS or Injection attacks.".into(),
            ),
        }
    }

    pub fn performance(message: impl Into<String>) -> Self {
        Self {
            category: RuleCategory::Performance,
            message: message.into(),
            span: None,
            help: None,
        }
    }

    pub fn quality(message: impl Into<String>) -> Self {
        Self {
            category: RuleCategory::Quality,
            message: message.into(),
            span: None,
            help: None,
        }
    }

    pub fn severity(&self) -> Severity {
        match self.category {
            RuleCategory::A11y | RuleCategory::Performance => Severity::Warning,
            RuleCategory::Security | RuleCategory::Quality => Severity::Error,
        }
    }

    pub fn is_error(&self) -> bool {
        self.severity() == Severity::Error
    }

    pub fn code(&self) -> &'static str {
        match self.category {
            RuleCategory::A11y => "guardian::a11y",
            RuleCategory::Security => "guardian::security",
            RuleCategory::Performance => "guardian::perf",
            RuleCategory::Quality => "guardian::quality",
        }
    }
}

impl fmt::Display for GuardianViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.category, self.message)
    }
}

impl std::error::Error for GuardianViolation {}

impl Diagnostic for GuardianViolation {
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        Some(Box::new(self.code()))
    }

    fn severity(&self) -> Option<miette::Severity> {
        Some(match self.severity() {
            Severity::Warning => miette::Severity::Warning,
            Severity::Error => miette::Severity::Error,
        })
    }

    fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        self.help
            .as_ref()
            .map(|h| Box::new(h.as_str()) as Box<dyn fmt::Display>)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        self.span.map(|s| {
            Box::new(std::iter::once(miette::LabeledSpan::new_with_span(
                Some("here".to_string()),
                s,
            ))) as Box<dyn Iterator<Item = miette::LabeledSpan>>
        })
    }
}

// Keep the old enum name as an alias for backwards compatibility
pub type GuardianRule = GuardianViolation;

pub struct Guardian;

impl Guardian {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, nodes: &[Node]) -> Vec<GuardianViolation> {
        let mut violations = Vec::new();

        self.validate_structure(nodes, &mut violations);
        self.validate_nodes(nodes, &mut violations);

        violations
    }

    fn validate_structure(&self, nodes: &[Node], violations: &mut Vec<GuardianViolation>) {
        let has_script = nodes.iter().any(|n| matches!(n, Node::Script { .. }));
        let has_spec = nodes
            .iter()
            .any(|n| matches!(n, Node::Spec(_) | Node::Test(_)));

        if has_script && !has_spec {
            violations.push(GuardianViolation::quality(
                "Found <n:script> logic without corresponding <n:spec> or <n:test>.",
            ));
        }
    }

    fn validate_nodes(&self, nodes: &[Node], violations: &mut Vec<GuardianViolation>) {
        for node in nodes {
            match node {
                Node::Element(el) => {
                    // A11y: Images must have alt
                    if el.tag_name == "img" && !el.attributes.iter().any(|(k, _)| k == "alt") {
                        violations.push(GuardianViolation::a11y(
                            "<img> tag missing 'alt' attribute.",
                        ));
                    }

                    // A11y: Inputs need labels
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
                            violations.push(GuardianViolation::a11y(
                                "<input> missing accessible label (aria-label, id, placeholder, etc).",
                            ));
                        }
                    }

                    // Perf: Large Inline Styles
                    if let Some((_, style)) = el.attributes.iter().find(|(k, _)| k == "style") {
                        if style.len() > 150 {
                            violations.push(GuardianViolation::performance(
                                "Inline style is too long (>150 chars). Extract to CSS class for better caching.",
                            ));
                        }
                    }

                    // Security: iFrames
                    if el.tag_name == "iframe" && !el.attributes.iter().any(|(k, _)| k == "sandbox")
                    {
                        violations.push(GuardianViolation::security(
                            "<iframe> detected without 'sandbox' attribute.",
                        ));
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
        assert!(issues.iter().any(|i| i.category == RuleCategory::A11y));
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
            .any(|i| i.category == RuleCategory::Performance));
    }

    #[test]
    fn test_guardian_security() {
        let nodes = vec![Node::Element(crate::ast::Element {
            tag_name: "iframe".to_string(),
            attributes: vec![("src".to_string(), "https://example.com".to_string())],
            children: vec![],
        })];
        let guardian = Guardian::new();
        let issues = guardian.validate(&nodes);
        assert!(issues.iter().any(|i| i.category == RuleCategory::Security));
        assert!(issues.iter().any(|i| i.is_error()));
    }

    #[test]
    fn test_violation_display() {
        let v = GuardianViolation::a11y("Test message");
        let display = format!("{}", v);
        assert!(display.contains("Accessibility"));
        assert!(display.contains("Test message"));
    }
}
