use crate::storage::Storage;
use anyhow::Result;

pub fn add(storage: &dyn Storage, tags: String, scripts: Vec<String>) -> Result<()> {
    let tags_to_add: Vec<String> = tags
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let mut config = storage.load_config()?;

    // Validate all scripts exist first
    for script_name in &scripts {
        if !config.scripts.iter().any(|s| s.name == *script_name) {
            anyhow::bail!("Script '{}' not found", script_name);
        }
    }

    let mut modified = false;
    for script_name in &scripts {
        if let Some(script) = config.scripts.iter_mut().find(|s| s.name == *script_name) {
            for tag in &tags_to_add {
                if !script.tags.contains(tag) {
                    script.tags.push(tag.clone());
                    modified = true;
                }
            }
        }
    }

    if modified {
        storage.save_config(&config)?;
        println!("Tags added successfully!");
    } else {
        println!("No changes made.");
    }

    Ok(())
}

pub fn remove(storage: &dyn Storage, tags: String, scripts: Vec<String>) -> Result<()> {
    let tags_to_remove: Vec<String> = tags
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let mut config = storage.load_config()?;

    // Validate all scripts exist first
    for script_name in &scripts {
        if !config.scripts.iter().any(|s| s.name == *script_name) {
            anyhow::bail!("Script '{}' not found", script_name);
        }
    }

    let mut modified = false;
    for script_name in &scripts {
        if let Some(script) = config.scripts.iter_mut().find(|s| s.name == *script_name) {
            let original_len = script.tags.len();
            script.tags.retain(|t| !tags_to_remove.contains(t));
            if script.tags.len() != original_len {
                modified = true;
            }
        }
    }

    if modified {
        storage.save_config(&config)?;
        println!("Tags removed successfully!");
    } else {
        println!("No changes made.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::script::Script;
    use crate::storage::fs::FileSystemStorage;
    use tempfile::tempdir;

    #[test]
    fn test_tag_add() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());

        storage.add_script(Script {
            name: "s1".to_string(),
            command: "echo 1".to_string(),
            description: None,
            tags: vec!["old".to_string()],
        })?;

        storage.add_script(Script {
            name: "s2".to_string(),
            command: "echo 2".to_string(),
            description: None,
            tags: vec![],
        })?;

        add(
            &storage,
            "t1, t2".to_string(),
            vec!["s1".to_string(), "s2".to_string()],
        )?;

        let s1 = storage.get_script("s1")?;
        assert_eq!(s1.tags, vec!["old", "t1", "t2"]);

        let s2 = storage.get_script("s2")?;
        assert_eq!(s2.tags, vec!["t1", "t2"]);

        Ok(())
    }

    #[test]
    fn test_tag_add_duplicate() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());

        storage.add_script(Script {
            name: "s1".to_string(),
            command: "echo 1".to_string(),
            description: None,
            tags: vec!["t1".to_string()],
        })?;

        add(&storage, "t1".to_string(), vec!["s1".to_string()])?;

        let s1 = storage.get_script("s1")?;
        assert_eq!(s1.tags, vec!["t1"]); // Still only one t1

        Ok(())
    }

    #[test]
    fn test_tag_remove() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());

        storage.add_script(Script {
            name: "s1".to_string(),
            command: "echo 1".to_string(),
            description: None,
            tags: vec!["t1".to_string(), "t2".to_string()],
        })?;

        storage.add_script(Script {
            name: "s2".to_string(),
            command: "echo 2".to_string(),
            description: None,
            tags: vec!["t2".to_string(), "t3".to_string()],
        })?;

        remove(
            &storage,
            "t2, t3".to_string(),
            vec!["s1".to_string(), "s2".to_string()],
        )?;

        let s1 = storage.get_script("s1")?;
        assert_eq!(s1.tags, vec!["t1"]);

        let s2 = storage.get_script("s2")?;
        assert!(s2.tags.is_empty());

        Ok(())
    }

    #[test]
    fn test_tag_validate_existence() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());

        let result = add(&storage, "t1".to_string(), vec!["nonexistent".to_string()]);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Script 'nonexistent' not found"
        );

        Ok(())
    }
}
