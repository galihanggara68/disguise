use crate::core::script::Script;
use crate::storage::Storage;
use crate::ui::prompts::prompt_for_script;
use anyhow::{Context, Result};
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

pub fn handle(
    storage: &dyn Storage,
    name: String,
    new_name: Option<String>,
    command: Option<String>,
    file: Option<PathBuf>,
    description: Option<String>,
    tags: Option<String>,
    interactive: bool,
) -> Result<()> {
    let existing_script = storage.get_script(&name)?;

    let resolved_command = if let Some(cmd) = command {
        Some(cmd)
    } else if let Some(path) = file {
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

    let updated_script = if interactive {
        prompt_for_script(
            Some(new_name.unwrap_or_else(|| existing_script.name.clone())),
            Some(resolved_command.unwrap_or_else(|| existing_script.command.clone())),
            description.or(existing_script.description.clone()),
            tags.or_else(|| {
                if existing_script.tags.is_empty() {
                    None
                } else {
                    Some(existing_script.tags.join(","))
                }
            }),
        )?
    } else {
        let final_name = new_name.unwrap_or_else(|| existing_script.name.clone());
        let final_command = resolved_command.unwrap_or_else(|| existing_script.command.clone());
        let final_description = description.or(existing_script.description.clone());
        let final_tags = tags
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

    storage.update_script(&name, updated_script)?;
    println!("Script '{}' updated successfully!", name);
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

        let result = handle(
            &storage,
            "non-existent".to_string(),
            None,
            None,
            None,
            None,
            None,
            false,
        );
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

        handle(
            &storage,
            "test".to_string(),
            Some("new-test".to_string()),
            Some("echo new".to_string()),
            None,
            Some("new desc".to_string()),
            Some("tag1,tag2".to_string()),
            false,
        )?;

        let updated = storage.get_script("new-test")?;
        assert_eq!(updated.name, "new-test");
        assert_eq!(updated.command, "echo new");
        assert_eq!(updated.description, Some("new desc".to_string()));
        assert_eq!(updated.tags, vec!["tag1".to_string(), "tag2".to_string()]);

        Ok(())
    }
}
