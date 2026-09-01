use async_trait::async_trait;

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

#[async_trait]
impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-wardrobe-set"
    }

    #[tracing::instrument(name = "fashion-wardrobe-set", skip_all)]
    async fn execute(&self) -> anyhow::Result<()> {
        todo!()
    }
}
