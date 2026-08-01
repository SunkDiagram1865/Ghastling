<template>
	<NewModal ref="modal" :on-hide="handleModalHide">
		<template #title>
			<span class="text-lg font-extrabold text-contrast">
				{{ formatMessage(mode === 'edit' ? messages.editSkinTitle : messages.addSkinTitle) }}
			</span>
		</template>

		<div class="flex flex-col md:flex-row gap-6">
			<div class="h-[25rem] w-[16rem] min-w-[16rem] flex-shrink-0 md:self-center">
				<SkinPreviewRenderer
					:variant="variant"
					:texture-src="previewSkin || ''"
					:cape-src="selectedCapeTexture"
					framing="modal"
					:initial-rotation="Math.PI / 8"
					class="h-full w-full"
				/>
			</div>

			<div class="flex flex-col gap-4 w-full min-h-[20rem]">
				<section v-if="mode === 'edit' && canEditTextureAndModel">
					<h2 class="text-base font-semibold mb-2">{{ formatMessage(messages.textureSection) }}</h2>
					<ButtonStyled>
						<button class="!shadow-none" @click="openTextureFileBrowser">
							<UploadIcon /> {{ formatMessage(messages.replaceTextureButton) }}
						</button>
					</ButtonStyled>
					<input
						ref="textureFileInput"
						type="file"
						accept="image/png"
						class="hidden"
						@change="onTextureFileInputChange"
					/>
				</section>

				<section v-if="canEditTextureAndModel">
					<h2 class="text-base font-semibold mb-2">
						{{ formatMessage(messages.armStyleSection) }}
					</h2>
					<RadioButtons v-model="variant" :items="['CLASSIC', 'SLIM']">
						<template #default="{ item }">
							{{
								formatMessage(item === 'CLASSIC' ? messages.wideArmStyle : messages.slimArmStyle)
							}}
						</template>
					</RadioButtons>
				</section>
			</div>
		</div>

		<template #actions>
			<div class="flex gap-2 justify-end">
				<ButtonStyled type="outlined">
					<button :disabled="isSaving" @click="hide">
						<XIcon />{{ formatMessage(commonMessages.cancelButton) }}
					</button>
				</ButtonStyled>
				<ButtonStyled color="brand">
					<button v-tooltip="saveTooltip" :disabled="disableSave || isSaving" @click="save">
						<SpinnerIcon v-if="isSaving" class="animate-spin" />
						<CheckIcon v-else-if="mode === 'new'" />
						<SaveIcon v-else />
						{{ formatMessage(mode === 'new' ? messages.addSkinButton : messages.saveSkinButton) }}
					</button>
				</ButtonStyled>
			</div>
		</template>
	</NewModal>
</template>

<script setup lang="ts">
import { CheckIcon, SaveIcon, SpinnerIcon, UploadIcon, XIcon } from '@modrinth/assets'
import {
	ButtonStyled,
	commonMessages,
	defineMessages,
	injectNotificationManager,
	NewModal,
	RadioButtons,
	SkinPreviewRenderer,
	useVIntl,
} from '@modrinth/ui'
import { arrayBufferToBase64 } from '@modrinth/utils'
import { computed, ref, useTemplateRef, watch } from 'vue'

import {
	type Cape,
	determineModelType,
	equip_skin,
	get_normalized_skin_texture,
	normalize_skin_texture,
	save_custom_skin,
	type Skin,
	type SkinModel,
	type SkinTextureUrl,
} from '@/helpers/skins.ts'

const messages = defineMessages({
	editSkinTitle: {
		id: 'app.skins.modal.edit-title',
		defaultMessage: 'Editing skin',
	},
	addSkinTitle: {
		id: 'app.skins.modal.add-title',
		defaultMessage: 'Adding a skin',
	},
	textureSection: {
		id: 'app.skins.modal.texture-section',
		defaultMessage: 'Texture',
	},
	replaceTextureButton: {
		id: 'app.skins.modal.replace-texture-button',
		defaultMessage: 'Replace texture',
	},
	armStyleSection: {
		id: 'app.skins.modal.arm-style-section',
		defaultMessage: 'Arm style',
	},
	wideArmStyle: {
		id: 'app.skins.modal.arm-style-wide',
		defaultMessage: 'Wide',
	},
	slimArmStyle: {
		id: 'app.skins.modal.arm-style-slim',
		defaultMessage: 'Slim',
	},
	savingTooltip: {
		id: 'app.skins.modal.saving-tooltip',
		defaultMessage: 'Saving...',
	},
	uploadSkinFirstTooltip: {
		id: 'app.skins.modal.upload-skin-first-tooltip',
		defaultMessage: 'Upload a skin first!',
	},
	makeEditFirstTooltip: {
		id: 'app.skins.modal.make-edit-first-tooltip',
		defaultMessage: 'Make an edit to the skin first!',
	},
	addSkinButton: {
		id: 'app.skins.modal.add-skin-button',
		defaultMessage: 'Add skin',
	},
	saveSkinButton: {
		id: 'app.skins.modal.save-skin-button',
		defaultMessage: 'Save skin',
	},
})

const { formatMessage } = useVIntl()
const { handleError } = injectNotificationManager()

const modal = useTemplateRef('modal')
const textureFileInput = useTemplateRef<HTMLInputElement>('textureFileInput')
const mode = ref<'new' | 'edit'>('new')
const currentSkin = ref<Skin | null>(null)
const isSaving = ref(false)

const uploadedTextureUrl = ref<SkinTextureUrl | null>(null)
const previewSkin = ref<string>('')

const variant = ref<SkinModel>('CLASSIC')
const selectedCape = ref<Cape | undefined>(undefined)
const props = defineProps<{ capes?: Cape[] }>()

const selectedCapeTexture = computed(() => selectedCape.value?.texture)
const canEditTextureAndModel = computed(() => currentSkin.value?.source !== 'default')

async function loadPreviewSkin() {
	if (uploadedTextureUrl.value) {
		previewSkin.value = uploadedTextureUrl.value.normalized
	} else if (currentSkin.value) {
		try {
			previewSkin.value = await get_normalized_skin_texture(currentSkin.value)
		} catch (error) {
			console.error('Failed to load skin texture:', error)
			previewSkin.value = '/src/assets/skins/steve.png'
		}
	} else {
		previewSkin.value = '/src/assets/skins/steve.png'
	}
}

const hasEdits = computed(() => {
	if (mode.value !== 'edit') return true
	if (uploadedTextureUrl.value) return true
	if (!currentSkin.value) return false
	if (variant.value !== currentSkin.value.variant) return true
	if ((selectedCape.value?.id || null) !== (currentSkin.value.cape_id || null)) return true
	return false
})

const disableSave = computed(
	() =>
		(mode.value === 'new' && !uploadedTextureUrl.value) ||
		(mode.value === 'edit' && !hasEdits.value),
)

const saveTooltip = computed(() => {
	if (isSaving.value) return formatMessage(messages.savingTooltip)
	if (mode.value === 'new' && !uploadedTextureUrl.value) {
		return formatMessage(messages.uploadSkinFirstTooltip)
	}
	if (mode.value === 'edit' && !hasEdits.value) {
		return formatMessage(messages.makeEditFirstTooltip)
	}
	return undefined
})

function resetState() {
	mode.value = 'new'
	currentSkin.value = null
	uploadedTextureUrl.value = null
	previewSkin.value = ''
	variant.value = 'CLASSIC'
	selectedCape.value = undefined
	isSaving.value = false
}

function handleModalHide() {
	setTimeout(() => resetState(), 250)
}

async function show(e: MouseEvent, skin?: Skin) {
	mode.value = skin ? 'edit' : 'new'
	currentSkin.value = skin ?? null
	if (skin) {
		variant.value = skin.variant
		selectedCape.value = props.capes?.find((c) => c.id === skin.cape_id)
	} else {
		variant.value = 'CLASSIC'
		selectedCape.value = undefined
	}

	await loadPreviewSkin()

	modal.value?.show(e)
}

async function showNew(e: MouseEvent, skinTextureUrl: SkinTextureUrl) {
	mode.value = 'new'
	currentSkin.value = null
	uploadedTextureUrl.value = skinTextureUrl
	variant.value = await determineModelType(skinTextureUrl.original)
	selectedCape.value = undefined

	await loadPreviewSkin()

	modal.value?.show(e)
}

async function setUploadedTexture(skinTextureUrl: SkinTextureUrl) {
	uploadedTextureUrl.value = skinTextureUrl
	await loadPreviewSkin()
}

function hide() {
	modal.value?.hide()
}

function openTextureFileBrowser() {
	textureFileInput.value?.click()
}

async function onTextureFileInputChange(e: Event) {
	const files = (e.target as HTMLInputElement).files
	const file = files?.[0]

	if (!file) {
		return
	}

	try {
		const originalSkinTexUrl = `data:image/png;base64,${arrayBufferToBase64(
			await file.arrayBuffer(),
		)}`
		const skinTextureNormalized = await normalize_skin_texture(originalSkinTexUrl)
		await setUploadedTexture({
			original: originalSkinTexUrl,
			normalized: `data:image/png;base64,${arrayBufferToBase64(skinTextureNormalized)}`,
		})
	} catch (error) {
		handleError(error)
	} finally {
		if (textureFileInput.value) {
			textureFileInput.value.value = ''
		}
	}
}

async function save() {
	isSaving.value = true

	try {
		let textureUrl: string

		if (uploadedTextureUrl.value) {
			textureUrl = uploadedTextureUrl.value.original
		} else {
			textureUrl = currentSkin.value!.texture
		}

		const bytes: Uint8Array = new Uint8Array(await (await fetch(textureUrl)).arrayBuffer())

		if (mode.value === 'new') {
			const addedSkin = await save_custom_skin(
				{
					texture_key: '',
					variant: variant.value,
					cape_id: selectedCape.value?.id,
					texture: textureUrl,
					source: 'custom',
					is_equipped: false,
				},
				bytes,
				variant.value,
				selectedCape.value,
				true,
			)
			emit('saved', {
				applied: false,
				skin: addedSkin,
			})
		} else {
			const updatedSkin = await save_custom_skin(
				currentSkin.value!,
				bytes,
				variant.value,
				selectedCape.value,
				!!uploadedTextureUrl.value && textureUrl !== currentSkin.value?.texture,
			)

			if (currentSkin.value?.is_equipped) {
				await equip_skin(updatedSkin)
			}

			emit('saved', {
				applied: !!currentSkin.value?.is_equipped,
				skin: updatedSkin,
				previousSkin: currentSkin.value!,
			})
		}

		hide()
	} catch (err) {
		handleError(err)
	} finally {
		isSaving.value = false
	}
}

watch([uploadedTextureUrl, currentSkin], async () => {
	await loadPreviewSkin()
})

const emit = defineEmits<{
	(event: 'saved', options: { applied: boolean; skin?: Skin; previousSkin?: Skin }): void
	(event: 'deleted', skin: Skin): void
}>()

defineExpose({
	show,
	showNew,
	hide,
})
</script>
