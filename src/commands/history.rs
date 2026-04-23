use crate::storage::Storage;
use anyhow::Result;
use chrono::{DateTime, Local, Utc};
use comfy_table::Table;

pub fn handle(storage: &dyn Storage, limit: usize, script_name: Option<String>) -> Result<()> {
    let mut history = storage.list_history()?;

    // Filter by script name
    if let Some(name) = script_name {
        history.retain(|e| e.script_name == name);
    }

    // Sort by timestamp descending (newest first)
    history.sort_by(|a, b| b.start_timestamp.cmp(&a.start_timestamp));

    // Limit results
    let history = history.into_iter().take(limit).collect::<Vec<_>>();

    if history.is_empty() {
        println!("No history found.");
        return Ok(());
    }

    let mut table = Table::new();
    table.set_header(vec!["Script Name", "Start Time", "Duration", "Status"]);

    for entry in history {
        let dt = DateTime::<Utc>::from_timestamp(entry.start_timestamp as i64, 0)
            .map(|dt| dt.with_timezone(&Local))
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let duration = if entry.duration_ms == 0 && entry.exit_code.is_none() {
            "Background".to_string()
        } else {
            format!("{}ms", entry.duration_ms)
        };

        let status = match entry.exit_code {
            Some(0) => "Success".to_string(),
            Some(code) => format!("Failed ({})", code),
            None => "Background".to_string(),
        };

        table.add_row(vec![entry.script_name, dt, duration, status]);
    }

    println!("{table}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::history::HistoryEntry;
    use crate::storage::fs::FileSystemStorage;
    use tempfile::tempdir;

    #[test]
    fn test_handle_history_filtering() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());

        storage.add_history_entry(HistoryEntry {
            script_name: "test1".to_string(),
            start_timestamp: 1000,
            duration_ms: 100,
            exit_code: Some(0),
        })?;

        storage.add_history_entry(HistoryEntry {
            script_name: "test2".to_string(),
            start_timestamp: 2000,
            duration_ms: 200,
            exit_code: Some(1),
        })?;

        // Filter by script2
        // We can't easily capture stdout here, but we can verify it doesn't crash
        let result = handle(&storage, 10, Some("test2".to_string()));
        assert!(result.is_ok());

        Ok(())
    }
}
