use std::io::{Read, Write};

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
    #[tracing::instrument(skip_all, err)]
    pub fn load<R>(reader: R) -> anyhow::Result<Self>
    where
        R: Read + std::fmt::Debug,
    {
        let hooks = serde_yaml::from_reader::<_, Hooks>(reader)?;
        Ok(hooks)
    }

    #[tracing::instrument(skip_all, err)]
    pub async fn save<W>(&self, writer: &mut W) -> anyhow::Result<()>
    where
        W: Write + std::fmt::Debug,
    {
        let contents = serde_yaml::to_string(self)?;
        let contents = crate::proc::prettier::prettier_yaml(contents.as_str()).await;
        writer.write_all(contents.as_bytes())?;
        Ok(())
    }
}
