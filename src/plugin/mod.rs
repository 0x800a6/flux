use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use colored::*;
use dialoguer::{Confirm, Select};
use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use uuid::Uuid;

pub trait FluxPlugin {
    /// Get the name of the plugin
    fn name(&self) -> &str;

    /// Get the version of the plugin
    fn version(&self) -> &str {
        "0.1.0" // Default version
    }

    /// Get plugin description
    fn description(&self) -> &str {
        "No description provided"
    }

    /// Get available commands
    fn commands(&self) -> Vec<(&str, &str)> {
        Vec::new() // (command, description) pairs
    }

    /// Initialize the plugin
    fn init(&mut self) -> Result<(), String>;

    /// Execute a plugin command
    fn execute(&self, args: &[String]) -> Result<(), String>;

    /// Clean up plugin resources
    fn cleanup(&mut self) -> Result<(), String>;

    /// Handle plugin configuration
    #[allow(dead_code)]
    fn configure(&mut self) -> Result<(), String> {
        Ok(()) // Default: no configuration needed
    }

    /// Get plugin help text
    #[allow(dead_code)]
    fn help(&self) -> String {
        format!(
            "Plugin: {}\nVersion: {}\n{}\n\nCommands:\n{}",
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
        let plugin_dir: PathBuf = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rip.choco.flux")
            .join("plugins");

        let temp_dir: PathBuf = plugin_dir.join("temp");

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
            let entry: fs::DirEntry = entry.map_err(|e| e.to_string())?;
            let path: PathBuf = entry.path();

            if let Some(extension) = path.extension() {
                if extension == "flp" {
                    self.load_plugin(&path)?;
                }
            }
        }
        Ok(())
    }

    fn load_plugin(&mut self, path: &PathBuf) -> Result<(), String> {
        unsafe {
            let library: Library = Library::new(path).map_err(|e| e.to_string())?;

            let create_plugin: Symbol<fn() -> Box<dyn FluxPlugin>> =
                library.get(b"create_plugin").map_err(|e| e.to_string())?;

            let mut plugin: Box<dyn FluxPlugin> = create_plugin();
            let name: String = plugin.name().to_string();

            plugin.init()?;
            self.plugins.insert(name, (library, plugin));
        }
        Ok(())
    }

    pub fn execute_plugin(&self, name: &str, args: &[String]) -> Result<(), String> {
        if let Some((_, plugin)) = self.plugins.get(name) {
            plugin.execute(args)
        } else {
            Err(format!(
                "Plugin '{}' not found. Available plugins: {}",
                name,
                self.plugins
                    .keys()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
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
        println!(
            "{}",
            "Please review the code before installing.".bright_yellow()
        );

        let uuid: Uuid = Uuid::new_v4();
        let temp_path: PathBuf = self.temp_dir.join(uuid.to_string());

        Command::new("git")
            .args(["clone", url, temp_path.to_str().unwrap()])
            .output()
            .map_err(|e| format!("Failed to clone repository: {}", e))?;

        let options = vec![
            "[O]pen code for review",
            "[P]roceed with installation",
            "[C]ancel",
        ];
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
                    .unwrap_or(false)
                {
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

        let encoded_url: String = BASE64.encode(url);
        let plugin_file: String = format!("{}.{}.flp", uuid, encoded_url);
        let target_path: PathBuf = self.plugin_dir.join(&plugin_file);

        let lib_prefix: &str = "lib";
        let lib_name: &str = "test_lib";
        let lib_ext: &str = if cfg!(target_os = "windows") {
            "dll"
        } else if cfg!(target_os = "macos") {
            "dylib"
        } else {
            "so"
        };

        let lib_file: String = format!("{}{}.{}", lib_prefix, lib_name, lib_ext);
        let source_path: PathBuf = temp_path.join("target").join("release").join(lib_file);

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

        fs::write(PathBuf::from(name).join("Cargo.toml"), cargo_toml)
            .map_err(|e| format!("Failed to write Cargo.toml: {}", e))?;

        let example_code = include_str!("../../examples/plugin/src/lib.rs");
        fs::write(PathBuf::from(name).join("src/lib.rs"), example_code)
            .map_err(|e| format!("Failed to write lib.rs: {}", e))?;

        println!("{}", "Plugin project created successfully!".green());
        println!("You can find it in the '{}' directory", name);
        Ok(())
    }

    pub fn list_plugins(&self) -> Result<(), String> {
        println!("{}", "Installed plugins:".bright_yellow());
        for entry in std::fs::read_dir(&self.plugin_dir).map_err(|e| e.to_string())? {
            let entry: fs::DirEntry = entry.map_err(|e| e.to_string())?;
            let path: PathBuf = entry.path();

            if let Some(extension) = path.extension() {
                if extension == "flp" && path.file_name().unwrap().to_string_lossy() != "temp" {
                    if let Some(name) = path.file_stem() {
                        let name_str = name.to_string_lossy();
                        let parts: Vec<&str> = name_str.split('.').collect();
                        if parts.len() >= 2 {
                            if let Ok(url) = BASE64.decode(parts[1]) {
                                if let Ok(url_str) = String::from_utf8(url) {
                                    if let Ok(info) = self.get_plugin_info(&path) {
                                        println!(
                                            "  {} (v{}) - {}",
                                            info.name, info.version, info.description
                                        );
                                        println!("    Source: {}", url_str);
                                        println!("    Commands:");
                                        for (cmd, desc) in info.commands {
                                            println!("      {} - {}", cmd, desc);
                                        }
                                        println!();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn get_plugin_info(&self, path: &PathBuf) -> Result<PluginInfo, String> {
        unsafe {
            let library: Library = Library::new(path).map_err(|e| e.to_string())?;
            let create_plugin: Symbol<fn() -> Box<dyn FluxPlugin>> =
                library.get(b"create_plugin").map_err(|e| e.to_string())?;

            let plugin: Box<dyn FluxPlugin> = create_plugin();
            Ok(PluginInfo {
                name: plugin.name().to_string(),
                version: plugin.version().to_string(),
                description: plugin.description().to_string(),
                commands: plugin
                    .commands()
                    .into_iter()
                    .map(|(cmd, desc)| (cmd.to_string(), desc.to_string()))
                    .collect(),
            })
        }
    }

    pub fn remove_plugin(&self, name: &str) -> Result<(), String> {
        for entry in std::fs::read_dir(&self.plugin_dir).map_err(|e| e.to_string())? {
            let entry: fs::DirEntry = entry.map_err(|e| e.to_string())?;
            let path: PathBuf = entry.path();

            if let Some(extension) = path.extension() {
                if extension == "flp" {
                    if let Ok(info) = self.get_plugin_info(&path) {
                        if info.name == name {
                            return std::fs::remove_file(path)
                                .map_err(|e| format!("Failed to remove plugin: {}", e));
                        }
                    }
                }
            }
        }
        Err(format!("Plugin '{}' not found", name))
    }

    pub fn update_plugin(&self, name: &str) -> Result<(), String> {
        // Find the plugin and its URL
        for entry in std::fs::read_dir(&self.plugin_dir).map_err(|e| e.to_string())? {
            let entry: fs::DirEntry = entry.map_err(|e| e.to_string())?;
            let path: PathBuf = entry.path();

            if let Some(extension) = path.extension() {
                if extension == "flp" {
                    if let Ok(info) = self.get_plugin_info(&path) {
                        if info.name == name {
                            let name_str = path.file_stem().unwrap().to_string_lossy();
                            let parts: Vec<&str> = name_str.split('.').collect();
                            if parts.len() >= 2 {
                                if let Ok(url) = BASE64.decode(parts[1]) {
                                    if let Ok(url_str) = String::from_utf8(url) {
                                        // Remove old plugin
                                        std::fs::remove_file(&path).map_err(|e| {
                                            format!("Failed to remove old plugin: {}", e)
                                        })?;

                                        // Install updated version
                                        return self.install_from_git(&url_str);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(format!("Plugin '{}' not found", name))
    }

    pub fn new_without_loading() -> Self {
        let plugin_dir: PathBuf = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rip.choco.flux")
            .join("plugins");

        let temp_dir: PathBuf = plugin_dir.join("temp");

        std::fs::create_dir_all(&plugin_dir).unwrap_or_default();
        std::fs::create_dir_all(&temp_dir).unwrap_or_default();

        PluginManager {
            plugins: HashMap::new(),
            plugin_dir,
            temp_dir,
        }
    }
}

#[derive(Debug)]
struct PluginInfo {
    name: String,
    version: String,
    description: String,
    commands: Vec<(String, String)>,
}
