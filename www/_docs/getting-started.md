---
layout: docs
title: Getting Started
description: Learn how to install and set up Flux Shell
order: 1
show_edit_button: true
---

# Getting Started with Flux Shell

Flux is a modern shell implementation written in Rust that combines the power of traditional Unix shells with modern features and customization options.

## Quick Installation

### Building from Source
```bash
git clone https://github.com/chocoOnEstrogen/flux.git
cd flux
make install
```

## First Steps

1. Set Flux as your default shell:
```bash
chsh -s $(which flux)
```

2. Create your configuration file:
```bash
mkdir -p ~/.config/rip.choco.flux
touch ~/.config/rip.choco.flux/config.fl
```

3. Start using Flux:
```bash
flux
```

## Basic Configuration

Here's a minimal configuration to get you started:

```json
{
  "prompt": "flux λ ",
  "prompt_template": "{dir} λ ",
  "show_git_branch": false,
  "show_time": false,
  "show_username": false,
  "show_hostname": false,
  "theme": {
    "prompt_color": "white",
    "directory_color": "cyan",
    "git_branch_color": "white",
    "error_color": "red",
    "success_color": "green",
    "username_color": "white",
    "hostname_color": "white",
    "time_color": "white"
  }
}
```

## Next Steps

- Learn about [Configuration Options](/docs/configuration)
- Explore available [Themes](/docs/themes)
- Check out [Built-in Commands](/docs/commands) 