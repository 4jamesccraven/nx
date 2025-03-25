use crate::cli::Cli;

use std::io;

use clap::CommandFactory;
use clap_complete::{generate, Shell};

pub fn completions(shell: Shell) {
    let mut cli = Cli::command();
    generate(shell, &mut cli, "nx", &mut io::stdout());
}
