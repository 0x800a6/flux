---
layout: docs
title: Plugins
description: Learn how to create and manage Flux Shell plugins
order: 3
show_edit_button: true
---

# Plugin System

Flux Shell supports a powerful plugin system that allows you to extend the shell's functionality through dynamically loaded libraries.

## Managing Plugins

### Installing Plugins

To install a plugin from a git repository:


```bash
flux plugin install <repository-url>
```

The installation process includes a security review step where you can:
- Review the plugin code before installation
- Proceed with installation directly
- Cancel the installation

### Listing Plugins

View all installed plugins and their details:

```bash
flux plugin list
```

This shows:
- Plugin name and version
- Description
- Source repository
- Available commands

### Updating Plugins

Update an installed plugin to its latest version:

```bash
flux plugin update <plugin-name>
```

### Removing Plugins

Remove an installed plugin:
```bash
flux plugin remove <plugin-name>
```

## Creating Plugins

### Initialize a New Plugin

Create a new plugin project:
```bash
flux plugin init <plugin-name>
```

This creates a new Rust project with the necessary boilerplate for a Flux plugin.

### Plugin Structure

A basic plugin implements the `FluxPlugin` trait:

```rust
use flux::FluxPlugin;

pub struct MyPlugin {
    config: Option<String>
}

impl FluxPlugin for MyPlugin {
    fn name(&self) -> &str {
        "my-plugin"
    }

    fn description(&self) -> &str {
        "Description of my plugin"
    }

    fn init(&mut self) -> Result<(), String> {
        println!("Plugin initialized!");
        Ok(())
    }

    fn execute(&self, args: &[String]) -> Result<(), String> {
        // Handle plugin commands
        Ok(())
    }

    fn cleanup(&mut self) -> Result<(), String> {
        Ok(())
    }
}

#[no_mangle]
pub fn create_plugin() -> Box<dyn FluxPlugin> {
    Box::new(MyPlugin { config: None })
}

### Required Methods

- `name()` - Returns the plugin's name
- `init()` - Called when the plugin is loaded
- `execute()` - Handles plugin commands
- `cleanup()` - Called when the plugin is unloaded

### Optional Methods

- `version()` - Plugin version (defaults to "0.1.0")
- `description()` - Plugin description
- `commands()` - List of available commands
- `configure()` - Handle plugin configuration
- `help()` - Custom help text

### Building and Installing

1. Build your plugin:
```bash
cargo build --release
```

2. Install the compiled plugin:
```bash
cp target/release/libmyplugin.* ~/.config/rip.choco.flux/plugins/
```

## Security Considerations

Plugins run with the same permissions as the shell itself. When installing plugins:
- Always review the source code when possible
- Only install from trusted sources
- Be cautious of plugins that require elevated privileges

## Example Plugin

Here's a complete example of a simple math operations plugin:

```rust
use flux::FluxPlugin;

pub struct MathPlugin;

impl FluxPlugin for MathPlugin {
    fn name(&self) -> &str {
        "math"
    }

    fn description(&self) -> &str {
        "Basic math operations"
    }

    fn commands(&self) -> Vec<(&str, &str)> {
        vec![
            ("add", "Add two numbers"),
            ("sub", "Subtract two numbers"),
            ("mul", "Multiply two numbers"),
            ("div", "Divide two numbers")
        ]
    }

    fn execute(&self, args: &[String]) -> Result<(), String> {
        match args.get(0).map(String::as_str) {
            Some("add") => self.handle_math(args, |a, b| Ok(a + b)),
            Some("sub") => self.handle_math(args, |a, b| Ok(a - b)),
            Some("mul") => self.handle_math(args, |a, b| Ok(a * b)),
            Some("div") => self.handle_math(args, |a, b| {
                if b == 0 {
                    return Err("Division by zero!".to_string());
                }
                Ok(a / b)
            }),
            _ => Err("Unknown command".to_string())
        }
    }
}
```
