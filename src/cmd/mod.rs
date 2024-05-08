use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use cli::log::{DefaultLevel, LogInit};

mod filter;
mod sort;

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
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        self.verbose.init();
        match &self.cmd {
            SubCmd::Filter(cmd) => cmd.run().await,
            SubCmd::Sort(cmd) => cmd.run().await,
        }
    }
}
