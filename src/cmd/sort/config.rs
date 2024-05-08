use std::path::PathBuf;

use clap::Args;

use crate::schema::config::Config;

#[derive(Debug, Args)]
pub struct Cmd {
    #[arg(default_value = ".pre-commit-config.yaml")]
    config: PathBuf,
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        let mut cfg = Config::load(&self.config).await?;
        cfg.ci.skip.sort();
        for repo in cfg.repos.iter_mut() {
            repo.hooks.sort_by(|a, b| a.id.cmp(&b.id));
        }
        cfg.repos.sort_by(|a, b| a.repo.cmp(&b.repo));
        cfg.save(&self.config).await?;
        Ok(())
    }
}
