use crate::commands::{self, args::ChatLinkFormat, fashion::args::FashionIdentifier};

#[derive(clap::Args, Debug)]
#[command(mut_group("identifier", |g| g.required(true)))]
pub struct Command {
    #[arg(from_global)]
    clipboard: bool,

    #[command(flatten)]
    id: FashionIdentifier,

    /// Input format.
    #[arg(short, long, value_enum, default_value_t = ChatLinkFormat::Auto)]
    format: ChatLinkFormat,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-travel-set"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-travel-set", skip_all)]
    pub async fn execute(&self) -> anyhow::Result<()> {
        todo!()
    }
}
