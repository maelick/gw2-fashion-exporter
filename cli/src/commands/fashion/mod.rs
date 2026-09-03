use crate::commands::Command;

mod args;
mod create;
mod get;
mod list;
mod patch;
mod set;
mod tag;
mod travel;
mod wardrobe;

#[derive(clap::Args, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Read or write data from clipboard.
    /// If any other data is also provided on stdin or as arguments, it overrides the data from the clipboard.
    #[arg(long, global = true)]
    clipboard: bool,
}

impl Args {
    pub(crate) fn command(&self) -> &dyn Command {
        match &self.command {
            Commands::Create(cmd) => cmd,
            Commands::Get(cmd) => cmd,
            Commands::Set(cmd) => cmd,
            Commands::Patch(cmd) => cmd,
            Commands::List(cmd) => cmd,
            Commands::Wardrobe(args) => args.command(),
            Commands::Travel(args) => args.command(),
            Commands::Tag(args) => args.command(),
        }
    }

    pub async fn execute(&self) -> anyhow::Result<()> {
        match &self.command {
            Commands::Create(cmd) => cmd.execute().await,
            Commands::Get(cmd) => cmd.execute().await,
            Commands::Set(cmd) => cmd.execute().await,
            Commands::Patch(cmd) => cmd.execute().await,
            Commands::List(cmd) => cmd.execute().await,
            Commands::Wardrobe(args) => args.execute().await,
            Commands::Travel(args) => args.execute().await,
            Commands::Tag(args) => args.execute().await,
        }
    }
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Create a fashion template.
    Create(create::Command),
    /// Get an existing fashion template.
    Get(get::Command),
    /// Set an existing fashion template, overriding all fields with those provided and removing the ones not set.
    Set(set::Command),
    /// Patch an existing fashion, overriding only provided fields.
    Patch(patch::Command),
    /// List existing fashion templates.
    #[command(visible_alias = "ls")]
    List(list::Command),
    /// Get or set the wardrobe template of a fashion template.
    Wardrobe(wardrobe::Args),
    /// Get or set the travel template of a fashion template.
    Travel(travel::Args),
    /// Manage fashion template tags
    #[command(visible_alias = "tags")]
    Tag(tag::Args),
}
