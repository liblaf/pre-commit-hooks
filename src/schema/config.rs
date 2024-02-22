use std::io::{Read, Write};

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::log::LogResult;

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
    pub rev: String,
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
    pub fn load<R>(reader: R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let config = serde_yaml::from_reader(reader).log()?;
        Ok(config)
    }

    pub async fn save<W>(&self, writer: &mut W) -> anyhow::Result<()>
    where
        W: Write,
    {
        let content = serde_yaml::to_string(self).log()?;
        let content = crate::proc::prettier::prettier_yaml(content.as_str()).await;
        writer.write_all(content.as_bytes()).log()?;
        Ok(())
    }
}
