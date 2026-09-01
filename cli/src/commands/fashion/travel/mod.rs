use crate::commands::Command;

mod get;
mod set;

#[derive(clap::Args, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

impl Args {
    pub(crate) fn command(&self) -> &dyn Command {
        match &self.command {
            Commands::Get(cmd) => cmd,
            Commands::Set(cmd) => cmd,
        }
    }
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Get the travel template of a fashion template.
    Get(get::Command),
    /// Set the travel template of a fashion template.
    Set(set::Command),
}
