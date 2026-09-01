use async_trait::async_trait;

use crate::commands::{self, args::DataFormat};

#[derive(clap::Args, Debug)]
pub struct Command {
    #[arg(from_global)]
    clipboard: bool,

    /// Output format. Auto is based on whether stdout is a TTY (CSV for TTY, JSON if not).
    #[arg(short, long, value_enum, default_value_t = DataFormat::Auto)]
    format: DataFormat,

    /// Prefix to filter tags.
    #[arg(long)]
    prefix: Option<String>,

    /// Suffix to filter tags.
    #[arg(long)]
    suffix: Option<String>,

    /// Infix to filter tags.
    #[arg(long)]
    contains: Vec<String>,
}

#[async_trait]
impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-tag-list"
    }

    #[tracing::instrument(name = "fashion-tag-list", skip_all)]
    async fn execute(&self) -> anyhow::Result<()> {
        todo!()
    }
}
