use crate::config::FluxConfig;
use chrono::Local;
use colored::*;
use std::env;
use std::process::Command;

/// Formats the shell prompt according to the configuration
/// 
/// Replaces placeholders in the prompt template with actual values:
/// - {user}: Current username
/// - {host}: System hostname
/// - {dir}: Current directory
/// - {git}: Git branch (if applicable)
/// - {time}: Current time
/// 
/// # Arguments
/// * `config` - Shell configuration containing prompt settings
/// 
/// # Returns
/// * Formatted prompt string with colors and replacements
pub(crate) fn format_prompt(config: &FluxConfig) -> String {
    let mut prompt: String = config.prompt_template.clone();

    // Username with separator
    if config.show_username {
        let username: String = env::var("USER")
            .or_else(|_| env::var("USERNAME"))
            .unwrap_or_else(|_| "user".to_string());
        let formatted_username = username
            .color(config.theme.username_color.as_str())
            .to_string();
        prompt = prompt.replace("{user}", &formatted_username);
    }

    // Hostname with separator
    if config.show_hostname {
        let hostname: String = hostname::get()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let formatted_hostname = hostname
            .color(config.theme.hostname_color.as_str())
            .to_string();
        prompt = prompt.replace("{host}", &formatted_hostname);
    }

    // Current directory
    let home_dir: String = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| {
            dirs::home_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        });

    let current_dir: String = env::current_dir()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
        .replace(&home_dir, "~");

    let formatted_dir: String = current_dir
        .color(config.theme.directory_color.as_str())
        .to_string();
    prompt = prompt.replace("{dir}", &formatted_dir);

    // Git branch
    if config.show_git_branch {
        let git_branch: String = get_git_branch()
            .map(|b| b.trim().to_string())
            .unwrap_or_default();
        if !git_branch.is_empty() {
            let formatted_branch: String = git_branch
                .color(config.theme.git_branch_color.as_str())
                .to_string();
            prompt = prompt.replace(" on {git}", &format!(" on {}", formatted_branch));
        } else {
            prompt = prompt.replace(" on {git}", "");
        }
    }

    // Time
    if config.show_time {
        let time: String = Local::now().format(&config.time_format).to_string();
        let formatted_time: String = time.color(config.theme.time_color.as_str()).to_string();
        prompt = prompt.replace("{time}", &formatted_time);
    }

    // Add final prompt symbol with accent color
    prompt = prompt.replace(
        "λ",
        &"λ".color(config.theme.accent_color.as_str()).to_string(),
    );

    prompt
}

/// Gets the current git branch name
/// 
/// Executes `git rev-parse --abbrev-ref HEAD` to get the branch name
/// 
/// # Returns
/// * `Option<String>` - Branch name if in a git repository, None otherwise
pub(crate) fn get_git_branch() -> Option<String> {
    let output: std::process::Output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()?;

    if output.status.success() {
        String::from_utf8(output.stdout)
            .ok()
            .map(|s| s.trim().to_string())
    } else {
        None
    }
}
