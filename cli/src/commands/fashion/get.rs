use async_trait::async_trait;

use crate::commands::{self, args::DataFormat, fashion::args::FashionIdentifier};

#[derive(clap::Args, Debug)]
#[command(mut_group("identifier", |g| g.required(true)))]
pub struct Command {
    #[arg(from_global)]
    clipboard: bool,

    #[command(flatten)]
    id: FashionIdentifier,

    /// Output format. Auto is based on whether stdout is a TTY (CSV for TTY, JSON if not).
    #[arg(short, long, value_enum, default_value_t = DataFormat::Auto)]
    format: DataFormat,
}

#[async_trait]
impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-get"
    }

    #[tracing::instrument(name = "fashion-get", skip_all)]
    async fn execute(&self) -> anyhow::Result<()> {
        todo!()
    }
}
