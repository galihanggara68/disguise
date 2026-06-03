use crate::storage::Storage;
use crate::ui::prompts::confirm_removal;
use anyhow::Result;

pub struct RemoveOptions {
    pub name: String,
    pub interactive: bool,
    pub force: bool,
}

pub fn handle(storage: &dyn Storage, options: RemoveOptions) -> Result<()> {
    // Check if script exists first
    let _ = storage.get_script(&options.name)?;

    let should_remove = if options.force {
        true
    } else if options.interactive {
        confirm_removal(&options.name)?
    } else {
        // Default behavior: confirm if not forced
        confirm_removal(&options.name)?
    };

    if should_remove {
        storage.remove_script(&options.name)?;
        println!("Script '{}' removed successfully.", options.name);
    } else {
        println!("Removal cancelled.");
    }

    Ok(())
}
