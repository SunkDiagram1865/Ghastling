use serde::Serialize;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, Ordering};
use sysinfo::{Disks, System};

#[derive(Debug, Serialize, Clone)]
pub struct SystemInfo {
	pub cpu_usage: f32,
	pub cpu_cores: usize,
	pub memory_total: u64,
	pub memory_used: u64,
	pub disk_total: u64,
	pub disk_used: u64,
	pub disk_name: String,
	pub gpu_usage: f32,
}

// 全局缓存 GPU 占用率（以 AtomicU32 存储，乘以 100 后取整），由后台线程定期更新
static GPU_USAGE: OnceLock<AtomicU32> = OnceLock::new();

#[cfg(target_os = "windows")]
fn query_gpu_usage() -> f32 {
	use std::os::windows::process::CommandExt;

	// CREATE_NO_WINDOW = 0x08000000，阻止 GUI 进程中启动子进程时弹出控制台窗口
	const CREATE_NO_WINDOW: u32 = 0x0800_0000;

	// 通过 PowerShell 读取 GPU 3D 引擎占用率性能计数器，求和所有实例
	let output = std::process::Command::new("powershell")
		.creation_flags(CREATE_NO_WINDOW)
		.args([
			"-NoProfile",
			"-NonInteractive",
			"-Command",
			"(Get-Counter '\\GPU Engine(*engtype_3D)\\Utilization Percentage' -ErrorAction SilentlyContinue).CounterSamples.CookedValue | Measure-Object -Sum | ForEach-Object { $_.Sum }",
		])
		.output();
	match output {
		Ok(out) => {
			let s = String::from_utf8_lossy(&out.stdout);
			s.trim().parse::<f32>().unwrap_or(0.0)
		}
		Err(_) => 0.0,
	}
}

#[cfg(not(target_os = "windows"))]
fn query_gpu_usage() -> f32 {
	0.0
}

// 在应用启动时调用，启动后台线程定期刷新 GPU 占用
// 使用独立线程 + 自建 runtime，避免依赖 Tauri 运行时上下文
pub fn init_gpu_monitor() {
	GPU_USAGE.get_or_init(|| {
		let atomic = AtomicU32::new(0);
		std::thread::spawn(|| {
			let rt = match tokio::runtime::Builder::new_current_thread()
				.enable_all()
				.build()
			{
				Ok(rt) => rt,
				Err(_) => return,
			};
			rt.block_on(async move {
				loop {
					let usage = tokio::task::spawn_blocking(query_gpu_usage)
						.await
						.unwrap_or(0.0);
					if let Some(atomic) = GPU_USAGE.get() {
						// 将 f32 乘以 100 后取整存入 AtomicU32
						atomic.store((usage * 100.0) as u32, Ordering::Relaxed);
					}
					tokio::time::sleep(std::time::Duration::from_secs(3)).await;
				}
			});
		});
		atomic
	});
}

#[tauri::command]
pub async fn get_system_info() -> SystemInfo {
	let mut sys = System::new_all();
	sys.refresh_all();

	let cpu_usage = sys.global_cpu_usage();
	let cpu_cores = sys.cpus().len();

	let memory_total = sys.total_memory();
	let memory_used = sys.used_memory();

	let mut disk_total: u64 = 0;
	let mut disk_used: u64 = 0;
	let mut disk_name = String::new();

	let disks = Disks::new_with_refreshed_list();
	if let Some(disk) = disks.list().first() {
		disk_total = disk.total_space();
		disk_used = disk_total.saturating_sub(disk.available_space());
		disk_name = disk.name().to_string_lossy().to_string();
	}

	// 读取后台缓存的 GPU 占用（非阻塞）
	let gpu_usage = match GPU_USAGE.get() {
		Some(atomic) => atomic.load(Ordering::Relaxed) as f32 / 100.0,
		None => 0.0,
	};

	SystemInfo {
		cpu_usage,
		cpu_cores,
		memory_total,
		memory_used,
		disk_total,
		disk_used,
		disk_name,
		gpu_usage,
	}
}
