use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    process::Stdio,
};

use clap::Args;
use tokio::{io::AsyncWriteExt, process::Command};

use crate::{log::LogResult, schema::config::Config};

#[derive(Debug, Args)]
pub struct Cmd {
    #[arg(default_value = ".pre-commit-config.yaml")]
    config: PathBuf,
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        let unapplied = unapplied_hooks(self.config.as_path()).await?;
        let mut cfg = Config::load(self.config.as_path()).await?;
        cfg.ci.skip.retain(|s| !unapplied.contains(s.as_str()));
        cfg.repos.retain_mut(|r| {
            r.hooks.retain(|h| !unapplied.contains(h.id.as_str()));
            !r.hooks.is_empty()
        });
        cfg.save(self.config.as_path()).await?;
        Ok(())
    }
}

async fn unapplied_hooks(cfg: &Path) -> anyhow::Result<HashSet<String>> {
    let mut cmd = Command::new("pre-commit");
    cmd.args([
        "run",
        "--color",
        "never",
        "--config",
        cfg.to_str().unwrap(),
        "--verbose",
        "--all-files",
        "check-hooks-apply",
    ])
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::inherit());
    tracing::debug!(?cmd);
    let child = cmd.spawn().log()?;
    let output = child.wait_with_output().await.log()?;
    tracing::debug!(%output.status);
    tokio::io::stdout()
        .write_all(output.stdout.as_slice())
        .await
        .log()?;
    let mut unapplied_hooks: HashSet<String> = HashSet::from(["check-hooks-apply".to_string()]);
    for line in String::from_utf8(output.stdout).log()?.lines() {
        if let Some(hook) = line.strip_suffix(" does not apply to this repository") {
            unapplied_hooks.insert(hook.to_string());
        }
    }
    Ok(unapplied_hooks)
}
