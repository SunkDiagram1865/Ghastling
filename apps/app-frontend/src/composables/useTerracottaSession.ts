import { injectNotificationManager } from '@modrinth/ui'
import { onMounted, onUnmounted, ref } from 'vue'

import { terracotta, type TerracottaState } from '@/helpers/terracotta'

const DEFAULT_POLL_INTERVAL = 1000
const DOWNLOAD_POLL_INTERVAL = 500

export function useTerracottaSession() {
	const { handleError } = injectNotificationManager()
	const state = ref<TerracottaState | null>(null)
	const playerName = ref('')
	const roomCodeInput = ref('')
	const platformKey = ref('unknown')
	const isActionPending = ref(false)

	let mounted = false
	let pollTimer: ReturnType<typeof setTimeout> | undefined
	let pollInterval = DEFAULT_POLL_INTERVAL
	let pollPromise: Promise<void> | undefined

	function schedulePoll(delay = pollInterval) {
		if (!mounted) return
		clearTimeout(pollTimer)
		pollTimer = setTimeout(() => void pollState(), delay)
	}

	async function pollState() {
		if (!mounted) return
		if (pollPromise) return
		pollPromise = terracotta
			.getState()
			.then((nextState) => {
				if (mounted) state.value = nextState
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

	async function runAction(action: () => Promise<void>, interval = DEFAULT_POLL_INTERVAL) {
		if (isActionPending.value) return
		isActionPending.value = true
		pollInterval = interval
		try {
			await action()
		} catch (error: unknown) {
			handleError(error)
		} finally {
			isActionPending.value = false
			pollInterval = DEFAULT_POLL_INTERVAL
			await pollState()
		}
	}

	const start = () => runAction(terracotta.start)
	const host = () => runAction(() => terracotta.host(playerName.value))
	const join = () => runAction(() => terracotta.join(playerName.value, roomCodeInput.value))
	const reset = () => runAction(terracotta.reset)
	const stop = () => runAction(terracotta.stop)
	const download = () => runAction(terracotta.download, DOWNLOAD_POLL_INTERVAL)

	onMounted(() => {
		mounted = true
		void pollState()
		void terracotta
			.getPlatformKey()
			.then((value) => {
				if (mounted) platformKey.value = value
			})
			.catch(() => undefined)
		void terracotta
			.getPlayerName()
			.then((value) => {
				if (mounted && value) playerName.value = value
			})
			.catch(() => undefined)
	})

	onUnmounted(() => {
		mounted = false
		clearTimeout(pollTimer)
	})

	return {
		download,
		host,
		isActionPending,
		join,
		platformKey,
		playerName,
		reset,
		roomCodeInput,
		start,
		state,
		stop,
	}
}
