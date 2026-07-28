use std::{collections::HashMap, path::PathBuf};

use super::instance_json;
use crate::{
    State,
    install::{InstallPhaseDetails, InstallProgressReporter},
    launcher::get_loader_version_from_profile,
    pack::{
        import::finish_import,
        install_from::{self, CreatePackDescription, PackDependency},
    },
    state::ModLoader,
};

pub async fn import_generic(
    instance_folder: PathBuf,
    instance_id: &str,
    reporter: InstallProgressReporter,
    details: InstallPhaseDetails,
    symlink: bool,
) -> crate::Result<()> {
    let name = instance_folder
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "imported".to_string());

    let dotminecraft = instance_folder.join(".minecraft");
    let dotminecraft = if dotminecraft.is_dir() {
        tracing::debug!(
            "import_generic: using .minecraft subdir at {}",
            dotminecraft.display()
        );
        dotminecraft
    } else {
        tracing::debug!(
            "import_generic: using folder directly at {}",
            instance_folder.display()
        );
        instance_folder
    };

    tracing::debug!(
        "import_generic: about to detect instance_json at dotminecraft={}",
        dotminecraft.display()
    );
    let info = instance_json::detect(&dotminecraft).ok_or_else(|| {
        tracing::warn!(
            "import_generic: instance_json::detect returned None for {}",
            dotminecraft.display()
        );
		crate::ErrorKind::InputError(
			"Could not detect Minecraft version. Make sure the folder contains a valid version JSON.".into(),
		)
	})?;
    tracing::debug!(
        "import_generic: detect result: vanilla_name={} loader={:?} loader_version={:?}",
        info.vanilla_name,
        info.loader,
        info.loader_version
    );

    let description = CreatePackDescription {
        icon: None,
        override_title: Some(name),
        project_id: None,
        version_id: None,
        instance_id: instance_id.to_string(),
        source_filename: None,
    };

    let mut dependencies =
        HashMap::from([(PackDependency::Minecraft, info.vanilla_name.clone())]);
    if let Some(ref loader) = info.loader {
        let dep = match loader.as_str() {
            "forge" => Some(PackDependency::Forge),
            "neoforge" => Some(PackDependency::NeoForge),
            "fabric" => Some(PackDependency::FabricLoader),
            "quilt" => Some(PackDependency::QuiltLoader),
            _ => None,
        };
        let mut loader_version = info.loader_version.clone();
        // If loader was detected but version couldn't be extracted, try to
        // resolve the latest version for this loader + game version.
        if loader_version.is_none() {
            let mod_loader = match loader.as_str() {
                "forge" => Some(ModLoader::Forge),
                "neoforge" => Some(ModLoader::NeoForge),
                "fabric" => Some(ModLoader::Fabric),
                "quilt" => Some(ModLoader::Quilt),
                _ => None,
            };
            if let Some(mod_loader) = mod_loader {
                tracing::debug!(
                    "import_generic: loader={} has no version, resolving latest for game_version={}",
                    loader,
                    info.vanilla_name
                );
                match get_loader_version_from_profile(
                    &info.vanilla_name,
                    mod_loader,
                    None,
                )
                .await
                {
                    Ok(Some(lv)) => {
                        tracing::debug!(
                            "import_generic: resolved latest loader version: {}",
                            lv.id
                        );
                        loader_version = Some(lv.id);
                    }
                    Ok(None) => {
                        tracing::warn!(
                            "import_generic: no loader version found for {} {}",
                            mod_loader.as_str(),
                            info.vanilla_name
                        );
                    }
                    Err(e) => {
                        tracing::warn!(
                            "import_generic: failed to resolve loader version: {e}",
                        );
                    }
                }
            }
        }
        tracing::debug!(
            "import_generic: loader={} dep={:?} version={:?}",
            loader,
            dep,
            loader_version
        );
        if let (Some(dep), Some(version)) = (dep, loader_version) {
            dependencies.insert(dep, version);
        } else {
            tracing::warn!(
                "import_generic: loader={} could not be mapped to PackDependency",
                loader
            );
        }
    } else {
        tracing::debug!("import_generic: no loader detected, will be Vanilla");
    }

    tracing::debug!(
        "import_generic: setting instance info with dependencies={:?}",
        dependencies
    );
    install_from::set_instance_information(
        instance_id.to_string(),
        &description,
        "Imported from folder",
        None,
        &dependencies,
        false,
    )
    .await?;

    let state = State::get().await?;
    tracing::debug!(
        "import_generic: finishing import for instance_id={}",
        instance_id
    );
    finish_import(
        instance_id,
        dotminecraft,
        &state.io_semaphore,
        reporter,
        details,
        symlink,
    )
    .await
}
