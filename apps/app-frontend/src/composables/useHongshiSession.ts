import { injectNotificationManager } from '@modrinth/ui'
import { computed, onMounted, onUnmounted, ref } from 'vue'

import {
	type DetectedLanPort,
	type HongshiNode,
	hongshi,
	type HongshiState,
} from '@/helpers/hongshi'

const DEFAULT_POLL_INTERVAL = 2000
const ACTIVE_POLL_INTERVAL = 500

export function useHongshiSession() {
	const { handleError } = injectNotificationManager()
	const state = ref<HongshiState | null>(null)
	const nodes = ref<HongshiNode[]>([])
	const detectedPorts = ref<DetectedLanPort[]>([])
	const isActionPending = ref(false)
	const isNodesLoading = ref(false)

	let mounted = false
	let pollTimer: ReturnType<typeof setTimeout> | undefined
	let pollPromise: Promise<void> | undefined

	const isActive = computed(() => {
		const status = state.value?.status
		return (
			status === 'open' ||
			status === 'starting' ||
			status === 'selecting_node' ||
			status === 'downloading' ||
			status === 'waiting_for_port'
		)
	})

	function pollInterval() {
		return isActive.value ? ACTIVE_POLL_INTERVAL : DEFAULT_POLL_INTERVAL
	}

	function schedulePoll() {
		if (!mounted) return
		clearTimeout(pollTimer)
		pollTimer = setTimeout(() => void pollState(), pollInterval())
	}

	async function pollState() {
		if (!mounted) return
		if (pollPromise) return pollPromise
		pollPromise = Promise.all([hongshi.getState(), hongshi.getDetectedPorts()])
			.then(([nextState, ports]) => {
				if (!mounted) return
				state.value = nextState
				detectedPorts.value = ports
			})
			.catch((error: unknown) => {
				if (mounted) console.error(error)
			})
			.finally(() => {
				pollPromise = undefined
				schedulePoll()
			})
		return pollPromise
	}

	async function runAction(action: () => Promise<void>): Promise<boolean> {
		if (isActionPending.value) return false
		isActionPending.value = true
		try {
			await action()
			return true
		} catch (error: unknown) {
			handleError(error)
			return false
		} finally {
			isActionPending.value = false
			await pollState()
		}
	}

	async function refreshNodes(forceRefresh = false) {
		if (isNodesLoading.value) return
		isNodesLoading.value = true
		try {
			nodes.value = await hongshi.getNodes(forceRefresh)
		} catch (error: unknown) {
			handleError(error)
		} finally {
			isNodesLoading.value = false
		}
	}

	const host = (localPort: number, nodeName: string | null, instanceId: string | null) =>
		runAction(() => hongshi.host(localPort, nodeName, instanceId))
	const stop = () => runAction(hongshi.stop)
	const reset = () => runAction(hongshi.reset)
	const download = () => runAction(hongshi.download)
	const openLogs = () => runAction(hongshi.openLogs)

	onMounted(() => {
		mounted = true
		void pollState()
		void refreshNodes()
	})

	onUnmounted(() => {
		mounted = false
		clearTimeout(pollTimer)
	})

	return {
		detectedPorts,
		download,
		host,
		isActionPending,
		isActive,
		isNodesLoading,
		nodes,
		openLogs,
		refreshNodes,
		reset,
		state,
		stop,
	}
}
