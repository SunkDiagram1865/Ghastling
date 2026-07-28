<template>
	<NewModal
		ref="modal"
		fade="warning"
		max-width="480px"
		@hide="$emit('close')"
	>
		<template #title>
			<span class="font-extrabold text-contrast text-lg">{{ formatMessage(messages.title) }}</span>
		</template>
		<div class="flex flex-col gap-4">
			<Admonition type="warning" :header="formatMessage(messages.confirm)">
				<p class="text-sm text-tertiary">
					{{
						formatMessage(messages.warning, {
							modVersion: modVersion ?? 'any',
							modLoader: modLoader ?? 'any',
							instVersion: instVersion ?? 'any',
							instLoader: instLoader ?? 'none',
						})
					}}
				</p>
			</Admonition>

			<div class="grid grid-cols-2 gap-2">
				<div class="flex flex-col gap-1">
					<span class="text-xs text-tertiary">{{ formatMessage(messages.modVersion) }}</span>
					<span class="font-medium">{{ modVersion ?? 'unknown' }}</span>
				</div>
				<div class="flex flex-col gap-1">
					<span class="text-xs text-tertiary">{{ formatMessage(messages.instVersion) }}</span>
					<span class="font-medium">{{ instVersion ?? 'unknown' }}</span>
				</div>
				<div class="flex flex-col gap-1">
					<span class="text-xs text-tertiary">{{ formatMessage(messages.modLoader) }}</span>
					<FormattedTag :tag="modLoader ?? 'unknown'" enforce-type="loader" />
				</div>
				<div class="flex flex-col gap-1">
					<span class="text-xs text-tertiary">{{ formatMessage(messages.instLoader) }}</span>
					<FormattedTag :tag="instLoader ?? 'unknown'" enforce-type="loader" />
				</div>
			</div>
		</div>
		<template #actions>
			<div class="flex gap-3 w-full">
				<ButtonStyled>
					<button @click="$emit('keep')">
						{{ formatMessage(messages.confirm) }}
					</button>
				</ButtonStyled>
				<ButtonStyled color="brand">
					<button @click="$emit('switch')">
						{{ formatMessage(messages.search) }}
					</button>
				</ButtonStyled>
			</div>
		</template>
	</NewModal>
</template>

<script setup lang="ts">
import { defineMessages } from '@modrinth/ui'
import { useVIntl } from '@modrinth/ui'

import Admonition from '../../base/Admonition.vue'
import ButtonStyled from '../../base/ButtonStyled.vue'
import FormattedTag from '../../base/FormattedTag.vue'
import NewModal from '../../modal/NewModal.vue'

const { formatMessage } = useVIntl()

const props = defineProps<{
	modVersion?: string
	modLoader?: string
	instVersion?: string
	instLoader?: string
	modName?: string
	hasModrinthMatch?: boolean
}>()

defineEmits<{
	keep: []
	switch: []
	close: []
}>()

const messages = defineMessages({
	title: {
		id: 'app.drop.mod-compatibility-title',
		defaultMessage: 'Version Mismatch',
	},
	confirm: {
		id: 'app.drop.mod-compatibility-confirm',
		defaultMessage: 'Install anyway?',
	},
	warning: {
		id: 'app.drop.mod-compatibility-warning',
		defaultMessage:
			'This mod targets {modVersion} ({modLoader}), but the instance is {instVersion} ({instLoader}).',
	},
	modVersion: {
		id: 'app.drop.mod-version',
		defaultMessage: 'Mod Version',
	},
	instVersion: {
		id: 'app.drop.inst-version',
		defaultMessage: 'Instance Version',
	},
	modLoader: {
		id: 'app.drop.mod-loader',
		defaultMessage: 'Mod Loader',
	},
	instLoader: {
		id: 'app.drop.inst-loader',
		defaultMessage: 'Instance Loader',
	},
	search: {
		id: 'app.drop.mod-compatibility-search',
		defaultMessage: 'Find compatible version',
	},
})
</script>
