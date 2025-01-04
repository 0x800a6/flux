use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Theme {
    pub prompt_color: String,
    pub error_color: String,
    pub success_color: String,
    pub username_color: String,
    pub hostname_color: String,
    pub directory_color: String,
    pub git_branch_color: String,
    pub time_color: String,
    pub command_color: String,
    pub args_color: String,
    pub path_color: String,
    pub accent_color: String,
    pub separator_color: String,
}

impl Theme {
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
