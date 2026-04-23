use crate::storage::Storage;
use anyhow::Result;
use comfy_table::Table;

pub fn handle(storage: &dyn Storage, search: Option<String>, tags: Option<String>) -> Result<()> {
    let mut scripts = storage.list_scripts()?;

    if scripts.is_empty() {
        println!("No scripts managed yet. Use 'disguise add' to add some.");
        return Ok(());
    }

    // Filter by search
    if let Some(search) = search {
        let search = search.to_lowercase();
        scripts.retain(|s| {
            s.name.to_lowercase().contains(&search)
                || s.description
                    .as_ref()
                    .map(|d| d.to_lowercase().contains(&search))
                    .unwrap_or(false)
        });
    }

    // Filter by tags (OR logic for tags, but AND logic with search)
    if let Some(tags_str) = tags {
        let filter_tags: Vec<String> = tags_str
            .split(',')
            .map(|t| t.trim().to_lowercase())
            .filter(|t| !t.is_empty())
            .collect();

        if !filter_tags.is_empty() {
            scripts.retain(|s| {
                s.tags
                    .iter()
                    .any(|t| filter_tags.contains(&t.to_lowercase()))
            });
        }
    }

    if scripts.is_empty() {
        println!("No scripts found matching your filters.");
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::script::Script;
    use crate::storage::fs::FileSystemStorage;
    use tempfile::tempdir;

    fn setup_storage(dir: &std::path::Path) -> Result<FileSystemStorage> {
        let storage = FileSystemStorage::new(dir);
        let scripts = vec![
            Script {
                name: "test-script-1".to_string(),
                command: "echo 1".to_string(),
                description: Some("Description for first script".to_string()),
                tags: vec!["tag1".to_string(), "common".to_string()],
                env: std::collections::HashMap::new(),
            },
            Script {
                name: "another-script".to_string(),
                command: "echo 2".to_string(),
                description: Some("Second script description".to_string()),
                tags: vec!["tag2".to_string(), "common".to_string()],
                env: std::collections::HashMap::new(),
            },
            Script {
                name: "third".to_string(),
                command: "echo 3".to_string(),
                description: None,
                tags: vec!["unique".to_string()],
                env: std::collections::HashMap::new(),
            },
        ];

        for script in scripts {
            storage.add_script(script)?;
        }

        Ok(storage)
    }

    #[test]
    fn test_handle_list_no_filter() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = setup_storage(tmp_dir.path())?;

        let result = handle(&storage, None, None);
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_handle_list_filter_search_name() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = setup_storage(tmp_dir.path())?;

        // Should find "test-script-1" and "another-script" if searching "script"
        let result = handle(&storage, Some("script".to_string()), None);
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_handle_list_filter_search_description() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = setup_storage(tmp_dir.path())?;

        // Should find "test-script-1" if searching "first"
        let result = handle(&storage, Some("first".to_string()), None);
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_handle_list_filter_tags() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = setup_storage(tmp_dir.path())?;

        // Should find "test-script-1" and "another-script" if filtering by "common"
        let result = handle(&storage, None, Some("common".to_string()));
        assert!(result.is_ok());

        // Should find all if filtering by "tag1,tag2,unique"
        let result = handle(&storage, None, Some("tag1,tag2,unique".to_string()));
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_handle_list_filter_combined() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = setup_storage(tmp_dir.path())?;

        // Should find "test-script-1" if searching "script" and tag "tag1"
        let result = handle(
            &storage,
            Some("script".to_string()),
            Some("tag1".to_string()),
        );
        assert!(result.is_ok());

        // Should find nothing if searching "script" and tag "unique"
        let result = handle(
            &storage,
            Some("script".to_string()),
            Some("unique".to_string()),
        );
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_handle_list_no_scripts() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());

        let result = handle(&storage, None, None);
        assert!(result.is_ok());

        Ok(())
    }
}
