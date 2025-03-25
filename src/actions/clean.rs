use super::change_to_config;
use crate::{run_command, run_command_silent};

pub fn clean(no_optimisation: bool, no_cache: bool) -> Result<(), String> {
    change_to_config()?;

    run_command!("sudo -v")?;

    println!("Collecting garbage...");
    run_command_silent!("sudo nix-collect-garbage -d")?;
    run_command_silent!("nix-collect-garbage -d")?;
    run_command!("nx build --fast")?;

    if !no_cache {
        run_command!("nx update --shells")?;
    }

    if !no_optimisation {
        run_command!("nix store optimise")?;
    }

    Ok(())
}
