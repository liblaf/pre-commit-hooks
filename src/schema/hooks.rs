use std::io::{Read, Write};

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
    pub fn load<R>(reader: R) -> anyhow::Result<Self>
    where
        R: Read,
    {
        let hooks = serde_yaml::from_reader::<_, Hooks>(reader).log()?;
        Ok(hooks)
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
