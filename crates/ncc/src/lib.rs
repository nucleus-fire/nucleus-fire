pub mod ast;
pub mod codegen;
pub mod css;
pub mod dax;
pub mod db;
pub mod diagnostics;
pub mod errors;
pub mod guardian;
pub mod nir;
pub mod parser;
pub mod rosetta;

#[cfg(test)]
mod kitchen_sink_test;
#[cfg(test)]
mod parser_edge_tests;
#[cfg(test)]
mod spec_compliance_test;
#[cfg(test)]
mod tests_action;

pub use codegen::{
    find_action_recursive, generate_action_handler_fn, generate_model, generate_nodes_handler_body,
    generate_view_handler_fn, generate_wasm_footer, generate_wasm_header, render_html,
};
pub use parser::{parse_code, parse_node, parse_root};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Server,
    Client,
}
