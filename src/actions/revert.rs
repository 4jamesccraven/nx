use super::{Result, change_to_config, cmd};

use std::process::Command;

pub fn revert() -> Result<()> {
    change_to_config()?;

    reversion_warning()?;

    let mut response = String::new();

    std::io::stdin()
        .read_line(&mut response)
        .expect("This shouldn't fail, and it's okay if it does");

    let response = response.trim();

    match response {
        "y" | "Y" => {
            let _ = cmd!("git", "reset", "--hard", "HEAD").run()?;
        }
        _ => {
            println!("Successfully aborted reversion.");
        }
    }

    Ok(())
}

/// Error message to warn user about the reverting the config
fn reversion_warning() -> Result<()> {
    let git_commit = Command::new("git")
        .args(["log", "-1", "--pretty=%B"])
        .output()?
        .stdout;

    let git_commit =
        String::from_utf8(git_commit).expect("This is originally a string, it should be a string");

    let red_warning = format!("\x1b[91m{}\x1b[0m", "WARNING");

    println!(
        "{}: You're attempting to revert to this commit:",
        red_warning
    );
    println!("    {}", git_commit.trim_end_matches('\n'));
    println!("Are you sure? [y/N]");

    Ok(())
}
