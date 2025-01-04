# FLUX(1) Shell Manual

## NAME
flux - An advanced, customizable shell for modern systems

## SYNOPSIS
`flux [OPTIONS]`

## DESCRIPTION
Flux is a modern shell implementation written in Rust that combines the power of traditional Unix shells with modern features and customization options. It provides a rich set of features including:

- Customizable prompts with git integration
- Theme support with multiple presets
- Path and command aliasing
- Environment variable management
- Command history
- Execution time tracking

## CONFIGURATION
Flux uses a configuration file located at:
- Unix: `$XDG_CONFIG_HOME/rip.choco.flux/config.fl`
- Windows: `%APPDATA%\rip.choco.flux\config.fl`

### Configuration Modes
Flux supports three configuration modes:

1. **Minimal**
   ```
   #minimal
   ```
   Basic prompt with essential features.

2. **Full** (Default)
   ```json
   {
     "prompt_template": "[{time}] {user}@{host} {dir} {git}\nλ ",
     "show_git_branch": true,
     "show_time": true
     // ... other options
   }
   ```
   Complete feature set with git integration.

3. **Powerline**
   ```
   #powerline
   ```
   Enhanced visual style with Unicode characters.

## PROMPT CUSTOMIZATION
The prompt can be customized using the following placeholders:
- `{user}` - Current username
- `{host}` - Hostname
- `{dir}` - Current directory
- `{git}` - Git branch (when enabled)
- `{time}` - Current time

## BUILT-IN COMMANDS

### Navigation
- `cd [DIR]` - Change directory
- `pwd` - Print working directory

### Environment
- `env` - List environment variables
- `env set KEY VALUE` - Set environment variable
- `env unset KEY` - Unset environment variable

### Shell Control
- `exit` - Exit the shell
- `clear` - Clear the screen
- `alias` - List defined aliases

## PATH ALIASES
Default path aliases:
- `~` - Home directory
- `@docs` - Documents directory
- `@dl` - Downloads directory

## THEME
Colors can be customized for various elements:
- `prompt_color`
- `error_color`
- `success_color`
- `username_color`
- `hostname_color`
- `directory_color`
- `git_branch_color`
- `time_color`

Available colors include: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`, and their bright variants.

## EXAMPLES

### Basic Usage
```bash
# Start the shell
$ flux

# Navigate using path aliases
flux λ @docs
flux λ pwd
/home/user/Documents

# Use git integration
flux λ cd ~/projects/flux
flux λ git checkout -b feature
(feature) flux λ
```

### Custom Configuration
```json
{
  "prompt_template": "[ {time} ] {dir} {git} → ",
  "theme": {
    "prompt_color": "cyan",
    "directory_color": "bright magenta",
    "git_branch_color": "bright green"
  }
}
```

## SEE ALSO
- bash(1)
- zsh(1)
- fish(1)

## BUGS
Report bugs at: https://github.com/chocoOnEstrogen/flux/issues

## AUTHOR
Written by chocoOnEstrogen

## COPYRIGHT
Copyright © 2025 chocoOnEstrogen. License MIT.