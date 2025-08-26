use clap::{Parser, Subcommand};

use crate::commands::{self, Command};

#[derive(Parser, Debug)]
#[command(
    name = "ayiou",
    author,
    version,
    long_about = None,
    propagate_version = true,
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Start(commands::start::Start),
}

pub async fn run() -> anyhow::Result<()> {
    let cmd = Cli::parse();

    match cmd.command {
        Commands::Start(cmd) => cmd.run().await,
    }
}
