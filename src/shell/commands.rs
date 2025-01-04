use crate::config::FluxConfig;
use crate::shell::Shell;
use crate::utils::env::expand_env_vars;
use crate::utils::env::{list_internal_envs, remove_internal_env, store_internal_env};
use colored::*;
use std::process::{Command, Stdio};
use std::time::Instant;
use which;

/// Executes a shell command with the given configuration
///
/// # Arguments
/// * `cmd` - Command string to execute
/// * `config` - Shell configuration settings
///
/// # Returns
/// * `bool` - Whether the shell should continue running
pub(crate) fn execute_command(cmd: &str, shell: &Shell) -> bool {
    let start_time: Instant = Instant::now();
    let cmd: String = expand_env_vars(cmd);
    let args: Vec<&str> = cmd.split_whitespace().collect();

    if args.is_empty() {
        return true;
    }

    // First try built-in commands
    if handle_builtin_command(&args, &shell.config) {
        if shell.config.show_execution_time {
            let duration: std::time::Duration = start_time.elapsed();
            print_success(
                &format!("Completed in {:.2}ms", duration.as_secs_f64() * 1000.0),
                &shell.config,
            );
        }
        return true;
    }

    // Then try plugin commands
    if let Some(plugin_name) = args.first() {
        match shell.plugin_manager.execute_plugin(
            plugin_name,
            &args[1..].iter().map(|s| s.to_string()).collect::<Vec<_>>(),
        ) {
            Ok(_) => return true,
            Err(e) if e.contains("not found") => {
                // Continue to system commands if plugin not found
            }
            Err(e) => {
                print_error(&format!("Plugin error: {}", e), &shell.config);
                return true;
            }
        }
    }

    // Finally, try system commands
    if is_interactive_command(args[0]) {
        let mut command: Command = Command::new(args[0]);
        command.args(&args[1..]);

        command
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        match command.spawn() {
            Ok(mut child) => {
                let status: Result<std::process::ExitStatus, std::io::Error> = child.wait();
                match status {
                    Ok(exit_status) if !exit_status.success() => {
                        print_error(
                            &format!("Command exited with code: {}", exit_status),
                            &shell.config,
                        );
                    }
                    Err(e) => {
                        print_error(&format!("Failed to run command: {}", e), &shell.config);
                    }
                    _ => {}
                }
            }
            Err(e) => {
                print_error(&format!("Failed to launch command: {}", e), &shell.config);
            }
        }
        return true;
    }

    if let Some(path) = shell.config.path_aliases.get(&cmd) {
        if let Err(e) = std::env::set_current_dir(path) {
            print_error(&format!("Failed to change directory: {}", e), &shell.config);
        }
        if shell.config.show_execution_time {
            let duration: std::time::Duration = start_time.elapsed();
            print_success(
                &format!("Completed in {:.2}ms", duration.as_secs_f64() * 1000.0),
                &shell.config,
            );
        }
        return true;
    }

    if let Some(alias) = shell.config.aliases.get(&cmd) {
        alias.clone()
    } else {
        cmd.clone()
    };

    let output: Result<std::process::Output, std::io::Error> = Command::new(args[0])
        .args(&args[1..])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output();

    match output {
        Ok(output) if !output.status.success() => {
            print_error(
                &format!("Command failed with code: {}", output.status),
                &shell.config,
            );
        }
        Err(e) => {
            print_error(&format!("Failed to execute command: {}", e), &shell.config);
        }
        Ok(_) if shell.config.show_execution_time => {
            let duration: std::time::Duration = start_time.elapsed();
            print_success(
                &format!("Completed in {:.2}ms", duration.as_secs_f64() * 1000.0),
                &shell.config,
            );
        }
        _ => {}
    }

    // Return true to keep the shell running
    true
}

/// Handles built-in shell commands
///
/// Processes internal commands like cd, exit, env, etc.
///
/// # Arguments
/// * `args` - Command arguments split into words
/// * `config` - Shell configuration settings
///
/// # Returns
/// * `bool` - Whether the command was handled as a builtin
pub(crate) fn handle_builtin_command(args: &[&str], config: &FluxConfig) -> bool {
    match args[0] {
        "exit" => {
            if args.len() > 1 {
                let exit_code: i32 = args[1].parse().unwrap_or(0);
                std::process::exit(exit_code);
            } else {
                std::process::exit(0);
            }
        }
        "cd" => {
            if args.len() > 1 {
                let path: String = resolve_path(args[1], config);
                if let Err(e) = std::env::set_current_dir(&path) {
                    print_error(&format!("Error: {}", e), config);
                }
            } else if let Some(home) = dirs::home_dir() {
                if let Err(e) = std::env::set_current_dir(home) {
                    print_error(&format!("Error: {}", e), config);
                }
            }
            true
        }
        "alias" => {
            if args.len() == 1 {
                for (alias, cmd) in &config.aliases {
                    println!("{} = '{}'", alias, cmd);
                }
            }
            true
        }
        "env" => {
            if args.len() == 1 {
                for (key, value) in std::env::vars() {
                    println!("{}={}", key, value);
                }
                return true;
            }

            match args[1] {
                "-s" | "-S" | "--set" => {
                    if args.len() < 3 {
                        print_error("Usage: env -s KEY=VALUE [system|internal]", config);
                        return true;
                    }
                    let kv_pair: &str = args[2];
                    let storage: String = args
                        .get(3)
                        .map(|s| s.to_lowercase())
                        .unwrap_or("system".to_string());

                    if let Some((key, value)) = kv_pair.split_once('=') {
                        match storage.as_str() {
                            "system" => std::env::set_var(key, value),
                            "internal" => {
                                if let Err(e) = store_internal_env(key, value) {
                                    print_error(
                                        &format!("Failed to store internal env: {}", e),
                                        config,
                                    );
                                    return true;
                                }
                            }
                            _ => {
                                print_error(
                                    "Invalid storage type. Use 'system' or 'internal'",
                                    config,
                                );
                                return true;
                            }
                        }
                        print_success(&format!("Set {}={}", key, value), config);
                    } else {
                        print_error("Invalid format. Use KEY=VALUE", config);
                    }
                }
                "-r" | "-R" | "--remove" => {
                    if args.len() < 3 {
                        print_error("Usage: env -r KEY [system|internal]", config);
                        return true;
                    }
                    let key: &str = args[2];
                    let storage: String = args
                        .get(3)
                        .map(|s| s.to_lowercase())
                        .unwrap_or("system".to_string());

                    match storage.as_str() {
                        "system" => std::env::remove_var(key),
                        "internal" => {
                            if let Err(e) = remove_internal_env(key) {
                                print_error(
                                    &format!("Failed to remove internal env: {}", e),
                                    config,
                                );
                                return true;
                            }
                        }
                        _ => {
                            print_error("Invalid storage type. Use 'system' or 'internal'", config);
                            return true;
                        }
                    }
                    print_success(&format!("Removed {}", key), config);
                }
                "-l" | "-L" | "--list" => {
                    let storage: String = args
                        .get(2)
                        .map(|s| s.to_lowercase())
                        .unwrap_or("system".to_string());

                    match storage.as_str() {
                        "system" => {
                            for (key, value) in std::env::vars() {
                                println!("{}={}", key, value);
                            }
                        }
                        "internal" => match list_internal_envs() {
                            Ok(vars) => {
                                for (key, value) in vars {
                                    println!("{}={}", key, value);
                                }
                            }
                            Err(e) => {
                                print_error(&format!("Failed to list internal envs: {}", e), config)
                            }
                        },
                        _ => {
                            print_error("Invalid storage type. Use 'system' or 'internal'", config);
                        }
                    }
                }
                _ => {
                    print_error(
                        "Invalid option. Use -s (set), -r (remove), or -l (list)",
                        config,
                    );
                }
            }
            true
        }
        "clear" => {
            print!("\x1B[2J\x1B[1;1H");
            true
        }
        "pwd" => {
            if let Ok(path) = std::env::current_dir() {
                println!("{}", path.display());
            }
            true
        }
        "help" => {
            println!("Flux Shell - An advanced, customizable shell for modern systems");
            true
        }
        _ => false,
    }
}

/// Prints an error message with appropriate formatting
///
/// # Arguments
/// * `message` - Error message to display
/// * `config` - Shell configuration for styling
fn print_error(message: &str, config: &FluxConfig) {
    let prefix: ColoredString = "Error:".color(config.theme.error_color.as_str());
    let message: ColoredString = message.color(config.theme.error_color.as_str());
    println!("{} {}", prefix, message);
}

/// Prints a success message with appropriate formatting
///
/// # Arguments
/// * `message` - Success message to display
/// * `config` - Shell configuration for styling
fn print_success(message: &str, config: &FluxConfig) {
    let prefix: ColoredString = "Success:".color(config.theme.success_color.as_str());
    let message: ColoredString = message.color(config.theme.success_color.as_str());
    println!("{} {}", prefix, message);
}

/// Resolves a path using configured path aliases
///
/// # Arguments
/// * `path` - Path string to resolve
/// * `config` - Shell configuration containing aliases
///
/// # Returns
/// * Resolved path with aliases expanded
fn resolve_path(path: &str, config: &FluxConfig) -> String {
    let mut resolved: String = path.to_string();
    for (alias, real_path) in &config.path_aliases {
        if resolved.starts_with(alias) {
            resolved = resolved.replace(alias, real_path);
            break;
        }
    }
    resolved
}

/// Determines if a command requires interactive terminal access
///
/// Checks if the command is known to be interactive (like vim, nano)
/// or if it uses terminal-related libraries.
///
/// # Arguments
/// * `cmd` - Command name to check
///
/// # Returns
/// * `bool` - Whether the command is interactive
fn is_interactive_command(cmd: &str) -> bool {
    let known_interactive: [&str; 39] = [
        "nano",
        "vim",
        "vi",
        "emacs",
        "less",
        "more",
        "top",
        "htop",
        "python",
        "python3",
        "ipython",
        "node",
        "mysql",
        "psql",
        "sqlite3",
        "mongo",
        "redis-cli",
        "ssh",
        "telnet",
        "ftp",
        "sftp",
        "tmux",
        "screen",
        "man",
        "info",
        "lynx",
        "w3m",
        "irssi",
        "weechat",
        "mutt",
        "alpine",
        "code",
        "nvim",
        "pico",
        "joe",
        "mc",
        "ranger",
        "nnn",
        "vifm",
    ];

    if known_interactive.contains(&cmd) {
        return true;
    }

    if let Ok(output) = Command::new("ldd")
        .arg(which::which(cmd).unwrap_or_default())
        .output()
    {
        if let Ok(libs) = String::from_utf8(output.stdout) {
            let interactive_libs = [
                "libncurses",
                "libtinfo",
                "libreadline",
                "libslang",
                "libtermcap",
                "libvterm",
            ];

            if interactive_libs.iter().any(|lib| libs.contains(lib)) {
                return true;
            }
        }
    }

    // Check if the command has a terminal as a dependency in its package metadata
    if let Ok(output) = Command::new("dpkg").args(["-s", cmd]).output() {
        if let Ok(info) = String::from_utf8(output.stdout) {
            let terminal_deps = [
                "ncurses", "readline", "terminal", "tty", "console", "screen", "vte",
            ];

            if terminal_deps.iter().any(|dep| info.contains(dep)) {
                return true;
            }
        }
    }

    false
}
