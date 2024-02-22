use std::{fs::File, path::PathBuf};

use clap::Args;

use crate::{log::LogResult, schema::hooks::Hooks};

#[derive(Debug, Args)]
pub struct Cmd {
    #[arg(default_value = ".pre-commit-hooks.yaml")]
    hooks: PathBuf,
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        let mut hooks = Hooks::load(
            File::options()
                .read(true)
                .open(self.hooks.as_path())
                .log()?,
        )?;
        hooks
            .0
            .sort_unstable_by(|a, b| a.id.as_str().cmp(b.id.as_str()));
        hooks
            .save(
                &mut File::options()
                    .write(true)
                    .open(self.hooks.as_path())
                    .log()?,
            )
            .await?;
        Ok(())
    }
}
