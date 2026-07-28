use serde::{Deserialize, Serialize};

use super::unknown_value;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentProvider {
    Modrinth,
    #[serde(rename = "curseforge")]
    CurseForge,
}

impl ContentProvider {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Modrinth => "modrinth",
            Self::CurseForge => "curseforge",
        }
    }

    pub fn from_str(value: &str) -> crate::Result<Self> {
        match value {
            "modrinth" => Ok(Self::Modrinth),
            "curseforge" => Ok(Self::CurseForge),
            other => Err(unknown_value("content provider", other)),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContentProviderRef {
    pub provider: ContentProvider,
    pub project_id: String,
    pub version_id: Option<String>,
    pub primary: bool,
}
