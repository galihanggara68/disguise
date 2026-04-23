use crate::core::script::Script;
use anyhow::Result;
use dialoguer::{Confirm, Input};

pub fn prompt_for_script(
    initial_name: Option<String>,
    initial_command: Option<String>,
    initial_description: Option<String>,
    initial_tags: Option<String>,
) -> Result<Script> {
    let name = Input::<String>::new()
        .with_prompt("Script Name")
        .default(initial_name.unwrap_or_default())
        .interact_text()?;

    let command = Input::<String>::new()
        .with_prompt("Command")
        .default(initial_command.unwrap_or_default())
        .interact_text()?;

    let description_input = Input::<String>::new()
        .with_prompt("Description (optional)")
        .default(initial_description.unwrap_or_default())
        .allow_empty(true)
        .interact_text()?;

    let tags_str = Input::<String>::new()
        .with_prompt("Tags (comma-separated, optional)")
        .default(initial_tags.unwrap_or_default())
        .allow_empty(true)
        .interact_text()?;

    let description = if description_input.is_empty() {
        None
    } else {
        Some(description_input)
    };
    let tags = tags_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(Script {
        name,
        command,
        description,
        tags,
        env: std::collections::HashMap::new(),
    })
}

pub fn confirm_removal(name: &str) -> Result<bool> {
    Confirm::new()
        .with_prompt(format!(
            "Are you sure you want to remove script '{}'?",
            name
        ))
        .default(false)
        .interact()
        .map_err(Into::into)
}
