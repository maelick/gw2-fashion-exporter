use crate::{
    commands::{self, args::DataFormat, fashion::args::FashionFields},
    environment::Environment,
};

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

impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-create"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-create", skip_all)]
    pub async fn execute(&self, mut env: Environment) -> anyhow::Result<()> {
        let service = env.fashion_service().await?;
        let fashion = (&self.data).try_into()?;
        service.create(&fashion).await?;
        Ok(())
    }
}
