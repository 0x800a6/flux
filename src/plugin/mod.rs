use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use uuid::Uuid;
use dialoguer::{Confirm, Select};
use colored::*;
use std::fs;

pub trait FluxPlugin {
    /// Get the name of the plugin
    fn name(&self) -> &str;
    
    /// Get the version of the plugin
    fn version(&self) -> &str {
        "0.1.0"  // Default version
    }
    
    /// Get plugin description
    fn description(&self) -> &str {
        "No description provided"
    }
    
    /// Get available commands
    fn commands(&self) -> Vec<(&str, &str)> {
        Vec::new()  // (command, description) pairs
    }
    
    /// Initialize the plugin
    fn init(&mut self) -> Result<(), String>;
    
    /// Execute a plugin command
    fn execute(&self, args: &[String]) -> Result<(), String>;
    
    /// Clean up plugin resources
    fn cleanup(&mut self) -> Result<(), String>;
    
    /// Handle plugin configuration
    fn configure(&mut self, config: &str) -> Result<(), String> {
        Ok(())  // Default: no configuration needed
    }
    
    /// Get plugin help text
    fn help(&self) -> String {
        format!("Plugin: {}\nVersion: {}\n{}\n\nCommands:\n{}", 
            self.name(),
            self.version(),
            self.description(),
            self.commands()
                .iter()
                .map(|(cmd, desc)| format!("  {} - {}", cmd, desc))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

pub struct PluginManager {
    plugins: HashMap<String, (Library, Box<dyn FluxPlugin + 'static>)>,
    plugin_dir: PathBuf,
    temp_dir: PathBuf,
}

impl PluginManager {
    pub fn new() -> Self {
        let plugin_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rip.choco.flux")
            .join("plugins");
        
        let temp_dir = plugin_dir.join("temp");
        
        std::fs::create_dir_all(&plugin_dir).unwrap_or_default();
        std::fs::create_dir_all(&temp_dir).unwrap_or_default();
        
        PluginManager {
            plugins: HashMap::new(),
            plugin_dir,
            temp_dir,
        }
    }

    pub fn load_plugins(&mut self) -> Result<(), String> {
        for entry in std::fs::read_dir(&self.plugin_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            
            if let Some(extension) = path.extension() {
                if extension == std::env::consts::DLL_EXTENSION {
                    self.load_plugin(&path)?;
                }
            }
        }
        Ok(())
    }

    fn load_plugin(&mut self, path: &PathBuf) -> Result<(), String> {
        unsafe {
            let library = Library::new(path).map_err(|e| e.to_string())?;
            
            // Get the plugin creation function
            let create_plugin: Symbol<fn() -> Box<dyn FluxPlugin>> = 
                library.get(b"create_plugin").map_err(|e| e.to_string())?;
            
            let mut plugin = create_plugin();
            let name = plugin.name().to_string();
            
            plugin.init()?;
            self.plugins.insert(name, (library, plugin));
        }
        Ok(())
    }

    pub fn execute_plugin(&self, name: &str, args: &[String]) -> Result<(), String> {
        if let Some((_, plugin)) = self.plugins.get(name) {
            plugin.execute(args)
        } else {
            Err(format!("Plugin '{}' not found", name))
        }
    }

    pub fn cleanup(&mut self) {
        for (_, (_, mut plugin)) in self.plugins.drain() {
            plugin.cleanup().unwrap_or_default();
        }
    }

    pub fn install_from_git(&self, url: &str) -> Result<(), String> {
        println!("{}", "⚠️  Warning: Installing plugins can be dangerous as they run with the same permissions as the shell."
            .bright_yellow());
        println!("{}", "Please review the code before installing.".bright_yellow());

        let uuid = Uuid::new_v4();
        let temp_path = self.temp_dir.join(uuid.to_string());
        
        Command::new("git")
            .args(["clone", url, temp_path.to_str().unwrap()])
            .output()
            .map_err(|e| format!("Failed to clone repository: {}", e))?;

        let options = vec!["[O]pen code for review", "[P]roceed with installation", "[C]ancel"];
        let selection = Select::new()
            .with_prompt("What would you like to do?")
            .items(&options)
            .default(0)
            .interact()
            .map_err(|e| e.to_string())?;

        match selection {
            0 => {
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
                println!("{}", "Proceeding without code review...".bright_yellow());
            }
            _ => {
                fs::remove_dir_all(&temp_path).ok();
                return Ok(());
            }
        }

        Command::new("cargo")
            .args(["build", "--release"])
            .current_dir(&temp_path)
            .status()
            .map_err(|e| format!("Failed to build plugin: {}", e))?;

        let plugin_file = format!("{}.flp", uuid);
        let target_path = self.plugin_dir.join(&plugin_file);
        
        let lib_name = temp_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| "Invalid temp directory name".to_string())?;

        let lib_file = if cfg!(target_os = "windows") {
            format!("lib{}.dll", lib_name)
        } else if cfg!(target_os = "macos") {
            format!("lib{}.dylib", lib_name)
        } else {
            format!("lib{}.so", lib_name)
        };

        let source_path = temp_path
            .join("target")
            .join("release")
            .join(lib_file);

        fs::copy(&source_path, &target_path)
            .map_err(|e| format!("Failed to install plugin: {}", e))?;

        fs::remove_dir_all(&temp_path).ok();
        
        println!("{}", "Plugin installed successfully!".green());
        Ok(())
    }

    pub fn init_plugin(&self, name: &str) -> Result<(), String> {

        // Check if the plugin already exists
        if self.plugins.contains_key(name) {
            return Err(format!("Plugin '{}' already exists", name));
        }

        // Check if rust is installed
        if !which::which("cargo").is_ok() {
            return Err("Cargo is not installed".to_string());
        }

        Command::new("cargo")
            .args(["new", "--lib", name])
            .status()
            .map_err(|e| format!("Failed to create plugin project: {}", e))?;

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

        let example_code = include_str!("../../examples/plugin/src/lib.rs");
        fs::write(
            PathBuf::from(name).join("src/lib.rs"),
            example_code
        ).map_err(|e| format!("Failed to write lib.rs: {}", e))?;

        println!("{}", "Plugin project created successfully!".green());
        println!("You can find it in the '{}' directory", name);
        Ok(())
    }
} 