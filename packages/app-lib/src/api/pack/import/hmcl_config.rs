//! HMCL data directory discovery.
//!
//! HMCL stores its configuration (`launcher-settings.json`) in a data directory
//! that depends on how the launcher was installed.  This module finds that
//! directory with a three‑priority strategy:
//!
//! 1. Portable mode — `{launcher_dir}/.hmcl/config/launcher-settings.json`
//! 2. System install — platform‑specific application data directory
//! 3. Environment variable — `$HMCL_DATA_DIR`

use std::path::{Path, PathBuf};

/// Try to locate the HMCL data directory by probing the three priority levels
/// in order.  Returns the first directory that contains
/// `launcher-settings.json`, or `None` if none does.
pub fn find_hmcl_data_dir(launcher_dir: &Path) -> Option<PathBuf> {
    // 1. Portable mode — side‑car `.hmcl` folder next to the launcher jar
    let portable = launcher_dir.join(".hmcl").join("config");
    if portable.join("launcher-settings.json").exists() {
        return Some(portable);
    }

    // 2. System install — standard platform data directory
    if let Some(system_dir) = system_data_dir()
        && system_dir.join("launcher-settings.json").exists()
    {
        return Some(system_dir);
    }

    // 3. Environment variable override
    if let Ok(env_dir) = std::env::var("HMCL_DATA_DIR") {
        let env_path = PathBuf::from(env_dir);
        if env_path.join("launcher-settings.json").exists() {
            return Some(env_path);
        }
    }

    None
}

/// Return the platform‑specific HMCL config directory for a system install.
fn system_data_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        dirs::data_dir().map(|d| d.join(".hmcl").join("config"))
    }
    #[cfg(target_os = "macos")]
    {
        dirs::data_dir().map(|d| d.join("hmcl").join("config"))
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        dirs::data_dir().map(|d| d.join("hmcl").join("config"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_data_dir_is_some() {
        // On any real OS this should return a path (it may or may not exist).
        assert!(system_data_dir().is_some());
    }

    #[test]
    fn test_find_hmcl_data_dir_returns_none_for_bogus_path() {
        let bogus = Path::new("/tmp/this-does-not-exist-12345");
        assert!(find_hmcl_data_dir(bogus).is_none());
    }
}
