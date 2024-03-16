use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

use crate::log::{DefaultLevel, LogInit};

mod filter;
mod sort;
mod update;

#[derive(Debug, Parser)]
#[command(name = "pch", version, author, about)]
pub struct Cmd {
    #[command(subcommand)]
    cmd: SubCmd,
    #[command(flatten)]
    verbose: Verbosity<DefaultLevel>,
}

#[derive(Debug, Subcommand)]
enum SubCmd {
    Filter(filter::Cmd),
    Sort(sort::Cmd),
    Update(update::Cmd),
}

impl Cmd {
    #[tracing::instrument(err)]
    pub async fn run(&self) -> anyhow::Result<()> {
        self.verbose.init();
        match &self.cmd {
            SubCmd::Filter(cmd) => cmd.run().await,
            SubCmd::Sort(cmd) => cmd.run().await,
            SubCmd::Update(cmd) => cmd.run().await,
        }
    }
}
