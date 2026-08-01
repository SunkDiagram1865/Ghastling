<script setup lang="ts">
import { WrenchIcon } from '@modrinth/assets'
import { ButtonStyled, defineMessages, injectNotificationManager, Toggle, useVIntl } from '@modrinth/ui'
import { inject, ref, watch } from 'vue'

import { isDev } from '@/helpers/utils'
import { handleSevereError } from '@/store/error.js'

const { formatMessage } = useVIntl()

const STARTUP_SOUND_KEY = 'ghastling-startup-sound-enabled'

const startupSoundEnabled = ref(localStorage.getItem(STARTUP_SOUND_KEY) !== 'false')

watch(startupSoundEnabled, (val) => {
	localStorage.setItem(STARTUP_SOUND_KEY, String(val))
})

const isDevEnvironment = await isDev()
const { addNotification } = injectNotificationManager()
const replayOnboarding = inject<(mode: 'main' | 'instance') => Promise<void>>('replayOnboarding')
const previewMinecraftCrashModal = inject<() => void>('previewMinecraftCrashModal')

function triggerTestError() {
	handleSevereError(new Error(formatMessage(messages.testErrorMessage)))
}

function triggerTestNotificationError() {
	addNotification({
		title: formatMessage(messages.testNotificationErrorTitle),
		text: formatMessage(messages.testErrorMessage),
		type: 'error',
	})
}

const messages = defineMessages({
	startupSoundTitle: {
		id: 'app.advanced-settings.startup-sound.title',
		defaultMessage: '启动音效',
	},
	startupSoundDescription: {
		id: 'app.advanced-settings.startup-sound.description',
		defaultMessage: '启动器启动时随机播放一段音效（在开屏界面期间）。',
	},
	debugTitle: {
		id: 'app.advanced-settings.debug.title',
		defaultMessage: '调试',
	},
	replayOnboarding: {
		id: 'app.settings.about.replay-onboarding',
		defaultMessage: 'Replay tour',
	},
	testError: {
		id: 'app.settings.about.test-error',
		defaultMessage: 'Trigger test error',
	},
	testErrorMessage: {
		id: 'app.settings.about.test-error-message',
		defaultMessage: 'Test error triggered from the development settings.',
	},
	testNotificationError: {
		id: 'app.settings.about.test-notification-error',
		defaultMessage: 'Trigger notification test error',
	},
	testNotificationErrorTitle: {
		id: 'app.settings.about.test-notification-error-title',
		defaultMessage: 'Test notification error',
	},
	previewMinecraftCrashModal: {
		id: 'app.settings.about.preview-minecraft-crash-modal',
		defaultMessage: 'Preview Minecraft crash window',
	},
})
</script>

<template>
	<div class="flex flex-col gap-6">
		<div class="flex items-center justify-between gap-4">
			<div>
				<h2 class="m-0 text-lg font-semibold text-contrast">
					{{ formatMessage(messages.startupSoundTitle) }}
				</h2>
				<p class="m-0 mt-1 text-sm text-secondary">
					{{ formatMessage(messages.startupSoundDescription) }}
				</p>
			</div>
			<Toggle
				id="startup-sound-enabled"
				:model-value="startupSoundEnabled"
				@update:model-value="(e) => (startupSoundEnabled = !!e)"
			/>
		</div>

		<div>
			<h3 class="m-0 mb-3 text-base font-semibold text-contrast">
				{{ formatMessage(messages.debugTitle) }}
			</h3>
			<div class="flex flex-wrap gap-2">
				<ButtonStyled>
					<button @click="replayOnboarding?.('main')">
						{{ formatMessage(messages.replayOnboarding) }}
					</button>
				</ButtonStyled>
				<template v-if="isDevEnvironment">
					<ButtonStyled>
						<button @click="triggerTestError">
							<WrenchIcon /> {{ formatMessage(messages.testError) }}
						</button>
					</ButtonStyled>
					<ButtonStyled>
						<button @click="triggerTestNotificationError">
							<WrenchIcon /> {{ formatMessage(messages.testNotificationError) }}
						</button>
					</ButtonStyled>
					<ButtonStyled>
						<button @click="previewMinecraftCrashModal?.()">
							<WrenchIcon /> {{ formatMessage(messages.previewMinecraftCrashModal) }}
						</button>
					</ButtonStyled>
				</template>
			</div>
		</div>
	</div>
</template>
