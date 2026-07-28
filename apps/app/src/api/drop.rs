use serde::{Deserialize, Serialize};
use theseus::drop_classifier::{
    DroppedItemType, ModrinthLookupResult, classify_dropped_item,
    classify_zip_with_extraction, lookup_mod_hash,
};
use theseus::pack::import::{ImportLauncherType, get_importable_instances};
use theseus::{LockingProcess, get_locking_processes};
use tracing::{debug, info, warn};

/// Serializable classification result mapped from `DroppedItemType`.
///
/// All `PathBuf` fields are converted to `String` via `to_string_lossy()`.
/// The JSON representation uses an `item_type` tag (via `#[serde(tag = "item_type")]`)
/// so the frontend can discriminate variants with a string switch.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "item_type")]
pub enum ClassificationResult {
    #[serde(rename = "launcher")]
    Launcher {
        launcher_type: String,
        base_path: String,
    },
    #[serde(rename = "hmcl_launcher")]
    HmclLauncher {
        launcher_dir: String,
        data_dir: String,
    },
    #[serde(rename = "mod")]
    Mod { file_path: String },
    #[serde(rename = "litematic")]
    Litematic { file_path: String },
    #[serde(rename = "resource_pack")]
    ResourcePack { file_path: String },
    #[serde(rename = "shader_pack")]
    ShaderPack { file_path: String },
    #[serde(rename = "world_save")]
    WorldSave { file_path: String },
    #[serde(rename = "modpack")]
    Modpack { file_path: String },
    #[serde(rename = "shortcut_resolved")]
    ShortcutResolved {
        original: String,
        resolved_to: Box<ClassificationResult>,
    },
    #[serde(rename = "unknown")]
    Unknown { reason: String },
}

impl From<DroppedItemType> for ClassificationResult {
    fn from(item: DroppedItemType) -> Self {
        match item {
            DroppedItemType::Launcher {
                launcher_type,
                base_path,
            } => ClassificationResult::Launcher {
                launcher_type: launcher_type.to_string(),
                base_path: base_path.to_string_lossy().to_string(),
            },
            DroppedItemType::HmclLauncher {
                launcher_dir,
                data_dir,
            } => ClassificationResult::HmclLauncher {
                launcher_dir: launcher_dir.to_string_lossy().to_string(),
                data_dir: data_dir.to_string_lossy().to_string(),
            },
            DroppedItemType::Mod { file_path } => ClassificationResult::Mod {
                file_path: file_path.to_string_lossy().to_string(),
            },
            DroppedItemType::Litematic { file_path } => {
                ClassificationResult::Litematic {
                    file_path: file_path.to_string_lossy().to_string(),
                }
            }
            DroppedItemType::ResourcePack { file_path } => {
                ClassificationResult::ResourcePack {
                    file_path: file_path.to_string_lossy().to_string(),
                }
            }
            DroppedItemType::ShaderPack { file_path } => {
                ClassificationResult::ShaderPack {
                    file_path: file_path.to_string_lossy().to_string(),
                }
            }
            DroppedItemType::WorldSave { file_path } => {
                ClassificationResult::WorldSave {
                    file_path: file_path.to_string_lossy().to_string(),
                }
            }
            DroppedItemType::ShortcutResolved {
                original,
                resolved_to,
            } => ClassificationResult::ShortcutResolved {
                original: original.to_string_lossy().to_string(),
                resolved_to: Box::new(ClassificationResult::from(*resolved_to)),
            },
            DroppedItemType::Modpack { file_path } => {
                ClassificationResult::Modpack {
                    file_path: file_path.to_string_lossy().to_string(),
                }
            }
            DroppedItemType::Unknown { reason } => {
                ClassificationResult::Unknown { reason }
            }
        }
    }
}

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("drop")
        .invoke_handler(tauri::generate_handler![
            drop_classify,
            drop_classify_extract,
            drop_scan_launcher_instances,
            drop_detect_file_lock,
            drop_extract_mod_metadata,
            drop_lookup_mod_hash,
        ])
        .build()
}

/// Classify a dropped file or folder path.
///
/// Returns a `ClassificationResult` with an `item_type` tag that the frontend
/// can use to decide what UI to show (confirm dialog, error, etc.).
#[tauri::command]
pub async fn drop_classify(
    path: String,
) -> Result<ClassificationResult, String> {
    debug!("Drop event received: {}", path);
    let path = std::path::PathBuf::from(&path);
    let result = classify_dropped_item(&path);
    let classification = ClassificationResult::from(result);
    info!("Classification result: {:?}", classification);
    Ok(classification)
}

/// Classify ZIP

#[tauri::command]
pub async fn drop_classify_extract(
    path: String,
) -> Result<ClassificationResult, String> {
    debug!("Drop classify with extraction: {}", path);
    let path = std::path::PathBuf::from(&path);
    let result = classify_zip_with_extraction(&path);
    let classification = ClassificationResult::from(result);
    info!(
        "Classification result (with extraction): {:?}",
        classification
    );
    Ok(classification)
}

/// Scan for importable instances in a launcher's data directory.
///
/// `launcher_type` must be one of the `ImportLauncherType` variant names
/// (e.g. `"MultiMC"`, `"PrismLauncher"`, `"HMCL"`).
#[tauri::command]
pub async fn drop_scan_launcher_instances(
    launcher_type: String,
    base_path: String,
) -> Result<Vec<String>, String> {
    info!(
        "Scanning launcher instances — type: {launcher_type}, path: {base_path}"
    );
    let lt: ImportLauncherType =
        serde_json::from_str(&format!("\"{launcher_type}\"")).map_err(|e| {
            format!("Invalid launcher type '{launcher_type}': {e}")
        })?;
    let base = std::path::PathBuf::from(&base_path);
    let instances = get_importable_instances(lt, base)
        .await
        .map_err(|e| e.to_string())?;
    info!("Scan complete — found {} instance(s)", instances.len());
    Ok(instances.into_iter().map(|i| i.name).collect())
}

/// Detect processes holding a file lock on the given path.
///
/// Returns an empty list when detection is unavailable on the current platform
/// or the required tools are not installed.
#[tauri::command]
pub async fn drop_detect_file_lock(
    path: String,
) -> Result<Vec<LockingProcess>, String> {
    let path = std::path::PathBuf::from(&path);
    info!("Detecting file lock for: {}", path.display());
    let processes = get_locking_processes(&path);
    if !processes.is_empty() {
        warn!("File locked by {} process(es)", processes.len());
    }
    Ok(processes)
}

/// Extract mod metadata from a JAR file without installing it.
///
/// Reads the JAR bytes, extracts embedded mod metadata (fabric.mod.json,
/// quilt.mod.json, META-INF/mods.toml, etc.), and returns the parsed
/// `LocalModMetadata` as a JSON string.
#[tauri::command]
pub async fn drop_extract_mod_metadata(path: String) -> Result<String, String> {
    let path = std::path::PathBuf::from(&path);

    let file_bytes = tokio::fs::read(&path)
        .await
        .map_err(|e| format!("Failed to read file: {e}"))?;
    let bytes = bytes::Bytes::from(file_bytes);
    let meta = theseus::mod_metadata::extract_mod_metadata(&bytes)
        .ok_or_else(|| "No mod metadata found in file".to_string())?;
    serde_json::to_string(&meta)
        .map_err(|e| format!("Failed to serialize metadata: {e}"))
}

/// Look up a mod file by SHA1 hash to find matching Modrinth project and version.
///
/// Computes the SHA1 hash of the given file and queries the Modrinth API
/// to find matching versions. Returns project and version information if found.
#[tauri::command]
pub async fn drop_lookup_mod_hash(
    path: String,
) -> Result<Option<ModrinthLookupResult>, String> {
    let path = std::path::PathBuf::from(&path);
    info!("Looking up mod hash for: {}", path.display());

    lookup_mod_hash(&path)
        .await
        .map_err(|e| format!("Failed to lookup mod hash: {e}"))
}
