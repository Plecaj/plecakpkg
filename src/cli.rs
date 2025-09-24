use clap::Parser;
use crate::commands::Commands;

#[derive(Parser)]
#[command(name = "plecakpkg")]
#[command(version = "0.1")]
#[command(about = "Simple dependency manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>
}