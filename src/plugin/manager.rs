use std::path::PathBuf;
use std::process::Command;
use uuid::Uuid;
use dialoguer::{Confirm, Select};
use colored::*;
use std::fs;

pub struct PluginInstaller {
    plugin_dir: PathBuf,
    temp_dir: PathBuf,
}

impl PluginInstaller {
    pub fn new() -> Self {
        let plugin_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rip.choco.flux")
            .join("plugins");
        
        let temp_dir = plugin_dir.join("temp");
        
        std::fs::create_dir_all(&plugin_dir).unwrap_or_default();
        std::fs::create_dir_all(&temp_dir).unwrap_or_default();
        
        Self { plugin_dir, temp_dir }
    }

    pub fn install_from_git(&self, url: &str) -> Result<(), String> {
        println!("{}", "⚠️  Warning: Installing plugins can be dangerous as they run with the same permissions as the shell."
            .bright_yellow());
        println!("{}", "Please review the code before installing.".bright_yellow());

        // Clone to temp directory
        let uuid = Uuid::new_v4();
        let temp_path = self.temp_dir.join(uuid.to_string());
        
        Command::new("git")
            .args(["clone", url, temp_path.to_str().unwrap()])
            .output()
            .map_err(|e| format!("Failed to clone repository: {}", e))?;

        // Ask user to review or proceed
        let options = vec!["[O]pen code for review", "[P]roceed with installation", "[C]ancel"];
        let selection = Select::new()
            .with_prompt("What would you like to do?")
            .items(&options)
            .default(0)
            .interact()
            .map_err(|e| e.to_string())?;

        match selection {
            0 => {
                // Open code in default editor or 'less'
                if let Ok(editor) = std::env::var("EDITOR") {
                    Command::new(editor)
                        .arg(&temp_path)
                        .status()
                        .map_err(|e| format!("Failed to open editor: {}", e))?;
                } else {
                    Command::new("less")
                        .arg(temp_path.join("src/lib.rs"))
                        .status()
                        .map_err(|e| format!("Failed to open less: {}", e))?;
                }
                
                // Ask if they want to proceed after review
                if !Confirm::new()
                    .with_prompt("Proceed with installation?")
                    .default(false)
                    .interact()
                    .unwrap_or(false) {
                    fs::remove_dir_all(&temp_path).ok();
                    return Ok(());
                }
            }
            1 => {
                // Proceed directly
                println!("{}", "Proceeding without code review...".bright_yellow());
            }
            _ => {
                fs::remove_dir_all(&temp_path).ok();
                return Ok(());
            }
        }

        // Build plugin
        Command::new("cargo")
            .args(["build", "--release"])
            .current_dir(&temp_path)
            .status()
            .map_err(|e| format!("Failed to build plugin: {}", e))?;

        // Move to plugins directory
        let plugin_file = format!("{}.flp", uuid);
        let target_path = self.plugin_dir.join(&plugin_file);
        
        let isWindows = std::env::consts::OS == "windows";
        fs::copy(
            temp_path.join(if isWindows { "target/release/lib*.dll" } else { "target/release/lib*.dylib" }),
            &target_path
        ).map_err(|e| format!("Failed to install plugin: {}", e))?;

        // Cleanup
        fs::remove_dir_all(&temp_path).ok();
        
        println!("{}", "Plugin installed successfully!".green());
        Ok(())
    }

    pub fn init_plugin(&self, name: &str) -> Result<(), String> {
        // Create new cargo project
        Command::new("cargo")
            .args(["new", "--lib", name])
            .status()
            .map_err(|e| format!("Failed to create plugin project: {}", e))?;

        // Update Cargo.toml
        let cargo_toml = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
flux = {{ git = "https://github.com/chocoOnEstrogen/flux" }}
"#,
            name
        );

        fs::write(
            PathBuf::from(name).join("Cargo.toml"),
            cargo_toml
        ).map_err(|e| format!("Failed to write Cargo.toml: {}", e))?;

        // Create example plugin code
        let example_code = include_str!("../../example-plugin/src/lib.rs");
        fs::write(
            PathBuf::from(name).join("src/lib.rs"),
            example_code
        ).map_err(|e| format!("Failed to write lib.rs: {}", e))?;

        println!("{}", "Plugin project created successfully!".green());
        println!("You can find it in the '{}' directory", name);
        Ok(())
    }
} 