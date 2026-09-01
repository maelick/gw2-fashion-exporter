use async_trait::async_trait;

use crate::commands;

#[derive(clap::Args, Debug)]
pub struct Command {
}

#[async_trait]
impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-patch"
    }

    #[tracing::instrument(name = "fashion-patch", skip_all)]
    async fn execute(&self) -> anyhow::Result<()> {
        todo!()
    }
}
