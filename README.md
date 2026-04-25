# Disguise

[![CI](https://github.com/galihanggara68/disguise/actions/workflows/ci.yml/badge.svg)](https://github.com/galihanggara68/disguise/actions/workflows/ci.yml)

**Disguise** is a powerful CLI tool designed to simplify script management. It provides a central repository for your most-used shell scripts, allowing you to easily add, list, search, and execute them with advanced environment and history tracking.

All script definitions are stored in a human-readable TOML file located at `~/.config/disguise/scripts.toml`.

## Features

- **Unified Interface:** Manage all your shell scripts and commands from one place.
- **Background Execution:** Run scripts in the background with automatic logging to `~/.config/disguise/logs/`.
- **Interactive Shell Support:** Runs scripts in an interactive shell by default (bash/zsh), ensuring your aliases and functions are available.
- **Dynamic Arguments:** Pass extra arguments to your scripts at runtime using the `--` separator.
- **Environment Management:** Define script-specific environment variables and automatically load `.env` files.
- **Execution History:** Track when scripts were run, their duration, and exit status.
- **Search & Filter:** Quickly find scripts by name, description, or tags.
- **Import/Export:** Easily backup or share your script collections.
- **Shell Completions:** Native tab-completion support for Bash, Zsh, and Fish.

## Installation

### Using Cargo

If you have Rust installed, you can install directly via Cargo:

```bash
cargo install --git https://github.com/galihanggara68/disguise.git
```

### Using curl

To install the latest stable version of Disguise:

```bash
curl -sSL https://raw.githubusercontent.com/galihanggara68/disguise/main/install.sh | bash
```

## Usage

### Adding a script

```bash
disguise add --name "deploy" --command "npm run build" --description "Builds project" --tags "web,prod"
```

Or use the interactive mode:
```bash
disguise add --interactive
```

### Running a script

Foreground:
```bash
disguise run deploy
```

With extra arguments:
```bash
disguise run deploy -- --verbose --force
```

Background:
```bash
disguise run deploy --background
```
*Logs available at `~/.config/disguise/logs/deploy.log`.*

### Listing and Searching

```bash
# List all
disguise list

# Search by name/description
disguise list --search "build"

# Filter by tags
disguise list --tags "web,prod"
```

### Viewing History

```bash
disguise history --limit 20
disguise history --script deploy
```

### Managing Tags

```bash
disguise tag add "important,v2" script1 script2
disguise tag remove "old" script3
```

### Export and Import

```bash
disguise export my_scripts.toml
disguise import backup.toml --merge
```

### Shell Completions

Generate completion scripts for your shell:
```bash
# For Zsh
disguise completions zsh > ~/.oh-my-zsh/completions/_disguise
```

## Configuration

Disguise stores its data in:
- Configuration: `~/.config/disguise/scripts.toml`
- History: `~/.config/disguise/history.json`
- Logs: `~/.config/disguise/logs/`

## Development

### Prerequisites
- [Rust](https://www.rust-lang.org/) (latest stable)

### Building and Testing

```bash
cargo build
cargo test
cargo clippy
cargo fmt
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
