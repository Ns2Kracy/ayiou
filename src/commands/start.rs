use crate::{commands::Command, server::run_server};

#[derive(clap::Parser, Debug, Default)]
pub struct Start {
    // addr or socket
    #[clap(short, long)]
    pub addr: Option<String>,
}

#[async_trait::async_trait]
impl Command for Start {
    async fn run(&self) -> anyhow::Result<()> {
        run_server().await?;
        Ok(())
    }
}
