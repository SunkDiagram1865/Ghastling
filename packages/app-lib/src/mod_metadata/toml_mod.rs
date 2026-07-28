use serde::Deserialize;
use std::collections::HashMap;

/// Forge / NeoForge mods.toml — `[[mods]]` array of tables plus root metadata.
#[derive(Debug, Deserialize)]
pub(crate) struct ModsToml {
    /// Name of the mod loader (e.g. "javafml").
    #[allow(dead_code)]
    pub mod_loader: Option<String>,
    /// Required loader version range (e.g. "[52,)"). For Forge this IS the Forge version.
    pub loader_version: Option<String>,
    #[serde(rename = "mods")]
    pub mods: Option<Vec<ModsTomlEntry>>,
    /// Dependencies keyed by modId: `[[dependencies.<modId>]]`.
    pub dependencies: Option<HashMap<String, Vec<ForgeDependencyEntry>>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ModsTomlEntry {
    pub mod_id: Option<String>,
    pub display_name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub authors: Option<String>,
    pub logo_file: Option<String>,
    pub display_url: Option<String>,
    #[allow(dead_code)]
    pub credits: Option<String>,
}

/// An entry in a Forge/NeoForge `[[dependencies.<modId>]]` array.
#[derive(Debug, Deserialize)]
pub(crate) struct ForgeDependencyEntry {
    pub mod_id: Option<String>,
    #[allow(dead_code)]
    pub mandatory: Option<bool>,
    pub version_range: Option<String>,
    #[allow(dead_code)]
    pub ordering: Option<String>,
    #[allow(dead_code)]
    pub side: Option<String>,
}
