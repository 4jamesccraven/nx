use super::{Result, change_to_config, cmd};

pub fn push() -> Result<()> {
    change_to_config()?;

    cmd!("git", "add", "flake.lock").run()?;

    cmd!("git", "commit", "-m", "chore: system update").run()?;

    cmd!("git", "push").run()?;

    Ok(())
}
