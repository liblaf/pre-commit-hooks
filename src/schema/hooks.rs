use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::log::LogResult;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Hook {
    pub id: String,
    #[serde(flatten)]
    extra: Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Hooks(pub Vec<Hook>);

impl Hooks {
    pub async fn load(path: &Path) -> anyhow::Result<Self> {
        let content = tokio::fs::read_to_string(path).await.log()?;
        let hooks = serde_yaml::from_str::<Hooks>(&content).log()?;
        Ok(hooks)
    }

    pub async fn save(&self, path: &Path) -> anyhow::Result<()> {
        let content = serde_yaml::to_string(self).log()?;
        let content = crate::proc::prettier::prettier_yaml(content.as_str()).await;
        tokio::fs::write(path, content.as_bytes()).await.log()?;
        Ok(())
    }
}
