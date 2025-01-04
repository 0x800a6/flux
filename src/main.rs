use std::path::PathBuf;
mod config;
mod plugin;
mod shell;
mod utils;

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
                println!("    update       Update a plugin from git");
                println!("    remove       Remove a plugin");
                println!("    list         List installed plugins");
                return;
            }
            "-v" | "--version" => {
                println!("flux {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            "config" => {
                let config_path: PathBuf = Shell::get_config_path();
                if let Err(e) = std::fs::remove_file(&config_path) {
                    if !matches!(e.kind(), std::io::ErrorKind::NotFound) {
                        eprintln!("Failed to remove config: {}", e);
                        return;
                    }
                }
                println!("Reconfiguring Flux Shell...");
                let _config: config::FluxConfig = config::FluxConfig::load(&config_path);
                println!("Reconfiguration complete. Please restart the shell to apply changes.");
            }
            "plugin" => {
                if args.len() < 3 {
                    println!("Usage: flux plugin <command> [args...]");
                    println!("\nCommands:");
                    println!("  install <git-url>  Install a plugin from git");
                    println!("  init <name>        Create a new plugin project");
                    println!("  list               List installed plugins");
                    println!("  remove <name>      Remove an installed plugin");
                    println!("  update <name>      Update an installed plugin");
                    return;
                }

                let plugin_manager: plugin::PluginManager =
                    plugin::PluginManager::new_without_loading();
                match args[2].as_str() {
                    "install" => {
                        if args.len() != 4 {
                            println!("Usage: flux plugin install <git-url>");
                            return;
                        }
                        if let Err(e) = plugin_manager.install_from_git(&args[3]) {
                            eprintln!("Failed to install plugin: {}", e);
                        }
                    }
                    "init" => {
                        if args.len() != 4 {
                            println!("Usage: flux plugin init <name>");
                            return;
                        }
                        if let Err(e) = plugin_manager.init_plugin(&args[3]) {
                            eprintln!("Failed to create plugin: {}", e);
                        }
                    }
                    "list" => {
                        if let Err(e) = plugin_manager.list_plugins() {
                            eprintln!("Failed to list plugins: {}", e);
                        }
                    }
                    "remove" => {
                        if args.len() != 4 {
                            println!("Usage: flux plugin remove <name>");
                            return;
                        }
                        if let Err(e) = plugin_manager.remove_plugin(&args[3]) {
                            eprintln!("Failed to remove plugin: {}", e);
                        }
                    }
                    "update" => {
                        if args.len() != 4 {
                            println!("Usage: flux plugin update <name>");
                            return;
                        }
                        if let Err(e) = plugin_manager.update_plugin(&args[3]) {
                            eprintln!("Failed to update plugin: {}", e);
                        }
                    }
                    _ => {
                        println!(
                            "Unknown plugin command. Use: install, init, list, remove, or update"
                        );
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
