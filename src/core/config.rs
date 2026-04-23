use crate::core::script::Script;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub scripts: Vec<Script>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::script::Script;

    #[test]
    fn test_config_serialization_deserialization() -> anyhow::Result<()> {
        let mut config = Config::default();
        let mut env = std::collections::HashMap::new();
        env.insert("FOO".to_string(), "BAR".to_string());
        config.scripts.push(Script {
            name: "test".to_string(),
            command: "echo test".to_string(),
            description: Some("desc".to_string()),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            env,
        });

        let toml_str = toml::to_string(&config)?;
        let deserialized: Config = toml::from_str(&toml_str)?;

        assert_eq!(config.scripts.len(), deserialized.scripts.len());
        assert_eq!(config.scripts[0], deserialized.scripts[0]);

        Ok(())
    }
}
