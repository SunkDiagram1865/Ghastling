<script setup>
import {
	DownloadIcon,
	FolderSearchIcon,
	ScanEyeIcon,
	SearchIcon,
	TrashIcon,
} from '@modrinth/assets'
import {
	ButtonStyled,
	Table,
	Toggle,
	defineMessages,
	injectNotificationManager,
	useVIntl,
} from '@modrinth/ui'
import { open } from '@tauri-apps/plugin-dialog'
import { platform } from '@tauri-apps/plugin-os'
import { computed, ref, watch } from 'vue'

import ConfirmModalWrapper from '@/components/ui/modal/ConfirmModalWrapper.vue'
import DownloadJavaModal from '@/components/ui/settings/DownloadJavaModal.vue'
import { trackEvent } from '@/helpers/analytics'
import { get, set } from '@/helpers/settings.ts'
import {
	delete_managed_java,
	find_filtered_jres,
	get_java_versions,
	get_jre,
	list_managed_java_versions,
	remove_java_version,
	set_java_version,
} from '@/helpers/jre'

const { addNotification, handleError } = injectNotificationManager()
const { formatMessage } = useVIntl()

const messages = defineMessages({
	version: {
		id: 'app.settings.java.table.version',
		defaultMessage: 'Java Version',
	},
	distribution: {
		id: 'app.settings.java.table.distribution',
		defaultMessage: 'Distribution',
	},
	path: {
		id: 'app.settings.java.table.path',
		defaultMessage: 'Path',
	},
	actions: {
		id: 'app.settings.java.table.actions',
		defaultMessage: '',
	},
	removeFromList: {
		id: 'app.settings.java.table.remove-from-list',
		defaultMessage: 'Remove from list',
	},
	findJava: {
		id: 'app.settings.java.find-java',
		defaultMessage: 'Find Java',
	},
	deepScan: {
		id: 'app.settings.java.deep-scan',
		defaultMessage: 'Deep Scan',
	},
	manualAdd: {
		id: 'app.settings.java.manual-add',
		defaultMessage: 'Manual Add',
	},
	downloadJava: {
		id: 'app.settings.java.download-java',
		defaultMessage: 'Download Java',
	},
	deleteJava: {
		id: 'app.settings.java.delete-java',
		defaultMessage: 'Delete Java',
	},
	deleteModeTitle: {
		id: 'app.settings.java.delete-mode-title',
		defaultMessage: 'Launcher-managed Java installations',
	},
	deleteModeDescription: {
		id: 'app.settings.java.delete-mode-description',
		defaultMessage: 'These are Java installations downloaded by the launcher. Select and confirm to delete their directories from disk.',
	},
	deleteConfirmTitle: {
		id: 'app.settings.java.delete-confirm-title',
		defaultMessage: 'Delete selected Java installations?',
	},
	deleteConfirmDescription: {
		id: 'app.settings.java.delete-confirm-description',
		defaultMessage: 'This will permanently delete the selected Java installation directories from disk. This cannot be undone. Are you sure?',
	},
	deleteConfirmProceed: {
		id: 'app.settings.java.delete-confirm-proceed',
		defaultMessage: 'Delete',
	},
	deleteConfirmCancel: {
		id: 'app.settings.java.delete-confirm-cancel',
		defaultMessage: 'Cancel',
	},
	deleteSuccess: {
		id: 'app.settings.java.delete-success',
		defaultMessage: 'Successfully deleted {count} Java installation(s)',
	},
	deletePartial: {
		id: 'app.settings.java.delete-partial',
		defaultMessage: 'Deleted {success}, failed {fail}',
	},
	deleteEmpty: {
		id: 'app.settings.java.delete-empty',
		defaultMessage: 'No launcher-managed Java installations found.',
	},
	deleteCancel: {
		id: 'app.settings.java.delete-cancel',
		defaultMessage: 'Cancel',
	},
	autoHighPerformanceMode: { id: 'app.settings.java.auto-high-performance-mode', defaultMessage: 'Automatically use high-performance GPU for Java' },
	autoHighPerformanceModeDescription: { id: 'app.settings.java.auto-high-performance-mode-description', defaultMessage: 'Sets the launcher and Java to use the high-performance GPU in Windows graphics settings when Minecraft launches. Windows only.' },
	scanning: {
		id: 'app.settings.java.scanning',
		defaultMessage: 'Scanning...',
	},
	noVersions: {
		id: 'app.settings.java.no-versions',
		defaultMessage: 'No Java versions configured. Use the buttons below to find or add Java installations.',
	},
	deepScanConfirm: {
		id: 'app.settings.java.deep-scan-confirm',
		defaultMessage: 'This will scan ALL directories on ALL drives. May take several minutes.',
	},
	scanAnyway: {
		id: 'app.settings.java.scan-anyway',
		defaultMessage: 'Scan Anyway',
	},
	cancel: {
		id: 'app.settings.java.cancel',
		defaultMessage: 'Cancel',
	},
})

const javaVersions = ref(await get_java_versions().catch(handleError))
const scanning = ref(false)
const scanMode = ref('')
const downloadJavaModal = ref(null)

const isWindows = (await platform()) === 'windows'
const settings = ref(await get().catch(handleError))
const autoHighPerformanceMode = ref(settings.value?.auto_set_java_high_performance_mode ?? false)

watch(autoHighPerformanceMode, async (val) => {
    settings.value = { ...settings.value, auto_set_java_high_performance_mode: val }
    await set(settings.value).catch(handleError)
})

const columns = [
	{ key: 'parsed_version', label: formatMessage(messages.version), width: '8rem' },
	{ key: 'distribution', label: formatMessage(messages.distribution), width: '10rem' },
	{ key: 'path', label: formatMessage(messages.path) },
	{ key: 'actions', label: formatMessage(messages.actions), align: 'right', width: '3rem' },
]

const tableData = ref([])
function refreshTable() {
	if (!javaVersions.value) {
		tableData.value = []
		return
	}
	tableData.value = javaVersions.value
		.map((val) => ({
			parsed_version: val.parsed_version,
			distribution: val.distribution || null,
			path: val.path || '',
			_java: val,
		}))
		.sort((a, b) => b.parsed_version - a.parsed_version)
}
refreshTable()

async function reloadJavaVersions() {
	javaVersions.value = await get_java_versions().catch(handleError)
	refreshTable()
}

async function runScan(exhaustive) {
	if (exhaustive) {
		showDeepScanConfirm.value = true
		return
	}
	scanning.value = true
	scanMode.value = 'quick'
	trackEvent('JavaQuickScan', { source: 'settings' })
	await find_filtered_jres(null, false, true, false).catch(handleError)
	await reloadJavaVersions()
	scanning.value = false
	scanMode.value = ''
}

const showDeepScanConfirm = ref(false)

async function confirmDeepScan() {
	showDeepScanConfirm.value = false
	scanning.value = true
	scanMode.value = 'deep'
	trackEvent('JavaDeepScan', { source: 'settings' })
	await find_filtered_jres(null, true, true, true).catch(handleError)
	await reloadJavaVersions()
	scanning.value = false
	scanMode.value = ''
}

async function handleManualAdd() {
	const result = await open({ multiple: false })
	if (!result) return

	const filePath = result.path ?? result
	let javaInfo = await get_jre(filePath).catch(handleError)
	if (!javaInfo) {
		javaInfo = {
			parsed_version: 0,
			path: filePath,
			version: '',
			architecture: 'x86',
			distribution: null,
		}
	}

	await set_java_version(javaInfo).catch(handleError)
	trackEvent('JavaManualSelect', { path: filePath })
	await reloadJavaVersions()
}

async function removeJavaEntry(javaVersion) {
	await remove_java_version(javaVersion.path).catch(handleError)
	javaVersions.value = javaVersions.value.filter((jv) => jv.path !== javaVersion.path)
	refreshTable()
}

async function onJavaDownloaded(_path, _parsedVersion) {
	await reloadJavaVersions()
}

// ── Delete Java mode ───────────────────────────────────────────────────────
const deleteMode = ref(false)
const managedJavaVersions = ref([])
const deleteJavaConfirmModal = ref(null)
const pendingDeleteDir = ref(null)
const loadingManaged = ref(false)

const managedColumns = [
	{ key: 'parsed_version', label: formatMessage(messages.version), width: '8rem' },
	{ key: 'distribution', label: formatMessage(messages.distribution), width: '10rem' },
	{ key: 'path', label: formatMessage(messages.path) },
	{ key: 'actions', label: formatMessage(messages.actions), align: 'right', width: '3rem' },
]

const managedTableData = computed(() => {
	return managedJavaVersions.value.map((item) => ({
		parsed_version: item.java_version?.parsed_version ?? 0,
		distribution: item.java_version?.distribution ?? null,
		path: item.java_version?.path ?? item.dir_name,
		_dir_name: item.dir_name,
	}))
})

async function enterDeleteMode() {
	deleteMode.value = true
	loadingManaged.value = true
	try {
		managedJavaVersions.value = await list_managed_java_versions()
	} catch (error) {
		handleError(error)
		managedJavaVersions.value = []
	} finally {
		loadingManaged.value = false
	}
}

function exitDeleteMode() {
	deleteMode.value = false
	managedJavaVersions.value = []
}

function handleDeleteSingle(dirName) {
	pendingDeleteDir.value = dirName
	deleteJavaConfirmModal.value?.show()
}

async function doDeleteConfirmed() {
	const dirName = pendingDeleteDir.value
	if (!dirName) return

	try {
		await delete_managed_java(dirName)
		addNotification({
			type: 'success',
			title: formatMessage(messages.deleteJava),
			text: formatMessage(messages.deleteSuccess, { count: 1 }),
		})
	} catch (error) {
		console.error(`Failed to delete ${dirName}:`, error)
		addNotification({
			type: 'error',
			title: formatMessage(messages.deleteJava),
			text: formatMessage(messages.deletePartial, { success: 0, fail: 1 }),
		})
	}

	pendingDeleteDir.value = null
	await enterDeleteMode()
	await reloadJavaVersions()
}

</script>
<template>
	<DownloadJavaModal ref="downloadJavaModal" @downloaded="onJavaDownloaded" />
	<ConfirmModalWrapper
		ref="deleteJavaConfirmModal"
		:title="formatMessage(messages.deleteConfirmTitle)"
		:description="formatMessage(messages.deleteConfirmDescription)"
		:has-to-type="false"
		:proceed-label="formatMessage(messages.deleteConfirmProceed)"
		:cancel-label="formatMessage(messages.deleteConfirmCancel)"
		:danger="true"
		:show-ad-on-close="false"
		@proceed="doDeleteConfirmed"
	/>
	<div class="flex flex-col gap-3">
		<!-- Normal mode: show all Java versions -->
		<template v-if="!deleteMode">
			<Table :columns="columns" :data="tableData" row-key="path">
				<template #cell-parsed_version="{ value }">
					<span class="font-semibold tabular-nums">Java {{ value }}</span>
				</template>
				<template #cell-distribution="{ value }">
					<span class="text-sm">{{ value || '—' }}</span>
				</template>
				<template #cell-path="{ value }">
					<span v-tooltip="value" class="block truncate font-mono text-xs max-w-72">{{ value }}</span>
				</template>
				<template #cell-actions="{ row }">
					<button
						class="p-1 text-secondary hover:text-red transition-colors"
						v-tooltip="formatMessage(messages.removeFromList)"
						@click="removeJavaEntry(row._java)"
					>
						<TrashIcon class="h-4 w-4" />
					</button>
				</template>
				<template #empty-state>
					<div class="py-8 text-center text-sm text-secondary">
						{{ formatMessage(messages.noVersions) }}
					</div>
				</template>
			</Table>

			<div class="flex flex-wrap gap-2 pt-3 border-t border-button-border">
				<ButtonStyled>
					<button class="!shadow-none" :disabled="scanning" @click="runScan(false)">
						<SearchIcon class="h-4 w-4" />
						{{ scanning && scanMode === 'quick' ? formatMessage(messages.scanning) : formatMessage(messages.findJava) }}
					</button>
				</ButtonStyled>
				<ButtonStyled>
					<button class="!shadow-none" :disabled="scanning" @click="runScan(true)">
						<ScanEyeIcon class="h-4 w-4" />
						{{ scanning && scanMode === 'deep' ? formatMessage(messages.scanning) : formatMessage(messages.deepScan) }}
					</button>
				</ButtonStyled>
				<ButtonStyled>
					<button class="!shadow-none" :disabled="scanning" @click="handleManualAdd">
						<FolderSearchIcon class="h-4 w-4" />
						{{ formatMessage(messages.manualAdd) }}
					</button>
				</ButtonStyled>
				<ButtonStyled>
					<button class="!shadow-none" :disabled="scanning" @click="downloadJavaModal?.show()">
						<DownloadIcon class="h-4 w-4" />
						{{ formatMessage(messages.downloadJava) }}
					</button>
				</ButtonStyled>
				<ButtonStyled color="red">
					<button class="!shadow-none" :disabled="scanning" @click="enterDeleteMode">
						<TrashIcon class="h-4 w-4" />
						{{ formatMessage(messages.deleteJava) }}
					</button>
				</ButtonStyled>
			</div>
		</template>

		<!-- Delete mode: show launcher-managed Java installations with delete buttons -->
		<template v-else>
			<div class="flex flex-col gap-1">
				<h2 class="m-0 text-lg font-semibold text-contrast">
					{{ formatMessage(messages.deleteModeTitle) }}
				</h2>
				<p class="m-0 text-sm text-secondary">
					{{ formatMessage(messages.deleteModeDescription) }}
				</p>
			</div>

			<div v-if="loadingManaged" class="py-8 text-center text-sm text-secondary">
				{{ formatMessage(messages.scanning) }}
			</div>

			<Table v-else :columns="managedColumns" :data="managedTableData" row-key="_dir_name">
				<template #cell-parsed_version="{ value }">
					<span class="font-semibold tabular-nums">Java {{ value }}</span>
				</template>
				<template #cell-distribution="{ value }">
					<span class="text-sm">{{ value || '—' }}</span>
				</template>
				<template #cell-path="{ value }">
					<span v-tooltip="value" class="block truncate font-mono text-xs max-w-72">{{ value }}</span>
				</template>
				<template #cell-actions="{ row }">
					<button
						class="p-1 text-secondary hover:text-red transition-colors"
						v-tooltip="formatMessage(messages.deleteJava)"
						@click="handleDeleteSingle(row._dir_name)"
					>
						<TrashIcon class="h-4 w-4" />
					</button>
				</template>
				<template #empty-state>
					<div class="py-8 text-center text-sm text-secondary">
						{{ formatMessage(messages.deleteEmpty) }}
					</div>
				</template>
			</Table>

			<div class="flex flex-wrap gap-2 pt-3 border-t border-button-border">
				<ButtonStyled type="outlined">
					<button @click="exitDeleteMode">
						{{ formatMessage(messages.deleteCancel) }}
					</button>
				</ButtonStyled>
			</div>
		</template>

		<div v-if="showDeepScanConfirm && !deleteMode" class="flex flex-col gap-2 p-2 bg-warning/10 rounded-lg border border-warning text-sm">
			<span>{{ formatMessage(messages.deepScanConfirm) }}</span>
			<div class="flex gap-2">
				<ButtonStyled color="red">
					<button @click="confirmDeepScan">{{ formatMessage(messages.scanAnyway) }}</button>
				</ButtonStyled>
				<ButtonStyled type="outlined">
					<button @click="showDeepScanConfirm = false">{{ formatMessage(messages.cancel) }}</button>
				</ButtonStyled>
			</div>
		</div>

		<div v-if="isWindows && !deleteMode" class="flex flex-col gap-1 pt-2 border-t border-button-border">
			<div class="flex items-center justify-between">
				<div class="flex flex-col">
					<span class="text-sm font-semibold">{{ formatMessage(messages.autoHighPerformanceMode) }}</span>
					<span class="text-xs text-secondary">{{ formatMessage(messages.autoHighPerformanceModeDescription) }}</span>
				</div>
				<Toggle id="auto-java-high-performance-mode" v-model="autoHighPerformanceMode" />
			</div>
		</div>
	</div>
</template>
