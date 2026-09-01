use async_trait::async_trait;

use crate::commands::{self, args::DataFormat, fashion::args::FashionFields};

#[derive(clap::Args, Debug)]
pub struct Command {
    #[arg(from_global)]
    clipboard: bool,

    #[command(flatten)]
    data: FashionFields,

    /// Input and output format. Auto is based on whether stdin and stdout are TTYs (CSV for TTY, JSON if not).
    #[arg(short, long, value_enum, default_value_t = DataFormat::Auto)]
    format: DataFormat,
}

#[async_trait]
impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-create"
    }

    #[tracing::instrument(name = "fashion-create", skip_all)]
    async fn execute(&self) -> anyhow::Result<()> {
        todo!()
    }
}
