use async_trait::async_trait;

use crate::commands::{self, args::DataFormat};

#[derive(clap::Args, Debug)]
pub struct Command {
    /// Tags to list fashion templates for.
    #[arg(value_name = "TAG")]
    tags: Vec<String>,

    #[arg(from_global)]
    clipboard: bool,

    /// Output format. Auto is based on whether stdout is a TTY (CSV for TTY, JSON if not).
    #[arg(short, long, value_enum, default_value_t = DataFormat::Auto)]
    format: DataFormat,
}

#[async_trait]
impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-list"
    }

    #[tracing::instrument(name = "fashion-list", skip_all)]
    async fn execute(&self) -> anyhow::Result<()> {
        todo!()
    }
}
