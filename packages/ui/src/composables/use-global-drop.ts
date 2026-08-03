import { onMounted, onUnmounted, ref, watch, type Ref } from 'vue'

import type { NativeFileDropEvent, FileDropProvider } from '#ui/providers/file-drop'
import { injectFileDrop } from '#ui/providers/file-drop'
import { injectLoadingState } from '#ui/providers/loading-state'
import { injectNotificationManager } from '#ui/providers/web-notifications'
import { useDebugLogger } from '#ui/composables/debug-logger'

/**
 * Classification result returned by the Rust backend.
 */
export interface ClassificationResult {
	item_type:
		| 'launcher'
		| 'hmcl_launcher'
		| 'mod'
		| 'modpack'
		| 'litematic'
		| 'resource_pack'
		| 'shader_pack'
		| 'world_save'
		| 'shortcut_resolved'
		| 'unknown'
	file_path?: string
	launcher_type?: string
	base_path?: string
	launcher_dir?: string
	data_dir?: string
	original?: string
	resolved_to?: ClassificationResult
	reason?: string
}

export interface UseGlobalDropOptions {
	/**
	 * Function that classifies a dropped file by its filesystem path.
	 * Typically wraps `classifyDroppedItem` from `@/helpers/drop`.
	 */
	classifyFile: (path: string) => Promise<ClassificationResult>

	/** Called when classification begins (before the loading bar). Receives the file name. */
	onClassifyStart?: (fileName: string) => void

	/**
	 * Called when a droppable item is successfully classified.
	 * @param type  - 'launcher' | 'content'
	 * @param data  - The (resolved) ClassificationResult
	 */
	onImportStart?: (type: string, data: ClassificationResult) => void

	/** Called after processing ends (success or error). */
	onImportEnd?: () => void

	/** Called when an error occurs (unknown type, too many files, etc.). */
	onError?: (message: string) => void
}

export function useGlobalDrop(
	options: UseGlobalDropOptions,
	fileDropOverride?: FileDropProvider | null,
	enabled: Ref<boolean> | boolean = true,
) {
	const fileDrop = fileDropOverride ?? injectFileDrop(null)
	const loadingState = injectLoadingState(null)
	const notificationManager = injectNotificationManager(null)
	const debug = useDebugLogger('useGlobalDrop')

	const isDragging = ref(false)
	const isProcessing = ref(false)
	const droppedFileName = ref<string | null>(null)

	let nativeFileDropUnlisten: (() => void) | null = null
	let unmounted = false

	const isEnabled = typeof enabled === 'boolean' ? ref(enabled) : enabled

	function cleanup() {
		isDragging.value = false
		isProcessing.value = false
		if (nativeFileDropUnlisten) {
			nativeFileDropUnlisten()
			nativeFileDropUnlisten = null
			debug('cleanup: native file drop listener removed')
		}
	}

	function setup() {
		if (!fileDrop) {
			debug('setup: fileDrop provider not available — native drops disabled')
			return
		}

		if (!isEnabled.value) {
			debug('setup: drag-and-drop disabled by user setting')
			return
		}

		cleanup()

		fileDrop.listenNativeFileDrop(handleNativeDrop)
			.then((unlisten) => {
				if (unmounted || !isEnabled.value) {
					debug('setup: component unmounted or disabled before listener was ready, cleaning up')
					unlisten()
					return
				}

				nativeFileDropUnlisten = unlisten
				debug('setup: native file drop listener registered successfully')
			})
			.catch((err) => {
				debug('setup: failed to register native file drop listener', err)
			})
	}

	/**
	 * Walk the `shortcut_resolved` chain up to 3 levels.
	 * If still a shortcut after depth 3, return the terminal node
	 * (the caller will treat it as unknown).
	 */
	function resolveClassification(result: ClassificationResult, depth = 0): ClassificationResult {
		if (result.item_type === 'shortcut_resolved' && result.resolved_to && depth < 3) {
			debug('resolveClassification: shortcut chain', { depth, from: result.file_path, to: result.resolved_to.file_path })
			return resolveClassification(result.resolved_to, depth + 1)
		}
		if (depth > 0) debug('resolveClassification: resolved at depth', { depth, item_type: result.item_type, file_path: result.file_path })
		return result
	}

	async function handleNativeDrop(event: NativeFileDropEvent) {
		if (!isEnabled.value) {
			debug('handleNativeDrop: drag-and-drop disabled, ignoring event')
			return
		}

		if (event.type === 'enter' || event.type === 'over') {
			isDragging.value = true
			return
		}

		if (event.type === 'leave') {
			isDragging.value = false
			return
		}

		// ── type === 'drop' ──
		isDragging.value = false

		const { paths } = event
		if (!paths?.length) {
			debug('handleNativeDrop: drop event with empty paths — skipping')
			return
		}

		// Only one file at a time
		if (paths.length > 1) {
			options.onError?.('multiple-files')
			return
		}

		isProcessing.value = true
		droppedFileName.value = paths[0].split(/[/\\]/).pop() ?? 'file'
		options.onClassifyStart?.(droppedFileName.value)

		const loadToken = loadingState?.begin()

		try {
			const raw = await options.classifyFile(paths[0])
			debug('classifyFile raw result', { item_type: raw.item_type, file_path: raw.file_path, launcher_type: raw.launcher_type, reason: raw.reason })

			const resolved = resolveClassification(raw)
			debug('resolveClassification final', { item_type: resolved.item_type, file_path: resolved.file_path, launcher_type: resolved.launcher_type })

			if (resolved.item_type === 'unknown') {
				debug('routing: unknown type — passing to onImportStart so the consumer can decide', { reason: resolved.reason })
				options.onImportStart?.('unknown', resolved)
				return
			}

			// If still a shortcut after max depth — treat as unknown
			if (resolved.item_type === 'shortcut_resolved') {
				debug('routing: shortcut exceeded max depth', { file_path: resolved.file_path })
				options.onError?.('shortcut-exceeded')
				return
			}

			if (resolved.item_type === 'launcher' || resolved.item_type === 'hmcl_launcher') {
				debug('routing: launcher import', { launcher_type: resolved.launcher_type, base_path: resolved.base_path, data_dir: resolved.data_dir })
				options.onImportStart?.('launcher', resolved)
				return
			}

			// Content types: mod, litematic, resource_pack, shader_pack, world_save
			debug('routing: content import', { item_type: resolved.item_type })
			options.onImportStart?.('content', resolved)
		} catch (error) {
			const message = error instanceof Error ? error.message : String(error)
			options.onError?.(message)
		} finally {
			isProcessing.value = false
			if (loadToken) loadingState?.end(loadToken)
			options.onImportEnd?.()
		}
	}

	onMounted(() => {
		void setup()
	})

	onUnmounted(() => {
		unmounted = true
		cleanup()
	})

	watch(isEnabled, (newVal, oldVal) => {
		if (newVal === oldVal) return
		debug('isEnabled changed', { newVal })
		if (newVal) {
			setup()
		} else {
			cleanup()
		}
	})

	return {
		isDragging,
		isProcessing,
		droppedFileName,
	}
}