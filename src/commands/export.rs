use crate::storage::Storage;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub fn handle(storage: &dyn Storage, path: PathBuf) -> Result<()> {
    let config = storage
        .load_config()
        .context("Failed to load current configuration")?;
    let content = toml::to_string_pretty(&config).context("Failed to serialize configuration")?;

    fs::write(&path, content).with_context(|| format!("Failed to write export to {:?}", path))?;

    println!("Configuration exported successfully to {:?}", path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;
    use crate::core::script::Script;
    use crate::storage::fs::FileSystemStorage;
    use std::collections::HashMap;
    use tempfile::tempdir;

    #[test]
    fn test_export_success() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());

        let script = Script {
            name: "test".to_string(),
            command: "echo test".to_string(),
            description: Some("desc".to_string()),
            tags: vec!["tag".to_string()],
            env: HashMap::new(),
        };
        storage.add_script(script.clone())?;

        let export_path = tmp_dir.path().join("export.toml");
        handle(&storage, export_path.clone())?;

        assert!(export_path.exists());
        let content = fs::read_to_string(export_path)?;
        let exported_config: Config = toml::from_str(&content)?;

        assert_eq!(exported_config.scripts.len(), 1);
        assert_eq!(exported_config.scripts[0], script);

        Ok(())
    }
}
