mod commands;
mod completion;
mod prompt;

use crate::config::FluxConfig;
use crate::plugin::PluginManager;
use crate::shell::completion::FluxCompleter;
use crate::utils::env::set_initial_env_vars;
use rustyline::config::Configurer;
use rustyline::history::FileHistory;
use rustyline::{error::ReadlineError, Editor};
use std::path::PathBuf;

/// Main shell implementation
pub struct Shell {
    /// Shell configuration settings
    config: FluxConfig,
    /// Line editor with history and completion
    editor: Editor<FluxCompleter, FileHistory>,
    plugin_manager: PluginManager,
}

impl Shell {
    /// Creates a new shell instance with default configuration
    ///
    /// Initializes the line editor, loads history, and sets up
    /// command completion and environment variables.
    pub fn new() -> Self {
        let config_path: PathBuf = Self::get_config_path();
        let config: FluxConfig = FluxConfig::load(&config_path);

        // Create history file in config directory
        let mut history_path: PathBuf = config_path.parent().unwrap().to_path_buf();
        history_path.push("history.txt");

        // Initialize editor with custom completer
        let completer: FluxCompleter = FluxCompleter::new(config.aliases.clone());
        let mut editor: Editor<FluxCompleter, FileHistory> =
            Editor::new().expect("Failed to create editor");

        // Configure editor
        editor.set_helper(Some(completer));
        let _ = editor.set_max_history_size(config.history_size);
        editor.set_auto_add_history(true);

        // Enable completion features
        editor.set_completion_type(rustyline::CompletionType::List);
        editor.set_edit_mode(rustyline::EditMode::Emacs);

        // Load history from file
        if let Err(e) = editor.load_history(&history_path) {
            // It's okay if the file doesn't exist yet
            if !matches!(e, ReadlineError::Io(_)) {
                eprintln!("Failed to load history: {}", e);
            }
        }

        // Set environment variables from config
        set_initial_env_vars(&config.environment_variables);

        let mut plugin_manager = PluginManager::new();
        if let Err(e) = plugin_manager.load_plugins() {
            eprintln!("Failed to load plugins: {}", e);
        }

        Shell {
            config,
            editor,
            plugin_manager,
        }
    }

    /// Gets the path to the shell configuration file
    ///
    /// Creates necessary directories if they don't exist.
    ///
    /// # Returns
    /// * Path to the configuration file
    pub fn get_config_path() -> PathBuf {
        let mut path: PathBuf = if let Some(app_data) = dirs::config_dir() {
            app_data
        } else {
            PathBuf::from(".")
        };
        path.push("rip.choco.flux");
        std::fs::create_dir_all(&path).expect("Failed to create config directory");
        path.push("config.fl");
        path
    }

    /// Runs the main shell loop
    ///
    /// Continuously reads commands, processes them, and maintains
    /// command history until exit is requested.
    pub fn run(&mut self) {
        let config_path: PathBuf = Self::get_config_path();
        let history_path: PathBuf = config_path.parent().unwrap().join("history.txt");

        loop {
            let formatted_prompt: String = prompt::format_prompt(&self.config);
            match self.editor.readline(&formatted_prompt) {
                Ok(line) => {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        if let Err(e) = self.editor.add_history_entry(trimmed) {
                            eprintln!("Failed to add history entry: {}", e);
                        }
                        // Save history after each command
                        if let Err(e) = self.editor.save_history(&history_path) {
                            eprintln!("Failed to save history: {}", e);
                        }
                        commands::execute_command(trimmed, &self);
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!("exit");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
    }
}

impl Drop for Shell {
    fn drop(&mut self) {
        self.plugin_manager.cleanup();
    }
}
