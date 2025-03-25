use std::process::Command;

pub fn develop(shell: Option<String>, command: Option<String>) -> Result<(), String> {
    let shell = shell.unwrap_or("default".into());
    let command = command.unwrap_or("zsh".into());

    let local = Command::new("nix")
        .args([
            "develop".into(),
            format!(".#{}", shell),
            "-c".into(),
            command.clone(),
        ])
        .status()
        .map_err(|_| "nix develop brokeded")?;

    if local.success() {
        return Ok(());
    }

    let _ = Command::new("nix")
        .args([
            "develop".into(),
            format!("/home/jamescraven/nixos#{}", shell),
            "-c".into(),
            command,
        ])
        .status()
        .map_err(|_| format!("unable to find shell '{}'", shell))?;

    Ok(())
}
