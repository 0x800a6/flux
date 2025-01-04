mod commands;
mod completion;
mod prompt;

use crate::config::FluxConfig;
use crate::shell::completion::FluxCompleter;
use crate::utils::env::set_initial_env_vars;
use rustyline::config::Configurer;
use rustyline::history::FileHistory;
use rustyline::{error::ReadlineError, Editor};
use std::path::PathBuf;

pub struct Shell {
    config: FluxConfig,
    editor: Editor<FluxCompleter, FileHistory>,
}

impl Shell {
    pub fn new() -> Self {
        let config_path: PathBuf = Self::get_config_path();
        let config: FluxConfig = FluxConfig::load(&config_path);

        // Create history file in config directory
        let mut history_path: PathBuf = config_path.parent().unwrap().to_path_buf();
        history_path.push("history.txt");

        // Initialize editor with custom completer
        let completer = FluxCompleter::new(config.aliases.clone());
        let mut editor = Editor::new().expect("Failed to create editor");

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

        Shell { config, editor }
    }

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
                        commands::execute_command(trimmed, &self.config);
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
