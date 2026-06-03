use anyhow::{Context, Result};
use clap::Parser;
use directories::BaseDirs;
use std::fs;
use std::path::{Path, PathBuf};

use disguise_rs::FileSystemStorage;
use disguise_rs::commands;
use disguise_rs::{Cli, Commands, TagCommands};

fn main() -> Result<()> {
    let config_dir = get_config_dir()?;
    initialize_environment(&config_dir)?;

    let storage = FileSystemStorage::new(&config_dir);
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add {
            name,
            command,
            file,
            description,
            tags,
            interactive,
        }) => {
            let options = commands::add::AddOptions {
                name,
                command,
                file,
                description,
                tags,
                interactive,
            };
            commands::add::handle(&storage, options)?;
        }
        Some(Commands::List {
            search,
            tags,
            names_only,
        }) => {
            let options = commands::list::ListOptions {
                search,
                tags,
                names_only,
            };
            commands::list::handle(&storage, options)?;
        }
        Some(Commands::Detail { name }) => {
            let options = commands::detail::DetailOptions { name };
            commands::detail::handle(&storage, options)?;
        }
        Some(Commands::Run {
            name,
            background,
            no_dotenv,
            args,
        }) => {
            let options = commands::run::RunOptions {
                name,
                background,
                no_dotenv,
                args,
                config_dir,
            };
            commands::run::handle(&storage, options)?;
        }
        Some(Commands::Remove {
            name,
            interactive,
            force,
        }) => {
            let options = commands::remove::RemoveOptions {
                name,
                interactive,
                force,
            };
            commands::remove::handle(&storage, options)?;
        }
        Some(Commands::Update {
            name,
            new_name,
            command,
            file,
            description,
            tags,
            interactive,
        }) => {
            let options = commands::update::UpdateOptions {
                name,
                new_name,
                command,
                file,
                description,
                tags,
                interactive,
            };
            commands::update::handle(&storage, options)?;
        }
        Some(Commands::History { limit, script }) => {
            let options = commands::history::HistoryOptions {
                limit,
                script_name: script,
            };
            commands::history::handle(&storage, options)?;
        }
        Some(Commands::Tag { tag_command }) => match tag_command {
            TagCommands::Add { tags, scripts } => {
                let options = commands::tag::TagOptions { tags, scripts };
                commands::tag::add(&storage, options)?;
            }
            TagCommands::Remove { tags, scripts } => {
                let options = commands::tag::TagOptions { tags, scripts };
                commands::tag::remove(&storage, options)?;
            }
        },
        Some(Commands::Export { path }) => {
            let options = commands::export::ExportOptions { path };
            commands::export::handle(&storage, options)?;
        }
        Some(Commands::Import {
            path,
            replace,
            merge: _,
        }) => {
            let options = commands::import::ImportOptions { path, replace };
            commands::import::handle(&storage, options)?;
        }
        Some(Commands::Completions { shell }) => {
            let options = commands::completions::CompletionsOptions { shell };
            commands::completions::handle(options);
        }
        None => {
            println!("Use 'disguise --help' for usage information.");
        }
    }

    Ok(())
}

fn get_config_dir() -> Result<PathBuf> {
    let base_dirs = BaseDirs::new().context("Could not determine user directories")?;
    let mut config_dir = base_dirs.config_dir().to_path_buf();
    config_dir.push("disguise");
    Ok(config_dir)
}

fn initialize_environment(config_dir: &Path) -> Result<()> {
    ensure_dir(config_dir)?;
    ensure_file(&config_dir.join("scripts.toml"))?;
    ensure_dir(&config_dir.join("logs"))?;
    Ok(())
}

fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory at {:?}", path))?;
    }
    Ok(())
}

fn ensure_file(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::write(path, "").with_context(|| format!("Failed to create file at {:?}", path))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_initialize_environment_creates_structure() -> Result<()> {
        let tmp_dir = tempdir()?;
        let config_dir = tmp_dir.path().join("disguise");

        initialize_environment(&config_dir)?;

        assert!(config_dir.exists());
        assert!(config_dir.join("scripts.toml").exists());
        assert!(config_dir.join("logs").exists());

        Ok(())
    }
}
