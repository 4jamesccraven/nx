mod build;
mod clean;
mod completions;
mod develop;
mod push;
mod revert;
mod update;

pub use build::build;
pub use clean::clean;
pub use completions::completions;
pub use develop::develop;
pub use push::push;
pub use revert::revert;
pub use update::update;

use crate::config::Config;

use std::env;

/// Sets the environment's current directory to the nixos
/// config directory. TODO: remove hardcoding.
pub fn change_to_config() -> Result<(), String> {
    let config = Config::get()?;

    env::set_current_dir(config.nixos_config_dir)
        .map_err(|_| "unable to enter config directory".into())
}

/// Attempts to run the provided command, returning an error message
/// if it fails
pub fn run_command(cmd: &str) -> Result<(), String> {
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .status()
        .map_err(|_| format!("unable to run {}", cmd))?;

    if !status.success() {
        let msg = format!("`{}` failed.", cmd);
        return Err(msg);
    }

    Ok(())
}

/// like `run_command`, but with stderr suppressed
pub fn run_command_silent(cmd: &str) -> Result<(), String> {
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|_| format!("unable to run {}", cmd))?;

    if !status.success() {
        let msg = format!("`{}` failed.", cmd);
        return Err(msg);
    }

    Ok(())
}
