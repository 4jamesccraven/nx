use clap::clap_derive::{Parser, Subcommand};
use clap::crate_version;

#[derive(Parser, Debug)]
#[command(
    name = "nx",
    version = crate_version!(),
    about = concat!(
        "a wrapper of a wrapper of the nix cli ",
        "suite, which is a wrapper of the nix command"
    ),
    disable_help_subcommand = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[deny(missing_docs)]
#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Build the system
    #[command(alias = "b")]
    Build(Build),
    /// Clean unused store paths
    Clean(Clean),
    /// Push a system upate
    Push,
    /// Update the system
    Update,
    /// Revert to HEAD
    Revert,
    /// Activate a devShell
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
