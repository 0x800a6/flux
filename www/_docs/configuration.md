---
layout: docs
title: Configuration
description: Learn how to configure Flux Shell
order: 2
show_edit_button: true
---

# Configuration Guide

Flux Shell can be extensively customized through its configuration file. This guide covers all available options and their usage.

## Configuration File Location

The configuration file is located at:
- Unix: `$XDG_CONFIG_HOME/rip.choco.flux/config.fl`
- Windows: `%APPDATA%\rip.choco.flux\config.fl`

## Configuration Modes

Flux supports three configuration modes:

### 1. Minimal Mode
```
#minimal
```

### 2. Full Mode (Default)
```json
{
  "prompt_template": "[{time}] {user}@{host} {dir} {git}\nλ ",
  "show_git_branch": true,
  "show_time": true,
  "theme": {
    "prompt_color": "cyan",
    "directory_color": "magenta",
    "git_branch_color": "green"
  }
}
```

### 3. Powerline Mode
```
#powerline
```

## Available Options

### General Settings

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `prompt` | string | `"flux λ "` | Basic prompt string |
| `prompt_template` | string | `"[{time}] {user}@{host} {dir} {git}\nλ "` | Template for shell prompt |
| `show_git_branch` | boolean | `true` | Show git branch in prompt |
| `show_time` | boolean | `true` | Show current time in prompt |
| `show_username` | boolean | `true` | Show username in prompt |
| `show_hostname` | boolean | `true` | Show hostname in prompt |
| `time_format` | string | `"%H:%M:%S"` | Format string for time display |
| `history_size` | number | `10000` | Number of commands to keep in history |
| `show_execution_time` | boolean | `true` | Show command execution time |

### Theme Settings

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `theme.prompt_color` | string | `"cyan"` | Color of the prompt symbol |
| `theme.error_color` | string | `"red"` | Color for error messages |
| `theme.success_color` | string | `"green"` | Color for success messages |
| `theme.username_color` | string | `"yellow"` | Color of the username |
| `theme.hostname_color` | string | `"blue"` | Color of the hostname |
| `theme.directory_color` | string | `"magenta"` | Color of the current directory |
| `theme.git_branch_color` | string | `"green"` | Color of the git branch |
| `theme.time_color` | string | `"white"` | Color of the time display |

### Available Colors
- white
- red
- green
- yellow
- blue
- magenta
- cyan
- bright white
- bright red
- bright green
- bright yellow
- bright blue
- bright magenta
- bright cyan 