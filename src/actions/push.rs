use super::{change_to_config, run_command};

pub fn push() -> Result<(), String> {
    change_to_config()?;

    run_command("git add flake.lock")?;

    run_command("git commit -m 'Chore: system update'")?;

    run_command("git push")?;

    Ok(())
}
