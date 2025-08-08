use crate::commands::Command;

#[derive(clap::Parser, Debug, Default)]
pub struct Version {}

impl Command for Version {
    async fn run(&self) -> anyhow::Result<()> {
        println!("{}", env!("CARGO_PKG_VERSION"));
        Ok(())
    }
}
