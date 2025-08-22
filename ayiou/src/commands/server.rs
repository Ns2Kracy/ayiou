use crate::{commands::Command, server::run_server};

#[derive(clap::Parser, Debug, Default)]
pub struct Server {
    // addr or socket
    #[clap(short, long)]
    pub addr: Option<String>,

    // socket
    #[clap(short, long)]
    pub socket: Option<String>,
}

impl Command for Server {
    async fn run(&self) -> anyhow::Result<()> {
        run_server().await?;
        Ok(())
    }
}
