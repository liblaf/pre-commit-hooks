use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

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
        let contents = tokio::fs::read(path).await?;
        let hooks = serde_yaml::from_slice(&contents)?;
        Ok(hooks)
    }

    pub async fn save(&self, path: &Path) -> anyhow::Result<()> {
        let contents = serde_yaml::to_string(self)?;
        let contents = crate::proc::prettier::prettier_yaml(contents.as_str()).await;
        std::fs::write(path, contents.as_bytes())?;
        Ok(())
    }
}
