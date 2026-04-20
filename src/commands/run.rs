use crate::storage::Storage;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn handle(
    storage: &dyn Storage,
    name: String,
    background: bool,
    args: Vec<String>,
    config_dir: &Path,
) -> Result<()> {
    let script = storage.get_script(&name)?;

    let mut full_command = script.command.clone();
    if !args.is_empty() {
        full_command.push(' ');
        full_command.push_str(&args.join(" "));
    }

    let mut cmd = if cfg!(target_os = "windows") {
        let mut c = std::process::Command::new("cmd");
        c.arg("/C").arg(&full_command);
        c
    } else {
        let mut c = std::process::Command::new("sh");
        c.arg("-c").arg(&full_command);
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

        let result = handle(
            &storage,
            "non-existent".to_string(),
            false,
            vec![],
            tmp_dir.path(),
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));

        Ok(())
    }

    #[test]
    fn test_handle_run_with_args_background() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());

        let script = crate::core::script::Script {
            name: "test".to_string(),
            command: "echo".to_string(),
            description: None,
            tags: vec![],
        };
        storage.add_script(script)?;

        let result = handle(
            &storage,
            "test".to_string(),
            true,
            vec!["hello".to_string(), "world".to_string()],
            tmp_dir.path(),
        );
        assert!(result.is_ok());

        // Wait a bit for the process to finish
        std::thread::sleep(std::time::Duration::from_millis(200));

        let log_file_path = tmp_dir.path().join("logs").join("test.log");
        assert!(log_file_path.exists());
        let log_content = fs::read_to_string(log_file_path)?;
        assert_eq!(log_content.trim(), "hello world");

        Ok(())
    }
}
