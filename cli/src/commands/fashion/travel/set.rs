use async_trait::async_trait;

use crate::commands;

#[derive(clap::Args, Debug)]
pub struct Command {
}

#[async_trait]
impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-travel-set"
    }

    #[tracing::instrument(name = "fashion-travel-set", skip_all)]
    async fn execute(&self) -> anyhow::Result<()> {
        todo!()
    }
}
