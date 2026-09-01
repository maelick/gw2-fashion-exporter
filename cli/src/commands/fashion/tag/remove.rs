use async_trait::async_trait;

use crate::commands;

#[derive(clap::Args, Debug)]
pub struct Command {
}

#[async_trait]
impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-tag-remove"
    }

    #[tracing::instrument(name = "fashion-tag-remove", skip_all)]
    async fn execute(&self) -> anyhow::Result<()> {
        todo!()
    }
}
