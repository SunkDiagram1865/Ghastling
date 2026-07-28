use serde::Deserialize;

/// Fabric mod.json (and Quilt's similar quilt.mod.json wrapped under quilt_loader).
#[derive(Debug, Deserialize)]
pub(crate) struct FabricModJson {
    pub id: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub authors: Vec<FabricAuthorOrArray>,
    #[serde(default)]
    pub contributors: Vec<FabricAuthorOrArray>,
    pub icon: Option<String>,
    #[serde(rename = "contact")]
    pub _contact: Option<serde_json::Value>,
    /// Dependency resolution (Fabric: map of id→version, Quilt: array of objects).
    /// Uses `serde_json::Value` to handle both formats.
    pub depends: Option<serde_json::Value>,
    #[allow(dead_code)]
    pub recommends: Option<serde_json::Value>,
    #[allow(dead_code)]
    pub conflicts: Option<serde_json::Value>,
    #[allow(dead_code)]
    pub breaks: Option<serde_json::Value>,
}

/// An author/contributor entry: either a plain string or `{"name": "...", "contact": {...}}`.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum FabricAuthorOrArray {
    Plain(String),
    Object { name: Option<String> },
}

/// Quilt's wrapper: `{"quilt_loader": { "id": "...", ... }}`.
#[derive(Debug, Deserialize)]
pub(crate) struct QuiltModJson {
    pub quilt_loader: FabricModJson,
}

/// Extract a string value from a Fabric-style depends map.
pub(crate) fn fabric_dep_value(
    depends: &Option<serde_json::Value>,
    key: &str,
) -> Option<String> {
    let obj = depends.as_ref()?.as_object()?;
    obj.get(key).and_then(|v| match v {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Array(arr) => {
            arr.first().and_then(|v| v.as_str().map(String::from))
        }
        _ => None,
    })
}

/// Extract a string value from a Quilt-style depends array.
pub(crate) fn quilt_dep_value(
    depends: &Option<serde_json::Value>,
    key: &str,
) -> Option<String> {
    let arr = depends.as_ref()?.as_array()?;
    for dep in arr {
        let id = dep
            .as_object()
            .and_then(|obj| obj.get("id"))
            .and_then(|v| v.as_str())?;
        if id == key {
            return dep
                .as_object()
                .and_then(|obj| obj.get("versions"))
                .and_then(|v| v.as_str())
                .map(String::from);
        }
    }
    None
}
