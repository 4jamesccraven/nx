use super::{Result, cmd};
use crate::{cli::Develop, config::Config};

pub fn develop(args: Develop) -> Result<()> {
    // Set defaults if not provided
    let shell = args.shell.unwrap_or("default".into());
    let command = args.command.unwrap_or("zsh".into());

    let config = Config::get()?;
    let directory = if args.global {
        config.nixos_config_dir
    } else {
        ".".into()
    };

    cmd!(
        "nix",
        "develop",
        format!("{directory}#{shell}"),
        "-c",
        &command
    )
    .unchecked()
    .run()?;

    Ok(())
}
