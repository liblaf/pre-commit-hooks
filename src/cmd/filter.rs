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
    #[arg(short, long, default_value = ".pre-commit-config.yaml")]
    output: PathBuf,
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        let hooks = active_hooks(self.config.as_path()).await?;
        let mut cfg = Config::load(self.config.as_path()).await?;
        cfg.ci.skip.retain(|s| hooks.contains(s.as_str()));
        cfg.repos.retain_mut(|r| {
            r.hooks.retain(|h| hooks.contains(h.id.as_str()));
            !r.hooks.is_empty()
        });
        cfg.save(self.output.as_path()).await?;
        Ok(())
    }
}

#[derive(Debug)]
enum Status {
    Failed,
    Passed,
    Skipped,
}

async fn active_hooks(cfg: &Path) -> anyhow::Result<HashSet<String>> {
    let mut cmd = Command::new("pre-commit");
    cmd.args([
        "run",
        "--color",
        "never",
        "--config",
        cfg.to_str().unwrap(),
        "--verbose",
        "--all-files",
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
    let mut status = Status::Failed;
    let mut hooks = HashSet::from(["commitizen".to_string()]);
    for line in String::from_utf8_lossy(output.stdout.as_slice()).lines() {
        if line.len() == 79 {
            if line.ends_with("Failed") {
                status = Status::Failed;
            } else if line.ends_with("Passed") {
                status = Status::Passed;
            } else if line.ends_with("Skipped") {
                status = Status::Skipped;
            }
        } else if let Some(id) = line.strip_prefix("- hook id: ") {
            tracing::debug!("{} {:?}", id, status);
            match status {
                Status::Failed | Status::Passed => {
                    hooks.insert(id.to_string());
                }
                Status::Skipped => {}
            }
        }
    }
    Ok(hooks)
}
