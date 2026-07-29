<script setup lang="ts">
import { EyeIcon, RefreshCwIcon } from '@modrinth/assets'
import { ButtonStyled, defineMessages, injectNotificationManager, useVIntl } from '@modrinth/ui'
import { getVersion } from '@tauri-apps/api/app'
import { inject, ref } from 'vue'

import { AxolotlBrandConfig } from '@/config'
import { isDev } from '@/helpers/utils.js'
import { type AppUpdateCheckResult, checkForAppUpdate } from '@/providers/app-update.ts'

const { formatMessage } = useVIntl()
const { handleError } = injectNotificationManager()
const checking = ref(false)
const checkResult = ref<AppUpdateCheckResult | 'failed' | null>(null)
const currentVersion = await getVersion()
const isDevEnvironment = await isDev()
const previewUpdateAnnouncement = inject<(version: string) => void>('previewUpdateAnnouncement')

const messages = defineMessages({
	title: {
		id: 'app.settings.updates.title',
		defaultMessage: 'Updates',
	},
	description: {
		id: 'app.settings.updates.description',
		defaultMessage: 'Ghastling checks for launcher updates via GitHub releases.',
	},
	check: {
		id: 'app.settings.updates.check',
		defaultMessage: 'Check for updates',
	},
	checking: {
		id: 'app.settings.updates.checking',
		defaultMessage: 'Checking for updates…',
	},
	available: {
		id: 'app.settings.updates.available',
		defaultMessage: 'An update is available.',
	},
	upToDate: {
		id: 'app.settings.updates.up-to-date',
		defaultMessage: 'Ghastling is up to date.',
	},
	disabled: {
		id: 'app.settings.updates.disabled',
		defaultMessage: 'Updates are disabled in this build.',
	},
	offline: {
		id: 'app.settings.updates.offline',
		defaultMessage: 'Connect to the internet to check for updates.',
	},
	failed: {
		id: 'app.settings.updates.failed',
		defaultMessage: 'Could not check for updates.',
	},
	security: {
		id: 'app.settings.updates.security',
		defaultMessage: 'Updates are installed only when their cryptographic signature is valid.',
	},
	preview: {
		id: 'app.settings.updates.preview-announcement',
		defaultMessage: 'Preview update announcement',
	},
	viewAll: {
		id: 'app.settings.updates.view-all',
		defaultMessage: 'View full release log',
	},
})

const resultMessages: Record<AppUpdateCheckResult | 'failed', keyof typeof messages> = {
	available: 'available',
	'up-to-date': 'upToDate',
	disabled: 'disabled',
	offline: 'offline',
	failed: 'failed',
}

async function checkForUpdates() {
	checking.value = true
	checkResult.value = null

	try {
		checkResult.value = await checkForAppUpdate()
	} catch (error) {
		checkResult.value = 'failed'
		handleError(error)
	} finally {
		checking.value = false
	}
}
</script>

<template>
	<div class="flex flex-col gap-6">
		<div class="flex min-w-0 flex-col gap-1">
			<h2 class="m-0 text-lg font-semibold text-contrast">
				{{ formatMessage(messages.title) }}
			</h2>
			<p class="m-0 leading-relaxed text-secondary">
				{{ formatMessage(messages.description) }}
			</p>
		</div>

		<div class="flex flex-col items-start gap-3">
			<div class="flex flex-wrap gap-2">
				<ButtonStyled color="brand">
					<button :disabled="checking" @click="checkForUpdates">
						<RefreshCwIcon :class="{ 'animate-spin': checking }" />
						{{ formatMessage(checking ? messages.checking : messages.check) }}
					</button>
				</ButtonStyled>
				<ButtonStyled v-if="isDevEnvironment && previewUpdateAnnouncement" type="outlined">
					<button type="button" @click="previewUpdateAnnouncement(currentVersion)">
						<EyeIcon />
						{{ formatMessage(messages.preview) }}
					</button>
				</ButtonStyled>
				<ButtonStyled type="outlined">
					<a
						:href="AxolotlBrandConfig.releaseUrl"
						target="_blank"
						rel="noopener noreferrer"
					>
						{{ formatMessage(messages.viewAll) }}
					</a>
				</ButtonStyled>
			</div>
			<p v-if="checkResult" class="m-0 text-sm text-secondary" role="status">
				{{ formatMessage(messages[resultMessages[checkResult]]) }}
			</p>
		</div>

		<p class="m-0 rounded-xl bg-surface-4 p-4 text-sm leading-tight text-secondary">
			{{ formatMessage(messages.security) }}
		</p>
	</div>
</template>
