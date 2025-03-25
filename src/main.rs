mod actions;
mod cli;

use actions::*;
use cli::{Cli, SubCommand::*};

use clap::Parser;

fn main() {
    let args = Cli::parse();

    let ok = match args.command {
        Build { fast } => build(fast),
        Clean {
            no_optimise,
            no_cache,
        } => clean(no_optimise, no_cache),
        Push => push(),
        Update { shells_only } => update(shells_only),
        Revert => revert(),
        Develop { shell, command } => develop(shell, command),
    };

    if let Err(e) = ok {
        eprintln!("nx: {e}");
        std::process::exit(1);
    }
}
