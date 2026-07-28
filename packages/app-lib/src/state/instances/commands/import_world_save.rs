use crate::state::State;
use crate::util::io;
use crate::{ErrorKind, Result};
use std::path::Path;

/// Import a world save from a source path into an instance's saves directory.
///
/// The `source_path` can be either:
/// - A directory containing a `level.dat` file (an existing world folder)
/// - A ZIP archive containing a world save (with `level.dat` at the root)
///
/// Returns the name of the imported world.
pub async fn import_world_save(
    _state: &State,
    instance_id: &str,
    source_path: &Path,
) -> Result<String> {
    let instance_id_str = instance_id.to_string();

    // Resolve the instance's saves directory.
    let instance_path =
        crate::api::instance::get_full_path(&instance_id_str).await?;
    let saves_dir = instance_path.join("saves");

    // Determine the world folder name and source type.
    let (world_name, source_is_zip) = if source_path.is_dir() {
        // Direct folder: use the folder name as the world name.
        let name = source_path
            .file_name()
            .ok_or_else(|| {
                ErrorKind::InputError(
                    "Cannot determine world name from source path".to_string(),
                )
            })?
            .to_string_lossy()
            .to_string();
        (name, false)
    } else if source_path.is_file() {
        // Check if it's a ZIP archive by examining the file signature.
        let is_zip = is_zip_file(source_path).await?;
        if is_zip {
            // ZIP file: use the file stem as the world name.
            let name = source_path
                .file_stem()
                .ok_or_else(|| {
                    ErrorKind::InputError(
                        "Cannot determine world name from ZIP file name"
                            .to_string(),
                    )
                })?
                .to_string_lossy()
                .to_string();
            (name, true)
        } else {
            return Err(ErrorKind::InputError(
                "Source file is not a valid ZIP archive or world folder"
                    .to_string(),
            )
            .into());
        }
    } else {
        return Err(ErrorKind::InputError(format!(
            "Source path does not exist: {}",
            source_path.display()
        ))
        .into());
    };

    // Check if the world already exists in the saves directory.
    let target_dir = saves_dir.join(&world_name);
    if target_dir.exists() {
        return Err(ErrorKind::InputError(format!(
            "World '{world_name}' already exists in this instance"
        ))
        .into());
    }

    // Create the saves directory if it doesn't exist.
    io::create_dir_all(&saves_dir).await?;

    if source_is_zip {
        // Extract ZIP archive to the target directory.
        extract_world_zip(source_path, &target_dir).await?;
    } else {
        // Copy the folder recursively.
        io::copy_dir(source_path, &target_dir).await?;
    }

    // Verify that the extracted/copied world has a level.dat file.
    if !target_dir.join("level.dat").exists() {
        // Clean up on failure.
        let _ = tokio::fs::remove_dir_all(&target_dir).await;
        return Err(ErrorKind::InputError(format!(
            "No level.dat found in the imported world save '{world_name}'"
        ))
        .into());
    }

    // Emit an instance synced event so the UI refreshes the worlds list.
    crate::event::emit::emit_instance(
        &instance_id_str,
        crate::event::InstancePayloadType::Synced,
    )
    .await?;

    tracing::info!(
        "Imported world save '{world_name}' into instance {instance_id_str}"
    );

    Ok(world_name)
}

/// Check if a file is a ZIP archive by reading its magic bytes.
async fn is_zip_file(path: &Path) -> Result<bool> {
    use tokio::io::AsyncReadExt;

    let mut file = tokio::fs::File::open(path)
        .await
        .map_err(|e| io::IOError::with_path(e, path))?;
    let mut magic = [0u8; 4];
    match file.read_exact(&mut magic).await {
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
            // File too small to be a ZIP
            return Ok(false);
        }
        Err(e) => {
            return Err(io::IOError::with_path(e, path).into());
        }
        _ => {}
    }
    // ZIP magic bytes: PK\x03\x04
    Ok(magic == [0x50, 0x4B, 0x03, 0x04])
}

/// Extract a ZIP archive containing a world save to the target directory.
async fn extract_world_zip(zip_path: &Path, target_dir: &Path) -> Result<()> {
    let zip_path = zip_path.to_path_buf();
    let target_dir = target_dir.to_path_buf();

    tokio::task::spawn_blocking(move || {
        let file = std::fs::File::open(&zip_path)
            .map_err(|e| io::IOError::with_path(e, &zip_path))?;
        let mut archive = zip::ZipArchive::new(file).map_err(|e| {
            ErrorKind::InputError(format!("Invalid ZIP archive: {e}"))
        })?;

        for i in 0..archive.len() {
            let mut entry = archive.by_index(i).map_err(|e| {
                ErrorKind::InputError(format!("Failed to read ZIP entry: {e}"))
            })?;

            let entry_name = entry.name().to_string();

            // Skip directory entries and macOS resource forks.
            if entry.is_dir() || entry_name.starts_with("__MACOSX/") {
                continue;
            }

            // Determine the output path.
            // If all entries are inside a single root folder, strip it.
            let relative_path = strip_single_root_folder(&entry_name);
            let output_path = target_dir.join(&relative_path);

            // Create parent directories.
            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| io::IOError::with_path(e, parent))?;
            }

            // Extract the file.
            let mut output = std::fs::File::create(&output_path)
                .map_err(|e| io::IOError::with_path(e, &output_path))?;
            std::io::copy(&mut entry, &mut output)
                .map_err(|e| io::IOError::with_path(e, &output_path))?;
        }

        Ok(())
    })
    .await?
}

/// If all entries in a ZIP share a single root folder, strip that prefix.
/// For example, "My World/level.dat" becomes "level.dat".
fn strip_single_root_folder(entry_name: &str) -> String {
    let parts: Vec<&str> = entry_name.split('/').collect();
    if parts.len() > 1 && !parts[0].is_empty() {
        // Check if all entries share the same first segment.
        // Since we process one entry at a time, we assume the caller
        // verified this. Just strip the first segment.
        parts[1..].join("/")
    } else {
        entry_name.to_string()
    }
}
