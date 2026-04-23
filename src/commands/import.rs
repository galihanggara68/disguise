use crate::core::config::Config;
use crate::storage::Storage;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub fn handle(storage: &dyn Storage, path: PathBuf, replace: bool) -> Result<()> {
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read import file from {:?}", path))?;

    let imported_config: Config = toml::from_str(&content).with_context(|| {
        format!(
            "Failed to parse import file {:?}. Make sure it is a valid TOML configuration.",
            path
        )
    })?;

    if replace {
        storage
            .save_config(&imported_config)
            .context("Failed to save imported configuration")?;
        println!("Configuration replaced successfully from {:?}", path);
    } else {
        let mut current_config = storage
            .load_config()
            .context("Failed to load current configuration")?;

        for imported_script in imported_config.scripts {
            if let Some(existing) = current_config
                .scripts
                .iter_mut()
                .find(|s| s.name == imported_script.name)
            {
                *existing = imported_script;
            } else {
                current_config.scripts.push(imported_script);
            }
        }

        storage
            .save_config(&current_config)
            .context("Failed to save merged configuration")?;
        println!("Configuration merged successfully from {:?}", path);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::script::Script;
    use crate::storage::fs::FileSystemStorage;
    use std::collections::HashMap;
    use tempfile::tempdir;

    #[test]
    fn test_import_replace() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());

        // Initial script
        storage.add_script(Script {
            name: "old".to_string(),
            command: "echo old".to_string(),
            description: None,
            tags: vec![],
            env: HashMap::new(),
        })?;

        let import_path = tmp_dir.path().join("import.toml");
        let new_config = Config {
            scripts: vec![Script {
                name: "new".to_string(),
                command: "echo new".to_string(),
                description: None,
                tags: vec![],
                env: HashMap::new(),
            }],
        };
        fs::write(&import_path, toml::to_string(&new_config)?)?;

        handle(&storage, import_path, true)?;

        let scripts = storage.list_scripts()?;
        assert_eq!(scripts.len(), 1);
        assert_eq!(scripts[0].name, "new");

        Ok(())
    }

    #[test]
    fn test_import_merge() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());

        // Initial script
        storage.add_script(Script {
            name: "keep".to_string(),
            command: "echo keep".to_string(),
            description: None,
            tags: vec![],
            env: HashMap::new(),
        })?;
        storage.add_script(Script {
            name: "update".to_string(),
            command: "echo old".to_string(),
            description: None,
            tags: vec![],
            env: HashMap::new(),
        })?;

        let import_path = tmp_dir.path().join("import.toml");
        let new_config = Config {
            scripts: vec![
                Script {
                    name: "update".to_string(),
                    command: "echo new".to_string(),
                    description: None,
                    tags: vec![],
                    env: HashMap::new(),
                },
                Script {
                    name: "add".to_string(),
                    command: "echo add".to_string(),
                    description: None,
                    tags: vec![],
                    env: HashMap::new(),
                },
            ],
        };
        fs::write(&import_path, toml::to_string(&new_config)?)?;

        handle(&storage, import_path, false)?;

        let scripts = storage.list_scripts()?;
        assert_eq!(scripts.len(), 3);

        let keep = scripts.iter().find(|s| s.name == "keep").unwrap();
        assert_eq!(keep.command, "echo keep");

        let update = scripts.iter().find(|s| s.name == "update").unwrap();
        assert_eq!(update.command, "echo new");

        let add = scripts.iter().find(|s| s.name == "add").unwrap();
        assert_eq!(add.command, "echo add");

        Ok(())
    }
}
