use super::{Result, change_to_config, cmd};
use crate::cli::Update;

pub fn update(args: Update) -> Result<()> {
    change_to_config()?;

    cmd!("sudo", "-v").run()?;

    match args.input {
        Some(inp) => cmd!("nix", "flake", "update", inp).run()?,
        None => cmd!("nix", "flake", "update").run()?,
    };

    cmd!("nx", "build", "--fast").run()?;

    Ok(())
}
