# 🦀 Crab - Secure Credential Manager

A secure credential manager written in Rust for storing and managing sensitive information like passwords, API keys, and other credentials.

[![CI](https://github.com/Aoi-Takahashi/crab/workflows/CI/badge.svg)](https://github.com/Aoi-Takahashi/crab/actions)
[![Release](https://img.shields.io/github/v/release/Aoi-Takahashi/crab)](https://github.com/Aoi-Takahashi/crab/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## ✨ Features

- **Secure Storage**: Store credentials safely with local file-based storage
- **Cross-Platform**: Works on Linux, macOS, and Windows(WSL)
- **Interactive CLI**: User-friendly command-line interface with confirmation prompts
- **Local Time Display**: Shows creation and modification times in your local timezone
- **Backup & Restore**: Built-in database backup functionality
- **No External Dependencies**: Pure Rust implementation with minimal dependencies

## 📦 Installation

### Quick Install (Recommended)

#### Unix/Linux/macOS/Windows(WSL)

```bash
curl -fsSL https://raw.githubusercontent.com/Aoi-Takahashi/crab/main/scripts/install.sh | bash
```

#### From Source (Requires Rust)

```bash
git clone https://github.com/Aoi-Takahashi/crab.git
cd crab
cargo install --path .
```

### Verify Installation

```bash
crab --version
crab --help
```

## 🚀 Quick Start

### Add your first credential

```bash
# Interactive mode
crab add

# Or specify details directly
crab add --service github --account yourusername
```

### View stored credentials

```bash
# List all services
crab list

# Get specific credential (will prompt to show secret)
crab get github
```

### Manage credentials

```bash
# Edit existing credential
crab edit github

# Remove credential
crab remove github

# Show database information
crab info

# Create backup
crab backup

# Delete entire database (with backup option)
crab delete
```

## 💻 Usage

```bash
crab <COMMAND>
```

### Commands

| Command  | Description              | Example                          |
| -------- | ------------------------ | -------------------------------- |
| `add`    | Add new credential       | `crab add -s github -a username` |
| `get`    | Retrieve credential      | `crab get github`                |
| `list`   | List all services        | `crab list`                      |
| `edit`   | Edit existing credential | `crab edit github`               |
| `remove` | Remove credential        | `crab remove github`             |
| `info`   | Show database info       | `crab info`                      |
| `backup` | Create database backup   | `crab backup`                    |
| `delete` | Delete entire database   | `crab delete`                    |

### Options

| Option      | Short | Description                        |
| ----------- | ----- | ---------------------------------- |
| `--service` | `-s`  | Service name (for add command)     |
| `--account` | `-a`  | Account/username (for add command) |
| `--help`    | `-h`  | Show help information              |
| `--version` | `-V`  | Show version information           |

## 🔧 Configuration

Crab stores its data in:

- **Linux/macOS/Windows(WSL)**: `~/.crab/credentials.json`

The database file is automatically created with secure permissions (600 on Unix-like systems).

## 🏗️ Architecture

```
crab/
├── src/
│   ├── cli/           # Command-line interface
│   ├── error/         # Error handling
│   ├── models/        # Data models
│   ├── storage/       # File storage logic
│   └── util/          # Utilities (time formatting, etc.)
├── scripts/           # Installation scripts
└── .github/workflows/ # CI/CD workflows
```

## 🔒 Security Considerations

- **Local Storage Only**: No cloud synchronization, your data stays on your machine
- **File Permissions**: Database file is created with restricted permissions
- **Plain Text Storage**: Currently stores credentials in plain text JSON (encryption planned)
- **Backup Safety**: Backups include timestamps and are stored locally

⚠️ **Important**: This tool currently stores credentials in plain text. For production use, consider the encryption features that are planned for future releases.

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes** and add tests
4. **Run tests**: `cargo test`
5. **Run clippy**: `cargo clippy -- -D warnings`
6. **Format code**: `cargo fmt`
7. **Commit changes**: `git commit -m 'Add amazing feature'`
8. **Push to branch**: `git push origin feature/amazing-feature`
9. **Open a Pull Request**

### Development Setup

```bash
git clone https://github.com/Aoi-Takahashi/crab.git
cd crab

# Install dependencies and build
cargo build

# Run tests
cargo test

# Run with sample data
cargo run -- add --service test --account testuser
```

### Code Style

This project follows standard Rust conventions:

- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Add tests for new functionality
- Update documentation for API changes

## 📋 Roadmap

- [ ] **Encryption**: AES-256 encryption for stored credentials
- [ ] **Master Password**: Protect database with master password
- [ ] **Import/Export**: Support for CSV, JSON, and other formats
- [ ] **Search**: Fuzzy search across services
- [ ] **Tags/Categories**: Organize credentials with tags
- [ ] **Password Generator**: Built-in secure password generation
- [ ] **System Integration**: Keychain/Credential Manager integration

## 📄 License

This project is licensed under the [MIT License](LICENSE) - see the LICENSE file for details.

---

Made with ❤️ and 🦀 by [Aoi Takahashi](https://github.com/Aoi-Takahashi)
