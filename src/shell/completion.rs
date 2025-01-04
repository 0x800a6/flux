use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::Context;
use rustyline::Helper;
use std::collections::HashMap;
use std::fs;

/// Provides command and filename completion for the shell
pub struct FluxCompleter {
    filename_completer: FilenameCompleter,
    commands: Vec<String>,
}

impl FluxCompleter {
    /// Creates a new completer with the given command aliases
    /// 
    /// # Arguments
    /// * `aliases` - Map of command aliases to include in completion
    pub fn new(aliases: HashMap<String, String>) -> Self {
        // Get all directories in PATH for command completion
        let path_dirs: String = std::env::var("PATH").unwrap_or_default();
        let mut commands: Vec<String> = Vec::new();

        // Collect executables from PATH
        for dir in path_dirs.split(':') {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.filter_map(Result::ok) {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            if let Ok(metadata) = entry.metadata() {
                                // Check if file is executable
                                use std::os::unix::fs::PermissionsExt;
                                if metadata.permissions().mode() & 0o111 != 0 {
                                    if let Some(name) = entry.file_name().to_str() {
                                        commands.push(name.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Add built-in commands
        commands.extend(vec![
            "cd".to_string(),
            "exit".to_string(),
            "clear".to_string(),
            "pwd".to_string(),
            "help".to_string(),
            "alias".to_string(),
            "env".to_string(),
        ]);

        // Add aliases to commands
        commands.extend(aliases.keys().cloned());

        FluxCompleter {
            filename_completer: FilenameCompleter::new(),
            commands,
        }
    }
}

impl Completer for FluxCompleter {
    type Candidate = Pair;

    /// Provides completion suggestions for the current input
    /// 
    /// # Arguments
    /// * `line` - Current input line
    /// * `pos` - Cursor position in the line
    /// * `ctx` - Readline context
    /// 
    /// # Returns
    /// * Tuple of (start position, completion candidates)
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), rustyline::error::ReadlineError> {
        let (start, words) =
            rustyline::completion::extract_word(line, pos, None, |c| c == ' ' || c == '\t');

        // If this is the first word, complete commands
        if !line[..start].contains(char::is_whitespace) {
            let matches: Vec<Pair> = self
                .commands
                .iter()
                .filter(|cmd| cmd.starts_with(&words))
                .map(|cmd| Pair {
                    display: cmd.clone(),
                    replacement: cmd.clone(),
                })
                .collect();
            return Ok((start, matches));
        }

        // Otherwise, use filename completion
        self.filename_completer.complete(line, pos, ctx)
    }
}

impl Highlighter for FluxCompleter {}
impl Hinter for FluxCompleter {
    type Hint = String;
}
impl Validator for FluxCompleter {}
impl Helper for FluxCompleter {}
