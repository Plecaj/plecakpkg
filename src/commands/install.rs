use git2::Repository;
use crate::cli_commands::InstallCommand;
use crate::install_helpers::{resolve_version, extract_name_from_url, get_home_dir};

pub trait RepoFetcher {
    fn fetch(url: &str, path: &str) -> Result<(), String>;
}

pub struct GitFetcher;

impl RepoFetcher for GitFetcher {
    fn fetch(url: &str, path: &str) -> Result<(), String> {
        Repository::clone(url, path)
            .map(|_| ())
            .map_err(|e| format!("Couldnt fetch repository from github, Error: {e}"))
    }
}

pub fn handle_install<F: RepoFetcher>(
    args: &InstallCommand
) -> Result<(), String> {
    let version = resolve_version(args.version.clone());
    let name = extract_name_from_url(&args.url)?;

    println!("Installing package: {} - {}", name, version);

    let binding = get_home_dir()
        .map_err(|e| format!("Failed to get home dir: {e}"))?;
    let home_dir = binding.to_str().ok_or("Home dir is not valid UTF-8")?;

    let repo_path = format!("{home_dir}/plecakpkg/repos/{name}");
    F::fetch(&args.url, &repo_path)?;

    Ok(())
}