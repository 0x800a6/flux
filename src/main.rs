/// Main module for the Flux shell application
mod config;
mod plugin;
mod shell;
mod utils;

use shell::Shell;
use utils::env::{list_internal_envs, remove_internal_env, store_internal_env};

/// Main entry point for the Flux shell
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => {
                println!("Flux - An advanced, customizable shell for modern systems");
                println!("\nUSAGE:");
                println!("  flux [OPTIONS]");
                println!("  flux env [SUBCOMMAND]");
                println!("\nOPTIONS:");
                println!("  -h, --help     Display this help message");
                println!("  -v, --version  Display version information");
                println!("  config         Reconfigure the shell");
                println!("  plugin         Plugin management commands");
                println!("  env            Environment variable management");
                println!("\nENV SUBCOMMANDS:");
                println!("  env -s, --set KEY=VALUE [system|internal]    Set environment variable");
                println!("  env -r, --remove KEY [system|internal]       Remove environment variable");
                println!("  env -l, --list [system|internal]            List environment variables");
                return;
            }
            "-v" | "--version" => {
                println!("flux {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            "env" => {
                if args.len() < 3 {
                    // List all environment variables by default
                    for (key, value) in std::env::vars() {
                        println!("{}={}", key, value);
                    }
                    return;
                }

                match args[2].as_str() {
                    "-s" | "--set" => {
                        if args.len() < 4 {
                            println!("Usage: flux env -s KEY=VALUE [system|internal]");
                            return;
                        }
                        let kv_pair = &args[3];
                        let storage = args.get(4).map(|s| s.to_lowercase()).unwrap_or("system".to_string());

                        if let Some((key, value)) = kv_pair.split_once('=') {
                            match storage.as_str() {
                                "system" => std::env::set_var(key, value),
                                "internal" => {
                                    if let Err(e) = store_internal_env(key, value) {
                                        eprintln!("Failed to store internal env: {}", e);
                                        return;
                                    }
                                }
                                _ => {
                                    eprintln!("Invalid storage type. Use 'system' or 'internal'");
                                    return;
                                }
                            }
                            println!("Successfully set {}={}", key, value);
                        } else {
                            eprintln!("Invalid format. Use KEY=VALUE");
                        }
                    }
                    "-r" | "--remove" => {
                        if args.len() < 4 {
                            println!("Usage: flux env -r KEY [system|internal]");
                            return;
                        }
                        let key = &args[3];
                        let storage = args.get(4).map(|s| s.to_lowercase()).unwrap_or("system".to_string());

                        match storage.as_str() {
                            "system" => std::env::remove_var(key),
                            "internal" => {
                                if let Err(e) = remove_internal_env(key) {
                                    eprintln!("Failed to remove internal env: {}", e);
                                    return;
                                }
                            }
                            _ => {
                                eprintln!("Invalid storage type. Use 'system' or 'internal'");
                                return;
                            }
                        }
                        println!("Successfully removed {}", key);
                    }
                    "-l" | "--list" => {
                        let storage = args.get(3).map(|s| s.to_lowercase()).unwrap_or("system".to_string());

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
                                Err(e) => eprintln!("Failed to list internal envs: {}", e),
                            },
                            _ => {
                                eprintln!("Invalid storage type. Use 'system' or 'internal'");
                            }
                        }
                    }
                    _ => {
                        println!("Invalid option. Use -s (set), -r (remove), or -l (list)");
                    }
                }
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
