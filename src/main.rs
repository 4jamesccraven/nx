mod actions;
mod cli;
mod config;

use actions::*;
use cli::{Cli, SubCommand::*};

use clap::{CommandFactory, Parser};
use clap_complete::CompleteEnv;

fn main() {
    CompleteEnv::with_factory(Cli::command).complete();
    let args = Cli::parse();

    let ok = match args.command {
        Build(sub_args) => build(sub_args),
        Clean(sub_args) => clean(sub_args),
        Push => push(),
        Update => update(),
        Revert => revert(),
        Develop(sub_args) => develop(sub_args),
    };

    if let Err(e) = ok {
        eprintln!("nx: {e}");
        std::process::exit(1);
    }
}
