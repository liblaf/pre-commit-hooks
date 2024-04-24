use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use cli::{
    color::ColorInit,
    log::{DefaultLevel, LogInit},
};

mod filter;
mod sort;

#[derive(Debug, Parser)]
#[command(name = "pch", version, author, about)]
pub struct Cmd {
    #[command(subcommand)]
    cmd: SubCmd,
    #[command(flatten)]
    color: concolor_clap::Color,
    #[command(flatten)]
    verbose: Verbosity<DefaultLevel>,
}

#[derive(Debug, Subcommand)]
enum SubCmd {
    Filter(filter::Cmd),
    Sort(sort::Cmd),
}

impl Cmd {
    #[tracing::instrument(skip_all, err(Debug))]
    pub async fn run(&self) -> anyhow::Result<()> {
        self.color.init();
        self.verbose.init();
        match &self.cmd {
            SubCmd::Filter(cmd) => cmd.run().await,
            SubCmd::Sort(cmd) => cmd.run().await,
        }
    }
}
