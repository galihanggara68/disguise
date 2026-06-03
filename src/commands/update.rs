use crate::core::script::Script;
use crate::storage::Storage;
use crate::ui::prompts::prompt_for_script;
use anyhow::{Context, Result};
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

pub struct UpdateOptions {
    pub name: String,
    pub new_name: Option<String>,
    pub command: Option<String>,
    pub file: Option<PathBuf>,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub interactive: bool,
}

pub fn handle(storage: &dyn Storage, options: UpdateOptions) -> Result<()> {
    let existing_script = storage.get_script(&options.name)?;

    let resolved_command = if let Some(cmd) = options.command {
        Some(cmd)
    } else if let Some(path) = options.file {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read command from file {:?}", path))?;
        Some(content)
    } else if !io::IsTerminal::is_terminal(&io::stdin()) {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .context("Failed to read from stdin")?;
        let content = buffer.trim();
        if content.is_empty() {
            None
        } else {
            Some(content.to_string())
        }
    } else {
        None
    };

    let updated_script = if options.interactive {
        prompt_for_script(
            Some(
                options
                    .new_name
                    .unwrap_or_else(|| existing_script.name.clone()),
            ),
            Some(resolved_command.unwrap_or_else(|| existing_script.command.clone())),
            options.description.or(existing_script.description.clone()),
            options.tags.or_else(|| {
                if existing_script.tags.is_empty() {
                    None
                } else {
                    Some(existing_script.tags.join(","))
                }
            }),
        )?
    } else {
        let final_name = options
            .new_name
            .unwrap_or_else(|| existing_script.name.clone());
        let final_command = resolved_command.unwrap_or_else(|| existing_script.command.clone());
        let final_description = options.description.or(existing_script.description.clone());
        let final_tags = options
            .tags
            .map(|t| {
                t.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_else(|| existing_script.tags.clone());

        Script {
            name: final_name,
            command: final_command,
            description: final_description,
            tags: final_tags,
            env: existing_script.env,
        }
    };

    storage.update_script(&options.name, updated_script)?;
    println!("Script '{}' updated successfully!", options.name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::script::Script;
    use crate::storage::fs::FileSystemStorage;
    use tempfile::tempdir;

    #[test]
    fn test_handle_update_script_not_found() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());

        let options = UpdateOptions {
            name: "non-existent".to_string(),
            new_name: None,
            command: None,
            file: None,
            description: None,
            tags: None,
            interactive: false,
        };

        let result = handle(&storage, options);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));

        Ok(())
    }

    #[test]
    fn test_handle_update_script_success() -> Result<()> {
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

        let options = UpdateOptions {
            name: "test".to_string(),
            new_name: Some("new-test".to_string()),
            command: Some("echo new".to_string()),
            file: None,
            description: Some("new desc".to_string()),
            tags: Some("tag1,tag2".to_string()),
            interactive: false,
        };

        handle(&storage, options)?;

        let updated = storage.get_script("new-test")?;
        assert_eq!(updated.name, "new-test");
        assert_eq!(updated.command, "echo new");
        assert_eq!(updated.description, Some("new desc".to_string()));
        assert_eq!(updated.tags, vec!["tag1".to_string(), "tag2".to_string()]);

        Ok(())
    }
}
