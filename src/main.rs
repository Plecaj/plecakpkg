use plecakpkg::{handle_init, handle_install, Cli, Commands, GitFetcher};
use clap::Parser;
fn main() -> Result<(), String>{
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Init(args)) => {
            handle_init(&args)
        }
        Some(Commands::Install(args)) =>{
            handle_install::<GitFetcher>(&args)
        }
        None =>{
            println!("No command given!");
            Err("No command given!".to_string())
        }

    }
}
