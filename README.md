# Disguise

![Version](https://badges.datenblick.space/version?owner=galihanggara68&repo=disguise)
![Build Status](https://badges.datenblick.space/build-status?owner=galihanggara68&repo=disguise&label=CI)
![License](https://badges.datenblick.space/license?license=MIT&shimmerInterval=5)
![Stars](https://badges.datenblick.space/github/stars?owner=galihanggara68&repo=disguise)

**Disguise** is a powerful CLI tool designed to simplify script management. It provides a central repository for your most-used shell scripts, allowing you to easily add, list, search, and execute them with advanced environment and history tracking.

Stop hunting through your terminal history or scattered `.sh` files. Start using **Disguise**.

## Features

- **Dynamic Autocomplete:** Native tab-completion for **Bash**, **Zsh**, and **Fish** that suggests your registered script names in real-time.
- **Convenient Alias:** Includes an optional `dx` alias for `disguise run` to execute scripts even faster.
- **Unified Interface:** Manage all your shell scripts and commands from one place.
- **Background Execution:** Run scripts in the background with automatic logging and status tracking.
- **Environment Aware:** Script-specific environment variables and native `.env` file support with clear precedence logic.
- **Interactive Editor Support:** Add or update complex commands using your system's default editor.
- **Self-Updating:** Stay current with the built-in `update` script.
- **Execution History:** Comprehensive tracking of run timestamps, durations, and exit codes.
- **Search & Filter:** Find scripts instantly by name, description, or tags.
- **Import/Export:** Effortlessly backup or share your entire script collection.

## Installation

### The Easy Way (Recommended)

To install Disguise and automatically set up autocomplete and aliases:

```bash
curl -sSL https://raw.githubusercontent.com/galihanggara68/disguise/main/install.sh | bash
```

The installer will guide you through:
1. OS and Architecture detection.
2. Binary installation to `/usr/local/bin`.
3. Shell configuration for **Bash**, **Zsh**, or **Fish**.
4. Registering the `dx` alias and dynamic completions.

### Using Cargo

If you have Rust installed:

```bash
cargo install --git https://github.com/galihanggara68/disguise.git
```

## Usage

### The `dx` Alias
If enabled during installation, you can use `dx` as a shorthand for `disguise run`. Both support full tab-completion!

```bash
dx my-script
# same as
disguise run my-script
```

### Adding a script

```bash
disguise add --name "deploy" --command "npm run build" --description "Builds project" --tags "web,prod"
```

**Interactive/Editor Mode:**
```bash
disguise add --interactive
```
*If a command is complex or multi-line, Disguise will automatically open your default editor.*

### Running a script

```bash
# Foreground
dx deploy

# With extra arguments
dx deploy -- --verbose --force

# Background (logs to ~/.config/disguise/logs/deploy.log)
dx deploy --background
```

### Listing and Searching

```bash
# List all (formatted table)
disguise list

# Search by name/description
disguise list --search "build"

# Filter by tags
disguise list --tags "web,prod"

# Output names only (useful for scripts)
disguise list --names-only
```

### Self-Updating

Disguise can update itself to the latest version:

```bash
dx update
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

## Configuration

Disguise stores its data in your user configuration directory:
- **Scripts**: `~/.config/disguise/scripts.toml`
- **History**: `~/.config/disguise/history.json`
- **Logs**: `~/.config/disguise/logs/`

## Development

### Building and Testing

```bash
# Standard checks
cargo build
cargo test
cargo clippy
cargo fmt

# Run tests sequentially (recommended for environment-sensitive tests)
cargo test -- --test-threads=1
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
