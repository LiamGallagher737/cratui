use std::fs;

use anyhow::Result;
use toml_edit::{Document, Item};

use super::get_cargo_manifest_path;

pub fn remove(id: String) -> Result<()> {
    let path = get_cargo_manifest_path()?;
    let toml_text = fs::read_to_string(&path)?;
    let mut doc = toml_text.parse::<Document>()?;

    if let Item::Table(deps) = &mut doc["dependencies"] {
        if deps.contains_key(&id) {
            deps.remove(&id);
        }
    }

    fs::write(path, doc.to_string())?;

    Ok(())
}
