import { createContext } from '@modrinth/ui'
import { computed, type ComputedRef, type Ref, ref } from 'vue'

import { install_job_listener, loading_listener } from '@/helpers/events'
import {
	download_history_clear,
	download_job_cancel,
	download_job_delete,
	download_job_list,
	download_job_retry,
	type InstallJobSnapshot,
} from '@/helpers/install'
import type { LoadingBar } from '@/helpers/state'
import { progress_bars_list } from '@/helpers/state'

const activeStatuses = new Set(['queued', 'running', 'canceling', 'waiting_for_user'])
const downloadBarTypes = new Set([
	'java_download',
	'pack_file_download',
	'pack_download',
	'minecraft_download',
	'instance_update',
	'launcher_update',
])

export interface DownloadManager {
	jobs: Ref<InstallJobSnapshot[]>
	legacyDownloads: Ref<LoadingBar[]>
	activeJobs: ComputedRef<InstallJobSnapshot[]>
	historyJobs: ComputedRef<InstallJobSnapshot[]>
	activeCount: ComputedRef<number>
	queuedCount: ComputedRef<number>
	start: () => Promise<void>
	refresh: () => Promise<void>
	cancel: (jobId: string) => Promise<void>
	retry: (jobId: string) => Promise<void>
	remove: (jobId: string) => Promise<void>
	clearHistory: () => Promise<void>
	dispose: () => void
}

export function createDownloadManager(handleError: (error: unknown) => void): DownloadManager {
	const jobs = ref<InstallJobSnapshot[]>([])
	const legacyDownloads = ref<LoadingBar[]>([])
	let started = false
	let disposed = false
	let unlistenJobs: (() => void) | null = null
	let unlistenLoading: (() => void) | null = null

	function setJob(job: InstallJobSnapshot) {
		const current = jobs.value.find((candidate) => candidate.job_id === job.job_id)
		if (current && current.modified.localeCompare(job.modified) > 0) return
		jobs.value = [job, ...jobs.value.filter((candidate) => candidate.job_id !== job.job_id)].sort(
			(a, b) => b.created.localeCompare(a.created),
		)
	}

	async function refresh() {
		const page = await download_job_list({ limit: 250 }).catch((error) => {
			handleError(error)
			return null
		})
		if (page && !disposed) jobs.value = page.jobs
	}

	async function refreshLegacyDownloads() {
		const bars = await progress_bars_list().catch((error) => {
			handleError(error)
			return {}
		})
		legacyDownloads.value = Object.values(bars)
			.filter((bar) => downloadBarTypes.has(bar.bar_type?.type ?? ''))
			.map((bar) => ({
				...bar,
				title: bar.title ?? bar.bar_type?.pack_name ?? bar.bar_type?.instance_name ?? bar.message,
			}))
	}

	async function start() {
		if (started || disposed) return
		started = true
		await Promise.all([refresh(), refreshLegacyDownloads()])
		unlistenJobs = await install_job_listener((job: InstallJobSnapshot) => setJob(job))
		unlistenLoading = await loading_listener(() => void refreshLegacyDownloads())
	}

	async function cancel(jobId: string) {
		setJob(await download_job_cancel(jobId))
	}

	async function retry(jobId: string) {
		setJob(await download_job_retry(jobId))
	}

	async function remove(jobId: string) {
		await download_job_delete(jobId)
		jobs.value = jobs.value.filter((job) => job.job_id !== jobId)
	}

	async function clearHistory() {
		await download_history_clear()
		jobs.value = jobs.value.filter((job) => activeStatuses.has(job.status))
	}

	const activeJobs = computed(() => jobs.value.filter((job) => activeStatuses.has(job.status)))
	const historyJobs = computed(() => jobs.value.filter((job) => !activeStatuses.has(job.status)))

	return {
		jobs,
		legacyDownloads,
		activeJobs,
		historyJobs,
		activeCount: computed(() => activeJobs.value.length + legacyDownloads.value.length),
		queuedCount: computed(() => jobs.value.filter((job) => job.status === 'queued').length),
		start,
		refresh,
		cancel,
		retry,
		remove,
		clearHistory,
		dispose() {
			disposed = true
			unlistenJobs?.()
			unlistenLoading?.()
		},
	}
}

export const [injectDownloadManager, provideDownloadManager] = createContext<DownloadManager>(
	'root',
	'downloadManager',
)
