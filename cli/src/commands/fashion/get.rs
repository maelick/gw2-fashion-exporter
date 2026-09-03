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

impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-get"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-get", skip_all)]
    pub async fn execute(&self) -> anyhow::Result<()> {
        todo!()
    }
}
