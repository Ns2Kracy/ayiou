use mimalloc::MiMalloc;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod cli;
mod commands;
mod server;
mod shutdown;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry().with(fmt::layer()).try_init()?;

    cli::run().await
}
