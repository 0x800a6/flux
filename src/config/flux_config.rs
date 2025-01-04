use super::Theme;
use dialoguer::{Confirm, Select};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct FluxConfig {
    pub prompt: String,
    pub theme: Theme,
    pub aliases: HashMap<String, String>,
    pub history_size: usize,
    pub show_execution_time: bool,
    pub path_aliases: HashMap<String, String>,
    pub environment_variables: HashMap<String, String>,
    pub prompt_template: String,
    pub show_git_branch: bool,
    pub show_time: bool,
    pub show_username: bool,
    pub show_hostname: bool,
    pub time_format: String,
}

impl FluxConfig {
    pub fn minimal() -> Self {
        FluxConfig {
            prompt: "λ ".to_string(),
            prompt_template: "{dir} λ ".to_string(),
            show_git_branch: false,
            show_time: false,
            show_username: false,
            show_hostname: false,
            time_format: "%H:%M:%S".to_string(),
            theme: Theme::minimal(),
            aliases: HashMap::new(),
            path_aliases: Self::default_path_aliases(),
            environment_variables: HashMap::new(),
            show_execution_time: false,
            history_size: 1000,
        }
    }

    fn default_path_aliases() -> HashMap<String, String> {
        let mut aliases: HashMap<String, String> = HashMap::new();
        aliases.insert(
            "~".to_string(),
            dirs::home_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        );
        aliases.insert(
            "@docs".to_string(),
            dirs::document_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        );
        aliases.insert(
            "@dl".to_string(),
            dirs::download_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        );
        aliases
    }

    pub fn full() -> Self {
        let mut aliases: HashMap<String, String> = HashMap::new();
        aliases.insert("ll".to_string(), "ls -l".to_string());
        aliases.insert("la".to_string(), "ls -la".to_string());
        aliases.insert("gst".to_string(), "git status".to_string());
        aliases.insert("gp".to_string(), "git push".to_string());
        aliases.insert("gc".to_string(), "git commit".to_string());

        let mut path_aliases: HashMap<String, String> = HashMap::new();
        path_aliases.insert(
            "~".to_string(),
            dirs::home_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        );
        path_aliases.insert(
            "@docs".to_string(),
            dirs::document_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        );
        path_aliases.insert(
            "@dl".to_string(),
            dirs::download_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        );

        let mut env_vars: HashMap<String, String> = HashMap::new();
        env_vars.insert("EDITOR".to_string(), "vim".to_string());
        env_vars.insert("VISUAL".to_string(), "code".to_string());

        FluxConfig {
            prompt: "flux λ ".to_string(),
            theme: Theme::full(),
            aliases,
            history_size: 10000,
            show_execution_time: true,
            path_aliases,
            environment_variables: env_vars,
            prompt_template: "[{time}] {user}@{host} in {dir} on {git}\nλ ".to_string(),
            show_git_branch: true,
            show_time: true,
            show_username: true,
            show_hostname: true,
            time_format: "%H:%M:%S".to_string(),
        }
    }

    pub fn powerline() -> Self {
        let mut config: FluxConfig = Self::full();
        config.prompt_template = "╭─[{time}] {user}@{host} in {dir} on {git}\n╰─λ ".to_string();
        config.theme = Theme::powerline();
        config
    }

    pub fn load(config_path: &PathBuf) -> Self {
        if let Ok(contents) = fs::read_to_string(config_path) {
            if let Ok(config) = serde_json::from_str(&contents) {
                return config;
            }

            // Try to read config type from first line
            let first_line: Option<&str> = contents.lines().next();
            match first_line {
                Some("#minimal") => return FluxConfig::minimal(),
                Some("#powerline") => return FluxConfig::powerline(),
                _ => return FluxConfig::full(),
            }
        }

        // Config doesn't exist, show TUI
        println!("\nWelcome to Flux Shell! Let's set up your configuration.\n");

        let options: Vec<&str> = vec![
            "Minimal (Simple prompt)",
            "Full (Rich features)",
            "Powerline (Fancy style)",
        ];
        let selection: usize = Select::new()
            .with_prompt("Choose your preferred configuration style")
            .items(&options)
            .default(1)
            .interact()
            .unwrap_or(1);

        let show_git: bool = Confirm::new()
            .with_prompt("Show git branch in prompt?")
            .default(true)
            .interact()
            .unwrap_or(true);

        let show_time: bool = Confirm::new()
            .with_prompt("Show time in prompt?")
            .default(true)
            .interact()
            .unwrap_or(true);

        let mut config: FluxConfig = match selection {
            0 => FluxConfig::minimal(),
            2 => FluxConfig::powerline(),
            _ => FluxConfig::full(),
        };

        // Apply user choices
        config.show_git_branch = show_git;
        config.show_time = show_time;

        // Update prompt template based on choices
        let mut template_parts: Vec<&str> = Vec::new();

        if config.show_time {
            template_parts.push("[{time}]");
        }
        if config.show_username {
            template_parts.push("{user}@{host}");
        }
        template_parts.push("in {dir}");
        if config.show_git_branch {
            template_parts.push("on {git}");
        }

        match selection {
            2 => {
                // Powerline
                config.prompt_template = format!("╭─{}\n╰─λ ", template_parts.join(" "));
            }
            _ => {
                config.prompt_template = format!("{}\nλ ", template_parts.join(" "));
            }
        }

        // Save the config
        if let Ok(config_str) = serde_json::to_string_pretty(&config) {
            if let Err(e) = fs::write(config_path, config_str) {
                eprintln!("Failed to write config: {}", e);
            }
        }

        println!(
            "\nConfiguration saved! You can modify it later at: {}",
            config_path.display()
        );
        println!("To change your configuration, delete the file or run the command `flux config`.");

        config
    }
}

impl Default for FluxConfig {
    fn default() -> Self {
        Self::full()
    }
}
