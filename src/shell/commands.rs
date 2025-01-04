use crate::config::FluxConfig;
use crate::utils::env::expand_env_vars;
use colored::*;
use std::process::{Command, Stdio};
use std::time::Instant;

pub(crate) fn execute_command(cmd: &str, config: &FluxConfig) -> bool {
    let start_time: Instant = Instant::now();

    // Split command and colorize components
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if !parts.is_empty() {
        let command: ColoredString = parts[0].color(config.theme.command_color.as_str());
        let args: String = parts[1..]
            .iter()
            .map(|arg| arg.color(config.theme.args_color.as_str()).to_string())
            .collect::<Vec<_>>()
            .join(" ");

        println!("{} {}", command, args);
    }

    // First check if it's a path alias
    if let Some(path) = config.path_aliases.get(cmd) {
        if let Err(e) = std::env::set_current_dir(path) {
            print_error(&format!("Failed to change directory: {}", e), config);
        }
        if config.show_execution_time {
            let duration: std::time::Duration = start_time.elapsed();
            print_success(
                &format!("Completed in {:.2}ms", duration.as_secs_f64() * 1000.0),
                config,
            );
        }
        return true;
    }

    // Expand environment variables in the command
    let cmd: String = expand_env_vars(cmd);

    // Then check command aliases
    let cmd: String = if let Some(alias) = config.aliases.get(&cmd) {
        alias.clone()
    } else {
        cmd
    };

    let args: Vec<&str> = cmd.split_whitespace().collect();
    if args.is_empty() {
        return true;
    }

    if handle_builtin_command(&args, config) {
        if config.show_execution_time {
            let duration: std::time::Duration = start_time.elapsed();
            print_success(
                &format!("Completed in {:.2}ms", duration.as_secs_f64() * 1000.0),
                config,
            );
        }
        return true;
    }

    let output: Result<std::process::Output, std::io::Error> = Command::new(args[0])
        .args(&args[1..])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output();

    match output {
        Ok(output) if !output.status.success() => {
            print_error(
                &format!("Command failed with code: {}", output.status),
                config,
            );
        }
        Err(e) => {
            print_error(&format!("Failed to execute command: {}", e), config);
        }
        Ok(_) if config.show_execution_time => {
            let duration: std::time::Duration = start_time.elapsed();
            print_success(
                &format!("Completed in {:.2}ms", duration.as_secs_f64() * 1000.0),
                config,
            );
        }
        _ => {}
    }
    true
}

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
            match args.get(1) {
                Some(&"set") if args.len() == 4 => {
                    std::env::set_var(args[2], args[3]);
                    println!("Set {}={}", args[2], args[3]);
                }
                Some(&"unset") if args.len() == 3 => {
                    std::env::remove_var(args[2]);
                    println!("Unset {}", args[2]);
                }
                _ => {
                    for (key, value) in std::env::vars() {
                        println!("{}={}", key, value);
                    }
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

fn print_error(message: &str, config: &FluxConfig) {
    let prefix: ColoredString = "Error:".color(config.theme.error_color.as_str());
    let message: ColoredString = message.color(config.theme.error_color.as_str());
    println!("{} {}", prefix, message);
}

fn print_success(message: &str, config: &FluxConfig) {
    let prefix: ColoredString = "Success:".color(config.theme.success_color.as_str());
    let message: ColoredString = message.color(config.theme.success_color.as_str());
    println!("{} {}", prefix, message);
}

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
