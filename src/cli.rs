use clap::clap_derive::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "nx", about = "Command wrapper for my nixos system")]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[deny(missing_docs)]
#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Rebuild the system from the current configuration
    #[command(alias = "b")]
    Build(Build),
    /// Clean up unused derivations and profiles
    Clean(Clean),
    /// Push changes (use after an update)
    Push,
    /// Update the system config
    Update,
    /// Revert to HEAD
    Revert,
    /// Wrapper for nix develop
    #[command(aliases = ["dev", "d"])]
    Develop(Develop),
}

#[deny(missing_docs)]
#[derive(Parser, Debug)]
pub struct Build {
    /// Do not attempt to pull changes from GitHub before rebuilding
    #[arg(short, long)]
    pub fast: bool,
}

#[deny(missing_docs)]
#[derive(Parser, Debug)]
pub struct Clean {
    /// Do not run nix optimise after rebuilding
    #[arg(long)]
    pub no_optimise: bool,

    /// Do not cache shells
    #[arg(long)]
    pub no_cache: bool,
}

#[deny(missing_docs)]
#[derive(Parser, Debug)]
pub struct Develop {
    /// Shell name to run
    pub shell: Option<String>,

    /// Command to run (default zsh)
    #[arg(short, long)]
    pub command: Option<String>,

    /// Use a shell from the system config
    #[arg(short, long)]
    pub global: bool,
}
