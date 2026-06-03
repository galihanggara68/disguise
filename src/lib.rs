use clap::{Parser, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;

pub mod commands;
pub mod core;
pub mod storage;
pub mod ui;

pub use crate::core::config::Config;
pub use crate::core::script::Script;
pub use crate::storage::Storage;
pub use crate::storage::fs::FileSystemStorage;

#[derive(Parser)]
#[command(name = "disguise")]
#[command(about = "Disguise - A tool to manage and run scripts", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new script
    Add {
        /// Name of the script
        #[arg(short, long)]
        name: Option<String>,

        /// Command to execute
        #[arg(short, long)]
        command: Option<String>,

        /// File containing the command to execute
        #[arg(short, long)]
        file: Option<PathBuf>,

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

        /// Output only script names (useful for completions)
        #[arg(long)]
        names_only: bool,
    },
    /// View details of a specific script
    Detail {
        /// Name of the script
        #[arg(value_hint = clap::ValueHint::Other)]
        name: String,
    },
    /// Run a managed script
    Run {
        /// Name of the script
        #[arg(value_hint = clap::ValueHint::Other)]
        name: String,

        /// Run in background
        #[arg(short, long)]
        background: bool,

        /// Do not load .env file from current directory
        #[arg(long)]
        no_dotenv: bool,

        /// Extra arguments to pass to the script
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Remove a managed script
    Remove {
        /// Name of the script to remove
        #[arg(value_hint = clap::ValueHint::Other)]
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
        #[arg(value_hint = clap::ValueHint::Other)]
        name: String,

        /// New name of the script
        #[arg(short, long)]
        new_name: Option<String>,

        /// New command to execute
        #[arg(short, long)]
        command: Option<String>,

        /// File containing the new command to execute
        #[arg(short, long)]
        file: Option<PathBuf>,

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
    /// View script execution history
    History {
        /// Limit the number of history entries
        #[arg(short, long, default_value_t = 10)]
        limit: usize,

        /// Filter history by script name
        #[arg(short, long)]
        script: Option<String>,
    },
    /// Manage tags for scripts
    Tag {
        #[command(subcommand)]
        tag_command: TagCommands,
    },
    /// Export scripts to a file
    Export {
        /// Path to the export file
        path: PathBuf,
    },
    /// Import scripts from a file
    Import {
        /// Path to the import file
        path: PathBuf,

        /// Replace existing scripts (default is merge)
        #[arg(short, long)]
        replace: bool,

        /// Merge scripts with existing ones (default)
        #[arg(short, long, default_value_t = true, overrides_with = "replace")]
        merge: bool,
    },
    /// Generate shell completions
    Completions {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Subcommand)]
pub enum TagCommands {
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
