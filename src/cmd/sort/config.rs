use std::path::PathBuf;

use clap::Args;

use crate::{log::LogResult, schema::config::Config};

#[derive(Debug, Args)]
pub struct Cmd {
    #[arg(default_value = ".pre-commit-config.yaml")]
    config: PathBuf,
    #[arg(from_global)]
    dry_run: bool,
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        if self.dry_run {
            todo!()
        }
        let mut cfg = serde_yaml::from_str::<Config>(
            tokio::fs::read_to_string(self.config.as_path())
                .await?
                .as_str(),
        )
        .log()?;
        cfg.ci.skip.sort();
        for repo in cfg.repos.iter_mut() {
            repo.hooks.sort_unstable_by(|a, b| a.id.cmp(&b.id));
        }
        cfg.repos.sort_by(|a, b| a.repo.cmp(&b.repo));
        tokio::fs::write(
            self.config.as_path(),
            crate::proc::prettier::prettier_yaml(serde_yaml::to_string(&cfg).log()?.as_str())
                .await
                .as_bytes(),
        )
        .await
        .log()?;
        Ok(())
    }
}
