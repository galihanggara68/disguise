use crate::core::script::Script;
use crate::storage::Storage;
use crate::ui::prompts::prompt_for_script;
use anyhow::{Context, Result};
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

pub struct AddOptions {
    pub name: Option<String>,
    pub command: Option<String>,
    pub file: Option<PathBuf>,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub interactive: bool,
}

pub fn handle(storage: &dyn Storage, options: AddOptions) -> Result<()> {
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

    let script = if options.interactive || (options.name.is_none() && resolved_command.is_none()) {
        prompt_for_script(
            options.name,
            resolved_command,
            options.description,
            options.tags,
        )?
    } else {
        let name = options.name.context("Name is required in flag mode")?;
        let command = resolved_command.context("Command is required in flag mode")?;
        let tags_vec = options
            .tags
            .map(|t| {
                t.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default();
        Script {
            name,
            command,
            description: options.description,
            tags: tags_vec,
            env: Default::default(),
        }
    };

    storage.add_script(script)?;
    println!("Script added successfully!");
    Ok(())
}
