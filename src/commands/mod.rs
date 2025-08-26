pub mod start;

#[async_trait::async_trait]
pub trait Command {
    async fn run(&self) -> anyhow::Result<()>;
}
