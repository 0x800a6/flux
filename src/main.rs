use std::path::PathBuf;
mod config;
mod shell;
mod utils;
mod plugin;

use dialoguer::{Confirm, Select};
use shell::Shell;

/// Main entry point for the Flux shell
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => {
                println!("Flux - An advanced, customizable shell for modern systems");
                println!("\nUSAGE:");
                println!("  flux [OPTIONS]");
                println!("\nOPTIONS:");
                println!("  -h, --help     Display this help message");
                println!("  -v, --version  Display version information");
                println!("  config         Reconfigure the shell");
                println!("  plugin         Plugin management commands");
                println!("    install      Install a plugin from git");
                println!("    init         Create a new plugin project");
                return;
            }
            "-v" | "--version" => {
                println!("flux {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            "config" => {
                let config_path = Shell::get_config_path();
                if let Err(e) = std::fs::remove_file(&config_path) {
                    if !matches!(e.kind(), std::io::ErrorKind::NotFound) {
                        eprintln!("Failed to remove config: {}", e);
                        return;
                    }
                }
                println!("Reconfiguring Flux Shell...");
                let _config = config::FluxConfig::load(&config_path);
                let answer = Confirm::new()
                    .with_prompt("Do you want to restart the shell to apply changes?")
                    .default(true)
                    .interact()
                    .unwrap_or(true);
                if answer {
                    std::process::exit(0);
                }
                return;
            }
            "plugin" => {
                if args.len() < 3 {
                    println!("Usage: flux plugin <install|init> [args...]");
                    return;
                }

                match args[2].as_str() {
                    "install" => {
                        if args.len() != 4 {
                            println!("Usage: flux plugin install <git-url>");
                            return;
                        }
                        let installer = plugin::PluginManager::new();
                        if let Err(e) = installer.install_from_git(&args[3]) {
                            eprintln!("Failed to install plugin: {}", e);
                        }
                    }
                    "init" => {
                        if args.len() != 4 {
                            println!("Usage: flux plugin init <name>");
                            return;
                        }
                        let installer = plugin::PluginManager::new();
                        if let Err(e) = installer.init_plugin(&args[3]) {
                            eprintln!("Failed to create plugin: {}", e);
                        }
                    }
                    _ => {
                        println!("Unknown plugin command. Use: install or init");
                    }
                }
                return;
            }
            _ => {}
        }
    }

    let mut shell = Shell::new();
    shell.run();
}
