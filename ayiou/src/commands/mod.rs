pub mod server;

pub trait Command {
    async fn run(&self) -> anyhow::Result<()>;
}
