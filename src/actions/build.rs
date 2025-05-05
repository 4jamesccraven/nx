use crate::cli::Build;

use super::{Result, change_to_config, cmd};

pub fn build(args: Build) -> Result<()> {
    change_to_config()?;

    cmd!("sudo", "-v").run()?;

    if !args.fast {
        cmd!("git", "pull").run()?;
    }

    cmd!("sudo", "nixos-rebuild", "switch", "--flake", ".").run()?;

    Ok(())
}
