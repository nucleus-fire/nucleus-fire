pub mod errors;
pub mod ast;
pub mod parser;
pub mod codegen;
pub mod dax;
pub mod db;
pub mod css;
pub mod guardian;
pub mod diagnostics;
pub mod nir;
pub mod rosetta;

#[cfg(test)]
mod spec_compliance_test;
#[cfg(test)]
mod kitchen_sink_test;
#[cfg(test)]
mod parser_edge_tests;
#[cfg(test)]
mod tests_action;


pub use parser::{parse_root, parse_node, parse_code};
pub use codegen::{render_html, generate_view_handler_fn, generate_action_handler_fn, generate_model, generate_wasm_header, generate_wasm_footer, generate_nodes_handler_body, find_action_recursive};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Server,
    Client
}
