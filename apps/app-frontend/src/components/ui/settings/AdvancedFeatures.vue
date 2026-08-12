<script setup lang="ts">
import { WrenchIcon } from '@modrinth/assets'
import {
	ButtonStyled,
	Combobox,
	defineMessages,
	injectNotificationManager,
	Toggle,
	useVIntl,
} from '@modrinth/ui'
import { invoke } from '@tauri-apps/api/core'
import { computed, inject, ref, watch } from 'vue'

import { get, set } from '@/helpers/settings.ts'
import { handleSevereError } from '@/store/error.js'

const { formatMessage } = useVIntl()

const settings = ref(await get())

watch(
	settings,
	async () => {
		await set(settings.value)
	},
	{ deep: true },
)

const STARTUP_SOUND_KEY = 'ghastling-startup-sound-enabled'

const startupSoundEnabled = ref(localStorage.getItem(STARTUP_SOUND_KEY) !== 'false')

watch(startupSoundEnabled, (val) => {
	localStorage.setItem(STARTUP_SOUND_KEY, String(val))
})

const dragDropEnabled = inject<import('vue').Ref<boolean>>('dragDropEnabled', ref(true))

const CLOSE_BEHAVIOR_KEY = 'ghastling-close-to-tray'

// 关闭行为：'tray' 最小化到托盘，'close' 直接关闭
const closeBehavior = ref<'tray' | 'close'>(
	localStorage.getItem(CLOSE_BEHAVIOR_KEY) === 'tray' ? 'tray' : 'close',
)

const closeBehaviorOptions = computed(() => [
	{ value: 'tray', label: formatMessage(messages.closeBehaviorTray) },
	{ value: 'close', label: formatMessage(messages.closeBehaviorClose) },
])

watch(closeBehavior, (val) => {
	localStorage.setItem(CLOSE_BEHAVIOR_KEY, val)
	invoke('set_close_to_tray', { enabled: val === 'tray' }).catch(() => {})
})

// 组件挂载时同步设置到后端
invoke('set_close_to_tray', { enabled: closeBehavior.value === 'tray' }).catch(() => {})

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
	dragDropTitle: {
		id: 'app.advanced-settings.drag-drop.title',
		defaultMessage: '全局拖放导入',
	},
	dragDropDescription: {
		id: 'app.advanced-settings.drag-drop.description',
		defaultMessage: '拖放文件到窗口即可自动识别并导入（模组、整合包、存档等）。',
	},
	closeBehaviorTitle: {
		id: 'app.advanced-settings.close-behavior.title',
		defaultMessage: '关闭按钮行为',
	},
	closeBehaviorDescription: {
		id: 'app.advanced-settings.close-behavior.description',
		defaultMessage: '选择点击关闭按钮时是最小化到系统托盘还是直接退出启动器。',
	},
	closeBehaviorTray: {
		id: 'app.advanced-settings.close-behavior.tray',
		defaultMessage: '最小化到系统托盘',
	},
	closeBehaviorClose: {
		id: 'app.advanced-settings.close-behavior.close',
		defaultMessage: '直接关闭',
	},
	systemProxyTitle: {
		id: 'app.advanced-settings.system-proxy.title',
		defaultMessage: '系统代理',
	},
	systemProxyDescription: {
		id: 'app.advanced-settings.system-proxy.description',
		defaultMessage:
			'使用系统代理进行网络下载，适用于直连导致下载失败的场景。（注：为了启动器能够正常下载更新，不影响更新流程，下载启动器更新文件强制直连下载。）',
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

		<div class="flex items-center justify-between gap-4">
			<div>
				<h2 class="m-0 text-lg font-semibold text-contrast">
					{{ formatMessage(messages.dragDropTitle) }}
				</h2>
				<p class="m-0 mt-1 text-sm text-secondary">
					{{ formatMessage(messages.dragDropDescription) }}
				</p>
			</div>
			<Toggle
				id="drag-drop-enabled"
				:model-value="dragDropEnabled"
				@update:model-value="(e) => (dragDropEnabled = !!e)"
			/>
		</div>

		<div class="flex items-center justify-between gap-4">
			<div>
				<h2 class="m-0 text-lg font-semibold text-contrast">
					{{ formatMessage(messages.systemProxyTitle) }}
				</h2>
				<p class="m-0 mt-1 text-sm text-secondary">
					{{ formatMessage(messages.systemProxyDescription) }}
				</p>
			</div>
			<Toggle
				id="system-proxy-enabled"
				:model-value="settings.use_system_proxy"
				@update:model-value="(e) => (settings.use_system_proxy = !!e)"
			/>
		</div>

		<div class="grid grid-cols-[minmax(0,1fr)_11rem] items-center gap-6">
			<div class="flex min-w-0 flex-col gap-1">
				<h2 class="m-0 text-lg font-semibold text-contrast">
					{{ formatMessage(messages.closeBehaviorTitle) }}
				</h2>
				<p class="m-0 leading-relaxed text-secondary">
					{{ formatMessage(messages.closeBehaviorDescription) }}
				</p>
			</div>
			<div class="w-44">
				<Combobox
					id="close-behavior"
					v-model="closeBehavior"
					name="Close behavior"
					:options="closeBehaviorOptions"
				/>
			</div>
		</div>

		<div>
			<h2 class="m-0 mb-3 text-lg font-semibold text-contrast">
				{{ formatMessage(messages.debugTitle) }}
			</h2>
			<div class="flex flex-wrap gap-2">
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
				<ButtonStyled>
					<button @click="replayOnboarding?.('main')">
						{{ formatMessage(messages.replayOnboarding) }}
					</button>
				</ButtonStyled>
			</div>
		</div>
	</div>
</template>
