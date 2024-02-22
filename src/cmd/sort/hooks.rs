use std::path::PathBuf;

use clap::Args;

use crate::schema::hooks::Hooks;

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
        let mut hooks = Hooks::load(self.hooks.as_path()).await?;
        hooks
            .0
            .sort_unstable_by(|a, b| a.id.as_str().cmp(b.id.as_str()));
        hooks.save(self.hooks.as_path()).await?;
        Ok(())
    }
}
