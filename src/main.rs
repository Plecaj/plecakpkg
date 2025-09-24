mod database;
mod commands;
mod cli;

use database::{Database, BuildSystem};
use cli::Cli;
use commands::{Commands, InitCommand, InstallCommand};
use clap::Parser;

fn handle_init(args: &InitCommand) -> Result<(), String>{
    println!("Initializing {}...", args.name);
    let db = Database::new(args.name.to_string(), BuildSystem::Cmake);
    Ok(())
}

fn handle_install(args: &InstallCommand) -> Result<(), String>{
    let version = args.version.clone().unwrap_or("newest".to_string());
    println!("Installing package: {} - {}", args.name, version);
    Ok(())
}
fn main() -> Result<(), String>{
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Init(args)) => {
            handle_init(&args)
        }
        Some(Commands::Install(args)) =>{
            handle_install(&args)
        }
        None =>{
            println!("No command given!");
            Err("No command given!".to_string())
        }

    }
}
