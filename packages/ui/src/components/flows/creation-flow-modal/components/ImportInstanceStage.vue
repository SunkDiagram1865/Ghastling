<template>
	<div
		data-onboarding-id="creation-import"
		class="flex flex-col items-center gap-6 py-4"
	>
		<DropzoneFileInput
			:secondary-prompt="formatMessage(messages.dropZoneClick)"
			directory
			no-icon-box
			@change="onDropzoneChange"
		/>

		<!-- Launcher icons + caption -->
		<div class="flex flex-col items-center gap-2">
			<div class="flex items-end justify-center">
				<div
					v-for="(item, i) in launcherIcons"
					:key="item.key"
					class="flex size-10 items-center justify-center rounded-xl border border-surface-4 bg-surface-3 shadow-sm"
					:style="{
						transform: `rotate(${(i - 1) * 7}deg) translateY(${Math.abs(i - 1) * -4}px)`,
						marginLeft: i > 0 ? '-4px' : '0',
						zIndex: 3 - Math.abs(i - 1),
					}"
				>
					<img :src="item.url" class="size-5" :alt="item.alt" />
				</div>
			</div>
			<span class="text-center text-sm text-secondary">
				{{ formatMessage(messages.importPrompt) }}
			</span>
		</div>
	</div>
</template>

<script setup lang="ts">
import { defineMessages, useVIntl } from '@modrinth/ui'

import DropzoneFileInput from '../../../base/DropzoneFileInput.vue'
import { injectCreationFlowContext } from '../creation-flow-context'

const ctx = injectCreationFlowContext()
const { formatMessage } = useVIntl()

// ── Launcher icons (3, arc arrangement) ──
// @ts-ignore — Vite resolves .ico as static asset URL
import pcl2CeUrl from '@modrinth/assets/icons/PCL2_CE.ico'
// @ts-ignore — Vite resolves .ico as static asset URL
import pcl2Url from '@modrinth/assets/icons/PCL2.ico'
// @ts-ignore — Vite resolves .ico as static asset URL
import hmclUrl from '@modrinth/assets/icons/HMCL.ico'

const launcherIcons = [
	{ key: 'pcl2ce', url: pcl2CeUrl, alt: 'PCL2 CE' },
	{ key: 'pcl2', url: pcl2Url, alt: 'PCL2' },
	{ key: 'hmcl', url: hmclUrl, alt: 'HMCL' },
]

const messages = defineMessages({
	dropZoneClick: {
		id: 'creation-flow.modal.import-instance.drop-zone.click',
		defaultMessage: 'Click to select a file or drag & drop any file/folder',
	},
	importPrompt: {
		id: 'creation-flow.modal.import-instance.import-prompt',
		defaultMessage:
			'Drag & drop launcher folders, modpack files, or .minecraft folders to import an instance in one click',
	},
})

// ── Drop zone handler (via DropzoneFileInput) ──

function onDropzoneChange(files: File[]) {
	if (!files || files.length === 0) return

	const file = files[0]
	// Tauri adds a `path` property to File objects from native drag-drop
	const filePath: string | null = (file as any).path ?? null

	if (ctx.onImportFileReceived) {
		ctx.onImportFileReceived({
			file: filePath ? null : file,
			filePath,
			source: 'drag-drop',
		})
		return
	}

	// Fallback: treat as file-picker import
	ctx.modpackFile.value = filePath ? null : file
	ctx.modpackFilePath.value = filePath
	if (ctx.finishDisabled.value) return
	if (ctx.flowType === 'instance') {
		ctx.finish()
	} else {
		ctx.modal.value?.setStage('final-config')
	}
}

// ── File handling (reserved interface) ──
export interface ImportFilePayload {
	file: File | null
	filePath: string | null
	source: 'file-picker' | 'drag-drop'
}
</script>
