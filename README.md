# Disguise

[![CI](https://github.com/galihanggara68/disguise/actions/workflows/ci.yml/badge.svg)](https://github.com/galihanggara68/disguise/actions/workflows/ci.yml)

**Disguise** is a lightweight CLI tool designed to simplify script management. It provides a central repository for your most-used shell scripts, allowing you to easily add, list, detail, and execute them (even in the background).

All script definitions are stored in a human-readable TOML file located at `~/.config/disguise/scripts.toml`.

## Features

- **Unified Interface:** Manage all your shell scripts and commands from one place.
- **Background Execution:** Run scripts in the background with automatic logging to `~/.config/disguise/logs/`.
- **Interactive Mode:** Add or update scripts via a guided interactive prompt.
- **Table-based View:** Get a clean, formatted list of all your managed scripts.
- **Persistence:** Automatic configuration and storage management.

## Installation

### Using curl (Recommended)

To install the latest stable version of Disguise, run the following command in your terminal:

```bash
curl -sSL https://raw.githubusercontent.com/galihanggara68/disguise/main/install.sh | bash
```

Alternatively, you can manually download the binary for your architecture from the [Releases page](https://github.com/galihanggara68/disguise/releases/latest).

#### Manual Download Example (Linux x86_64)

```bash
curl -L https://github.com/galihanggara68/disguise/releases/latest/download/disguise-x86_64-unknown-linux-gnu -o disguise
chmod +x disguise
sudo mv disguise /usr/local/bin/
```

### Using Cargo

If you have Rust installed, you can install directly via Cargo:

```bash
cargo install --git https://github.com/galihanggara68/disguise.git
```

## Usage

### Adding a script

You can add a script using flags:
```bash
disguise add --name "deploy" --command "npm run build" --description "Builds project" --tags "web,prod"
```

Or use the interactive mode:
```bash
disguise add --interactive
```

### Listing scripts

```bash
disguise list
```

### Running a script

Foreground:
```bash
disguise run deploy
```

Background:
```bash
disguise run deploy --background
```
*Logs will be available at `~/.config/disguise/logs/deploy.log`.*

### Viewing script details

```bash
disguise detail deploy
```

### Removing a script

```bash
disguise remove deploy
```

## Configuration

Disguise stores its data in:
- Configuration: `~/.config/disguise/scripts.toml`
- Logs: `~/.config/disguise/logs/`

## Development

### Prerequisites
- [Rust](https://www.rust-lang.org/) (latest stable)

### Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run linting
cargo clippy -- -D warnings

# Format code
cargo fmt
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
