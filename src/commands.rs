use clap::{Args, Subcommand};

#[derive(Args)]
#[command(
    about = "Initializes a new project",
    long_about = "This command sets up a new project with the necessary configuration files and directories."
)]
pub struct InitCommand{
    #[arg(short, long, help = "The name of the project to initialize")]
    pub name: String
}
#[derive(Args)]
#[command(
    about = "Installs a dependency from GitHub and builds it",
    long_about = "The install command fetches the specified dependency from GitHub, optionally at a specific version, and builds it locally."
)]
pub struct InstallCommand{
    #[arg(short, long, help = "The GitHub repository name of the dependency")]
    pub name: String,
    #[arg(short, long, help = "Optional version of the dependency to install")]
    pub version: Option<String>,
}
#[derive(Subcommand)]
pub enum Commands {
    Init(InitCommand),
    Install(InstallCommand),
}

