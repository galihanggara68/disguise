use crate::storage::Storage;
use anyhow::Result;
use comfy_table::Table;

pub fn handle(storage: &dyn Storage) -> Result<()> {
    let scripts = storage.list_scripts()?;

    if scripts.is_empty() {
        println!("No scripts managed yet. Use 'disguise add' to add some.");
        return Ok(());
    }

    let mut table = Table::new();
    table.set_header(vec!["Name", "Description", "Tags"]);

    for script in scripts {
        table.add_row(vec![
            script.name.clone(),
            script.description.clone().unwrap_or_default(),
            script.tags.join(", "),
        ]);
    }

    println!("{table}");
    Ok(())
}
