pub mod server;
pub mod version;

pub trait Command {
    async fn run(&self) -> anyhow::Result<()>;
}
