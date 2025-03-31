use dirs::config_dir;
use ron;
use serde::{Deserialize, Serialize};

use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub nixos_config_dir: String
}

impl Config {
    pub fn get() -> Result<Self, String> {
        let mut config_dir = config_dir().expect("Config dir should exist");

        config_dir.push("nx");
        config_dir.push("config.ron");

        let config = fs::read_to_string(&config_dir)
            .map_err(|_| "config.ron is missing.".to_string())?;

        let config: Config = ron::from_str(&config.trim())
            .map_err(|why| format!("Unable to parse config.ron, {}", why))?;

        Ok(config)
    }
}
