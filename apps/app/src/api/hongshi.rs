use crate::api::Result;

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("hongshi")
        .invoke_handler(tauri::generate_handler![
            hongshi_get_state,
            hongshi_get_nodes,
            hongshi_get_detected_ports,
            hongshi_download,
            hongshi_host,
            hongshi_stop,
            hongshi_reset,
            hongshi_open_logs,
        ])
        .build()
}

#[tauri::command]
pub async fn hongshi_get_state() -> Result<theseus::hongshi::HongshiState> {
    Ok(theseus::hongshi::get_state().await)
}

#[tauri::command]
pub async fn hongshi_get_nodes(
    force_refresh: Option<bool>,
) -> Result<Vec<theseus::hongshi::HongshiNode>> {
    Ok(theseus::hongshi::get_nodes(force_refresh.unwrap_or(false))
        .await
        .map_err(theseus::Error::from)?)
}

#[tauri::command]
pub async fn hongshi_get_detected_ports()
-> Result<Vec<theseus::hongshi::DetectedLanPort>> {
    Ok(theseus::hongshi::get_detected_ports().await)
}

#[tauri::command]
pub async fn hongshi_download() -> Result<()> {
    Ok(theseus::hongshi::download()
        .await
        .map_err(theseus::Error::from)?)
}

#[tauri::command]
pub async fn hongshi_host(
    local_port: u16,
    node_name: Option<String>,
    instance_id: Option<String>,
) -> Result<()> {
    Ok(theseus::hongshi::start(local_port, node_name, instance_id)
        .await
        .map_err(theseus::Error::from)?)
}

#[tauri::command]
pub async fn hongshi_stop() -> Result<()> {
    Ok(theseus::hongshi::stop()
        .await
        .map_err(theseus::Error::from)?)
}

#[tauri::command]
pub async fn hongshi_reset() -> Result<()> {
    Ok(theseus::hongshi::reset_state()
        .await
        .map_err(theseus::Error::from)?)
}

#[tauri::command]
pub async fn hongshi_open_logs<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<()> {
    tokio::fs::create_dir_all(theseus::hongshi::logs_dir()).await?;
    crate::api::utils::open_path(app, theseus::hongshi::logs_dir()).await;
    Ok(())
}
