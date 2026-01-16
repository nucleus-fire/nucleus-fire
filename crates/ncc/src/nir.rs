use crate::ast::Node;
use serde::Serialize;
// use serde_json::Value;

#[derive(Serialize)]
pub struct NirPackage {
    pub version: String,
    pub nodes: Vec<Node>,
}

pub fn to_nir(nodes: &[Node]) -> String {
    let package = NirPackage {
        version: "1.0.0".to_string(),
        nodes: nodes.to_vec(),
    };
    serde_json::to_string_pretty(&package).unwrap_or_else(|_| "{}".to_string())
}
