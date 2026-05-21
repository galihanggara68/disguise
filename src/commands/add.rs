use crate::core::script::Script;
use crate::storage::Storage;
use crate::ui::prompts::prompt_for_script;
use anyhow::{Context, Result};
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

pub fn handle(
    storage: &dyn Storage,
    name: Option<String>,
    command: Option<String>,
    file: Option<PathBuf>,
    description: Option<String>,
    tags: Option<String>,
    interactive: bool,
) -> Result<()> {
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

    let script = if interactive || (name.is_none() && resolved_command.is_none()) {
        prompt_for_script(name, resolved_command, description, tags)?
    } else {
        let name = name.context("Name is required in flag mode")?;
        let command = resolved_command.context("Command is required in flag mode")?;
        let tags_vec = tags
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
            description,
            tags: tags_vec,
            env: Default::default(),
        }
    };

    storage.add_script(script)?;
    println!("Script added successfully!");
    Ok(())
}
