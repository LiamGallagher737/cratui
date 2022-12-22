use std::fs;

use anyhow::Result;
use toml_edit::{Document, value};

use super::get_cargo_manifest_path;

pub fn add(id: String, version: String) -> Result<()> {
    let path = get_cargo_manifest_path()?;
    let toml_text = fs::read_to_string(&path)?;
    let mut doc = toml_text.parse::<Document>()?;

    doc["dependencies"][id] = value(version.to_string());

    fs::write(path, doc.to_string())?;

    Ok(())
}
