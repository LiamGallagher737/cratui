use anyhow::{anyhow, Result};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::{fs, path::PathBuf};

use crate::BINARY_NAME;

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub terminal: Terminal,
    pub colors: Colors,
    pub favourites: Favourites,
    pub search: Search,
}

#[derive(SmartDefault, Serialize, Deserialize)]
pub struct Terminal {
    #[default(250)]
    pub tick_rate: u8,
}

#[derive(SmartDefault, Serialize, Deserialize)]
pub struct Colors {
    #[default([238, 111, 248])]
    pub primary: [u8; 3],
    #[default([255, 0, 255])]
    pub secondary: [u8; 3],
    #[default([230, 126, 34])]
    pub warn: [u8; 3],
    #[default([255, 3, 3])]
    pub error: [u8; 3],
}

#[derive(SmartDefault, Serialize, Deserialize)]
pub struct Favourites {
    pub crates: Vec<String>,
}

#[derive(SmartDefault, Serialize, Deserialize)]
pub struct Search {
    #[default(20)]
    pub max_pages: u8,
}

pub fn load_config() -> Result<Config> {
    let path = config_path()?;
    if !path.exists() {
        fs::create_dir_all(
            path.parent()
                .ok_or(anyhow!("Unable to get directory from config path"))?,
        )?;
        save_config(Config::default())?;
        return Ok(Config::default());
    }
    let config_text = fs::read_to_string(path)?;
    Ok(toml::from_str(&config_text)?)
}

pub fn save_config(config: Config) -> Result<()> {
    let config_text = toml::to_string_pretty(&config)?;
    fs::write(config_path()?, config_text)?;
    Ok(())
}

fn config_path() -> Result<PathBuf> {
    let mut path = BaseDirs::new()
        .ok_or(anyhow!("Failed to get config path"))?
        .config_dir()
        .to_path_buf();
    path.push(BINARY_NAME);
    path.push("config.toml");
    Ok(path)
}
