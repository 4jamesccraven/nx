use super::change_to_config;
use crate::run_command;

use std::process::Command;

pub fn revert() -> Result<(), String> {
    change_to_config()?;

    let git_commit = Command::new("git")
        .args(["log", "-1", "--pretty=%B"])
        .output()
        .map_err(|_| "error running git log")?
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

    let mut response = String::new();

    std::io::stdin()
        .read_line(&mut response)
        .expect("This shouldn't fail, and it's okay if it does");

    match response.as_str() {
        "y" | "Y" => run_command!("git reset --hard HEAD")?,
        _ => {
            println!("Successfully aborted reversion.");
        }
    }

    Ok(())
}
