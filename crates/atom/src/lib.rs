#[cfg(feature = "hot-reload")]
pub mod hot_swap;
pub mod memory;
pub mod middleware;
pub mod runtime;

pub use runtime::NucleusRuntime;

use std::collections::HashMap;

use nucleus_std::stream::StreamHandler;
use std::sync::Arc;

pub async fn start_reactor(
    routes: Option<HashMap<String, String>>,
    handler: Option<Arc<dyn StreamHandler>>,
) {
    NucleusRuntime::start(routes, handler).await;
}

#[cfg(test)]
mod tests;
