//! Parse mod metadata files from inside JAR archives to extract mod identity
//! information (mod ID, name, version, authors, etc.) when Modrinth API
//! lookups fail or have no match.
//!
//! Supported formats:
//! - Fabric: `fabric.mod.json` (JSON)
//! - Quilt: `quilt.mod.json` (JSON, same shape wrapped under `quilt_loader`)
//! - Forge: `META-INF/mods.toml` (TOML)
//! - NeoForge: `META-INF/neoforge.mods.toml` (TOML)
//! - Legacy Forge: `mcmod.info` (JSON array)

mod fabric;
pub mod manifest;
mod mcmod_info;
mod toml_mod;

use bytes::Bytes;
use serde::{Deserialize, Serialize};

/// Unified local mod metadata extracted from inside a JAR.
///
/// Only `mod_id` is required; all other fields are best-effort.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalModMetadata {
    /// Unique mod identifier (e.g. "sodium", "minecraft")
    pub mod_id: String,
    /// Human-readable display name
    pub name: Option<String>,
    /// Mod version string
    pub version: Option<String>,
    /// Author list
    #[serde(default)]
    pub authors: Vec<String>,
    /// Short description
    pub description: Option<String>,
    /// Website or project URL
    pub url: Option<String>,
    /// Path to icon inside the JAR (e.g. "icon.png" or "assets/.../icon.png")
    pub icon_path: Option<String>,
    /// Supported Minecraft version range (e.g. ">=1.20", "[1.20,1.21)", "1.12.2")
    pub minecraft_version: Option<String>,
    /// Required loader version (e.g. ">=0.15.0", "[52,)")
    pub loader_version: Option<String>,
    /// Loader type (e.g. "fabric", "forge", "neoforge", "quilt")
    pub loader: Option<String>,
}

/// Try to extract `LocalModMetadata` from raw JAR bytes.
///
/// Returns `None` when the JAR does not contain any known mod metadata file
/// or when none of the supported formats can be successfully parsed.
pub fn extract_mod_metadata(bytes: &Bytes) -> Option<LocalModMetadata> {
    let cursor = std::io::Cursor::new(&**bytes);
    let mut archive = zip::ZipArchive::new(cursor).ok()?;

    // Try each known metadata path in priority order.
    if let Some(meta) = try_fabric(&mut archive) {
        return Some(meta);
    }
    if let Some(meta) = try_quilt(&mut archive) {
        return Some(meta);
    }
    if let Some(meta) =
        try_toml_path(&mut archive, "META-INF/neoforge.mods.toml")
    {
        return Some(meta);
    }
    if let Some(meta) = try_toml_path(&mut archive, "META-INF/mods.toml") {
        return Some(meta);
    }
    if let Some(meta) = try_mcmod_info(&mut archive) {
        return Some(meta);
    }

    None
}

// ── format-specific parsers ────────────────────────────────────────────────

fn try_fabric(
    archive: &mut zip::ZipArchive<std::io::Cursor<&[u8]>>,
) -> Option<LocalModMetadata> {
    let mut file = archive.by_name("fabric.mod.json").ok()?;
    let parsed: fabric::FabricModJson =
        serde_json::from_reader(&mut file).ok()?;

    let authors = merge_authors(&parsed.authors, &parsed.contributors);

    Some(LocalModMetadata {
        mod_id: parsed.id.clone().unwrap_or_else(|| "unknown".into()),
        name: parsed.name,
        version: parsed.version,
        authors,
        description: parsed.description,
        url: extract_contact_url(&parsed._contact),
        icon_path: parsed.icon,
        minecraft_version: fabric::fabric_dep_value(
            &parsed.depends,
            "minecraft",
        ),
        loader_version: fabric::fabric_dep_value(
            &parsed.depends,
            "fabricloader",
        ),
        loader: Some("fabric".into()),
    })
}

fn try_quilt(
    archive: &mut zip::ZipArchive<std::io::Cursor<&[u8]>>,
) -> Option<LocalModMetadata> {
    let mut file = archive.by_name("quilt.mod.json").ok()?;
    let parsed: fabric::QuiltModJson =
        serde_json::from_reader(&mut file).ok()?;

    let inner = parsed.quilt_loader;
    let authors = merge_authors(&inner.authors, &inner.contributors);

    Some(LocalModMetadata {
        mod_id: inner.id.unwrap_or_else(|| "unknown".into()),
        name: inner.name,
        version: inner.version,
        authors,
        description: inner.description,
        url: extract_contact_url(&inner._contact),
        icon_path: inner.icon,
        minecraft_version: fabric::quilt_dep_value(&inner.depends, "minecraft"),
        loader_version: fabric::quilt_dep_value(&inner.depends, "quilt_loader"),
        loader: Some("quilt".into()),
    })
}

fn try_toml_path(
    archive: &mut zip::ZipArchive<std::io::Cursor<&[u8]>>,
    path: &str,
) -> Option<LocalModMetadata> {
    let mut file = archive.by_name(path).ok()?;
    let mut content = String::new();
    std::io::Read::read_to_string(&mut file, &mut content).ok()?;
    let parsed: toml_mod::ModsToml = toml::from_str(&content).ok()?;

    let entry = parsed.mods?.into_iter().next()?;
    let mod_id = entry.mod_id.clone().unwrap_or_else(|| "unknown".into());

    let authors: Vec<String> = entry
        .authors
        .as_deref()
        .map(|s| {
            s.split(',')
                .map(|a| a.trim().to_string())
                .filter(|a| !a.is_empty())
                .collect()
        })
        .unwrap_or_default();

    // Determine loader type from the file path.
    let is_neoforge = path.contains("neoforge");
    let loader = if is_neoforge {
        Some("neoforge".into())
    } else {
        Some("forge".into())
    };
    // The root `loaderVersion` IS the Forge/NeoForge loader version.
    let loader_version = parsed.loader_version.clone();

    // Look up dependencies for this mod's modId.
    let minecraft_version = parsed
        .dependencies
        .as_ref()
        .and_then(|deps| deps.get(&mod_id))
        .and_then(|entries| {
            entries
                .iter()
                .find(|dep| dep.mod_id.as_deref() == Some("minecraft"))
                .and_then(|dep| dep.version_range.clone())
        });

    Some(LocalModMetadata {
        mod_id,
        name: entry.display_name,
        version: entry.version,
        authors,
        description: entry.description,
        url: entry.display_url,
        icon_path: entry.logo_file,
        minecraft_version,
        loader_version,
        loader,
    })
}

fn try_mcmod_info(
    archive: &mut zip::ZipArchive<std::io::Cursor<&[u8]>>,
) -> Option<LocalModMetadata> {
    let mut file = archive.by_name("mcmod.info").ok()?;
    let entries: Vec<mcmod_info::McmodInfoEntry> =
        serde_json::from_reader(&mut file).ok()?;

    let entry = entries.into_iter().next()?;

    Some(LocalModMetadata {
        mod_id: entry.modid.unwrap_or_else(|| "unknown".into()),
        name: entry.name,
        version: entry.version,
        authors: entry.authors.unwrap_or_default(),
        description: entry.description,
        url: entry.url,
        icon_path: entry.logo_file,
        minecraft_version: entry.mcversion,
        loader_version: None,
        loader: Some("forge".into()),
    })
}

// ── helpers ────────────────────────────────────────────────────────────────

fn merge_authors(
    primary: &[fabric::FabricAuthorOrArray],
    contributors: &[fabric::FabricAuthorOrArray],
) -> Vec<String> {
    primary
        .iter()
        .chain(contributors.iter())
        .filter_map(|author| match author {
            fabric::FabricAuthorOrArray::Plain(s) => Some(s.clone()),
            fabric::FabricAuthorOrArray::Object { name } => name.clone(),
        })
        .collect()
}

/// Extract a URL from Fabric's `contact` object (often has `"homepage"`, `"sources"`, etc.).
fn extract_contact_url(contact: &Option<serde_json::Value>) -> Option<String> {
    let obj = contact.as_ref()?.as_object()?;
    // Prefer homepage, then sources, then any string value.
    if let Some(homepage) = obj.get("homepage").and_then(|v| v.as_str()) {
        return Some(homepage.to_string());
    }
    if let Some(sources) = obj.get("sources").and_then(|v| v.as_str()) {
        return Some(sources.to_string());
    }
    // Fallback: return the first string field found.
    obj.values().find_map(|v| v.as_str().map(String::from))
}
