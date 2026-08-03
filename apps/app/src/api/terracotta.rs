use crate::api::Result;
use serde::Serialize;

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("terracotta")
        .invoke_handler(tauri::generate_handler![
            terracotta_get_state,
            terracotta_get_meta,
            terracotta_start,
            terracotta_stop,
            terracotta_host,
            terracotta_join,
            terracotta_reset,
            terracotta_get_platform_key,
            terracotta_download,
            terracotta_get_player_name,
        ])
        .build()
}

#[tauri::command]
pub async fn terracotta_get_state(
) -> Result<theseus::terracotta::TerracottaState> {
    Ok(theseus::terracotta::get_state().await)
}

#[derive(Serialize)]
pub struct TerracottaMetaResponse {
    pub version: String,
    pub compile_timestamp: String,
    pub easytier_version: String,
    pub yggdrasil_port: u16,
    pub target_tuple: String,
    pub target_os: String,
}

#[tauri::command]
pub async fn terracotta_get_meta() -> Result<TerracottaMetaResponse> {
    let meta = theseus::terracotta::get_meta()
        .await
        .map_err(theseus::Error::from)?;
    Ok(TerracottaMetaResponse {
        version: meta.version,
        compile_timestamp: meta.compile_timestamp,
        easytier_version: meta.easytier_version,
        yggdrasil_port: meta.yggdrasil_port,
        target_tuple: meta.target_tuple,
        target_os: meta.target_os,
    })
}

#[tauri::command]
pub async fn terracotta_start(
    binary_path: Option<String>,
    auto_download: Option<bool>,
) -> Result<()> {
    theseus::terracotta::start_terracotta(
        binary_path,
        auto_download.unwrap_or(true),
    )
    .await
    .map_err(theseus::Error::from)?;
    Ok(())
}

#[tauri::command]
pub async fn terracotta_stop() -> Result<()> {
    theseus::terracotta::stop_terracotta()
        .await
        .map_err(theseus::Error::from)?;
    Ok(())
}

#[tauri::command]
pub async fn terracotta_host(
    player_name: String,
    room_code: Option<String>,
) -> Result<()> {
    theseus::terracotta::start_hosting(room_code, player_name)
        .await
        .map_err(theseus::Error::from)?;
    Ok(())
}

#[tauri::command]
pub async fn terracotta_join(
    player_name: String,
    room_code: String,
) -> Result<()> {
    theseus::terracotta::start_joining(room_code, player_name)
        .await
        .map_err(theseus::Error::from)?;
    Ok(())
}

#[tauri::command]
pub async fn terracotta_reset() -> Result<()> {
    theseus::terracotta::reset_state()
        .await
        .map_err(theseus::Error::from)?;
    Ok(())
}

#[tauri::command]
pub async fn terracotta_get_platform_key() -> Result<String> {
    Ok(theseus::terracotta::terracotta_platform_key().to_string())
}

#[tauri::command]
pub async fn terracotta_download(version: Option<String>) -> Result<()> {
    theseus::terracotta::download_terracotta(version)
        .await
        .map_err(theseus::Error::from)?;
    Ok(())
}

#[tauri::command]
pub async fn terracotta_get_player_name() -> Result<String> {
    let name = theseus::terracotta::get_player_name().await;
    Ok(name)
}
