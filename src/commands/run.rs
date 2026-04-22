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

    let shell = std::env::var("SHELL")
        .or_else(|_| std::env::var("COMSPEC"))
        .unwrap_or_else(|_| {
            if cfg!(target_os = "windows") {
                "cmd.exe".to_string()
            } else {
                "/bin/bash".to_string()
            }
        });

    let mut cmd = if cfg!(target_os = "windows") {
        let mut c = std::process::Command::new(&shell);
        if shell.to_lowercase().ends_with("cmd") || shell.to_lowercase().ends_with("cmd.exe") {
            c.arg("/C").arg(&full_command);
        } else {
            c.arg("-c").arg(&full_command);
        }
        c
    } else {
        let mut c = std::process::Command::new(&shell);
        // Use -i for bash and zsh to source rc files
        if shell.ends_with("bash") || shell.ends_with("zsh") {
            c.arg("-i");
        }
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

        // Wait up to 2 seconds for the process to finish and write logs
        let log_file_path = tmp_dir.path().join("logs").join("test.log");
        let mut log_content = String::new();
        for _ in 0..20 {
            if log_file_path.exists() {
                log_content = fs::read_to_string(&log_file_path)?;
                if log_content.trim() == "hello world" {
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        if !log_content.trim().ends_with("hello world") {
            println!("Log file path: {:?}", log_file_path);
            println!("Log content: {:?}", log_content);
            if log_file_path.exists() {
                let metadata = fs::metadata(&log_file_path)?;
                println!("Log file size: {}", metadata.len());
            } else {
                println!("Log file does not exist");
            }
        }
        assert!(log_content.trim().ends_with("hello world"));

        Ok(())
    }

    #[test]
    fn test_handle_run_environment_aware() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());

        // Set an env var
        unsafe { std::env::set_var("DISGUISE_TEST_VAR", "it_works") };

        let script = crate::core::script::Script {
            name: "env_test".to_string(),
            command: "echo $DISGUISE_TEST_VAR".to_string(),
            description: None,
            tags: vec![],
        };
        storage.add_script(script)?;

        let result = handle(
            &storage,
            "env_test".to_string(),
            true,
            vec![],
            tmp_dir.path(),
        );
        assert!(result.is_ok());

        let log_file_path = tmp_dir.path().join("logs").join("env_test.log");
        let mut log_content = String::new();
        for _ in 0..20 {
            if log_file_path.exists() {
                log_content = fs::read_to_string(&log_file_path)?;
                if log_content.trim().ends_with("it_works") {
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        assert!(log_content.trim().ends_with("it_works"));
        unsafe { std::env::remove_var("DISGUISE_TEST_VAR") };

        Ok(())
    }

    #[test]
    #[cfg(unix)]
    fn test_handle_run_bashrc_aware() -> Result<()> {
        let tmp_dir = tempdir()?;
        let storage = FileSystemStorage::new(tmp_dir.path());

        // Mock HOME to tmp_dir
        let old_home = std::env::var("HOME").ok();
        unsafe { std::env::set_var("HOME", tmp_dir.path()) };

        // Create a mock .bashrc
        let bashrc_path = tmp_dir.path().join(".bashrc");
        fs::write(&bashrc_path, "export MOCK_BASHRC_VAR=sourced")?;

        let script = crate::core::script::Script {
            name: "bashrc_test".to_string(),
            command: "echo $MOCK_BASHRC_VAR".to_string(),
            description: None,
            tags: vec![],
        };
        storage.add_script(script)?;

        // Ensure we are using bash for this test
        let old_shell = std::env::var("SHELL").ok();
        unsafe { std::env::set_var("SHELL", "/bin/bash") };

        let result = handle(
            &storage,
            "bashrc_test".to_string(),
            true,
            vec![],
            tmp_dir.path(),
        );
        assert!(result.is_ok());

        let log_file_path = tmp_dir.path().join("logs").join("bashrc_test.log");
        let mut log_content = String::new();
        for _ in 0..40 {
            // Give it more time as bash -i can be slow
            if log_file_path.exists() {
                log_content = fs::read_to_string(&log_file_path)?;
                if log_content.trim().contains("sourced") {
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        // Restore environment
        if let Some(home) = old_home {
            unsafe { std::env::set_var("HOME", home) };
        }
        if let Some(shell) = old_shell {
            unsafe { std::env::set_var("SHELL", shell) };
        }

        assert!(
            log_content.trim().contains("sourced"),
            "Log content was: {:?}",
            log_content
        );

        Ok(())
    }
}
