<div align="center">
  <h1>Flux Shell</h1>
  <p>A modern, blazingly fast shell written in Rust</p>

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

</div>

## Features

- **Rich Customization**
  - Multiple theme presets (minimal, full, powerline)
  - Configurable prompt elements
  - Custom color schemes
- **Modern Experience**
  - Git integration
  - Smart path aliases
  - Command history
  - Execution time tracking
- **Performance Focused**
  - Built in Rust for speed and reliability
  - Minimal resource usage
- **Developer Friendly**
  - Environment management
  - Intuitive configuration
  - Extensive documentation

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/0x800a6/flux.git
cd flux

# Build and install
cargo build --release
cargo install --path .
```

### Basic Usage

```bash
# Start Flux
flux

# Navigate with smart aliases
flux λ @docs     # Jump to Documents
flux λ @dl       # Jump to Downloads

# Git integration
flux λ cd ~/projects
flux λ git checkout -b feature
(feature) flux λ
```

## Configuration

Flux uses a JSON configuration file located at:

- 🐧 Unix: `$XDG_CONFIG_HOME/sh.lrr.flux/config.fl`
- 🪟 Windows: `%APPDATA%\sh.lrr.flux\config.fl`

### Theme Presets

1. **Minimal** - Clean and simple

```
#minimal
```

2. **Full** (Default) - Rich features

```json
{
  "prompt_template": "[{time}] {user}@{host} {dir} {git}\nλ ",
  "show_git_branch": true,
  "show_time": true
}
```

3. **Powerline** - Enhanced visuals

```
#powerline
```

## Documentation

- [Online Documentation](https://flux.lrr.sh)
- Manual: `man flux`
- Configuration Guide: `flux --help`

## Built-in Commands

| Command    | Description                  |
| ---------- | ---------------------------- |
| `cd [DIR]` | Change directory             |
| `pwd`      | Print working directory      |
| `env`      | Manage environment variables |
| `clear`    | Clear screen                 |
| `exit`     | Exit shell                   |
| `alias`    | List aliases                 |

## Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please read our [Contributing Guide](.github/CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by modern shells like Fish and Zsh
- Built with Rust and ❤️

## Author

**0x800a6**

- GitHub: [@0x800a6](https://github.com/0x800a6)
