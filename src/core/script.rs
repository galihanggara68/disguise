use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Script {
    pub name: String,
    pub command: String,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}
