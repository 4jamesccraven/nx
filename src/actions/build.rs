use super::{change_to_config, run_command};

pub fn build(fast: bool) -> Result<(), String> {
    change_to_config()?;

    run_command("sudo -v")?;

    if !fast {
        run_command("git pull")?;
    }

    run_command("sudo nixos-rebuild switch --flake .")?;

    Ok(())
}
