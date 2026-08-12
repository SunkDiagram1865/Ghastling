use eyre::{Context, bail};
use flate2::read::GzDecoder;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::{Arc, LazyLock};
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tracing::{info, warn};

pub use self::binary::terracotta_platform_key;
use self::binary::{
    cleanup_legacy_versions, find_terracotta_executable,
    install_terracotta_binary, is_terracotta_executable,
    resolve_installed_terracotta_binary_path, resolve_terracotta_binary_path,
    terracotta_binary_name, terracotta_binary_path, terracotta_download_urls,
    validate_terracotta_version, versioned_terracotta_binary_name,
};
use self::lan::MinecraftLanAnnouncer;
use super::data::Credentials;

mod binary;
mod lan;

const MAX_TERRACOTTA_ARCHIVE_SIZE: u64 = 256 * 1024 * 1024;
const MAX_CONSECUTIVE_POLL_FAILURES: u8 = 3;
const TERRACOTTA_REQUEST_TIMEOUT: std::time::Duration =
    std::time::Duration::from_secs(10);

static TERRACOTTA_STATE: LazyLock<Mutex<TerracottaState>> =
    LazyLock::new(|| Mutex::new(TerracottaState::default()));

static TERRACOTTA_RUNTIME: LazyLock<Mutex<TerracottaRuntime>> =
    LazyLock::new(|| Mutex::new(TerracottaRuntime::default()));

static TERRACOTTA_OPERATION: LazyLock<Mutex<()>> =
    LazyLock::new(|| Mutex::new(()));

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TerracottaState {
    pub status: TerracottaStatus,
    pub http_port: Option<u16>,
    pub room_code: Option<String>,
    pub server_port: Option<u16>,
    pub players: Vec<PlayerInfo>,
    pub download_progress: Option<u8>,
    pub download_stage: Option<TerracottaDownloadStage>,
    pub binary_installed: bool,
    pub error_type: Option<TerracottaErrorType>,
    pub error_message: Option<String>,
    pub profile_index: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerracottaStatus {
    Idle,
    Starting,
    Downloading,
    Waiting,
    HostScanning,
    HostStarting,
    HostReady,
    GuestConnecting,
    GuestStarting,
    GuestReady,
    Error,
    Fatal,
}

impl Default for TerracottaStatus {
    fn default() -> Self {
        Self::Idle
    }
}

impl TerracottaStatus {
    fn from_api(value: &str) -> Option<Self> {
        match value {
            "idle" => Some(Self::Idle),
            "starting" => Some(Self::Starting),
            "waiting" => Some(Self::Waiting),
            "host-scanning" => Some(Self::HostScanning),
            "host-starting" => Some(Self::HostStarting),
            "host-ok" => Some(Self::HostReady),
            "guest-connecting" => Some(Self::GuestConnecting),
            "guest-starting" => Some(Self::GuestStarting),
            "guest-ok" => Some(Self::GuestReady),
            "exception" => Some(Self::Error),
            "fatal" => Some(Self::Fatal),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerracottaDownloadStage {
    Preparing,
    Downloading,
    Verifying,
    Extracting,
    Installing,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerracottaErrorType {
    Os,
    Network,
    Install,
    Terracotta,
    Unknown,
}

impl From<i32> for TerracottaErrorType {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Os,
            1 => Self::Network,
            2 => Self::Install,
            3 => Self::Terracotta,
            _ => Self::Unknown,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub machine_id: String,
    pub name: String,
    pub vendor: String,
    pub kind: TerracottaPlayerKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TerracottaPlayerKind {
    Host,
    Guest,
    Unknown,
}

impl From<&str> for TerracottaPlayerKind {
    fn from(value: &str) -> Self {
        match value.to_ascii_uppercase().as_str() {
            "HOST" => Self::Host,
            "GUEST" => Self::Guest,
            _ => Self::Unknown,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerracottaMeta {
    pub version: String,
    pub compile_timestamp: String,
    pub easytier_version: String,
    pub yggdrasil_port: u16,
    pub target_tuple: String,
    pub target_os: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TerracottaApiState {
    state: String,
    #[serde(default)]
    index: Option<u32>,
    #[serde(default)]
    room: Option<String>,
    #[serde(default)]
    profile_index: Option<u32>,
    #[serde(default)]
    profiles: Option<Vec<TerracottaApiProfile>>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    r#type: Option<i32>,
    #[serde(default)]
    difficulty: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TerracottaApiProfile {
    machine_id: String,
    name: String,
    vendor: String,
    kind: String,
}

struct TerracottaProcess {
    child: Child,
    output: Arc<Mutex<VecDeque<String>>>,
}

#[derive(Default)]
struct TerracottaRuntime {
    process: Option<TerracottaProcess>,
    state_poller: Option<JoinHandle<()>>,
    lan_announcer: MinecraftLanAnnouncer,
}

impl Drop for TerracottaProcess {
    fn drop(&mut self) {
        let _ = self.child.start_kill();
    }
}

fn capture_terracotta_output<R>(
    reader: R,
    stream: &'static str,
    pid: u32,
    output: Arc<Mutex<VecDeque<String>>>,
) where
    R: AsyncRead + Unpin + Send + 'static,
{
    tokio::spawn(async move {
        let mut lines = BufReader::new(reader).lines();
        while let Ok(Some(text)) = lines.next_line().await {
            tracing::debug!(target: "terracotta", pid, stream, "{text}");
            let mut captured = output.lock().await;
            captured.push_back(format!("{stream}: {text}"));
            if captured.len() > 50 {
                captured.pop_front();
            }
        }
    });
}

async fn take_terminated_terracotta_process() -> eyre::Result<
    Option<(std::process::ExitStatus, Arc<Mutex<VecDeque<String>>>)>,
> {
    let mut runtime = TERRACOTTA_RUNTIME.lock().await;
    let Some(process) = runtime.process.as_mut() else {
        return Ok(None);
    };
    let Some(status) = process
        .child
        .try_wait()
        .wrap_err("failed to inspect terracotta process")?
    else {
        return Ok(None);
    };
    let output = process.output.clone();
    runtime.process.take();
    Ok(Some((status, output)))
}

async fn format_terracotta_exit(
    status: std::process::ExitStatus,
    output: Arc<Mutex<VecDeque<String>>>,
) -> String {
    tokio::time::sleep(std::time::Duration::from_millis(25)).await;
    let captured = output.lock().await;
    let details = captured.iter().cloned().collect::<Vec<_>>().join("\n");
    if details.is_empty() {
        format!("terracotta exited before startup completed ({status})")
    } else {
        format!(
            "terracotta exited before startup completed ({status}):\n{details}"
        )
    }
}

async fn get_latest_terracotta_version() -> eyre::Result<String> {
    #[derive(Deserialize)]
    struct ReleaseInfo {
        tag_name: String,
    }

    let endpoints = [
        "https://gitee.com/api/v5/repos/burningtnt/Terracotta/releases/latest",
        "https://api.github.com/repos/burningtnt/Terracotta/releases/latest",
    ];
    let mut failures = Vec::new();

    for endpoint in endpoints {
        let response = crate::util::fetch::INSECURE_REQWEST_CLIENT
            .get(endpoint)
            .header("Accept", "application/json")
            .header("User-Agent", crate::launcher_user_agent())
            .send()
            .await;
        match response {
            Ok(response) if response.status().is_success() => {
                match response.json::<ReleaseInfo>().await {
                    Ok(info) => {
                        return Ok(info
                            .tag_name
                            .trim_start_matches('v')
                            .to_string());
                    }
                    Err(error) => failures.push(format!("{endpoint}: {error}")),
                }
            }
            Ok(response) => {
                failures.push(format!("{endpoint}: HTTP {}", response.status()))
            }
            Err(error) => failures.push(format!("{endpoint}: {error}")),
        }
    }

    bail!(
        "failed to fetch latest terracotta release info: {}",
        failures.join("; ")
    )
}

async fn download_terracotta_inner(
    version: Option<String>,
) -> eyre::Result<()> {
    {
        let mut state = TERRACOTTA_STATE.lock().await;
        state.status = TerracottaStatus::Downloading;
        state.download_progress = Some(0);
        state.download_stage = Some(TerracottaDownloadStage::Preparing);
        state.error_type = None;
        state.error_message = None;
    }

    let version = match version {
        Some(v) => v,
        None => get_latest_terracotta_version().await?,
    };
    validate_terracotta_version(&version)?;

    let platform = terracotta_platform_key();
    if platform == "unsupported" {
        bail!(
            "no terracotta binary available for {}/{}",
            std::env::consts::OS,
            std::env::consts::ARCH
        );
    }

    let urls = terracotta_download_urls(&version, platform);
    let mut archive_data: Option<Vec<u8>> = None;

    for url in &urls {
        info!("attempting to download terracotta from {url}");

        {
            let mut state = TERRACOTTA_STATE.lock().await;
            state.download_stage = Some(TerracottaDownloadStage::Downloading);
        }

        match crate::util::fetch::INSECURE_REQWEST_CLIENT
            .get(url)
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => {
                let total_size = response.content_length().unwrap_or(0);
                if total_size > MAX_TERRACOTTA_ARCHIVE_SIZE {
                    warn!(
                        "terracotta archive from {url} is too large: {total_size} bytes"
                    );
                    continue;
                }
                let mut downloaded: u64 = 0;
                let mut stream = response.bytes_stream();
                let mut data = Vec::with_capacity(total_size as usize);
                let mut hasher = Sha512::new();
                let mut stream_error = None;

                while let Some(chunk) = stream.next().await {
                    let chunk = match chunk {
                        Ok(chunk) => chunk,
                        Err(error) => {
                            stream_error = Some(error.to_string());
                            break;
                        }
                    };
                    hasher.update(&chunk);
                    data.extend_from_slice(&chunk);
                    downloaded += chunk.len() as u64;
                    if downloaded > MAX_TERRACOTTA_ARCHIVE_SIZE {
                        stream_error = Some(format!(
                            "archive exceeds {} bytes",
                            MAX_TERRACOTTA_ARCHIVE_SIZE
                        ));
                        break;
                    }

                    if total_size > 0 {
                        let pct = ((downloaded as f64 / total_size as f64)
                            * 100.0)
                            .clamp(0.0, 100.0)
                            as u8;
                        let mut state = TERRACOTTA_STATE.lock().await;
                        state.download_progress = Some(pct);
                    }
                }
                if let Some(error) = stream_error {
                    warn!("download from {url} failed: {error}");
                    continue;
                }

                let computed_hash = format!("{:x}", hasher.finalize());

                {
                    let mut state = TERRACOTTA_STATE.lock().await;
                    state.download_stage =
                        Some(TerracottaDownloadStage::Verifying);
                }

                let hash_url = format!("{url}.sha512");
                match crate::util::fetch::INSECURE_REQWEST_CLIENT
                    .get(&hash_url)
                    .send()
                    .await
                {
                    Ok(hash_resp) if hash_resp.status().is_success() => {
                        let checksum = hash_resp
                            .text()
                            .await
                            .unwrap_or_default()
                            .trim()
                            .to_string();
                        let expected_hash = checksum.split_whitespace().next();
                        if expected_hash.is_none_or(|hash| {
                            hash.len() != 128
                                || !hash.chars().all(|character| {
                                    character.is_ascii_hexdigit()
                                })
                                || !computed_hash.eq_ignore_ascii_case(hash)
                        }) {
                            warn!(
                                "SHA-512 mismatch for terracotta archive from {url}"
                            );
                            continue;
                        }
                        info!(
                            "SHA-512 verification passed for terracotta archive"
                        );
                    }
                    _ => {
                        warn!(
                            "no SHA-512 checksum available at {hash_url}, skipping verification"
                        );
                    }
                }

                archive_data = Some(data);
                break;
            }
            Ok(response) => {
                warn!(
                    "download from {url} returned HTTP {}",
                    response.status()
                );
            }
            Err(e) => {
                warn!("download from {url} failed: {e}");
            }
        }
    }

    let archive_data = archive_data.ok_or_else(|| {
        eyre::eyre!("all download sources failed for terracotta v{version}")
    })?;

    {
        let mut state = TERRACOTTA_STATE.lock().await;
        state.download_stage = Some(TerracottaDownloadStage::Extracting);
        state.download_progress = None;
    }

    let target_dir = terracotta_binary_path()
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("terracotta"));

    tokio::fs::create_dir_all(&target_dir)
        .await
        .wrap_err("failed to create terracotta directory")?;

    let staging_dir = tempfile::Builder::new()
        .prefix("terracotta-install-")
        .tempdir_in(&target_dir)
        .wrap_err(
            "failed to create terracotta installation staging directory",
        )?;
    let staging_path = staging_dir.path().to_path_buf();
    let archive_data_clone = archive_data;

    tokio::task::spawn_blocking(move || -> eyre::Result<()> {
        let decoder = GzDecoder::new(&archive_data_clone[..]);
        let mut archive = tar::Archive::new(decoder);
        archive
            .unpack(&staging_path)
            .wrap_err("failed to extract terracotta archive")?;
        Ok(())
    })
    .await??;

    {
        let mut state = TERRACOTTA_STATE.lock().await;
        state.download_stage = Some(TerracottaDownloadStage::Installing);
    }

    let expected_path = target_dir.join(terracotta_binary_name());
    let versioned_name = versioned_terracotta_binary_name(&version, platform);
    let candidate =
        find_terracotta_executable(staging_dir.path(), Some(&versioned_name))
            .ok_or_else(|| {
            eyre::eyre!(
                "no valid terracotta executable found in downloaded archive"
            )
        })?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = std::fs::metadata(&candidate)?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&candidate, permissions)?;
    }

    install_terracotta_binary(&candidate, &expected_path).await?;
    if !is_terracotta_executable(&expected_path) {
        bail!("installed terracotta binary failed executable validation");
    }
    info!(
        "installed {} as {}",
        candidate.display(),
        expected_path.display()
    );

    cleanup_legacy_versions(&version).await?;

    info!(
        "terracotta v{version} installed to {}",
        target_dir.display()
    );

    let mut state = TERRACOTTA_STATE.lock().await;
    state.download_progress = Some(100);
    state.download_stage = Some(TerracottaDownloadStage::Complete);
    drop(state);
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    let mut state = TERRACOTTA_STATE.lock().await;
    state.status = TerracottaStatus::Idle;
    state.download_progress = None;
    state.download_stage = None;
    Ok(())
}

async fn run_terracotta_download(version: Option<String>) -> eyre::Result<()> {
    let result = download_terracotta_inner(version).await;
    if let Err(error) = &result {
        let mut state = TERRACOTTA_STATE.lock().await;
        state.status = TerracottaStatus::Error;
        state.download_progress = None;
        state.download_stage = None;
        state.error_type = Some(TerracottaErrorType::Install);
        state.error_message = Some(format!("{error:#}"));
    }
    result
}

pub async fn download_terracotta(version: Option<String>) -> eyre::Result<()> {
    let _operation = TERRACOTTA_OPERATION.lock().await;
    if TERRACOTTA_STATE.lock().await.http_port.is_some() {
        bail!(
            "cannot replace terracotta while the multiplayer service is running"
        );
    }
    run_terracotta_download(version).await
}

fn terracotta_client() -> &'static reqwest::Client {
    static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
        reqwest::Client::builder()
            .no_proxy()
            .connect_timeout(std::time::Duration::from_secs(2))
            .timeout(TERRACOTTA_REQUEST_TIMEOUT)
            .build()
            .expect("terracotta client should build")
    });
    &CLIENT
}

async fn terracotta_get<T: serde::de::DeserializeOwned>(
    port: u16,
    path: &str,
) -> eyre::Result<T> {
    let client = terracotta_client();
    let url = format!("http://127.0.0.1:{port}{path}");
    let resp = client.get(&url).send().await.wrap_err_with(|| {
        format!("failed to connect to terracotta at {url}")
    })?;
    let status = resp.status();
    let body = resp
        .text()
        .await
        .wrap_err("failed to read terracotta response")?;
    if !status.is_success() {
        bail!(
            "terracotta returned {} for {}: {}",
            status.as_u16(),
            path,
            body
        )
    }
    let parsed: T = serde_json::from_str(&body).wrap_err_with(|| {
        format!("failed to parse terracotta response for {path}: {body}")
    })?;
    Ok(parsed)
}

fn terracotta_server_port(url: Option<&str>) -> Option<u16> {
    url?.parse::<std::net::SocketAddr>()
        .ok()
        .map(|url| url.port())
}

async fn sync_minecraft_lan_announcer(port: Option<u16>) {
    TERRACOTTA_RUNTIME.lock().await.lan_announcer.sync(port);
}

async fn spawn_terracotta_state_poller(port: u16) {
    let mut runtime = TERRACOTTA_RUNTIME.lock().await;
    if let Some(task) = runtime.state_poller.take() {
        task.abort();
    }
    runtime.state_poller = Some(tokio::spawn(poll_terracotta_state(port)));
}

async fn poll_terracotta_state(port: u16) {
    let mut last_index: u32 = 0;
    let mut consecutive_failures = 0;
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        match terracotta_get::<TerracottaApiState>(port, "/state").await {
            Ok(api_state) => {
                consecutive_failures = 0;
                let new_index = api_state.index.unwrap_or(0);
                if new_index > 0 && new_index <= last_index {
                    continue;
                }
                last_index = new_index;
                let server_port = if api_state.state == "guest-ok" {
                    terracotta_server_port(api_state.url.as_deref())
                } else {
                    None
                };

                let mut state = TERRACOTTA_STATE.lock().await;
                state.http_port = Some(port);
                state.room_code = api_state.room.clone();
                state.server_port = server_port;
                state.profile_index = api_state.profile_index;

                state.status = TerracottaStatus::from_api(&api_state.state)
                    .unwrap_or_else(|| {
                        warn!("unknown terracotta state: {}", api_state.state);
                        TerracottaStatus::Error
                    });

                if TerracottaStatus::from_api(&api_state.state).is_none() {
                    state.error_type = Some(TerracottaErrorType::Terracotta);
                    state.error_message = Some(format!(
                        "unsupported terracotta state: {}",
                        api_state.state
                    ));
                } else if state.status == TerracottaStatus::Fatal {
                    state.error_type =
                        api_state.r#type.map(TerracottaErrorType::from);
                    state.error_message = api_state.url.clone();
                } else if state.status != TerracottaStatus::Error {
                    state.error_type = None;
                    state.error_message = None;
                }

                if let Some(profiles) = api_state.profiles {
                    state.players = profiles
                        .into_iter()
                        .map(|p| PlayerInfo {
                            machine_id: p.machine_id,
                            name: p.name,
                            vendor: p.vendor,
                            kind: TerracottaPlayerKind::from(p.kind.as_str()),
                        })
                        .collect();
                }
                drop(state);
                sync_minecraft_lan_announcer(server_port).await;
            }
            Err(e) => {
                warn!("failed to poll terracotta state: {e:#}");
                consecutive_failures += 1;
                if consecutive_failures < MAX_CONSECUTIVE_POLL_FAILURES {
                    continue;
                }
                let mut state = TERRACOTTA_STATE.lock().await;
                if state.status != TerracottaStatus::Idle {
                    state.status = TerracottaStatus::Error;
                }
                state.http_port = None;
                state.server_port = None;
                drop(state);
                let mut runtime = TERRACOTTA_RUNTIME.lock().await;
                if let Some(mut process) = runtime.process.take() {
                    process.child.start_kill().ok();
                }
                runtime.lan_announcer.sync(None);
                break;
            }
        }
    }
}

fn is_binary_installed() -> bool {
    is_terracotta_executable(&resolve_installed_terracotta_binary_path())
}

pub async fn get_state() -> TerracottaState {
    let mut state = TERRACOTTA_STATE.lock().await.clone();
    state.binary_installed = is_binary_installed();
    state
}

pub async fn get_meta() -> eyre::Result<TerracottaMeta> {
    let state = TERRACOTTA_STATE.lock().await;
    let port = state
        .http_port
        .ok_or_else(|| eyre::eyre!("terracotta is not running"))?;
    drop(state);
    terracotta_get::<TerracottaMeta>(port, "/meta").await
}

pub async fn get_player_name() -> String {
    match crate::State::get_if_initialized() {
        Some(state_ref) => match Credentials::get_active(&state_ref.pool).await
        {
            Ok(Some(creds)) => creds.offline_profile.name,
            _ => "Anonymous".to_string(),
        },
        None => "Anonymous".to_string(),
    }
}

#[derive(Deserialize)]
struct TerracottaPortInfo {
    port: u16,
}

fn terracotta_port_file() -> PathBuf {
    std::env::temp_dir()
        .join(format!("terracotta_port_{}.json", std::process::id()))
}

pub async fn start_terracotta(
    binary_path: Option<String>,
    auto_download: bool,
) -> eyre::Result<()> {
    let _operation = TERRACOTTA_OPERATION.lock().await;
    let custom_bin_path = binary_path.map(PathBuf::from);
    let mut final_path = custom_bin_path.as_deref().map_or_else(
        resolve_installed_terracotta_binary_path,
        resolve_terracotta_binary_path,
    );

    if !is_terracotta_executable(&final_path) && auto_download {
        info!("terracotta binary not found, attempting auto-download");
        run_terracotta_download(None).await?;
        final_path = custom_bin_path.as_deref().map_or_else(
            resolve_installed_terracotta_binary_path,
            resolve_terracotta_binary_path,
        );
    }

    if !is_terracotta_executable(&final_path) {
        bail!(
            "valid terracotta executable not found at {} (platform: {}, expected name: {})",
            final_path.display(),
            terracotta_platform_key(),
            terracotta_binary_name()
        );
    }

    let mut runtime = TERRACOTTA_RUNTIME.lock().await;
    if let Some(process) = runtime.process.as_mut() {
        if process
            .child
            .try_wait()
            .wrap_err("failed to inspect existing terracotta process")?
            .is_none()
        {
            bail!("terracotta is already running");
        }
        warn!("discarding exited terracotta process before restart");
        runtime.process.take();
    }

    let port_file = terracotta_port_file();
    let _ = std::fs::remove_file(&port_file);

    let is_macos = cfg!(target_os = "macos");
    let mut command = Command::new(&final_path);
    if is_macos {
        command.arg("--daemon");
    } else {
        command.arg("--hmcl").arg(&port_file);
    }
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .wrap_err_with(|| {
            let mode = if is_macos { "daemon" } else { "--hmcl" };
            format!(
                "failed to start terracotta {mode} at {}",
                final_path.display()
            )
        })?;

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let pid = child.id().unwrap_or(0);
    let output = Arc::new(Mutex::new(VecDeque::new()));

    if let Some(stdout) = stdout {
        capture_terracotta_output(stdout, "stdout", pid, output.clone());
    }
    if let Some(stderr) = stderr {
        capture_terracotta_output(stderr, "stderr", pid, output.clone());
    }

    let pid = child.id().unwrap_or(0);
    info!("started terracotta (pid {pid})");

    runtime.process = Some(TerracottaProcess {
        child,
        output: output.clone(),
    });
    drop(runtime);

    {
        let mut state = TERRACOTTA_STATE.lock().await;
        state.status = TerracottaStatus::Starting;
        state.http_port = None;
        state.room_code = None;
        state.server_port = None;
        state.players.clear();
        state.error_type = None;
        state.error_message = None;
        state.profile_index = None;
    }

    let mut attempts = 0;
    const MAX_ATTEMPTS: u32 = 30;
    let mut hmcl_helper: Option<Child> = None;
    let port = loop {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        attempts += 1;
        if attempts > MAX_ATTEMPTS {
            let output = {
                let mut runtime = TERRACOTTA_RUNTIME.lock().await;
                runtime.process.take().map(|mut p| {
                    let _ = p.child.start_kill();
                    p.output.clone()
                })
            };
            let details = match output {
                Some(out) => {
                    tokio::time::sleep(std::time::Duration::from_millis(25))
                        .await;
                    out.lock()
                        .await
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join("\n")
                }
                None => String::new(),
            };
            let msg = if details.is_empty() {
                "timed out waiting for terracotta to start".to_string()
            } else {
                format!("timed out waiting for terracotta to start:\n{details}")
            };
            let mut state = TERRACOTTA_STATE.lock().await;
            state.status = TerracottaStatus::Error;
            state.error_type = Some(TerracottaErrorType::Terracotta);
            state.error_message = Some(msg.clone());
            bail!(msg);
        }

        match std::fs::read_to_string(&port_file) {
            Ok(contents) => {
                match serde_json::from_str::<TerracottaPortInfo>(&contents) {
                    Ok(info) => {
                        let _ = std::fs::remove_file(&port_file);
                        break info.port;
                    }
                    Err(e) => {
                        warn!("failed to parse terracotta port file: {}", e);
                    }
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                if is_macos {
                    let helper_exited = match hmcl_helper.as_mut() {
                        Some(helper) => helper
                            .try_wait()
                            .wrap_err(
                                "failed to inspect terracotta helper process",
                            )?
                            .is_some(),
                        None => true,
                    };
                    if helper_exited {
                        hmcl_helper = Some(
                            Command::new(&final_path)
                                .arg("--hmcl")
                                .arg(&port_file)
                                .stdin(Stdio::null())
                                .stdout(Stdio::null())
                                .stderr(Stdio::null())
                                .kill_on_drop(true)
                                .spawn()
                                .wrap_err_with(|| {
                                    format!(
                                        "failed to start terracotta --hmcl helper at {}",
                                        final_path.display()
                                    )
                                })?,
                        );
                    }
                }

                if !is_macos {
                    if let Some((status, output)) =
                        take_terminated_terracotta_process().await?
                    {
                        let message =
                            format_terracotta_exit(status, output).await;
                        if let Ok(contents) =
                            std::fs::read_to_string(&port_file)
                        {
                            if let Ok(info) =
                                serde_json::from_str::<TerracottaPortInfo>(
                                    &contents,
                                )
                            {
                                let _ = std::fs::remove_file(&port_file);
                                break info.port;
                            }
                        }
                        let msg = format!(
                            "terracotta exited before writing port file: {message}"
                        );
                        let mut state = TERRACOTTA_STATE.lock().await;
                        state.status = TerracottaStatus::Error;
                        state.error_type =
                            Some(TerracottaErrorType::Terracotta);
                        state.error_message = Some(msg.clone());
                        bail!(msg);
                    }
                }
            }
            Err(e) => {
                warn!("failed to read terracotta port file: {}", e);
            }
        }
    };

    info!("terracotta started on port {}", port);
    spawn_terracotta_state_poller(port).await;
    let mut state = TERRACOTTA_STATE.lock().await;
    state.http_port = Some(port);
    state.status = TerracottaStatus::Idle;
    Ok(())
}

pub async fn stop_terracotta() -> eyre::Result<()> {
    let _operation = TERRACOTTA_OPERATION.lock().await;
    let mut runtime = TERRACOTTA_RUNTIME.lock().await;
    if let Some(task) = runtime.state_poller.take() {
        task.abort();
    }
    if let Some(mut process) = runtime.process.take() {
        process.child.start_kill().ok();
        info!("stopped terracotta");
    }
    runtime.lan_announcer.sync(None);
    drop(runtime);

    let port = {
        let state = TERRACOTTA_STATE.lock().await;
        state.http_port.or_else(|| {
            std::fs::read_to_string(terracotta_port_file())
                .ok()
                .and_then(|contents| {
                    serde_json::from_str::<TerracottaPortInfo>(&contents).ok()
                })
                .map(|info| info.port)
        })
    };
    if let Some(port) = port {
        let _ = tokio::time::timeout(
            std::time::Duration::from_secs(3),
            terracotta_client()
                .get(format!("http://127.0.0.1:{port}/panic?peaceful=true"))
                .send(),
        )
        .await;
    }

    let mut state = TERRACOTTA_STATE.lock().await;
    *state = TerracottaState::default();
    state.binary_installed = is_binary_installed();
    Ok(())
}

fn build_room_url(
    port: u16,
    path: &str,
    room: &str,
    player: &str,
    nodes: &[String],
) -> String {
    let mut url = format!(
        "http://127.0.0.1:{port}{path}?room={}&player={}",
        urlencoding::encode(room),
        urlencoding::encode(player),
    );
    for node in nodes {
        url.push_str("&public_nodes=");
        url.push_str(&urlencoding::encode(node));
    }
    url
}

pub async fn start_hosting(
    room_code: Option<String>,
    player_name: String,
) -> eyre::Result<()> {
    let _operation = TERRACOTTA_OPERATION.lock().await;
    let player_name = player_name.trim();
    if player_name.is_empty() {
        bail!("player name cannot be empty");
    }
    let state = TERRACOTTA_STATE.lock().await;
    let port = state
        .http_port
        .ok_or_else(|| eyre::eyre!("terracotta is not running"))?;
    drop(state);

    let room_param = room_code.as_deref().unwrap_or("");
    let nodes: Vec<String> = Vec::new();

    let client = terracotta_client();
    let url = build_room_url(
        port,
        "/state/scanning",
        room_param,
        player_name,
        &nodes,
    );
    let resp = client
        .get(&url)
        .send()
        .await
        .wrap_err("failed to send hosting request to terracotta")?;
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        bail!(
            "terracotta hosting failed with status {}: {body}",
            status.as_u16()
        );
    }
    Ok(())
}

pub async fn start_joining(
    room_code: String,
    player_name: String,
) -> eyre::Result<()> {
    let _operation = TERRACOTTA_OPERATION.lock().await;
    let player_name = player_name.trim();
    if player_name.is_empty() {
        bail!("player name cannot be empty");
    }
    let room_code = parse_room_code(&room_code)?;
    let state = TERRACOTTA_STATE.lock().await;
    let port = state
        .http_port
        .ok_or_else(|| eyre::eyre!("terracotta is not running"))?;
    drop(state);

    let nodes: Vec<String> = Vec::new();

    let client = terracotta_client();
    let url = build_room_url(
        port,
        "/state/guesting",
        &room_code,
        player_name,
        &nodes,
    );
    let resp = client
        .get(&url)
        .send()
        .await
        .wrap_err("failed to send joining request to terracotta")?;
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        bail!(
            "terracotta joining failed with status {}: {body}",
            status.as_u16()
        );
    }
    Ok(())
}

pub async fn reset_state() -> eyre::Result<()> {
    let _operation = TERRACOTTA_OPERATION.lock().await;
    let state = TERRACOTTA_STATE.lock().await;
    let port = state.http_port;
    drop(state);

    if let Some(port) = port {
        let client = terracotta_client();
        let url = format!("http://127.0.0.1:{port}/state/ide");
        let response = client.get(&url).send().await.wrap_err_with(|| {
            format!("failed to connect to terracotta at {url}")
        })?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            bail!(
                "terracotta reset failed with status {}: {body}",
                status.as_u16()
            );
        }
    }

    let mut state = TERRACOTTA_STATE.lock().await;
    state.status = TerracottaStatus::Idle;
    state.room_code = None;
    state.server_port = None;
    state.players.clear();
    state.error_type = None;
    state.error_message = None;
    state.profile_index = None;
    drop(state);
    sync_minecraft_lan_announcer(None).await;
    Ok(())
}

pub fn parse_room_code(code: &str) -> eyre::Result<String> {
    if code.starts_with("U/") || code.starts_with("u/") {
        let inner = &code[2..];
        if inner.len() == 19 && inner.chars().filter(|&c| c == '-').count() == 3
        {
            let segments: Vec<&str> = inner.split('-').collect();
            if segments.len() == 4
                && segments.iter().all(|s| s.len() == 4)
                && segments
                    .iter()
                    .all(|s| s.chars().all(|c| c.is_ascii_alphanumeric()))
            {
                return Ok(format!("U/{inner}"));
            }
        }
    }
    bail!("invalid room code format: {code}")
}

pub async fn get_logs() -> eyre::Result<String> {
    let state = TERRACOTTA_STATE.lock().await;
    let port = state
        .http_port
        .ok_or_else(|| eyre::eyre!("terracotta is not running"))?;
    drop(state);

    let client = terracotta_client();
    let url = format!("http://127.0.0.1:{port}/log?fetch=");
    let resp = client
        .get(&url)
        .send()
        .await
        .wrap_err("failed to fetch terracotta logs")?;
    let body = resp
        .text()
        .await
        .wrap_err("failed to read terracotta logs")?;
    Ok(body)
}
