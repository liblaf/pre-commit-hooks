use std::path::PathBuf;

use clap::Args;
use octocrab::{Octocrab, OctocrabBuilder};
use regex::Regex;
use semver::Version;

use crate::schema::config::{Config, Repo};

#[derive(Debug, Args)]
pub struct Cmd {
    #[arg(default_value = ".pre-commit-config.yaml")]
    config: PathBuf,
    #[arg(short, long, env = "GH_TOKEN")]
    token: Option<String>,
}

impl Cmd {
    #[tracing::instrument(skip_all, err(Debug))]
    pub async fn run(&self) -> anyhow::Result<()> {
        let mut cfg = Config::load(self.config.as_path()).await?;
        let mut client = OctocrabBuilder::new();
        if let Some(token) = self.token.as_deref() {
            client = client.personal_token(token.to_string());
        }
        let client = client.build()?;
        futures::future::join_all(
            cfg.repos
                .iter_mut()
                .map(|r| update_repo(&client, r))
                .collect::<Vec<_>>(),
        )
        .await;
        cfg.save(self.config.as_path()).await?;
        Ok(())
    }
}

async fn update_repo(client: &Octocrab, repo: &mut Repo) {
    if let Err(err) = update_repo_unsafe(client, repo).await {
        tracing::error!("[{}]\n{}", repo.repo, err);
    }
}

#[tracing::instrument(skip_all, err(Debug))]
async fn update_repo_unsafe(client: &Octocrab, repo: &mut Repo) -> anyhow::Result<()> {
    let url_pattern: Regex =
        Regex::new(r"https://github.com/(?<owner>[^/]+)/(?<repo>[^/]+)").unwrap();
    let captures = match url_pattern.captures(repo.repo.as_str()) {
        Some(captures) => captures,
        None => return Ok(()),
    };
    let owner = captures.name("owner").unwrap().as_str();
    let repo_name = captures.name("repo").unwrap().as_str();
    let new_rev = if let Ok(tag) = get_latest_release(client, owner, repo_name).await {
        tag.to_string()
    } else {
        get_latest_tag(client, owner, repo_name).await?
    };
    let old_rev = repo.rev.as_deref().unwrap_or_default();
    if old_rev == new_rev {
        tracing::info!("[{}] already up to date!", repo.repo);
    } else {
        tracing::info!("[{}] updating {} -> {}", repo.repo, old_rev, new_rev);
        repo.rev = Some(new_rev);
    }
    Ok(())
}

#[tracing::instrument(skip_all, err(Debug))]
async fn get_latest_release(client: &Octocrab, owner: &str, repo: &str) -> anyhow::Result<String> {
    match client.repos(owner, repo).releases().get_latest().await {
        Ok(release) => Ok(release.tag_name),
        Err(err) => {
            tracing::warn!(
                "[https://github.com/{}/{}] latest release not found",
                owner,
                repo
            );
            Err(err.into())
        }
    }
}

#[tracing::instrument(skip_all, err(Debug))]
async fn get_latest_tag(client: &Octocrab, owner: &str, repo: &str) -> anyhow::Result<String> {
    let tags = client.repos(owner, repo).list_tags().send().await?;
    let tags = tags.into_iter().map(|t| t.name).collect::<Vec<_>>();
    let parse = |tag: &str| tag.strip_prefix('v').unwrap_or(tag).parse::<Version>();
    if let Some(tag) = tags
        .iter()
        .filter(|t| {
            if let Ok(version) = parse(t) {
                version.pre.is_empty()
            } else {
                false
            }
        })
        .max_by_key(|t| parse(t).unwrap())
    {
        Ok(tag.to_string())
    } else if let Some(tag) = tags
        .iter()
        .filter(|t| parse(t).is_ok())
        .max_by_key(|t| parse(t).unwrap())
    {
        tracing::warn!(
            "[https://github.com/{}/{}] stable tags not found",
            owner,
            repo
        );
        Ok(tag.to_string())
    } else {
        tracing::warn!(
            "[https://github.com/{}/{}] semver tags not found",
            owner,
            repo
        );
        Ok(tags.first().unwrap().to_string())
    }
}
