use std::fs;

use anyhow::{Context, Result};
use dirs::config_dir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub nixos_config_dir: String,
}

impl Config {
    pub fn get() -> Result<Self> {
        let mut config_dir = config_dir().expect("Config dir should exist");

        config_dir.push("nx");
        config_dir.push("config.ron");

        let config = fs::read_to_string(&config_dir).context("could not read config directory")?;

        let config: Config = ron::from_str(config.trim())?;

        Ok(config)
    }
}
