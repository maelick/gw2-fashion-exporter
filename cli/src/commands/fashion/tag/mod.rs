use crate::{commands::Command, environment::Environment};

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

    pub async fn execute(&self, env: Environment) -> anyhow::Result<()> {
        match &self.command {
            Commands::List(cmd) => cmd.execute(env).await,
            Commands::Clean(cmd) => cmd.execute(env).await,
            Commands::Add(cmd) => cmd.execute(env).await,
            Commands::Remove(cmd) => cmd.execute(env).await,
        }
    }
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// List existing tags.
    #[command(visible_alias = "ls")]
    List(list::Command),
    /// Remove unused tags.
    Clean(clean::Command),
    /// Add a tag to fashion template(s).
    Add(add::Command),
    /// Remove a tag to fashion template(s).
    Remove(remove::Command),
}
