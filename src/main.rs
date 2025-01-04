use std::path::PathBuf;
mod config;
mod shell;
mod utils;

use dialoguer::Confirm;
use shell::Shell;

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
                println!(
                    "\nFor more information, see the documentation: https://flux.choco.rip/docs"
                );
                println!("----------------------------------------");
                println!("Please consider sponsoring the project: https://github.com/sponsors/chocoOnEstrogen");
                return;
            }
            "-v" | "--version" => {
                println!("flux {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            "config" => {
                let config_path: PathBuf = Shell::get_config_path();
                // Remove existing config to trigger reconfiguration
                if let Err(e) = std::fs::remove_file(&config_path) {
                    if !matches!(e.kind(), std::io::ErrorKind::NotFound) {
                        eprintln!("Failed to remove config: {}", e);
                        return;
                    }
                }
                println!("Reconfiguring Flux Shell...");
                let _config: config::FluxConfig = config::FluxConfig::load(&config_path);
                // Yes or No
                let answer: bool = Confirm::new()
                    .with_prompt("Do you want to restart the shell to apply changes?")
                    .default(true)
                    .interact()
                    .unwrap_or(true);
                if answer {
                    std::process::exit(0);
                }
                return;
            }
            _ => {}
        }
    }

    let mut shell: Shell = Shell::new();
    shell.run();
}
