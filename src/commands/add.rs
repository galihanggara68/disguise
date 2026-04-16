use crate::core::script::Script;
use crate::storage::Storage;
use crate::ui::prompts::prompt_for_script;
use anyhow::{Context, Result};

pub fn handle(
    storage: &dyn Storage,
    name: Option<String>,
    command: Option<String>,
    description: Option<String>,
    tags: Option<String>,
    interactive: bool,
) -> Result<()> {
    let script = if interactive || (name.is_none() && command.is_none()) {
        prompt_for_script(name, command, description, tags)?
    } else {
        let name = name.context("Name is required in flag mode")?;
        let command = command.context("Command is required in flag mode")?;
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
        }
    };

    storage.add_script(script)?;
    println!("Script added successfully!");
    Ok(())
}
