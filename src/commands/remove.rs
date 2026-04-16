use crate::storage::Storage;
use crate::ui::prompts::confirm_removal;
use anyhow::Result;

pub fn handle(storage: &dyn Storage, name: String, interactive: bool, force: bool) -> Result<()> {
    // Check if script exists first
    let _ = storage.get_script(&name)?;

    let should_remove = if force {
        true
    } else if interactive {
        confirm_removal(&name)?
    } else {
        // Default behavior: confirm if not forced
        confirm_removal(&name)?
    };

    if should_remove {
        storage.remove_script(&name)?;
        println!("Script '{}' removed successfully.", name);
    } else {
        println!("Removal cancelled.");
    }

    Ok(())
}
