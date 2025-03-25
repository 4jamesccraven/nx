use std::env;
use std::io::Error;

use clap::CommandFactory;
use clap_complete::{generate_to, shells::*};

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(out) => out,
    };

    let mut parser = Cli::command();

    let path = generate_to(Zsh, &mut parser, "nx", outdir.clone());
    println!("cargo:warning=completion file is generated: {path:?}");

    let path = generate_to(Bash, &mut parser, "nx", outdir.clone());
    println!("cargo:warning=completion file is generated: {path:?}");

    let path = generate_to(Fish, &mut parser, "nx", outdir);
    println!("cargo:warning=completion file is generated: {path:?}");

    Ok(())
}
