<template>
	<ModalWrapper
		ref="detectJavaModal"
		:header="formatMessage(messages.selectJavaVersion)"
		:show-ad-on-close="false"
	>
		<div class="flex flex-col gap-4">
			<Table :columns="javaInstallColumns" :data="chosenInstallOptions" row-key="path">
				<template #cell-version="{ value }">
					<span class="font-semibold text-primary">{{ value }}</span>
				</template>
				<template #cell-path="{ value }">
					<span v-tooltip="value" class="block truncate font-mono text-xs">{{ value }}</span>
				</template>
				<template #cell-actions="{ row }">
					<div class="flex items-center justify-end">
						<ButtonStyled v-if="currentSelected.path === row.path">
							<button class="!shadow-none" disabled>
								<CheckIcon /> {{ formatMessage(commonMessages.selectedLabel) }}
							</button>
						</ButtonStyled>
						<ButtonStyled v-else>
							<button class="!shadow-none" @click="setJavaInstall(row)">
								<PlusIcon /> {{ formatMessage(messages.select) }}
							</button>
						</ButtonStyled>
					</div>
				</template>
				<template #empty-state>
					<div class="p-4 text-secondary">
						<SpinnerIcon v-if="fullScanRunning" class="animate-spin h-4 w-4 inline mr-2" />
						{{ fullScanRunning ? formatMessage(messages.scanning) : formatMessage(messages.noneFound) }}
					</div>
				</template>
			</Table>
			<div class="flex justify-between">
				<ButtonStyled type="outlined">
					<button
						class="!shadow-none !border-surface-4 !border"
						:disabled="fullScanRunning"
						@click="runFullScan"
					>
						<SpinnerIcon v-if="fullScanRunning" class="animate-spin h-4 w-4" />
						<SearchIcon v-else class="h-4 w-4" />
						{{ fullScanRunning ? formatMessage(messages.scanning) : formatMessage(messages.deepScan) }}
					</button>
				</ButtonStyled>
				<ButtonStyled type="outlined">
					<button
						class="!shadow-none !border-surface-4 !border"
						@click="$refs.detectJavaModal.hide()"
					>
						<XIcon />
						{{ formatMessage(commonMessages.cancelButton) }}
					</button>
				</ButtonStyled>
			</div>
		</div>
	</ModalWrapper>
</template>
<script setup>
import { CheckIcon, PlusIcon, SearchIcon, SpinnerIcon, XIcon } from '@modrinth/assets'
import {
	ButtonStyled,
	commonMessages,
	defineMessages,
	injectNotificationManager,
	Table,
	useVIntl,
} from '@modrinth/ui'
import { onUnmounted, ref } from 'vue'

import ModalWrapper from '@/components/ui/modal/ModalWrapper.vue'
import { trackEvent } from '@/helpers/analytics'
import { java_discovery_listener } from '@/helpers/events'
import { find_filtered_jres } from '@/helpers/jre.js'

const { handleError } = injectNotificationManager()
const { formatMessage } = useVIntl()
const messages = defineMessages({
	selectJavaVersion: {
		id: 'app.java.select-version',
		defaultMessage: 'Select Java version',
	},
	select: { id: 'app.java.select', defaultMessage: 'Select' },
	noneFound: {
		id: 'app.java.none-found',
		defaultMessage: 'No Java installations found!',
	},
	deepScan: {
		id: 'app.java.deep-scan',
		defaultMessage: 'Deep Scan',
	},
	scanning: {
		id: 'app.java.scanning',
		defaultMessage: 'Scanning...',
	},
	version: { id: 'app.java.table.version', defaultMessage: 'Version' },
	path: { id: 'app.java.table.path', defaultMessage: 'Path' },
	actions: { id: 'app.java.table.actions', defaultMessage: 'Actions' },
})

const chosenInstallOptions = ref([])
const detectJavaModal = ref(null)
const currentSelected = ref({})
const fullScanRunning = ref(false)
const javaInstallColumns = [
	{ key: 'version', label: formatMessage(messages.version), width: '9rem' },
	{ key: 'path', label: formatMessage(messages.path) },
	{ key: 'actions', label: formatMessage(messages.actions), align: 'right', width: '10rem' },
]

const lastRequestedVersion = ref(null)
let unlistenJavaDiscovery = null

defineExpose({
	show: async (version, currentSelectedJava, fullScan = false) => {
		lastRequestedVersion.value = version ?? null
		fullScanRunning.value = false
		chosenInstallOptions.value = await find_filtered_jres(version, false, false).catch(handleError)

		currentSelected.value = currentSelectedJava
		if (!currentSelected.value) {
			currentSelected.value = { path: '', version: '' }
		}

		if (!unlistenJavaDiscovery) {
			unlistenJavaDiscovery = await java_discovery_listener(refreshInstallOptions)
		}

		detectJavaModal.value.show()

		if (fullScan) {
			await runFullScan()
		}
	},
})

async function runFullScan() {
	fullScanRunning.value = true
	chosenInstallOptions.value = await find_filtered_jres(lastRequestedVersion.value, true, false).catch(handleError)
	fullScanRunning.value = false

	trackEvent('JavaFullScan', {
		version: lastRequestedVersion.value,
		count: chosenInstallOptions.value?.length ?? 0,
	})
}

async function refreshInstallOptions() {
	if (fullScanRunning.value) return
	const updated = await find_filtered_jres(lastRequestedVersion.value, false, false).catch(() => null)
	if (updated) {
		chosenInstallOptions.value = updated
	}
}

onUnmounted(() => {
	if (unlistenJavaDiscovery) {
		unlistenJavaDiscovery()
		unlistenJavaDiscovery = null
	}
})

const emit = defineEmits(['submit'])

function setJavaInstall(javaInstall) {
	emit('submit', javaInstall)
	detectJavaModal.value.hide()
	trackEvent('JavaAutoDetect', {
		path: javaInstall.path,
		version: javaInstall.version,
	})
}
</script>
