<template>
	<NewModal ref="modal" max-width="480px" :closable="true" hide-header @hide="onModalHide">
		<div class="flex flex-col gap-4 p-6">
			<!-- Title -->
			<div class="flex flex-col gap-1">
				<span class="text-lg font-semibold text-contrast">{{ formatMessage(messages.title) }}</span>
				<span class="text-sm text-secondary">
					{{ formatMessage(messages.subtitle, { n: internalInstanceNames.length }) }}
				</span>
			</div>

			<HorizontalRule />

			<SelectionCard
				:icon="CopyIcon"
				:title="formatMessage(messages.copyTitle)"
				:description="formatMessage(messages.copyDesc)"
				:selected="selected === 'copy'"
				value="copy"
				@select="select"
			>
				<p class="text-xs text-secondary m-0">{{ formatMessage(messages.copyDetail) }}</p>
			</SelectionCard>

			<SelectionCard
				:icon="LinkIcon"
				:title="formatMessage(messages.symlinkTitle)"
				:description="formatMessage(messages.symlinkDesc)"
				:selected="selected === 'symlink'"
				value="symlink"
				:disabled="!symlinkAllowed"
				@select="select"
			>
				<p class="text-xs text-secondary m-0">{{ formatMessage(messages.symlinkDetail) }}</p>
				<span
					v-if="internalSymlinkCapable === 'requires_admin'"
					class="text-xs text-warning mt-1 block"
				>
					{{ formatMessage(messages.requiresAdmin) }}
				</span>
				<span
					v-else-if="internalSymlinkCapable === 'unsupported'"
					class="text-xs text-danger mt-1 block"
				>
					{{ formatMessage(messages.unsupportedWarning) }}
				</span>
			</SelectionCard>
		</div>

		<template #actions>
			<div class="flex w-full items-center justify-between p-4 pt-0">
				<ButtonStyled type="transparent">
					<button @click="handleCancel">
						{{ formatMessage(messages.cancel) }}
					</button>
				</ButtonStyled>
				<ButtonStyled>
					<button class="flex items-center gap-2" :disabled="!selected" @click="handleConfirm">
						{{ formatMessage(messages.confirm) }}
					</button>
				</ButtonStyled>
			</div>
		</template>
	</NewModal>
</template>

<script setup lang="ts">
import { CopyIcon, LinkIcon } from '@modrinth/assets'
import type { PropType } from 'vue'
import { computed, ref } from 'vue'

import ButtonStyled from '#ui/components/base/ButtonStyled.vue'
import HorizontalRule from '#ui/components/base/HorizontalRule.vue'
import NewModal from '#ui/components/modal/NewModal.vue'
import SelectionCard from '#ui/components/base/SelectionCard.vue'
import { defineMessages, useVIntl } from '#ui/composables/i18n'

const { formatMessage } = useVIntl()

const messages = defineMessages({
	title: {
		id: 'drop.symlink_method.title',
		defaultMessage: 'Choose import method',
	},
	subtitle: {
		id: 'drop.symlink_method.subtitle',
		defaultMessage: 'Importing {n} instance(s)',
	},
	copyTitle: {
		id: 'drop.symlink_method.copy_title',
		defaultMessage: 'Copy files',
	},
	copyDesc: {
		id: 'drop.symlink_method.copy_desc',
		defaultMessage: 'Copy to Ghastling directory',
	},
	copyDetail: {
		id: 'drop.symlink_method.copy_detail',
		defaultMessage:
			"Instance files will be copied to Ghastling's data directory. This is the default option with the best compatibility.",
	},
	symlinkTitle: {
		id: 'drop.symlink_method.symlink_title',
		defaultMessage: 'Symbolic link',
	},
	symlinkDesc: {
		id: 'drop.symlink_method.symlink_desc',
		defaultMessage: 'Reference original location',
	},
	symlinkDetail: {
		id: 'drop.symlink_method.symlink_detail',
		defaultMessage:
			'Instance files stay in their original location. Ghastling references them via a symbolic link. Saves disk space.',
	},
	requiresAdmin: {
		id: 'drop.symlink_method.requires_admin',
		defaultMessage: 'Administrator permission required',
	},
	unsupportedWarning: {
		id: 'drop.symlink_method.unsupported_warning',
		defaultMessage: 'Symbolic links are not supported on this system',
	},
	cancel: {
		id: 'drop.symlink_method.cancel',
		defaultMessage: 'Cancel',
	},
	confirm: {
		id: 'drop.symlink_method.confirm',
		defaultMessage: 'Confirm',
	},
})

const props = defineProps({
	instanceNames: {
		type: Array<string>,
		default: () => [],
	},
	symlinkCapable: {
		type: String as PropType<'supported' | 'requires_admin' | 'unsupported'>,
		default: 'supported',
	},
})

const emit = defineEmits<{
	(e: 'confirm', symlink: boolean): void
	(e: 'cancel'): void
}>()

const modal = ref<InstanceType<typeof NewModal> | null>(null)
const isOpen = ref(false)
const selected = ref<'copy' | 'symlink' | null>(null)
const internalInstanceNames = ref<string[]>([])
const internalSymlinkCapable = ref<'supported' | 'requires_admin' | 'unsupported'>('supported')

const symlinkAllowed = computed(() => {
	return internalSymlinkCapable.value !== 'unsupported'
})

function select(value: 'copy' | 'symlink') {
	selected.value = value
}

function handleConfirm() {
	if (!selected.value) return
	isOpen.value = false
	modal.value?.hide()
	emit('confirm', selected.value === 'symlink')
}

function handleCancel() {
	isOpen.value = false
	modal.value?.hide()
	emit('cancel')
}

function onModalHide() {
	if (!isOpen.value) return
	isOpen.value = false
	emit('cancel')
}

function show(options: {
	instanceNames: string[]
	symlinkCapable: 'supported' | 'requires_admin' | 'unsupported'
}) {
	internalInstanceNames.value = options.instanceNames
	internalSymlinkCapable.value = options.symlinkCapable
	selected.value = null
	isOpen.value = true
	modal.value?.show()
}

function hide() {
	if (!isOpen.value) return
	isOpen.value = false
	modal.value?.hide()
}

defineExpose({ show, hide })
</script>
