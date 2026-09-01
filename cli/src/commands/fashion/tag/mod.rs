use crate::commands::Command;

mod add;
mod clean;
mod list;
mod remove;

#[derive(clap::Args, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

impl Args {
    pub(crate) fn command(&self) -> &dyn Command {
        match &self.command {
            Commands::List(cmd) => cmd,
            Commands::Clean(cmd) => cmd,
            Commands::Add(cmd) => cmd,
            Commands::Remove(cmd) => cmd,
        }
    }
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// List existing tags.
    List(list::Command),
    /// Remove unused tags.
    Clean(clean::Command),
    /// Add a tag to fashion template(s).
    Add(add::Command),
    /// Remove a tag to fashion template(s).
    Remove(remove::Command),
}
