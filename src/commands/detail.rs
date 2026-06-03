use crate::storage::Storage;
use anyhow::Result;
use comfy_table::Table;

pub struct DetailOptions {
    pub name: String,
}

pub fn handle(storage: &dyn Storage, options: DetailOptions) -> Result<()> {
    let script = storage.get_script(&options.name)?;

    let mut table = Table::new();
    table.set_header(vec!["Field", "Value"]);

    table.add_row(vec!["Name", &script.name]);
    table.add_row(vec!["Command", &script.command]);
    table.add_row(vec![
        "Description",
        script.description.as_deref().unwrap_or("None"),
    ]);
    table.add_row(vec!["Tags", &script.tags.join(", ")]);

    if !script.env.is_empty() {
        let env_str = script
            .env
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("\n");
        table.add_row(vec!["Environment", &env_str]);
    }

    println!("{table}");
    Ok(())
}
