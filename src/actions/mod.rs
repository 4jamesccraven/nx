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

use crate::config::Config;

use std::env;

use anyhow::{Context, Result};
use duct::cmd;

/// Sets the environment's current directory to the nixos
/// config directory. TODO: remove hardcoding.
pub fn change_to_config() -> Result<()> {
    let config = Config::get()?;

    env::set_current_dir(config.nixos_config_dir).context("failed to access nixos config directory")
}
