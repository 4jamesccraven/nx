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

use std::env;

/// Sets the environment's current directory to the nixos
/// config directory. TODO: remove hardcoding.
pub fn change_to_config() -> Result<(), String> {
    env::set_current_dir("/home/jamescraven/nixos")
        .map_err(|_| "unable to enter config directory".into())
}

/// Attempts to run the provided command, returning an error message
/// if it fails
#[macro_export]
macro_rules! run_command {
    // Command+args as a string
    ($cmd:expr) => {{
        let mut parsed = $cmd.split_whitespace();

        let cmd = parsed.next().unwrap();

        let mut output = std::process::Command::new(cmd)
            .args(parsed)
            .spawn()
            .map_err(|_| format!("unable to run {}", $cmd))?;

        if let Err(_) = output.wait() {
            return Err(format!("error running {}", $cmd));
        }

        let status: Result<(), String> = Ok(());

        status
    }};
    // Command as a string, args as an iterable of strings
    ($cmd:expr, $args:expr) => {{
        let mut output = std::process::Command::new($cmd)
            .args($args)
            .spawn()
            .map_err(|_| format!("unable to run {}", $cmd))?;

        if let Err(_) = output.wait() {
            return Err(format!("error running {}", $cmd));
        }

        let status: Result<(), String> = Ok(());

        status
    }};
}

/// like `run_command!`, but with stderr suppressed
#[macro_export]
macro_rules! run_command_silent {
    ($cmd:expr) => {{
        let mut parsed = $cmd.split_whitespace();

        let cmd = parsed.next().unwrap();

        let mut output = std::process::Command::new(cmd)
            .args(parsed)
            .stderr(std::process::Stdio::null())
            .spawn()
            .map_err(|_| format!("unable to run {}", $cmd))?;

        if let Err(_) = output.wait() {
            return Err(format!("error running {}", $cmd));
        }

        let status: Result<(), String> = Ok(());

        status
    }};
    ($cmd:expr, $args:expr) => {{
        let mut output = std::process::Command::new($cmd)
            .args($args)
            .stderr(std::process::Stdio::null())
            .spawn()
            .map_err(|_| format!("unable to run {}", $cmd))?;

        if let Err(_) = output.wait() {
            return Err(format!("error running {}", $cmd));
        }

        let status: Result<(), String> = Ok(());

        status
    }};
}
