use clap::{Args, Subcommand};

mod config;
mod hooks;

#[derive(Debug, Args)]
pub struct Cmd {
    #[command(subcommand)]
    cmd: SubCmd,
}

#[derive(Debug, Subcommand)]
enum SubCmd {
    Config(config::Cmd),
    Hooks(hooks::Cmd),
}

impl Cmd {
    #[tracing::instrument(skip_all, err(Debug))]
    pub async fn run(&self) -> anyhow::Result<()> {
        match &self.cmd {
            SubCmd::Config(cmd) => cmd.run().await,
            SubCmd::Hooks(cmd) => cmd.run().await,
        }
    }
}
