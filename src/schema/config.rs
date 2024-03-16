use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

// https://pre-commit.com/#pre-commit-configyaml---top-level
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub ci: CI,
    pub repos: Vec<Repo>,
    #[serde(flatten)]
    extra: Value,
}

// https://pre-commit.ci/#configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CI {
    #[serde(flatten)]
    extra: Value,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skip: Vec<String>,
}

// https://pre-commit.com/#pre-commit-configyaml---repos
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Repo {
    pub repo: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rev: Option<String>,
    pub hooks: Vec<Hook>,
}

// https://pre-commit.com/#pre-commit-configyaml---hooks
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Hook {
    pub id: String,
    #[serde(flatten)]
    extra: Value,
}

impl Config {
    #[tracing::instrument(err)]
    pub async fn load(path: &Path) -> anyhow::Result<Self> {
        let contents = tokio::fs::read_to_string(path).await?;
        let config = serde_yaml::from_str(contents.as_str())?;
        Ok(config)
    }

    #[tracing::instrument(err)]
    pub async fn save(&self, path: &Path) -> anyhow::Result<()> {
        let contents = serde_yaml::to_string(self)?;
        let contents = crate::proc::prettier::prettier_yaml(contents.as_str()).await;
        tokio::fs::write(path, contents.as_bytes()).await?;
        Ok(())
    }
}
