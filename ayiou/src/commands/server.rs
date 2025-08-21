// a command to manage the server, now only start and stop
use crate::{commands::Command, server::run_server};

#[derive(clap::Parser, Debug, Default)]
pub struct Server {}

impl Command for Server {
    async fn run(&self) -> anyhow::Result<()> {
        run_server().await?;
        Ok(())
    }
}
