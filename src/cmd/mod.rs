use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

use crate::log::{DefaultLevel, LogInit};

mod sort;

#[derive(Debug, Parser)]
#[command(name = "pch", version, author, about)]
pub struct Cmd {
    #[command(subcommand)]
    cmd: SubCmd,
    #[arg(short, long, env, global(true))]
    dry_run: bool,
    #[command(flatten)]
    verbose: Verbosity<DefaultLevel>,
}

#[derive(Debug, Subcommand)]
enum SubCmd {
    Sort(sort::Cmd),
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        self.verbose.init();
        std::env::set_var("DRY_RUN", self.dry_run.to_string());
        match &self.cmd {
            SubCmd::Sort(cmd) => cmd.run().await,
        }
    }
}
