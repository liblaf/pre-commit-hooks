use std::path::PathBuf;

use clap::Args;

use crate::schema::hooks::Hooks;

#[derive(Debug, Args)]
pub struct Cmd {
    #[arg(default_value = ".pre-commit-hooks.yaml")]
    hooks: PathBuf,
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        let mut hooks = Hooks::load(&self.hooks).await?;
        hooks.0.sort_unstable_by(|a, b| a.id.cmp(&b.id));
        hooks.save(&self.hooks).await?;
        Ok(())
    }
}
