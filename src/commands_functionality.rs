use crate::{InitCommand, InstallCommand};
use crate::{BuildSystem, Database};

pub fn handle_init(args: &InitCommand) -> Result<(), String>{
    println!("Initializing {}...", args.name);
    let _db = Database::new(args.name.to_string(), BuildSystem::Cmake);
    Ok(())
}

pub fn handle_install(args: &InstallCommand) -> Result<(), String>{
    let version = args.version.clone().unwrap_or("newest".to_string());
    println!("Installing package: {} - {}", args.name, version);
    Ok(())
}