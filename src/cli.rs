use clap::clap_derive::{Parser, Subcommand};
use clap_complete::shells::Shell;

#[derive(Clone, Debug, Parser)]
#[command(name = "nx", about = "Command wrapper for my nixos system")]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Clone, Debug, Subcommand)]
pub enum SubCommand {
    /// Rebuild the system from the current configuration
    #[command(alias = "b")]
    Build {
        /// Do not attempt to pull changes from GitHub before rebuilding
        #[arg(short, long)]
        fast: bool,
    },
    /// Clean up unused derivations and profiles
    Clean {
        /// Do not run nix optimise after rebuilding
        #[arg(long)]
        no_optimise: bool,
        /// Do not cache shells
        #[arg(long)]
        no_cache: bool,
    },
    /// Push changes (use after an update)
    Push,
    /// Update the system config
    Update {
        /// Only cache the shells, do not perform a system update
        #[arg(long)]
        shells_only: bool,
    },
    /// Revert to HEAD
    Revert,
    /// Wrapper for nix develop
    #[command(aliases = &["dev", "d"])]
    Develop {
        /// Shell name to run
        shell: Option<String>,
        /// Command to run (default zsh)
        command: Option<String>,
    },
    /// Generate completions to stdout
    Completions {
        /// Shell for which to generate completions
        shell: Shell,
    }
}
