use crate::core::config::Config;
use crate::core::history::HistoryEntry;
use crate::core::script::Script;
use crate::storage::Storage;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct FileSystemStorage {
    config_path: PathBuf,
    history_path: PathBuf,
}

impl FileSystemStorage {
    pub fn new(config_dir: &Path) -> Self {
        Self {
            config_path: config_dir.join("scripts.toml"),
            history_path: config_dir.join("history.json"),
        }
    }
}

impl Storage for FileSystemStorage {
    fn load_config(&self) -> Result<Config> {
        if !self.config_path.exists() {
            return Ok(Config::default());
        }
        let content = fs::read_to_string(&self.config_path)
            .with_context(|| format!("Failed to read config from {:?}", self.config_path))?;
        if content.trim().is_empty() {
            return Ok(Config::default());
        }
        toml::from_str(&content).with_context(|| "Failed to parse config file")
    }

    fn save_config(&self, config: &Config) -> Result<()> {
        let content =
            toml::to_string_pretty(config).with_context(|| "Failed to serialize config")?;
        fs::write(&self.config_path, content)
            .with_context(|| format!("Failed to write config to {:?}", self.config_path))?;
        Ok(())
    }

    fn add_script(&self, script: Script) -> Result<()> {
        let mut config = self.load_config()?;
        if config.scripts.iter().any(|s| s.name == script.name) {
            anyhow::bail!("A script with name '{}' already exists", script.name);
        }
        config.scripts.push(script);
        self.save_config(&config)
    }

    fn update_script(&self, name: &str, script: Script) -> Result<()> {
        let mut config = self.load_config()?;

        // If name is changing, check if the new name already exists
        if name != script.name && config.scripts.iter().any(|s| s.name == script.name) {
            anyhow::bail!("A script with name '{}' already exists", script.name);
        }

        if let Some(existing) = config.scripts.iter_mut().find(|s| s.name == name) {
            *existing = script;
            self.save_config(&config)
        } else {
            anyhow::bail!("Script '{}' not found", name);
        }
    }

    fn remove_script(&self, name: &str) -> Result<()> {
        let mut config = self.load_config()?;
        let original_len = config.scripts.len();
        config.scripts.retain(|s| s.name != name);
        if config.scripts.len() == original_len {
            anyhow::bail!("Script '{}' not found", name);
        }
        self.save_config(&config)
    }

    fn get_script(&self, name: &str) -> Result<Script> {
        let config = self.load_config()?;
        config
            .scripts
            .into_iter()
            .find(|s| s.name == name)
            .with_context(|| format!("Script '{}' not found", name))
    }

    fn list_scripts(&self) -> Result<Vec<Script>> {
        let config = self.load_config()?;
        let mut scripts = config.scripts;
        scripts.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(scripts)
    }

    fn add_history_entry(&self, entry: HistoryEntry) -> Result<()> {
        let mut history: Vec<HistoryEntry> = if self.history_path.exists() {
            let content = fs::read_to_string(&self.history_path)
                .with_context(|| format!("Failed to read history from {:?}", self.history_path))?;
            if content.trim().is_empty() {
                Vec::new()
            } else {
                serde_json::from_str(&content).unwrap_or_default()
            }
        } else {
            Vec::new()
        };

        history.push(entry);
        let content = serde_json::to_string_pretty(&history)
            .with_context(|| "Failed to serialize history")?;
        fs::write(&self.history_path, content)
            .with_context(|| format!("Failed to write history to {:?}", self.history_path))?;
        Ok(())
    }

    fn list_history(&self) -> Result<Vec<HistoryEntry>> {
        if !self.history_path.exists() {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(&self.history_path)
            .with_context(|| format!("Failed to read history from {:?}", self.history_path))?;
        if content.trim().is_empty() {
            return Ok(Vec::new());
        }
        serde_json::from_str(&content).with_context(|| "Failed to parse history")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_add_and_get_script() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());
        let script = Script {
            name: "test".to_string(),
            command: "echo test".to_string(),
            description: None,
            tags: vec![],
            env: Default::default(),
        };

        storage.add_script(script.clone())?;
        let retrieved = storage.get_script("test")?;
        assert_eq!(retrieved, script);

        // Duplicate
        let result = storage.add_script(script);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_update_script() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());
        let script = Script {
            name: "test".to_string(),
            command: "echo test".to_string(),
            description: None,
            tags: vec![],
            env: Default::default(),
        };

        storage.add_script(script)?;

        let updated_script = Script {
            name: "test".to_string(),
            command: "echo updated".to_string(),
            description: Some("desc".to_string()),
            tags: vec!["tag".to_string()],
            env: Default::default(),
        };

        storage.update_script("test", updated_script.clone())?;
        let retrieved = storage.get_script("test")?;
        assert_eq!(retrieved, updated_script);

        // Non-existent
        let result = storage.update_script("nonexistent", updated_script);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_remove_script() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());
        let script = Script {
            name: "test".to_string(),
            command: "echo test".to_string(),
            description: None,
            tags: vec![],
            env: Default::default(),
        };

        storage.add_script(script)?;
        storage.remove_script("test")?;
        let result = storage.get_script("test");
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_list_scripts() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());
        storage.add_script(Script {
            name: "b".to_string(),
            command: "cmd".to_string(),
            description: None,
            tags: vec![],
            env: Default::default(),
        })?;
        storage.add_script(Script {
            name: "a".to_string(),
            command: "cmd".to_string(),
            description: None,
            tags: vec![],
            env: Default::default(),
        })?;

        let list = storage.list_scripts()?;
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].name, "a");
        assert_eq!(list[1].name, "b");
        Ok(())
    }

    #[test]
    fn test_load_empty_config() -> Result<()> {
        let tmp_dir = tempdir()?;
        let config_path = tmp_dir.path().join("scripts.toml");
        fs::write(&config_path, "")?;

        let storage = FileSystemStorage::new(tmp_dir.path());
        let config = storage.load_config()?;
        assert!(config.scripts.is_empty());

        Ok(())
    }

    #[test]
    fn test_add_history_entry() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());
        let entry = HistoryEntry {
            script_name: "test".to_string(),
            start_timestamp: 123456789,
            duration_ms: 100,
            exit_code: Some(0),
        };

        storage.add_history_entry(entry.clone())?;

        let history_path = tmp_dir.path().join("history.json");
        assert!(history_path.exists());

        let content = fs::read_to_string(history_path)?;
        let history: Vec<HistoryEntry> = serde_json::from_str(&content)?;
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].script_name, "test");
        assert_eq!(history[0].start_timestamp, 123456789);
        assert_eq!(history[0].duration_ms, 100);
        assert_eq!(history[0].exit_code, Some(0));

        // Add another
        let entry2 = HistoryEntry {
            script_name: "test2".to_string(),
            start_timestamp: 123456790,
            duration_ms: 200,
            exit_code: Some(1),
        };
        storage.add_history_entry(entry2)?;

        let content = fs::read_to_string(tmp_dir.path().join("history.json"))?;
        let history: Vec<HistoryEntry> = serde_json::from_str(&content)?;
        assert_eq!(history.len(), 2);
        assert_eq!(history[1].script_name, "test2");

        Ok(())
    }

    #[test]
    fn test_list_history() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());
        let entry = HistoryEntry {
            script_name: "test".to_string(),
            start_timestamp: 123456789,
            duration_ms: 100,
            exit_code: Some(0),
        };

        storage.add_history_entry(entry.clone())?;
        let history = storage.list_history()?;
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].script_name, "test");
        Ok(())
    }
}
