use clap::{Args, Subcommand};

#[derive(Args)]
pub struct InitCommand{
    #[arg(short, long)]
    pub name: String
}
#[derive(Args)]
#[command(about = "Installs provided dependency from github and builds it")]
pub struct InstallCommand{
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long)]
    pub version: Option<String>,
}
#[derive(Subcommand)]
pub enum Commands {
    Init(InitCommand),
    Install(InstallCommand),
}
