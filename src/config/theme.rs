use serde::{Deserialize, Serialize};

/// Theme configuration for the shell's visual appearance
#[derive(Debug, Serialize, Deserialize)]
pub struct Theme {
    /// Color for the main prompt text
    pub prompt_color: String,
    /// Color for error messages
    pub error_color: String,
    /// Color for success messages
    pub success_color: String,
    /// Color for displaying username
    pub username_color: String,
    /// Color for displaying hostname
    pub hostname_color: String,
    /// Color for displaying current directory
    pub directory_color: String,
    /// Color for displaying git branch information
    pub git_branch_color: String,
    /// Color for displaying time
    pub time_color: String,
    /// Color for displaying command text
    pub command_color: String,
    /// Color for displaying command arguments
    pub args_color: String,
    /// Color for displaying file paths
    pub path_color: String,
    /// Color for accent elements in the prompt
    pub accent_color: String,
    /// Color for separator elements
    pub separator_color: String,
}

impl Theme {
    /// Creates a minimal theme with basic monochrome colors
    pub fn minimal() -> Self {
        Theme {
            prompt_color: "white".to_string(),
            error_color: "red".to_string(),
            success_color: "green".to_string(),
            username_color: "white".to_string(),
            hostname_color: "white".to_string(),
            directory_color: "cyan".to_string(),
            git_branch_color: "white".to_string(),
            time_color: "white".to_string(),
            command_color: "white".to_string(),
            args_color: "white".to_string(),
            path_color: "white".to_string(),
            accent_color: "white".to_string(),
            separator_color: "white".to_string(),
        }
    }

    /// Creates a full-featured theme with distinct colors
    pub fn full() -> Self {
        Theme {
            prompt_color: "cyan".to_string(),
            error_color: "red".to_string(),
            success_color: "green".to_string(),
            username_color: "yellow".to_string(),
            hostname_color: "blue".to_string(),
            directory_color: "magenta".to_string(),
            git_branch_color: "green".to_string(),
            time_color: "white".to_string(),
            command_color: "bright cyan".to_string(),
            args_color: "bright white".to_string(),
            path_color: "bright blue".to_string(),
            accent_color: "yellow".to_string(),
            separator_color: "white".to_string(),
        }
    }

    /// Creates a powerline-inspired theme with bright colors
    pub fn powerline() -> Self {
        Theme {
            prompt_color: "cyan".to_string(),
            error_color: "red".to_string(),
            success_color: "green".to_string(),
            username_color: "bright yellow".to_string(),
            hostname_color: "bright blue".to_string(),
            directory_color: "bright magenta".to_string(),
            git_branch_color: "bright green".to_string(),
            time_color: "bright white".to_string(),
            command_color: "bright cyan".to_string(),
            args_color: "bright white".to_string(),
            path_color: "bright blue".to_string(),
            accent_color: "bright yellow".to_string(),
            separator_color: "bright white".to_string(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::full()
    }
}
