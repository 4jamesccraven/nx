use crate::config::Config;

use std::process::Command;

pub fn develop(shell: Option<String>, command: Option<String>) -> Result<(), String> {
    // Set defaults if not provided
    let shell = shell.unwrap_or("default".into());
    let command = command.unwrap_or("zsh".into());
    let config = Config::get()?;

    let source_local = Command::new("nix")
        .args([
            "develop".into(),
            // in current directory
            format!(".#{}", shell),
            "-c".into(),
            command.clone(),
        ])
        .status()
        .map_err(|_| "nix develop brokeded")?;

    if source_local.success() {
        return Ok(());
    }

    let _source_from_config = Command::new("nix")
        .args([
            "develop".into(),
            format!("{}#{}", config.nixos_config_dir, shell),
            "-c".into(),
            command,
        ])
        .status()
        .map_err(|_| format!("unable to find shell '{}'", shell))?;

    Ok(())
}
