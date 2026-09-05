use crate::{
    commands::{self, args::DataFormat},
    environment::Environment,
};

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

impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-tag-list"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-tag-list", skip_all)]
    pub async fn execute(&self, _env: Environment) -> anyhow::Result<()> {
        todo!()
    }
}
