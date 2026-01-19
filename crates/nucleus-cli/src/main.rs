use mimalloc::MiMalloc;
use nucleus_cli::run_cli;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> miette::Result<()> {
    run_cli().await
}
