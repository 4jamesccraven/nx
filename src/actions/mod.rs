mod build;
mod clean;
mod develop;
mod push;
mod revert;
mod update;

pub use build::build;
pub use clean::clean;
pub use develop::develop;
pub use push::push;
pub use revert::revert;
pub use update::update;

use std::env;

pub fn change_to_config() -> Result<(), String> {
    env::set_current_dir("/home/jamescraven/nixos")
        .map_err(|_| "unable to enter config directory".into())
}

#[macro_export]
macro_rules! run_command {
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
