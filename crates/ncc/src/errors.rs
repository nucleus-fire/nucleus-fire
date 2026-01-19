#![allow(unused_assignments)]
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
#[allow(unused)]
pub enum NucleusError {
    // ═══ Parser Errors ═══
    #[error("Failed to parse Nucleus file: {kind}")]
    #[diagnostic(
        code(nucleus::parser::syntax_error),
        help("Check your syntax ensuring all tags are closed properly")
    )]
    ParseError {
        #[source_code]
        #[allow(dead_code)]
        src: String,

        #[label("Here")]
        #[allow(dead_code)]
        span: SourceSpan,

        #[allow(dead_code)]
        kind: String,
    },

    // ═══ Component Errors ═══
    #[error("Component '{name}' is missing required prop '{prop}'")]
    #[diagnostic(
        code(nucleus::component::missing_prop),
        help("Add the required prop: <{name} {prop}=\"value\" />")
    )]
    MissingRequiredProp { name: String, prop: String },

    #[error("Unknown component '{name}'")]
    #[diagnostic(
        code(nucleus::component::unknown),
        help("Did you mean one of: Button, Card, Badge, FeatureCard? Or create the component at components/{name}.ncl")
    )]
    UnknownComponent { name: String },

    #[error(
        "Invalid prop type for '{prop}' in component '{component}': expected {expected}, got {got}"
    )]
    #[diagnostic(
        code(nucleus::component::invalid_prop_type),
        help("Change the prop value to match the expected type: {expected}")
    )]
    InvalidPropType {
        component: String,
        prop: String,
        expected: String,
        got: String,
    },

    #[error("Component '{name}' is not properly closed")]
    #[diagnostic(code(nucleus::component::unclosed), help("Add closing tag: </{name}>"))]
    UnclosedComponent { name: String },

    #[error("Invalid component name '{name}': components must use PascalCase")]
    #[diagnostic(
        code(nucleus::component::invalid_name),
        help("Rename to PascalCase, e.g., 'MyComponent' instead of '{name}'")
    )]
    InvalidComponentName { name: String },

    // ═══ Slot Errors ═══
    #[error("Named slot '{slot}' not found in component '{component}'")]
    #[diagnostic(
        code(nucleus::slot::not_found),
        help(
            "Available slots: default. Add <n:slot name=\"{slot}\" /> to the component definition."
        )
    )]
    SlotNotFound { component: String, slot: String },

    #[error("Multiple default slots in component '{component}'")]
    #[diagnostic(
        code(nucleus::slot::duplicate_default),
        help("Use named slots instead: <n:slot name=\"header\" /> and <n:slot name=\"footer\" />")
    )]
    DuplicateDefaultSlot { component: String },

    // ═══ Props Errors ═══
    #[error("Invalid props syntax in component '{component}'")]
    #[diagnostic(
        code(nucleus::props::invalid_syntax),
        help("Use format: <n:props>\n  propName: Type = \"default\"\n</n:props>")
    )]
    InvalidPropsSyntax { component: String },

    #[error("Duplicate prop '{prop}' in component '{component}'")]
    #[diagnostic(
        code(nucleus::props::duplicate),
        help("Remove the duplicate prop definition")
    )]
    DuplicateProp { component: String, prop: String },

    // ═══ Style Errors ═══
    #[error("Invalid scoped style in component '{component}'")]
    #[diagnostic(
        code(nucleus::style::invalid_scoped),
        help("Use: <style scoped>...</style>")
    )]
    InvalidScopedStyle { component: String },

    // ═══ Validation Errors ═══
    #[error("{0}")]
    #[diagnostic(
        code(nucleus::validation::error),
        help("Check the specific validation rule mentioned in the error message.")
    )]
    ValidationError(String),

    // ═══ I/O Errors ═══
    #[error("Failed to load component '{path}'")]
    #[diagnostic(
        code(nucleus::component::load_failed),
        help("Check that the file exists at: {path}")
    )]
    ComponentLoadError { path: String },

    // ═══ Circular Dependency ═══
    #[error("Circular dependency detected: {chain}")]
    #[diagnostic(
        code(nucleus::component::circular_dep),
        help("Break the cycle by refactoring one of the components")
    )]
    CircularDependency { chain: String },
}

impl NucleusError {
    /// Create a missing required prop error
    pub fn missing_prop(component: &str, prop: &str) -> Self {
        Self::MissingRequiredProp {
            name: component.to_string(),
            prop: prop.to_string(),
        }
    }

    /// Create an unknown component error
    pub fn unknown_component(name: &str) -> Self {
        Self::UnknownComponent {
            name: name.to_string(),
        }
    }

    /// Create an invalid prop type error
    pub fn invalid_prop_type(component: &str, prop: &str, expected: &str, got: &str) -> Self {
        Self::InvalidPropType {
            component: component.to_string(),
            prop: prop.to_string(),
            expected: expected.to_string(),
            got: got.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_prop_error() {
        let err = NucleusError::missing_prop("Button", "variant");
        assert!(err.to_string().contains("Button"));
        assert!(err.to_string().contains("variant"));
    }

    #[test]
    fn test_unknown_component_error() {
        let err = NucleusError::unknown_component("MyButton");
        assert!(err.to_string().contains("MyButton"));
    }

    #[test]
    fn test_invalid_prop_type_error() {
        let err = NucleusError::invalid_prop_type("Card", "count", "i32", "String");
        assert!(err.to_string().contains("Card"));
        assert!(err.to_string().contains("count"));
        assert!(err.to_string().contains("i32"));
    }
}
