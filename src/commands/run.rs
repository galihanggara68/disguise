use crate::storage::Storage;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn handle(
    storage: &dyn Storage,
    name: String,
    background: bool,
    config_dir: &Path,
) -> Result<()> {
    let script = storage.get_script(&name)?;

    let mut cmd = if cfg!(target_os = "windows") {
        let mut c = std::process::Command::new("cmd");
        c.arg("/C").arg(&script.command);
        c
    } else {
        let mut c = std::process::Command::new("sh");
        c.arg("-c").arg(&script.command);
        c
    };

    if background {
        let logs_dir = config_dir.join("logs");
        fs::create_dir_all(&logs_dir).with_context(|| "Failed to create logs directory")?;
        let log_file_path = logs_dir.join(format!("{}.log", script.name));
        let log_file = fs::File::create(&log_file_path)
            .with_context(|| format!("Failed to create log file at {:?}", log_file_path))?;

        cmd.stdout(log_file.try_clone()?)
            .stderr(log_file)
            .spawn()
            .with_context(|| format!("Failed to spawn background process for '{}'", script.name))?;

        println!("Script '{}' started in background.", script.name);
        println!("Logs redirected to: {:?}", log_file_path);
    } else {
        let mut child = cmd
            .spawn()
            .with_context(|| format!("Failed to execute script '{}'", script.name))?;

        let status = child.wait()?;
        if !status.success() {
            std::process::exit(status.code().unwrap_or(1));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::fs::FileSystemStorage;
    use tempfile::tempdir;

    #[test]
    fn test_handle_run_script_not_found() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());

        let result = handle(&storage, "non-existent".to_string(), false, tmp_dir.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));

        Ok(())
    }
}
