<script setup>
import { ArrowLeftIcon, CoffeeIcon, SpinnerIcon, XIcon } from '@modrinth/assets'
import {
	ButtonStyled,
	commonMessages,
	defineMessages,
	injectNotificationManager,
	useVIntl,
} from '@modrinth/ui'
import { ref } from 'vue'

import ModalWrapper from '@/components/ui/modal/ModalWrapper.vue'
import { trackEvent } from '@/helpers/analytics'
import { download_java, list_java_feed_vendors, list_java_feed_versions } from '@/helpers/jre'

const { handleError } = injectNotificationManager()
const { formatMessage } = useVIntl()

const messages = defineMessages({
	downloadJava: { id: 'app.settings.java.download.title', defaultMessage: 'Download Java' },
	selectVendor: {
		id: 'app.settings.java.download.select-vendor',
		defaultMessage: 'Choose a distribution:',
	},
	selectVersion: {
		id: 'app.settings.java.download.select-version-feed',
		defaultMessage: 'Select a version of {vendor}:',
	},
	back: { id: 'app.settings.java.download.back', defaultMessage: 'Back to distributions' },
	loading: { id: 'app.settings.java.download.loading', defaultMessage: 'Loading...' },
	noVendors: {
		id: 'app.settings.java.download.no-vendors',
		defaultMessage: 'No distributions available.',
	},
	noVersions: {
		id: 'app.settings.java.download.no-versions',
		defaultMessage: 'No versions available.',
	},
	versionLabel: {
		id: 'app.settings.java.download.version-label',
		defaultMessage: 'Java {version}',
	},
})

const emit = defineEmits(['downloaded'])

const modal = ref(null)
const loading = ref(false)
const vendors = ref([])
const selectedVendor = ref(null)
const versions = ref([])
const downloading = ref(null)

defineExpose({
	show: async () => {
		selectedVendor.value = null
		versions.value = []
		downloading.value = null
		loading.value = true
		vendors.value = []
		modal.value.show()
		vendors.value = (await list_java_feed_vendors().catch(handleError)) || []
		loading.value = false
	},
})

async function selectVendor(vendor) {
	selectedVendor.value = vendor
	loading.value = true
	versions.value = []
	const result = await list_java_feed_versions(vendor).catch(handleError)
	versions.value = result || []
	loading.value = false
}

function backToVendors() {
	selectedVendor.value = null
	versions.value = []
}

async function downloadVersion(info) {
	downloading.value = info.major_version
	trackEvent('JavaDownload', { vendor: info.vendor, version: info.major_version })
	modal.value.hide()

	const job = await download_java(info.vendor, info.major_version).catch(handleError)
	downloading.value = null

	if (job) {
		emit('downloaded', job)
	}
}
</script>
<template>
	<ModalWrapper
		ref="modal"
		:header="formatMessage(messages.downloadJava)"
		:show-ad-on-close="false"
	>
		<div class="flex flex-col gap-4 min-h-32">
			<!-- Step 1: Vendor list -->
			<template v-if="!selectedVendor">
				<span class="font-semibold text-contrast">{{ formatMessage(messages.selectVendor) }}</span>
				<div v-if="loading" class="flex items-center gap-2 text-sm text-secondary py-4">
					<SpinnerIcon class="animate-spin h-4 w-4" /> {{ formatMessage(messages.loading) }}
				</div>
				<div v-else-if="vendors.length === 0" class="text-sm text-secondary py-4">
					{{ formatMessage(messages.noVendors) }}
				</div>
				<div v-else class="grid grid-cols-3 gap-2">
					<button
						v-for="vendor in vendors"
						:key="vendor"
						class="flex items-center gap-2 px-3 py-2.5 rounded-lg border border-button-border bg-button-bg hover:border-accent transition-colors cursor-pointer text-left"
						@click="selectVendor(vendor)"
					>
						<div
							class="w-8 h-8 flex items-center justify-center rounded-full bg-button-bg border border-button-border shrink-0"
						>
							<CoffeeIcon class="h-4 w-4" />
						</div>
						<span class="font-semibold text-sm">{{ vendor }}</span>
					</button>
				</div>
			</template>

			<!-- Step 2: Version list -->
			<template v-else>
				<span class="font-semibold text-contrast">
					{{ formatMessage(messages.selectVersion, { vendor: selectedVendor }) }}
				</span>
				<div v-if="loading" class="flex items-center gap-2 text-sm text-secondary py-4">
					<SpinnerIcon class="animate-spin h-4 w-4" /> {{ formatMessage(messages.loading) }}
				</div>
				<div v-else-if="versions.length === 0" class="text-sm text-secondary py-4">
					{{ formatMessage(messages.noVersions) }}
				</div>
				<div v-else class="grid grid-cols-4 gap-2">
					<button
						v-for="info in versions"
						:key="info.major_version"
						class="flex items-center gap-2 px-3 py-2.5 rounded-lg border border-button-border bg-button-bg hover:border-accent transition-colors cursor-pointer"
						:class="{ 'opacity-50 pointer-events-none': downloading !== null }"
						:disabled="downloading !== null"
						@click="downloadVersion(info)"
					>
						<SpinnerIcon
							v-if="downloading === info.major_version"
							class="animate-spin h-4 w-4 shrink-0"
						/>
						<CoffeeIcon v-else class="h-4 w-4 shrink-0" />
						<span class="font-semibold text-sm tabular-nums">{{
							formatMessage(messages.versionLabel, { version: info.major_version })
						}}</span>
					</button>
				</div>
			</template>

			<div class="flex justify-end gap-2 pt-2 border-t border-button-border">
				<ButtonStyled v-if="selectedVendor" type="outlined">
					<button
						class="!shadow-none !border-surface-4 !border"
						:disabled="downloading !== null"
						@click="backToVendors"
					>
						<ArrowLeftIcon /> {{ formatMessage(messages.back) }}
					</button>
				</ButtonStyled>
				<ButtonStyled type="outlined">
					<button
						class="!shadow-none !border-surface-4 !border"
						:disabled="downloading !== null"
						@click="modal.hide()"
					>
						<XIcon /> {{ formatMessage(commonMessages.cancelButton) }}
					</button>
				</ButtonStyled>
			</div>
		</div>
	</ModalWrapper>
</template>
