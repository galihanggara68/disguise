use crate::core::script::Script;
use anyhow::Result;
use dialoguer::{Confirm, Editor, Input};

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

    let command = if let Some(cmd) = initial_command {
        if cmd.contains('\n') || Confirm::new().with_prompt("Edit command in editor?").default(false).interact()? {
            Editor::new().edit(&cmd)?.unwrap_or(cmd)
        } else {
            Input::<String>::new()
                .with_prompt("Command")
                .default(cmd)
                .interact_text()?
        }
    } else {
        if Confirm::new().with_prompt("Open editor for command?").default(true).interact()? {
             Editor::new().edit("")?.unwrap_or_default()
        } else {
            Input::<String>::new()
                .with_prompt("Command")
                .interact_text()?
        }
    };

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
