use crate::cli::Clean;

use super::{Result, change_to_config, cmd};

pub fn clean(args: Clean) -> Result<()> {
    change_to_config()?;

    cmd!("sudo", "-v").run()?;

    cmd!("nh", "clean", "all").run()?;

    if !args.no_optimise {
        cmd!("nix", "store", "optimise").run()?;
    }

    Ok(())
}
