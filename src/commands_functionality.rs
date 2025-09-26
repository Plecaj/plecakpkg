use std::fmt::format;
use crate::{InitCommand, InstallCommand};
use crate::{BuildSystem, Database};
use git2::Repository;
pub fn handle_init(args: &InitCommand) -> Result<(), String>{
    println!("Initializing {}...", args.name);
    let _db = Database::new(args.name.to_string(), BuildSystem::Cmake);
    Ok(())
}

pub fn handle_install(args: &InstallCommand) -> Result<(), String>{
    let version = args.version.clone().unwrap_or("newest".to_string());
    let name = if let Some(index) = args.url.rfind('/') {
        &args.url[index+1..]
    } else {
       return Err("Couldnt find / in url".to_string())
    };

    println!("Installing package: {} - {}", name, version);
    let repo = match Repository::clone(args.url.as_str(), format!("C:/plecakpkg/repos/{name}")){
        Ok(repo) => repo,
        Err(e) => return Err(format!("Couldnt fetch repository from github, Error: {e}"))
    };

    Ok(())
}