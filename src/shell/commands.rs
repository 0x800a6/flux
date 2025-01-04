use crate::config::FluxConfig;
use crate::shell::Shell;
use crate::utils::env::expand_env_vars;
use colored::*;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Instant;
use which;

/// Gets the system shell command and arguments
fn get_system_shell() -> (&'static str, &'static str) {
    if cfg!(windows) {
        ("cmd.exe", "/C")
    } else {
        ("sh", "-c")
    }
}

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

    let commands: Vec<CommandPart> = parse_command_chain(&cmd);
    let mut last_success: bool = true;
    let mut last_output: Option<std::process::Output> = None;

    for command in commands {
        match command.operator {
            Operator::And if !last_success => continue,
            Operator::Or if last_success => continue,
            _ => {}
        }

        let args: Vec<&str> = command.command.split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        // Try built-in commands first
        if handle_builtin_command(&args, &shell.config) {
            last_success = true;
            continue;
        }

        // Then try plugin commands
        if let Some(plugin_name) = args.first() {
            match shell.plugin_manager.execute_plugin(
                plugin_name,
                &args[1..].iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            ) {
                Ok(_) => {
                    last_success = true;
                    continue;
                }
                Err(e) if e.contains("not found") => {}
                Err(e) => {
                    print_error(&format!("Plugin error: {}", e), &shell.config);
                    last_success = false;
                    continue;
                }
            }
        }

        // Check if this is an interactive command
        if let Some(cmd_name) = args.first() {
            if is_interactive_command(cmd_name) {
                let (shell_cmd, shell_arg) = get_system_shell();
                let status: Result<std::process::ExitStatus, std::io::Error> =
                    Command::new(shell_cmd)
                        .arg(shell_arg)
                        .arg(&command.command)
                        .status();

                match status {
                    Ok(exit_status) => {
                        last_success = exit_status.success();
                    }
                    Err(e) => {
                        print_error(&format!("Failed to execute command: {}", e), &shell.config);
                        last_success = false;
                    }
                }
                continue;
            }
        }

        // Handle non-interactive commands with pipes
        let (shell_cmd, shell_arg) = get_system_shell();
        let mut command_builder: Command = Command::new(shell_cmd);
        command_builder.arg(shell_arg).arg(&command.command);

        match command.operator {
            Operator::Pipe => {
                if let Some(previous_output) = last_output.clone() {
                    command_builder.stdin(Stdio::piped());
                    match command_builder
                        .stdout(Stdio::piped())
                        .stderr(Stdio::inherit())
                        .spawn()
                    {
                        Ok(mut child) => {
                            if let Some(mut stdin) = child.stdin.take() {
                                if let Err(e) = stdin.write_all(&previous_output.stdout) {
                                    print_error(
                                        &format!("Failed to pipe data: {}", e),
                                        &shell.config,
                                    );
                                    last_success = false;
                                    continue;
                                }
                            }

                            match child.wait_with_output() {
                                Ok(output) => {
                                    last_success = output.status.success();
                                    if !matches!(command.operator, Operator::Pipe) {
                                        if let Err(e) = std::io::stdout().write_all(&output.stdout)
                                        {
                                            print_error(
                                                &format!("Failed to write output: {}", e),
                                                &shell.config,
                                            );
                                            last_success = false;
                                        }
                                    }
                                    last_output = Some(output);
                                }
                                Err(e) => {
                                    print_error(
                                        &format!("Failed to wait for command: {}", e),
                                        &shell.config,
                                    );
                                    last_success = false;
                                }
                            }
                        }
                        Err(e) => {
                            print_error(&format!("Failed to spawn command: {}", e), &shell.config);
                            last_success = false;
                        }
                    }
                }
            }
            _ => {
                match command_builder
                    .stdout(Stdio::piped())
                    .stderr(Stdio::inherit())
                    .output()
                {
                    Ok(output) => {
                        last_success = output.status.success();

                        if !command.operator.is_pipe() {
                            if let Err(e) = std::io::stdout().write_all(&output.stdout) {
                                print_error(
                                    &format!("Failed to write output: {}", e),
                                    &shell.config,
                                );
                                last_success = false;
                            }
                        }

                        last_output = Some(output);
                    }
                    Err(e) => {
                        print_error(&format!("Failed to execute command: {}", e), &shell.config);
                        last_success = false;
                    }
                }
            }
        }
    }

    if shell.config.show_execution_time {
        let duration: std::time::Duration = start_time.elapsed();
        print_success(
            &format!("Completed in {:.2}ms", duration.as_secs_f64() * 1000.0),
            &shell.config,
        );
    }

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

    // Check if the command has terminal-related libraries as dependencies
    if let Ok(output) = Command::new("ldd")
        .arg(which::which(cmd).unwrap_or_default())
        .output()
    {
        if let Ok(libs) = String::from_utf8(output.stdout) {
            let interactive_libs: [&str; 6] = [
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
            let terminal_deps: [&str; 7] = [
                "ncurses", "readline", "terminal", "tty", "console", "screen", "vte",
            ];

            if terminal_deps.iter().any(|dep| info.contains(dep)) {
                return true;
            }
        }
    }

    false
}

#[derive(Debug, PartialEq)]
enum Operator {
    None,
    And,  // &&
    Or,   // ||
    Pipe, // |
}

impl Operator {
    fn is_pipe(&self) -> bool {
        matches!(self, Operator::Pipe)
    }
}

struct CommandPart {
    command: String,
    operator: Operator,
}

fn parse_command_chain(input: &str) -> Vec<CommandPart> {
    let mut commands: Vec<CommandPart> = Vec::new();
    let mut current: String = String::new();
    let mut chars: std::iter::Peekable<std::str::Chars<'_>> = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '&' if chars.peek() == Some(&'&') => {
                chars.next(); // consume second &
                if !current.trim().is_empty() {
                    commands.push(CommandPart {
                        command: current.trim().to_string(),
                        operator: Operator::And,
                    });
                }
                current.clear();
            }
            '|' if chars.peek() == Some(&'|') => {
                chars.next(); // consume second |
                if !current.trim().is_empty() {
                    commands.push(CommandPart {
                        command: current.trim().to_string(),
                        operator: Operator::Or,
                    });
                }
                current.clear();
            }
            '|' => {
                if !current.trim().is_empty() {
                    commands.push(CommandPart {
                        command: current.trim().to_string(),
                        operator: Operator::Pipe,
                    });
                }
                current.clear();
            }
            _ => current.push(c),
        }
    }

    if !current.trim().is_empty() {
        commands.push(CommandPart {
            command: current.trim().to_string(),
            operator: Operator::None,
        });
    }

    commands
}
