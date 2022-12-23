use super::API;
use anyhow::Result;
use serde::{Deserialize, Serialize};

// User Agent header, required by crates.io api
const UA_HEADER_KEY: &str = "User-Agent";
const UA_HEADER_VALUE: &str = "cratui (https://github.com/LiamGallagher737/cratui)";

pub fn search(query: &str, page: usize, limit: usize) -> Result<SearchResponse> {
    let limit = limit.min(100); // Crates.io only allows 100 per page
    let url = format!("{API}?q={query}&page={}&per_page={limit}", page + 1);

    let res = ureq::get(&url)
        .set(UA_HEADER_KEY, UA_HEADER_VALUE)
        .call()?
        .into_json()?;

    Ok(res)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResponse {
    pub crates: Vec<Crate>,
    pub meta: Meta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Crate {
    pub id: String,
    pub description: Option<String>,
    pub downloads: usize,
    pub recent_downloads: usize,
    // pub homepage: Option<String>,
    pub repository: Option<String>,
    // pub documentation: Option<String>,
    pub max_version: String,
    pub max_stable_version: Option<String>,
    // pub created_at: String,
    // pub exact_match: bool,
    // pub links: Links,
    // pub name: String,
    // pub newest_version: String,
    // pub updated_at: String,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Links {
//     pub owner_team: String,
//     pub owner_user: String,
//     pub owners: String,
//     pub reverse_dependencies: String,
//     pub version_downloads: String,
//     pub versions: String,
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub next_page: Option<String>,
    pub prev_page: Option<String>,
    pub total: usize,
}
