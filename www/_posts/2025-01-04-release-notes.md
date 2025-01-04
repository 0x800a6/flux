---
layout: post
title: "Flux Shell 0.1.0 Released"
date: 2024-01-20
author: chocoOnEstrogen
---

We're excited to announce the first stable release of Flux Shell! This release introduces a modern, customizable shell experience with powerful features and a user-friendly configuration system.

## Key Features

### Configuration & Theming
- **Interactive Setup**: First-time setup wizard for easy configuration
- **Multiple Configuration Styles**: Choose between Minimal, Full, or Powerline presets
- **Rich Theme Support**: Customizable colors for every shell element
- **Flexible Prompt Templates**: Highly configurable prompt with support for:
  - Git branch integration
  - Username and hostname
  - Current directory
  - Timestamp with custom formats
  - Powerline-style decorations

### Productivity Features
- **Path Aliases**: Built-in shortcuts for common directories (`~`, `@docs`, `@dl`)
- **Command Aliases**: Pre-configured aliases for git and common commands
- **Environment Management**: Easy environment variable management
- **Command History**: Persistent command history with configurable size
- **Execution Time**: Optional display of command execution duration

### Built-in Commands
- Directory navigation (`cd`, `pwd`)
- Environment management (`env set/unset`)
- Screen clearing (`clear`)
- Shell configuration (`flux config`)
- Comprehensive help system (`help`)

### Developer-Friendly
- Git integration in prompt
- Error handling with colored output
- Environment variable expansion
- Cross-platform support

## Installation


Currently, Flux Shell is only available by building from source.

```bash
git clone https://github.com/chocoOnEstrogen/flux.git
cd flux
make build
sudo make install # If you are on a UNIX-like system
```

## Configuration

To reconfigure Flux Shell at any time:

```bash
flux config
```

Your configuration is stored in:
- Linux: `~/.config/rip.choco.flux/config.fl`
- macOS: `~/Library/Application Support/rip.choco.flux/config.fl`
- Windows: `%APPDATA%\rip.choco.flux\config.fl`

## What's Next?

We're already working on exciting features for upcoming releases:
- Plugin system for extending functionality
- Additional theme presets
- Enhanced tab completion
- Performance optimizations
- More customization options

Stay tuned for more updates! 