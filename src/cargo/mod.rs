use anyhow::{anyhow, Result};
use std::{env, path::PathBuf};

pub mod search;
pub use search::*;

pub mod add;
pub use add::*;

pub mod remove;
pub use remove::*;

pub mod install;
pub use install::*;

const API: &str = "https://crates.io/api/v1/crates";

fn get_cargo_manifest_path() -> Result<PathBuf> {
    let mut dir = env::current_dir()?;
    dir.push("cargo.toml");
    match dir.try_exists() {
        Ok(true) => Ok(dir),
        Ok(false) => Err(anyhow!("Cannot find Cargo.toml")),
        Err(e) => Err(anyhow!("Unable to check if Cargo.toml exsists {e}")),
    }
}
