use serde::{Deserialize, Serialize};
use serde_yaml::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Hook {
    pub id: String,
    #[serde(flatten)]
    extra: Value,
}
