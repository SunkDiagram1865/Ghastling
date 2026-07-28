use std::path::Path;

use serde_json::Value;
use tracing::debug;

pub struct InstanceInfo {
    pub vanilla_name: String,
    pub loader: Option<String>,
    pub loader_version: Option<String>,
}

fn find_json(path: &Path) -> Option<(String, String)> {
    let name = path.file_name()?.to_string_lossy().to_string();
    let primary = path.join(format!("{name}.json"));
    debug!(
        "instance_json: path={} looking for primary={}",
        path.display(),
        primary.display()
    );
    if primary.exists() {
        debug!(
            "instance_json: path={} json={} (by name match)",
            path.display(),
            primary.display()
        );
        let content = std::fs::read_to_string(&primary).ok()?;
        debug!(
            "instance_json: path={} primary content (len={}, first_200={:?})",
            path.display(),
            content.len(),
            &content[..content.len().min(200)]
        );
        return Some((name, content));
    }
    debug!(
        "instance_json: path={} primary={} NOT FOUND, enumerating directory",
        path.display(),
        primary.display()
    );
    let mut json_files = Vec::new();
    if let Ok(dir) = std::fs::read_dir(path) {
        for entry in dir.flatten() {
            let p = entry.path();
            debug!(
                "instance_json: path={} entry={}",
                path.display(),
                p.display()
            );
            if p.extension().map(|e| e == "json").unwrap_or(false) {
                json_files.push(p);
            }
        }
    }
    debug!(
        "instance_json: path={} found {} json files",
        path.display(),
        json_files.len()
    );
    if json_files.len() == 1 {
        debug!(
            "instance_json: path={} json={} (sole json fallback)",
            path.display(),
            json_files[0].display()
        );
        let content = std::fs::read_to_string(&json_files[0]).ok()?;
        debug!(
            "instance_json: path={} sole json content (len={}, first_200={:?})",
            path.display(),
            content.len(),
            &content[..content.len().min(200)]
        );
        let name = json_files[0]
            .file_stem()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or(name);
        return Some((name, content));
    }
    // Multiple JSONs: try each one, return the first with a valid version
    for jf in &json_files {
        let content = std::fs::read_to_string(jf).ok()?;
        debug!(
            "instance_json: path={} trying json={} (len={}, first_200={:?})",
            path.display(),
            jf.display(),
            content.len(),
            &content[..content.len().min(200)]
        );
        let json: serde_json::Value = serde_json::from_str(&content).ok()?;
        let version = extract_version(&json, &content);
        if !version.is_empty() {
            let fname = jf
                .file_stem()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| name.clone());
            debug!(
                "instance_json: path={} json={} (multiple-json pick, version={})",
                path.display(),
                jf.display(),
                version
            );
            return Some((fname, content));
        }
    }
    debug!(
        "instance_json: path={} multiple={} json files, none yielded a version",
        path.display(),
        json_files.len()
    );
    None
}

pub fn detect(path: &Path) -> Option<InstanceInfo> {
    let (_name, content) = find_json(path)?;
    let json: Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            debug!("instance_json: path={} parse_err={}", path.display(), e);
            return None;
        }
    };
    let mut vanilla_name = extract_version(&json, &content);
    debug!(
        "instance_json: path={} extract_version returned {:?}",
        path.display(),
        vanilla_name
    );
    if vanilla_name.is_empty() {
        debug!(
            "instance_json: path={} version empty or Unknown",
            path.display()
        );
        return None;
    }
    vanilla_name = normalize_version(&vanilla_name);
    let loader = detect_loader(&content, &json);
    debug!(
        "instance_json: path={} version={} loader={:?}",
        path.display(),
        vanilla_name,
        loader.as_ref().map(|(t, _)| t.as_str())
    );
    Some(InstanceInfo {
        vanilla_name,
        loader: loader.as_ref().map(|(t, _)| t.clone()),
        loader_version: loader.and_then(|(_, v)| v),
    })
}

fn normalize_version(raw: &str) -> String {
    let mut v = raw.to_string();
    if (v.starts_with("20.") || v.starts_with("21.")) && !v.starts_with("1.") {
        v = format!("1.{v}");
    }
    v = v.replace("_unobfuscated", "");
    v = v.replace(" Unobfuscated", "");
    v.trim().to_string()
}

fn extract_version(json: &Value, json_str: &str) -> String {
    // ① PCL download record clientVersion
    if let Some(v) = json.get("clientVersion").and_then(|v| v.as_str())
        && !v.is_empty()
    {
        debug!("extract_version: method=① clientVersion value={}", v);
        return v.to_string();
    }

    // ② HMCL patches[].version (id == "game")
    if let Some(patches) = json.get("patches").and_then(|v| v.as_array()) {
        for patch in patches {
            if patch.get("id").and_then(|v| v.as_str()) == Some("game")
                && let Some(ver) = patch.get("version").and_then(|v| v.as_str())
                && !ver.is_empty()
            {
                debug!(
                    "extract_version: method=② patches.game.version value={}",
                    ver
                );
                return ver.to_string();
            }
        }
    }

    // ③ arguments.game --fml.mcVersion (Forge/NeoForge)
    if let Some(args) = json
        .get("arguments")
        .and_then(|v| v.get("game"))
        .and_then(|v| v.as_array())
    {
        let mut mark = false;
        for arg in args {
            if mark && let Some(v) = arg.as_str() {
                debug!("extract_version: method=③ --fml.mcVersion value={}", v);
                return v.to_string();
            }
            if arg.as_str() == Some("--fml.mcVersion") {
                mark = true;
            }
        }
    }

    // ④ jar field (used with inheritsFrom in version inheritance chains)
    if let Some(v) = json.get("jar").and_then(|v| v.as_str())
        && !v.is_empty()
    {
        debug!("extract_version: method=④ jar value={}", v);
        return v.to_string();
    }

    // ⑤ inheritsFrom (version inheritance)
    if let Some(v) = json.get("inheritsFrom").and_then(|v| v.as_str())
        && !v.is_empty()
    {
        debug!("extract_version: method=⑤ inheritsFrom value={}", v);
        return v.to_string();
    }

    // ⑥ libraries string regex fallback (Forge/OptiFine/FabricLike lib versions)
    // Use the original JSON string (from find_json) instead of re-serializing
    // the parsed Value, which would allocate a fresh string unnecessarily.
    if let Some(v) = extract_version_from_libraries(json_str) {
        debug!("extract_version: method=⑥ libraries value={}", v);
        return v;
    }

    // ⑦ JSON id field → extract leading version
    if let Some(id) = json.get("id").and_then(|v| v.as_str())
        && let Some(v) = extract_version_from_id(id)
    {
        debug!("extract_version: method=⑦ id id={} value={}", id, v);
        return v;
    }

    debug!("extract_version: method=✗ all methods failed");
    String::new()
}

/// Extracts Minecraft version from library artifact coordinates in the JSON string.
/// Matches PCLCE's approach scanning for Forge/OptiFine/FabricLike lib entries.
/// Order: NeoForge before Forge (NeoForge JSON often also contains forge references).
fn extract_version_from_libraries(content: &str) -> Option<String> {
    // NeoForge: net.neoforged:neoforge:1.20.1-44.0.3 → "1.20.1"
    // Try known Maven coordinate formats (neoforge before forge).
    for needle in [
        "net.neoforged:neoforge:",
        "net.neoforged.neoforge:neoforge:",
        "net.neoforged.fml:modern:",
    ] {
        if let Some(pos) = content.find(needle) {
            let after = &content[pos + needle.len()..];
            if let Some(end) = after.find(&['"', ',', '\n', '}'] as &[char]) {
                let ver = &after[..end];
                if let Some(dash) = ver.find('-') {
                    return Some(ver[..dash].to_string());
                }
                return Some(ver.to_string());
            }
        }
    }
    // Forge: minecraftforge:forge:1.8.9-11.15.1.1722 → "1.8.9"
    if let Some(pos) = content.find("minecraftforge:forge:") {
        let after = &content[pos + "minecraftforge:forge:".len()..];
        if let Some(end) = after.find(&['"', ',', '\n', '}'] as &[char]) {
            let ver = &after[..end];
            if let Some(dash) = ver.find('-') {
                return Some(ver[..dash].to_string());
            }
            return Some(ver.to_string());
        }
    }
    // OptiFine: optifine:OptiFine:1.8.9_HD_U_H5 → "1.8.9"
    if let Some(pos) = content.find("optifine:OptiFine:") {
        let after = &content[pos + "optifine:OptiFine:".len()..];
        if let Some(end) = after.find(&['"', ',', '\n', '}'] as &[char]) {
            let ver = &after[..end];
            if let Some(underscore) = ver.find('_') {
                return Some(ver[..underscore].to_string());
            }
            return Some(ver.to_string());
        }
    }
    // Fabric-like: net.fabricmc:fabric-loader:0.15.11-1.20.1 → "1.20.1"
    if let Some(pos) = content.find("net.fabricmc:fabric-loader:") {
        let after = &content[pos + "net.fabricmc:fabric-loader:".len()..];
        if let Some(end) = after.find(&['"', ',', '\n', '}'] as &[char]) {
            let ver = &after[..end];
            if let Some(dash) = ver.rfind('-') {
                return Some(ver[dash + 1..].to_string());
            }
        }
    }
    None
}

/// Extracts leading version number from the instance id.
/// e.g. "1.8.9-forge-11.15.1.1722" → "1.8.9"
/// Skips hash-like ids (≥32 chars, no separators).
fn extract_version_from_id(id: &str) -> Option<String> {
    let ver = id.trim();
    if ver.is_empty() {
        return None;
    }
    if ver.len() >= 32
        && !ver.contains('.')
        && !ver.contains('-')
        && !ver.contains('_')
    {
        return None;
    }
    if let Some(first_sep) = ver.find(['-', '_', ' ']) {
        let candidate = &ver[..first_sep];
        if candidate.starts_with("1.") || candidate.starts_with('2') {
            return Some(candidate.to_string());
        }
    }
    if ver.starts_with("1.") || ver.starts_with('2') {
        return Some(ver.to_string());
    }
    None
}

/// Detects which mod loader is used and extracts its version.
/// Only loaders we can actually install (mapped to PackDependency) are detected.
fn detect_loader(
    content: &str,
    json: &Value,
) -> Option<(String, Option<String>)> {
    // Order per PCLCE: check Fabric/Quilt before Forge, neoforge before forge
    let check_fabric = content.contains("net.fabricmc");
    debug!(
        "detect_loader: check_fabric content_contains('net.fabricmc')={}",
        check_fabric
    );
    let check_quilt = content.contains("org.quiltmc");
    debug!(
        "detect_loader: check_quilt content_contains('org.quiltmc')={}",
        check_quilt
    );
    let neoforge_in_content = content.contains("net.neoforged");
    let neoforge_in_id = json
        .get("id")
        .and_then(|v| v.as_str())
        .map(|id| id.to_lowercase().contains("neoforge"))
        .unwrap_or(false);
    debug!(
        "detect_loader: check_neoforge content_contains('net.neoforged')={} id_contains('neoforge')={}",
        neoforge_in_content, neoforge_in_id
    );
    let check_forge = content.contains("net.minecraftforge");
    debug!(
        "detect_loader: check_forge content_contains('net.minecraftforge')={}",
        check_forge
    );

    let loader_type = if check_fabric {
        "fabric"
    } else if check_quilt {
        "quilt"
    } else if neoforge_in_content || neoforge_in_id {
        "neoforge"
    } else if check_forge {
        "forge"
    } else {
        debug!("detect_loader: no known loader library found in JSON content");
        return None;
    };

    debug!("detect_loader: detected loader_type={}", loader_type);
    let version = extract_loader_version(content, json, loader_type);
    debug!(
        "detect_loader: loader_type={} extracted_version={:?}",
        loader_type, version
    );
    Some((loader_type.to_string(), version))
}

fn extract_loader_version(
    content: &str,
    json: &Value,
    loader_type: &str,
) -> Option<String> {
    // First: parse from id field (e.g. "1.8.9-forge-11.15.1.1722")
    if let Some(id) = json.get("id").and_then(|v| v.as_str())
        && let Some(ver) = parse_loader_version_from_id(id, loader_type)
    {
        debug!(
            "extract_loader_version: from id field id={} loader={} version={}",
            id, loader_type, ver
        );
        return Some(ver);
    }
    debug!(
        "extract_loader_version: id field did not yield version for loader={}",
        loader_type
    );

    // Second: extract from library entries
    let (needle, needle_fallback, split_at) = match loader_type {
        "forge" => ("minecraftforge:forge:", None, Some('-')),
        "neoforge" => (
            "net.neoforged:neoforge:",
            Some("net.neoforged.neoforge:neoforge:"),
            None,
        ),
        "fabric" => ("net.fabricmc:fabric-loader:", None, None),
        "quilt" => ("org.quiltmc:quilt-loader:", None, None),
        _ => return None,
    };

    if let Some(ver) =
        try_extract_version_from_needle(content, needle, split_at)
    {
        debug!(
            "extract_loader_version: from library needle={} version={}",
            needle, ver
        );
        return Some(ver);
    }
    if let Some(fallback) = needle_fallback {
        debug!(
            "extract_loader_version: primary needle={} not found, trying fallback={}",
            needle, fallback
        );
        if let Some(ver) =
            try_extract_version_from_needle(content, fallback, split_at)
        {
            debug!(
                "extract_loader_version: from library fallback={} version={}",
                fallback, ver
            );
            return Some(ver);
        }
        debug!(
            "extract_loader_version: fallback={} also not found",
            fallback
        );
    }
    None
}

/// Extracts the loader version string from JSON content by finding a needle
/// and reading until a terminator character.
fn try_extract_version_from_needle(
    content: &str,
    needle: &str,
    split_at: Option<char>,
) -> Option<String> {
    let pos = content.find(needle)?;
    let after = &content[pos + needle.len()..];
    let end = after.find(&['"', ',', '\n', '}'] as &[char])?;
    let ver = &after[..end];
    if let Some(ch) = split_at
        && let Some(pos) = ver.rfind(ch)
    {
        Some(ver[pos + 1..].to_string())
    } else {
        Some(ver.to_string())
    }
}

/// Parses loader version from the instance id.
/// e.g. "1.8.9-forge-11.15.1.1722" → "11.15.1.1722"
///      "1.20.1-fabric-0.15.11" → "0.15.11"
fn parse_loader_version_from_id(id: &str, loader_type: &str) -> Option<String> {
    let id_lower = id.to_lowercase();
    let keyword = match loader_type {
        "forge" => "forge",
        "neoforge" => "neoforge",
        "fabric" => "fabric",
        "quilt" => "quilt",
        _ => return None,
    };

    let pos = id_lower.find(keyword)?;
    let after = &id[pos + keyword.len()..];

    // Try "<keyword>-<version>" pattern (most common)
    let after_trimmed = after.trim_start_matches(['-', '_', ' ']);
    if !after_trimmed.is_empty()
        && after_trimmed.chars().next()?.is_ascii_digit()
    {
        // Take until next separator (or end of string)
        let end = after_trimmed
            .find(|c: char| !c.is_ascii_digit() && c != '.')
            .unwrap_or(after_trimmed.len());
        if end > 0 {
            return Some(after_trimmed[..end].to_string());
        }
    }

    // Try "<keyword><digits.>" pattern (no separator, e.g. "Forge11.15.1")
    if let Some(first_digit) = after.find(|c: char| c.is_ascii_digit()) {
        let ver = &after[first_digit..];
        let end = ver
            .find(|c: char| !c.is_ascii_digit() && c != '.')
            .unwrap_or(ver.len());
        if end > 0 {
            return Some(ver[..end].to_string());
        }
    }
    None
}
