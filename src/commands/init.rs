use crate::{BuildSystem, Package};
use crate::cli_commands::InitCommand;

pub fn handle_init(args: &InitCommand) -> Result<(), String>{
    println!("Initializing {}...", args.name);
    let _package = Package::new(args.name.to_string(), BuildSystem::Cmake);
    Ok(())
}