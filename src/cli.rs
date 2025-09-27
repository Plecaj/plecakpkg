use clap::Parser;
use crate::cli_commands::Commands;

#[derive(Parser)]
#[command(
    name = "plecakpkg",
    version = "1.0",
    about = "A CLI tool for managing and installing dependencies",
    long_about = "plecakpkg is a command-line tool that allows you to install dependencies and manage directly from GitHub."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>
}