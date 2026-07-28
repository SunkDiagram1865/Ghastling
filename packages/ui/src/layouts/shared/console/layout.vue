<template>
	<div
		class="flex min-h-0 flex-1 flex-col gap-4"
		:class="
			isFullscreen ? `fixed inset-0 z-[15] bg-surface-1 p-6 py-8 ${isApp ? 'pt-12' : ''}` : ''
		"
	>
		<div
			v-if="ctx.localCrashAnalysis?.value?.findings.length && !isFullscreen"
			class="flex flex-col gap-2"
		>
			<CollapsibleAdmonition type="critical" :header="localCrashHeader" :items="localCrashItems" />
			<div class="flex justify-end">
				<ButtonStyled type="outlined">
					<button :disabled="exportingCrashContext" @click="handleExportCrashContext">
						<DownloadIcon />
						{{ formatMessage(consoleMessages.exportCrashContext) }}
					</button>
				</ButtonStyled>
			</div>
		</div>
		<CollapsibleAdmonition
			v-if="ctx.crashAnalysis?.value && !isFullscreen"
			type="critical"
			:header="crashHeader"
			:items="crashItems"
			dismissible
			@dismiss="ctx.onDismissCrash?.()"
		/>

		<div class="flex items-center gap-2">
			<StyledInput
				v-model="searchQuery"
				:icon="SearchIcon"
				:placeholder="formatMessage(consoleMessages.searchLogs)"
				wrapper-class="flex-1"
				input-class="!h-10"
				clearable
			/>
			<div v-if="ctx.logSources?.value && ctx.activeLogSourceIndex" class="w-[220px]">
				<Combobox
					:model-value="ctx.activeLogSourceIndex.value"
					:options="logSourceOptions"
					@update:model-value="(v) => (ctx.activeLogSourceIndex!.value = v)"
				/>
			</div>
		</div>

		<div class="flex items-center justify-between">
			<ConsoleFilterPills v-model="activeFilters" @toggle="handleFilterToggle" />
			<ConsoleActionButtons
				:show-clear="isLiveSource"
				:has-logs="hasLogs"
				:share-disabled="resolvedShareDisabled"
				:sharing="isSharing"
				:fullscreen="isFullscreen"
				:clear-disabled="resolvedClearDisabled"
				:clear-disabled-tooltip="resolvedClearDisabledTooltip"
				:show-delete="showDelete"
				:delete-disabled="resolvedDeleteDisabled"
				:delete-disabled-tooltip="resolvedDeleteDisabledTooltip"
				@clear="handleClear"
				@share="handleShare"
				@toggle-fullscreen="toggleFullscreen"
				@delete="handleDelete"
			/>
		</div>

		<BaseTerminal
			ref="terminalRef"
			class="min-h-0 flex-1"
			:show-input="resolvedShowInput"
			:disable-input="resolvedInputDisabled"
			:disable-input-tooltip="resolvedInputDisabledTooltip"
			:disabled-input-placeholder="resolvedInputDisabledPlaceholder"
			:fullscreen="isFullscreen"
			:empty-state-type="ctx.emptyStateType"
			:loading="resolvedLoading"
			@command="handleCommand"
			@ready="handleTerminalReady"
		/>
	</div>
	<ShareModal
		ref="shareModal"
		:header="formatMessage(consoleMessages.shareLogs)"
		link
		:social-buttons="false"
	/>
	<NewModal
		ref="deleteModal"
		:header="formatMessage(consoleMessages.deleteLogFile)"
		:fade="'danger'"
		max-width="500px"
	>
		<div class="flex flex-col gap-6">
			<Admonition type="critical" :header="formatMessage(consoleMessages.deleteIrreversible)">
				{{ formatMessage(consoleMessages.deleteConfirmation) }}
			</Admonition>
		</div>
		<template #actions>
			<div class="flex justify-end gap-2">
				<ButtonStyled type="outlined">
					<button @click="deleteModal?.hide()">
						<XIcon />
						{{ formatMessage(commonMessages.cancelButton) }}
					</button>
				</ButtonStyled>
				<ButtonStyled color="red">
					<button :disabled="isDeleting" @click="confirmDelete">
						<TrashIcon />
						{{ formatMessage(commonMessages.deleteLabel) }}
					</button>
				</ButtonStyled>
			</div>
		</template>
	</NewModal>
</template>

<script setup lang="ts">
import { DownloadIcon, SearchIcon, TrashIcon, XIcon } from '@modrinth/assets'
import type { Terminal } from '@xterm/xterm'
import { computed, isRef, nextTick, onBeforeUnmount, ref, watch } from 'vue'

import Admonition from '#ui/components/base/Admonition.vue'
import BaseTerminal from '#ui/components/base/BaseTerminal.vue'
import ButtonStyled from '#ui/components/base/ButtonStyled.vue'
import type { CollapsibleAdmonitionItem } from '#ui/components/base/CollapsibleAdmonition.vue'
import CollapsibleAdmonition from '#ui/components/base/CollapsibleAdmonition.vue'
import Combobox from '#ui/components/base/Combobox.vue'
import StyledInput from '#ui/components/base/StyledInput.vue'
import { useVIntl } from '#ui/composables/i18n'
import NewModal from '#ui/components/modal/NewModal.vue'
import ShareModal from '#ui/components/modal/ShareModal.vue'
import { injectModrinthClient } from '#ui/providers'
import { injectModalBehavior } from '#ui/providers/modal-behavior'
import { injectPageContext } from '#ui/providers/page-context'
import { injectNotificationManager } from '#ui/providers/web-notifications.ts'
import { commonMessages } from '#ui/utils/common-messages'

import ConsoleActionButtons from './components/ConsoleActionButtons.vue'
import ConsoleFilterPills from './components/ConsoleFilterPills.vue'
import {
	clearSearchHighlights,
	colorize,
	getHighlightVersion,
	highlightAppendedRange,
	rewriteTerminal,
	useConsoleFilters,
} from './composables'
import { consoleMessages, localFindingMessages } from './messages'
import { injectConsoleManager } from './providers'
import type { LogLevel, LogLine } from './types'

const ctx = injectConsoleManager()
const client = injectModrinthClient()
const modalBehavior = injectModalBehavior()
const pageContext = injectPageContext(null)
const { addNotification } = injectNotificationManager()
const { formatMessage } = useVIntl()

const localFindingCopy = {
	jvm_arguments: {
		title: localFindingMessages.jvmArgumentsTitle,
		action: localFindingMessages.jvmArgumentsAction,
	},
	out_of_memory: {
		title: localFindingMessages.outOfMemoryTitle,
		action: localFindingMessages.outOfMemoryAction,
	},
	opengl_unsupported: {
		title: localFindingMessages.openglUnsupportedTitle,
		action: localFindingMessages.openglUnsupportedAction,
	},
	pixel_format: {
		title: localFindingMessages.pixelFormatTitle,
		action: localFindingMessages.pixelFormatAction,
	},
	openj9: {
		title: localFindingMessages.openj9Title,
		action: localFindingMessages.openj9Action,
	},
	java_too_new: {
		title: localFindingMessages.javaTooNewTitle,
		action: localFindingMessages.javaTooNewAction,
	},
	java_incompatible: {
		title: localFindingMessages.javaIncompatibleTitle,
		action: localFindingMessages.javaIncompatibleAction,
	},
	jdk_runtime: {
		title: localFindingMessages.jdkRuntimeTitle,
		action: localFindingMessages.jdkRuntimeAction,
	},
	java_32bit: {
		title: localFindingMessages.java32BitTitle,
		action: localFindingMessages.java32BitAction,
	},
	java_11_required: {
		title: localFindingMessages.java11RequiredTitle,
		action: localFindingMessages.java11RequiredAction,
	},
	forge_incomplete: {
		title: localFindingMessages.forgeIncompleteTitle,
		action: localFindingMessages.forgeIncompleteAction,
	},
	duplicate_mod: {
		title: localFindingMessages.duplicateModTitle,
		action: localFindingMessages.duplicateModAction,
	},
	incompatible_mods: {
		title: localFindingMessages.incompatibleModsTitle,
		action: localFindingMessages.incompatibleModsAction,
	},
	missing_dependency: {
		title: localFindingMessages.missingDependencyTitle,
		action: localFindingMessages.missingDependencyAction,
	},
	mod_id_limit: {
		title: localFindingMessages.modIdLimitTitle,
		action: localFindingMessages.modIdLimitAction,
	},
	forge_error: {
		title: localFindingMessages.forgeErrorTitle,
		action: localFindingMessages.forgeErrorAction,
	},
	mod_loader_error: {
		title: localFindingMessages.modLoaderErrorTitle,
		action: localFindingMessages.modLoaderErrorAction,
	},
	mod_loader_failure: {
		title: localFindingMessages.modLoaderFailureTitle,
		action: localFindingMessages.modLoaderFailureAction,
	},
	stack_analysis: {
		title: localFindingMessages.stackAnalysisTitle,
		action: localFindingMessages.stackAnalysisAction,
	},
	short_output: {
		title: localFindingMessages.shortOutputTitle,
		action: localFindingMessages.shortOutputAction,
	},
	extracted_mod: {
		title: localFindingMessages.extractedModTitle,
		action: localFindingMessages.extractedModAction,
	},
	mixin_bootstrap: {
		title: localFindingMessages.mixinBootstrapTitle,
		action: localFindingMessages.mixinBootstrapAction,
	},
	mixin_failure: {
		title: localFindingMessages.mixinFailureTitle,
		action: localFindingMessages.mixinFailureAction,
	},
	fabric_solution: {
		title: localFindingMessages.fabricSolutionTitle,
		action: localFindingMessages.fabricSolutionAction,
	},
	mod_config: {
		title: localFindingMessages.modConfigTitle,
		action: localFindingMessages.modConfigAction,
	},
	optifine_incompatible: {
		title: localFindingMessages.optifineIncompatibleTitle,
		action: localFindingMessages.optifineIncompatibleAction,
	},
	resource_pack: {
		title: localFindingMessages.resourcePackTitle,
		action: localFindingMessages.resourcePackAction,
	},
	large_resource_pack: {
		title: localFindingMessages.largeResourcePackTitle,
		action: localFindingMessages.largeResourcePackAction,
	},
	shaders_optifine: {
		title: localFindingMessages.shadersOptifineTitle,
		action: localFindingMessages.shadersOptifineAction,
	},
	multiple_forge_versions: {
		title: localFindingMessages.multipleForgeVersionsTitle,
		action: localFindingMessages.multipleForgeVersionsAction,
	},
	forge_java_incompatible: {
		title: localFindingMessages.forgeJavaIncompatibleTitle,
		action: localFindingMessages.forgeJavaIncompatibleAction,
	},
	content_verification: {
		title: localFindingMessages.contentVerificationTitle,
		action: localFindingMessages.contentVerificationAction,
	},
	optifine_world: {
		title: localFindingMessages.optifineWorldTitle,
		action: localFindingMessages.optifineWorldAction,
	},
	nightconfig_bug: {
		title: localFindingMessages.nightconfigBugTitle,
		action: localFindingMessages.nightconfigBugAction,
	},
	mod_filename: {
		title: localFindingMessages.modFilenameTitle,
		action: localFindingMessages.modFilenameAction,
	},
	definite_mod: {
		title: localFindingMessages.definiteModTitle,
		action: localFindingMessages.definiteModAction,
	},
	definite_mod_fabric: {
		title: localFindingMessages.definiteModFabricTitle,
		action: localFindingMessages.definiteModFabricAction,
	},
	intel_driver: {
		title: localFindingMessages.intelDriverTitle,
		action: localFindingMessages.intelDriverAction,
	},
	amd_driver: {
		title: localFindingMessages.amdDriverTitle,
		action: localFindingMessages.amdDriverAction,
	},
	nvidia_driver: {
		title: localFindingMessages.nvidiaDriverTitle,
		action: localFindingMessages.nvidiaDriverAction,
	},
	manual_debug_crash: {
		title: localFindingMessages.manualDebugCrashTitle,
		action: localFindingMessages.manualDebugCrashAction,
	},
	suspected_mod: {
		title: localFindingMessages.suspectedModTitle,
		action: localFindingMessages.suspectedModAction,
	},
	mod_initialization: {
		title: localFindingMessages.modInitializationTitle,
		action: localFindingMessages.modInitializationAction,
	},
	specific_block: {
		title: localFindingMessages.specificBlockTitle,
		action: localFindingMessages.specificBlockAction,
	},
	specific_entity: {
		title: localFindingMessages.specificEntityTitle,
		action: localFindingMessages.specificEntityAction,
	},
} as const

const localCrashHeader = computed(() => {
	const analysis = ctx.localCrashAnalysis?.value
	const findings = analysis?.findings.length ?? 0
	const sources = analysis?.sources.length ?? 0
	return formatMessage(consoleMessages.localCrashHeader, { findings, sources })
})

const localCrashItems = computed<CollapsibleAdmonitionItem[]>(() => {
	const analysis = ctx.localCrashAnalysis?.value
	if (!analysis) return []
	return analysis.findings.map((finding) => {
		const copy = localFindingCopy[finding.id as keyof typeof localFindingCopy]
		const title = copy
			? formatMessage(copy.title)
			: formatMessage(consoleMessages.fallbackFindingTitle, { finding: finding.id })
		const action = copy
			? formatMessage(copy.action)
			: formatMessage(consoleMessages.fallbackFindingAction)
		const evidence = finding.evidence.map((item) => `${item.filename}:${item.line} - ${item.text}`)
		const mods = analysis.mods.map((mod) => {
			const identity = mod.name || mod.id || mod.file_name
			const modId = mod.id && mod.id !== identity ? ` (${mod.id})` : ''
			return formatMessage(consoleMessages.matchedMod, {
				identity,
				modId,
				fileName: mod.file_name,
			})
		})
		return {
			title,
			descriptions: [action, ...mods, ...evidence],
		}
	})
})

const crashHeader = computed(() => {
	const problems = ctx.crashAnalysis?.value?.analysis.problems ?? []
	const count = problems.length
	return formatMessage(consoleMessages.problemsDetected, { count })
})

const crashItems = computed<CollapsibleAdmonitionItem[]>(() => {
	const problems = ctx.crashAnalysis?.value?.analysis.problems ?? []
	return problems.map((p) => ({
		title: p.message,
		descriptions: p.solutions.map((s) => s.message),
	}))
})

const terminalRef = ref<InstanceType<typeof BaseTerminal> | null>(null)
const shareModal = ref<InstanceType<typeof ShareModal> | null>(null)
const deleteModal = ref<InstanceType<typeof NewModal> | null>(null)
const isDeleting = ref(false)
const exportingCrashContext = ref(false)
const searchQuery = ref('')
const isFullscreen = ref(false)
const fullscreenBodyClass = 'modrinth-console-fullscreen-active'
const fullscreenIntercomPadding = 20
const fullscreenIntercomPaddingRequestId = Symbol('console-fullscreen')
const isApp =
	typeof window !== 'undefined' && !!(window as Record<string, unknown>).__TAURI_INTERNALS__
const isSharing = ref(false)
const { activeFilters, toggleFilter, buildFilterPredicate } = useConsoleFilters()
const hasLogs = computed(() => ctx.logLines.value.length > 0)
const isLiveSource = computed(() => {
	const sources = ctx.logSources?.value
	const index = ctx.activeLogSourceIndex?.value
	if (!sources || index === undefined) return true
	return sources[index]?.live ?? true
})
const logSourceOptions = computed(() =>
	(ctx.logSources?.value ?? []).map((s, i) => ({ value: i, label: s.name })),
)

async function handleExportCrashContext() {
	if (!ctx.onExportCrashContext || exportingCrashContext.value) return
	exportingCrashContext.value = true
	try {
		await ctx.onExportCrashContext()
	} finally {
		exportingCrashContext.value = false
	}
}

function buildCombinedPredicate(): ((line: LogLine) => boolean) | null {
	const levelPred = buildFilterPredicate()
	const query = searchQuery.value.trim().toLowerCase()
	if (!levelPred && !query) return null
	return (line: LogLine) => {
		if (levelPred && !levelPred(line)) return false
		if (query && !line.text.toLowerCase().includes(query)) return false
		return true
	}
}

onBeforeUnmount(() => {
	if (isFullscreen.value) {
		document.body.style.overflow = ''
		document.body.classList.remove(fullscreenBodyClass)
		pageContext?.intercomBubble?.requestHorizontalPadding?.(
			fullscreenIntercomPaddingRequestId,
			null,
		)
		modalBehavior?.onHide?.()
	}
})

let lastWrittenIndex = 0
let searchDebounce: ReturnType<typeof setTimeout> | null = null

const resolvedShowInput = computed(() => {
	const v = ctx.showCommandInput
	if (v === undefined) return false
	if (typeof v === 'boolean') return v
	return isRef(v) ? v.value : v
})

const resolvedDisableInput = computed(() => {
	const v = ctx.disableCommandInput
	if (!v) return false
	return isRef(v) ? v.value : v
})

function unwrapMaybeRef<T>(value: T | { value: T } | undefined): T | undefined {
	if (value === undefined) return undefined
	return isRef(value) ? value.value : value
}

// needs historical log start/end flags on ws to be properly useful
const resolvedLoading = computed(() => {
	const v = ctx.loading
	if (!v) return false
	return v.value
})

const resolvedInputDisabled = computed(() => resolvedDisableInput.value || resolvedLoading.value)

const resolvedInputDisabledTooltip = computed(() =>
	resolvedDisableInput.value ? unwrapMaybeRef(ctx.disableCommandInputTooltip) : undefined,
)

const resolvedInputDisabledPlaceholder = computed(() =>
	formatMessage(
		resolvedInputDisabledTooltip.value
			? consoleMessages.commandInputDisabled
			: consoleMessages.serverNotRunning,
	),
)

const resolvedShareDisabled = computed(() => {
	const v = ctx.shareDisabled
	if (!v) return false
	return isRef(v) ? v.value : v
})

const showDelete = computed(() => !isLiveSource.value && ctx.onDelete != null)

const resolvedDeleteDisabled = computed(() => {
	const v = ctx.deleteDisabled
	if (!v) return false
	return isRef(v) ? v.value : v
})

const resolvedDeleteDisabledTooltip = computed(() =>
	resolvedDeleteDisabled.value ? unwrapMaybeRef(ctx.deleteDisabledTooltip) : undefined,
)

const resolvedClearDisabled = computed(() => {
	const v = ctx.clearDisabled
	if (!v) return false
	return isRef(v) ? v.value : v
})

const resolvedClearDisabledTooltip = computed(() =>
	resolvedClearDisabled.value ? unwrapMaybeRef(ctx.clearDisabledTooltip) : undefined,
)

function handleTerminalReady(_terminal: Terminal) {
	rewriteFiltered()
}

function handleFilterToggle(value: LogLevel) {
	toggleFilter(value)
	rewriteFiltered()
}

function activeSearchQuery(): string {
	return searchQuery.value.trim().toLowerCase()
}

function rewriteFiltered() {
	const term = terminalRef.value?.terminal
	if (!term) return
	const lines = ctx.logLines.value
	if (resolvedLoading.value && lines.length === 0 && isLiveSource.value) {
		terminalRef.value?.clearEmptyState()
		lastWrittenIndex = 0
		return
	}
	if (lines.length === 0 && isLiveSource.value) {
		writeEmptyState()
		return
	}
	terminalRef.value?.clearEmptyState()
	const predicate = buildCombinedPredicate()
	rewriteTerminal(term, lines, predicate, activeSearchQuery())
	lastWrittenIndex = lines.length
}

function toggleFullscreen() {
	isFullscreen.value = !isFullscreen.value
	if (isFullscreen.value) {
		document.body.style.overflow = 'hidden'
		document.body.classList.add(fullscreenBodyClass)
		pageContext?.intercomBubble?.requestHorizontalPadding?.(
			fullscreenIntercomPaddingRequestId,
			fullscreenIntercomPadding,
		)
		modalBehavior?.onShow?.()
	} else {
		document.body.style.overflow = ''
		document.body.classList.remove(fullscreenBodyClass)
		pageContext?.intercomBubble?.requestHorizontalPadding?.(
			fullscreenIntercomPaddingRequestId,
			null,
		)
		modalBehavior?.onHide?.()
	}
	nextTick(() => {
		terminalRef.value?.fit()
	})
}

function writeEmptyState() {
	terminalRef.value?.writeEmptyState()
	lastWrittenIndex = 0
}

watch(ctx.logLines, (lines, oldLines) => {
	const term = terminalRef.value?.terminal
	if (!term) return

	if (lines.length === 0 && isLiveSource.value) {
		if (resolvedLoading.value) {
			terminalRef.value?.clearEmptyState()
			lastWrittenIndex = 0
			return
		}

		writeEmptyState()
		return
	}

	if (
		terminalRef.value?.showingEmptyState ||
		lines !== oldLines ||
		lines.length < lastWrittenIndex
	) {
		terminalRef.value?.clearEmptyState()
		rewriteFiltered()
		return
	}

	const predicate = buildCombinedPredicate()
	const newLines: string[] = []
	for (let i = lastWrittenIndex; i < lines.length; i++) {
		if (!predicate || predicate(lines[i])) {
			newLines.push(colorize(lines[i]))
		}
	}
	if (newLines.length > 0) {
		const buffer = term.buffer.active
		const onFreshLine = buffer.cursorX === 0
		const data = onFreshLine ? newLines.join('\r\n') : '\r\n' + newLines.join('\r\n')
		const fromRow = buffer.baseY + buffer.cursorY
		const version = getHighlightVersion(term)
		term.write(data, () => {
			highlightAppendedRange(term, fromRow, version)
		})
	}
	lastWrittenIndex = lines.length
})

watch(searchQuery, () => {
	if (searchDebounce) clearTimeout(searchDebounce)
	searchDebounce = setTimeout(() => {
		rewriteFiltered()
	}, 200)
})

watch(resolvedLoading, (loading) => {
	if (!loading) {
		rewriteFiltered()
	}
})

function handleCommand(cmd: string) {
	if (resolvedInputDisabled.value) return
	ctx.sendCommand?.(cmd)
}

function handleClear() {
	if (resolvedClearDisabled.value) return
	const term = terminalRef.value?.terminal
	if (term) clearSearchHighlights(term)
	terminalRef.value?.reset()
	lastWrittenIndex = 0
	ctx.onClear?.()
}

function handleDelete() {
	deleteModal.value?.show()
}

async function confirmDelete() {
	if (!ctx.onDelete) return
	isDeleting.value = true
	try {
		await ctx.onDelete()
		deleteModal.value?.hide()
	} catch (err) {
		console.error('Failed to delete log file:', err)
		addNotification({
			type: 'error',
			title: formatMessage(consoleMessages.deleteFailedTitle),
			text: typeof err === 'string' ? err : formatMessage(consoleMessages.unknownError),
		})
	} finally {
		isDeleting.value = false
	}
}

async function handleShare() {
	const predicate = buildCombinedPredicate()
	const lines = predicate ? ctx.logLines.value.filter(predicate) : ctx.logLines.value
	const content = lines.map((l) => l.text).join('\n')

	isSharing.value = true
	try {
		const data = await client.mclogs.logs_v1.create(content)
		if (data.url) {
			shareModal.value?.show(data.url)
		}
	} catch (err) {
		console.error('Failed to share logs:', err)
		addNotification({
			type: 'error',
			title: formatMessage(consoleMessages.shareFailedTitle),
			text: typeof err === 'string' ? err : formatMessage(consoleMessages.unknownError),
		})
	} finally {
		isSharing.value = false
	}
}
</script>

<style>
.modrinth-console-fullscreen-active .intercom-lightweight-app,
.modrinth-console-fullscreen-active .intercom-lightweight-app-launcher,
.modrinth-console-fullscreen-active .intercom-lightweight-app-messenger,
.modrinth-console-fullscreen-active .intercom-launcher-frame,
.modrinth-console-fullscreen-active .intercom-messenger-frame,
.modrinth-console-fullscreen-active #intercom-container,
.modrinth-console-fullscreen-active #intercom-frame,
.modrinth-console-fullscreen-active iframe[name='intercom-launcher-frame'],
.modrinth-console-fullscreen-active iframe[name='intercom-messenger-frame'] {
	z-index: 14 !important;
}

.modrinth-console-fullscreen-active .loading-indicator-container,
.modrinth-console-fullscreen-active .app-contents::before {
	z-index: 14 !important;
}

.modrinth-console-fullscreen-active .app-grid-navbar,
.modrinth-console-fullscreen-active .app-grid-statusbar {
	z-index: 0 !important;
}
</style>
