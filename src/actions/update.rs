use super::{Result, change_to_config, cmd};

pub fn update() -> Result<()> {
    change_to_config()?;

    cmd!("sudo", "-v").run()?;

    cmd!("nix", "flake", "update").run()?;

    cmd!("nx", "build", "--fast").run()?;

    Ok(())
}
