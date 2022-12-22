use anyhow::Result;
use std::process::{Child, Command};

pub fn install(id: String) -> Result<Child> {
    Ok(Command::new("cargo")
        .arg("install")
        .arg(id)
        .arg("-q")
        .spawn()?)
}
