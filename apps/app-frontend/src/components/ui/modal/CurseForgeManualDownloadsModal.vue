<template>
	<NewModal ref="modal" :header="formatMessage(messages.header)" fade="warning" max-width="640px">
		<div class="flex flex-col gap-3">
			<p class="m-0 text-secondary">
				{{
					formatMessage(messages.body, {
						installed,
						manual: items.length,
					})
				}}
			</p>

			<div class="max-h-72 overflow-y-auto rounded-xl border border-surface-5 bg-surface-2">
				<div
					v-for="item in items"
					:key="`${item.projectId}:${item.fileId}`"
					class="flex items-center justify-between gap-3 border-0 border-b border-solid border-surface-5 px-3 py-2 last:border-b-0"
				>
					<div class="min-w-0 flex flex-col">
						<span class="truncate font-medium text-contrast">{{ item.fileName }}</span>
						<span class="truncate text-sm text-secondary">
							{{
								formatMessage(messages.projectFile, {
									projectId: item.projectId,
									fileId: item.fileId,
								})
							}}
						</span>
					</div>
					<ButtonStyled type="outlined" size="small">
						<button :disabled="!item.websiteUrl" @click="openOne(item)">
							<ExternalIcon />
							{{ formatMessage(messages.open) }}
						</button>
					</ButtonStyled>
				</div>
			</div>
		</div>

		<template #actions>
			<div class="flex flex-wrap justify-end gap-2">
				<ButtonStyled type="outlined">
					<button @click="hide">
						{{ formatMessage(commonMessages.closeButton) }}
					</button>
				</ButtonStyled>
				<ButtonStyled v-if="instanceId" type="outlined">
					<button @click="goToInstance">
						{{ formatMessage(messages.viewInstance) }}
					</button>
				</ButtonStyled>
				<ButtonStyled color="orange">
					<button :disabled="!hasOpenableLinks" @click="openAll">
						<ExternalIcon />
						{{ formatMessage(messages.openAll) }}
					</button>
				</ButtonStyled>
			</div>
		</template>
	</NewModal>
</template>

<script setup lang="ts">
import { ExternalIcon } from '@modrinth/assets'
import { ButtonStyled, commonMessages, defineMessages, NewModal, useVIntl } from '@modrinth/ui'
import { openUrl } from '@tauri-apps/plugin-opener'
import { computed, ref } from 'vue'

import type { CurseForgeManualDownloadItem } from '@/helpers/curseforge-manual'

const { formatMessage } = useVIntl()

const messages = defineMessages({
	header: {
		id: 'app.curseforge.manual-downloads.header',
		defaultMessage: 'Manual CurseForge downloads needed',
	},
	body: {
		id: 'app.curseforge.manual-downloads.body',
		defaultMessage:
			'Installed {installed, number} files automatically, but {manual, number} could not be downloaded by the launcher. Open each page on CurseForge, download the file, then put it into this instance.',
	},
	projectFile: {
		id: 'app.curseforge.manual-downloads.project-file',
		defaultMessage: 'Project {projectId} · File {fileId}',
	},
	open: {
		id: 'app.curseforge.manual-downloads.open',
		defaultMessage: 'Open',
	},
	openAll: {
		id: 'app.curseforge.manual-downloads.open-all',
		defaultMessage: 'Open all',
	},
	viewInstance: {
		id: 'app.curseforge.manual-downloads.view-instance',
		defaultMessage: 'View instance',
	},
})

const emit = defineEmits<{
	(e: 'view-instance', instanceId: string): void
}>()

const modal = ref<InstanceType<typeof NewModal>>()
const items = ref<CurseForgeManualDownloadItem[]>([])
const installed = ref(0)
const instanceId = ref<string | null>(null)

const hasOpenableLinks = computed(() => items.value.some((item) => !!item.websiteUrl))

function show(payload: {
	items: CurseForgeManualDownloadItem[]
	installed: number
	instanceId?: string | null
}) {
	items.value = payload.items
	installed.value = payload.installed
	instanceId.value = payload.instanceId ?? null
	modal.value?.show()
}

function hide() {
	modal.value?.hide()
}

async function openOne(item: CurseForgeManualDownloadItem) {
	if (!item.websiteUrl) return
	await openUrl(item.websiteUrl)
}

async function openAll() {
	for (const item of items.value) {
		if (!item.websiteUrl) continue
		await openUrl(item.websiteUrl)
	}
}

function goToInstance() {
	if (!instanceId.value) return
	hide()
	emit('view-instance', instanceId.value)
}

defineExpose({
	show,
	hide,
})
</script>
