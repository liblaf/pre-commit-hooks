use std::path::PathBuf;

use clap::Args;

use crate::{log::LogResult, schema::hooks::Hook};

#[derive(Debug, Args)]
pub struct Cmd {
    #[arg(default_value = ".pre-commit-hooks.yaml")]
    hooks: PathBuf,
    #[arg(from_global)]
    dry_run: bool,
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        if self.dry_run {
            todo!()
        }
        let mut hooks = serde_yaml::from_str::<Vec<Hook>>(
            tokio::fs::read_to_string(self.hooks.as_path())
                .await?
                .as_str(),
        )?;
        hooks.sort_unstable_by(|a, b| a.id.cmp(&b.id));
        tokio::fs::write(
            self.hooks.as_path(),
            crate::proc::prettier::prettier_yaml(
                serde_yaml::to_string(hooks.as_slice()).log()?.as_str(),
            )
            .await
            .as_bytes(),
        )
        .await
        .log()?;
        Ok(())
    }
}
