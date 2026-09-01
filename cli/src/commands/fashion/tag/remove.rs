use async_trait::async_trait;

use crate::commands::{self, args::DataFormat, fashion::args::FashionIdentifier};

#[derive(clap::Args, Debug)]
pub struct Command {
    /// Tags to remove.
    #[arg(value_name = "TAG", required = true)]
    tags: Vec<String>,

    #[arg(from_global)]
    clipboard: bool,

    #[command(flatten)]
    id: Option<FashionIdentifier>,

    /// Input format. Auto is based on whether stdin is a TTY (CSV for TTY, JSON if not).
    #[arg(short, long, value_enum, default_value_t = DataFormat::Auto)]
    format: DataFormat,
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
