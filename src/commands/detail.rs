use crate::storage::Storage;
use anyhow::Result;
use comfy_table::Table;

pub fn handle(storage: &dyn Storage, name: String) -> Result<()> {
    let script = storage.get_script(&name)?;

    let mut table = Table::new();
    table.set_header(vec!["Field", "Value"]);

    table.add_row(vec!["Name", &script.name]);
    table.add_row(vec!["Command", &script.command]);
    table.add_row(vec![
        "Description",
        script.description.as_deref().unwrap_or("None"),
    ]);
    table.add_row(vec!["Tags", &script.tags.join(", ")]);

    println!("{table}");
    Ok(())
}
