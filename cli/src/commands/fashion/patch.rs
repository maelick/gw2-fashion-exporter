use crate::commands::{self, args::DataFormat, fashion::args::FashionFields};

#[derive(clap::Args, Debug)]
pub struct Command {
    #[arg(from_global)]
    clipboard: bool,

    /// Id of the fashion template.
    #[arg(long, value_name = "UUID")]
    id: Option<String>,

    #[command(flatten)]
    data: FashionFields,

    /// Input and output format. Auto is based on whether stdin and stdout are TTYs (CSV for TTY, JSON if not).
    #[arg(short, long, value_enum, default_value_t = DataFormat::Auto)]
    format: DataFormat,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-patch"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-patch", skip_all)]
    pub async fn execute(&self) -> anyhow::Result<()> {
        todo!()
    }
}
