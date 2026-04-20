use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use clap_complete::Shell;
use directories::BaseDirs;
use std::fs;
use std::path::{Path, PathBuf};

use disguise_rs::FileSystemStorage;
use disguise_rs::commands;

#[derive(Parser)]
#[command(name = "disguise")]
#[command(about = "Disguise - A tool to manage and run scripts", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new script
    Add {
        /// Name of the script
        #[arg(short, long)]
        name: Option<String>,

        /// Command to execute
        #[arg(short, long)]
        command: Option<String>,

        /// Description of the script
        #[arg(short, long)]
        description: Option<String>,

        /// Tags for the script (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,

        /// Run in interactive mode
        #[arg(short, long)]
        interactive: bool,
    },
    /// List all managed scripts
    List {
        /// Filter by name or description
        #[arg(short, long)]
        search: Option<String>,

        /// Filter by tags (comma-separated, OR logic)
        #[arg(short, long)]
        tags: Option<String>,
    },
    /// View details of a specific script
    Detail {
        /// Name of the script
        name: String,
    },
    /// Run a managed script
    Run {
        /// Name of the script
        name: String,

        /// Run in background
        #[arg(short, long)]
        background: bool,

        /// Extra arguments to pass to the script
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Remove a managed script
    Remove {
        /// Name of the script to remove
        name: String,

        /// Interactive confirmation
        #[arg(short, long)]
        interactive: bool,

        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
    /// Update an existing script
    Update {
        /// Name of the script to update
        name: String,

        /// New name of the script
        #[arg(short, long)]
        new_name: Option<String>,

        /// New command to execute
        #[arg(short, long)]
        command: Option<String>,

        /// New description of the script
        #[arg(short, long)]
        description: Option<String>,

        /// New tags for the script (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,

        /// Run in interactive mode
        #[arg(short, long)]
        interactive: bool,
    },
    /// Manage tags for scripts
    Tag {
        #[command(subcommand)]
        tag_command: TagCommands,
    },
    /// Generate shell completions
    Completions {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Subcommand)]
enum TagCommands {
    /// Add tags to scripts
    Add {
        /// Tags to add (comma-separated)
        tags: String,
        /// Scripts to add tags to
        #[arg(required = true)]
        scripts: Vec<String>,
    },
    /// Remove tags from scripts
    Remove {
        /// Tags to remove (comma-separated)
        tags: String,
        /// Scripts to remove tags from
        #[arg(required = true)]
        scripts: Vec<String>,
    },
}

fn main() -> Result<()> {
    let config_dir = get_config_dir()?;
    initialize_environment(&config_dir)?;

    let storage = FileSystemStorage::new(&config_dir);
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add {
            name,
            command,
            description,
            tags,
            interactive,
        }) => {
            commands::add::handle(&storage, name, command, description, tags, interactive)?;
        }
        Some(Commands::List { search, tags }) => {
            commands::list::handle(&storage, search, tags)?;
        }
        Some(Commands::Detail { name }) => {
            commands::detail::handle(&storage, name)?;
        }
        Some(Commands::Run {
            name,
            background,
            args,
        }) => {
            commands::run::handle(&storage, name, background, args, &config_dir)?;
        }
        Some(Commands::Remove {
            name,
            interactive,
            force,
        }) => {
            commands::remove::handle(&storage, name, interactive, force)?;
        }
        Some(Commands::Update {
            name,
            new_name,
            command,
            description,
            tags,
            interactive,
        }) => {
            commands::update::handle(
                &storage,
                name,
                new_name,
                command,
                description,
                tags,
                interactive,
            )?;
        }
        Some(Commands::Tag { tag_command }) => match tag_command {
            TagCommands::Add { tags, scripts } => {
                commands::tag::add(&storage, tags, scripts)?;
            }
            TagCommands::Remove { tags, scripts } => {
                commands::tag::remove(&storage, tags, scripts)?;
            }
        },
        Some(Commands::Completions { shell }) => {
            commands::completions::handle::<Cli>(shell);
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
