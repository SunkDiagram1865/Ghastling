import { invoke } from '@tauri-apps/api/core'
import { ref, onUnmounted } from 'vue'

export interface SystemInfo {
	cpu_usage: number
	cpu_cores: number
	memory_total: number
	memory_used: number
	disk_total: number
	disk_used: number
	disk_name: string
	gpu_usage: number
}

export function useSystemInfo(intervalMs = 2000) {
	const info = ref<SystemInfo | null>(null)
	const error = ref<string | null>(null)
	let timer: ReturnType<typeof setInterval> | null = null

	async function fetch() {
		try {
			const data = await invoke<SystemInfo>('get_system_info')
			info.value = data
			error.value = null
		} catch (e) {
			error.value = e instanceof Error ? e.message : String(e)
		}
	}

	async function start() {
		await fetch()
		timer = setInterval(fetch, intervalMs)
	}

	function stop() {
		if (timer) {
			clearInterval(timer)
			timer = null
		}
	}

	onUnmounted(() => {
		stop()
	})

	return { info, error, start, stop, fetch }
}
