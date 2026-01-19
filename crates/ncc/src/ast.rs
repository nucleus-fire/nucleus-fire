use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub fields: Vec<(String, String)>,
    pub methods: Vec<String>,
    pub attributes: Vec<String>,
}

/// Component property definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prop {
    pub name: String,
    pub prop_type: String,
    pub default: Option<String>,
    pub required: bool,
}

/// Component definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub props: Vec<Prop>,
    pub children: Vec<Node>,
    pub styles: Option<String>,
    pub scoped: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Node {
    Element(Element),
    Script {
        content: String,
        attributes: Vec<(String, String)>,
    }, // Script with attrs (e.g. lang="ts")
    Style(String), // Raw style content
    ScopedStyle {
        content: String,
        scope_id: String,
    }, // Scoped CSS with unique identifier
    Spec(String),  // Unit tests
    Test(String),  // Integration tests
    Model(Model),
    Client(String),        // <n:client> Rust code
    Interpolation(String), // {{ expression }}
    For {
        variable: String,
        iterable: String,
        children: Vec<Node>,
    },
    If {
        condition: String,
        children: Vec<Node>,
    },
    Include {
        path: String,
        attributes: Vec<(String, String)>,
    },
    Outlet, // <n:outlet /> for Nested Layouts
    Slot {
        name: Option<String>, // None = default slot
    },
    Island {
        path: String,
        directive: String,                 // e.g. "load", "visible", "idle"
        attributes: Vec<(String, String)>, // other props
    },
    Component(Component), // Component definition
    ComponentUse {
        name: String,
        props: Vec<(String, String)>,
        children: Vec<Node>, // For slot content
    },
    Loader(String), // <n:loader> Rust code (GET)
    Action(String), // <n:action> Rust code (POST)
    Text(String),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Element {
    pub tag_name: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn is_script(&self) -> bool {
        matches!(self, Node::Script { .. })
    }

    pub fn is_component(&self) -> bool {
        matches!(self, Node::Component(_))
    }
}

impl Component {
    /// Generate a unique scope ID for CSS isolation
    pub fn scope_id(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.name.hash(&mut hasher);
        format!("nc{:x}", hasher.finish() & 0xFFFFFF)
    }
}
