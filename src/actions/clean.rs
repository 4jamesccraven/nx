use crate::cli::Clean;

use super::{Result, change_to_config, cmd};

pub fn clean(args: Clean) -> Result<()> {
    change_to_config()?;

    cmd!("sudo", "-v").run()?;

    println!("Collecting garbage...");
    cmd!("sudo", "nix-collect-garbage", "-d")
        .stderr_null()
        .run()?;
    cmd!("nix-collect-garbage", "-d").stderr_null().run()?;
    cmd!("nx", "build", "--fast").run()?;

    if !args.no_optimise {
        cmd!("nix", "store", "optimise").run()?;
    }

    Ok(())
}
