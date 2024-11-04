pub(crate) use clap::Parser;
use clap::{Args, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about)]
pub(crate) struct Cli {
    /// Apply command to all configured package managers
    #[arg(short = 'a', long, global = true)]
    pub(crate) all_managers: bool,
    /// Run command with secified package manager
    #[arg(short = 'm', long, global = true)]
    pub(crate) manager: Option<String>,
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Create or update configuration
    Config,
    /// Show package details
    Info(PackageArgs),
    /// Install packages
    Install {
        #[arg(required = true)]
        packages: Vec<String>,
    },
    /// List installed packages
    List,
    /// Search for package
    Search(PackageArgs),
    /// List available package managers
    Managers,
    /// Update package database
    Update,
    /// Upgrade installed packages
    Upgrade,
    /// Show package manager version
    Version,
}

#[derive(Debug, Args)]
pub(crate) struct PackageArgs {
    pub(crate) package: String,
}
