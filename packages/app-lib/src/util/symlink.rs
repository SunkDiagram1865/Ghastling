use std::sync::OnceLock;

use tokio::fs;
use tracing::error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymlinkCapability {
    Supported,
    RequiresAdmin,
    Unsupported,
}

static SYMLINK_CAPABILITY: OnceLock<SymlinkCapability> = OnceLock::new();

pub async fn check_symlink_capability() -> SymlinkCapability {
    if let Some(capability) = SYMLINK_CAPABILITY.get() {
        return *capability;
    }

    let capability = check_symlink_capability_internal().await;
    let _ = SYMLINK_CAPABILITY.set(capability);
    capability
}

async fn check_symlink_capability_internal() -> SymlinkCapability {
    let temp_dir = match tempfile::tempdir() {
        Ok(dir) => dir,
        Err(e) => {
            error!("failed to create temp dir — {e}");
            return SymlinkCapability::Unsupported;
        }
    };

    let target_path = temp_dir.path().join("target");
    let link_path = temp_dir.path().join("link");

    if let Err(e) = fs::create_dir(&target_path).await {
        error!("failed to create target dir — {e}");
        return SymlinkCapability::Unsupported;
    }

    #[cfg(target_os = "windows")]
    {
        let target = target_path.clone();
        let link = link_path.clone();
        let junction_result = tokio::task::spawn_blocking(move || {
            junction::create(&target, &link)
        })
        .await;

        if matches!(junction_result, Ok(Ok(()))) {
            return SymlinkCapability::Supported;
        }
        if let Err(e) = junction_result {
            error!(error = %e, "junction::create task panicked");
        }

        let target = target_path.clone();
        let link = link_path.clone();
        let symlink_result = tokio::task::spawn_blocking(move || {
            symlink_rs::symlink_dir(&target, &link)
        })
        .await;

        match symlink_result {
            Ok(Ok(_)) => SymlinkCapability::Supported,
            Ok(Err(e)) => {
                let raw_os_error = e.raw_os_error();
                if e.kind() == std::io::ErrorKind::PermissionDenied
                    || raw_os_error == Some(1314)
                {
                    error!("symlink: permission denied — {e}");
                    SymlinkCapability::RequiresAdmin
                } else {
                    error!("symlink: system does not support it — {e}");
                    SymlinkCapability::Unsupported
                }
            }
            Err(e) => {
                error!("symlink: blocking task panicked — {e}");
                SymlinkCapability::Unsupported
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let target = target_path.clone();
        let link = link_path.clone();
        let result = tokio::task::spawn_blocking(move || {
            symlink_rs::symlink_dir(&target, &link)
        })
        .await;

        match result {
            Ok(Ok(_)) => SymlinkCapability::Supported,
            Ok(Err(e)) => {
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    error!("symlink: permission denied — {e}");
                    SymlinkCapability::RequiresAdmin
                } else {
                    error!("symlink: system does not support it — {e}");
                    SymlinkCapability::Unsupported
                }
            }
            Err(e) => {
                error!("symlink: blocking task panicked — {e}");
                SymlinkCapability::Unsupported
            }
        }
    }
}
