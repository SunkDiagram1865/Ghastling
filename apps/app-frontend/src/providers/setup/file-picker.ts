import { type PickedFile, provideFilePicker } from '@modrinth/ui'
import { convertFileSrc, invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { readFile } from '@tauri-apps/plugin-fs'
import { useTemplateRef } from 'vue'
import type { ComponentExposed } from 'vue-component-type-helpers'

import { builtInInstanceIcons } from '../../helpers/instance-icons'

function getFileName(path: string, fallback: string) {
	return path.split(/[\\/]/).pop() || fallback
}

function getDialogPath(result: string | { path?: string } | null | undefined) {
	if (!result) return null
	return typeof result === 'string' ? result : (result.path ?? null)
}

async function createFileFromPath(path: string, fallbackName: string, type?: string) {
	const bytes = await readFile(path)
	const name = getFileName(path, fallbackName)
	return new File([bytes], name, type ? { type } : undefined)
}

async function createNativeFileFromPath(path: string, fallbackName: string, type?: string) {
	const bytes = await invoke<number[]>('plugin:files|file_read_dragged_file', { path })
	const name = getFileName(path, fallbackName)
	return new File([new Uint8Array(bytes)], name, type ? { type } : undefined)
}

export async function pickImage(): Promise<PickedFile | null> {
	const result = await open({
		multiple: false,
		filters: [{ name: 'Image', extensions: ['png', 'jpeg', 'jpg', 'svg', 'webp', 'gif'] }],
	})
	if (!result) return null
	const path = getDialogPath(result)
	if (!path) return null
	const file = await createFileFromPath(path, 'icon')
	return { file, path, previewUrl: convertFileSrc(path) }
}

export function setupFilePickerProvider() {
	const instanceIconPickerModal =
		useTemplateRef<ComponentExposed<typeof InstanceIconPickerModal>>('instanceIconPickerModal')

	provideFilePicker({
		async pickFiles(options) {
			const result = await open({
				multiple: options?.multiple ?? true,
			})
			if (!result) return []

			const paths = Array.isArray(result) ? result : [result]
			return await Promise.all(
				paths
					.map(getDialogPath)
					.filter((path): path is string => !!path)
					.map(async (path) => ({
						file: await createNativeFileFromPath(path, 'file'),
						path,
						previewUrl: convertFileSrc(path),
					})),
			)
		},
		async pickFolder() {
			const result = await open({
				directory: true,
				multiple: false,
			})
			const path = getDialogPath(result)
			if (!path) return null
			return { path }
		},
		pickImage,
		pickInstanceIcon: () => instanceIconPickerModal.value?.show() ?? Promise.resolve(null),
		async setBuiltInInstanceIcon(iconId) {
			const icon = builtInInstanceIcons.find((i) => i.id === iconId)
			if (!icon) return null
			const response = await fetch(icon.url)
			const blob = await response.blob()
			const file = new File([blob], `${iconId}.png`, { type: blob.type })
			return { file, previewUrl: icon.url }
		},
		async pickModpackFile(options) {
			const result = await open({
				multiple: false,
				filters: [{ name: 'Modpack', extensions: ['mrpack', 'zip'] }],
			})
			if (!result) return null
			const path = getDialogPath(result)
			if (!path) return null
			if (options?.readFile === false) {
				// Instance imports stream from the native path, keeping large packs out of JS memory.
				return { path, previewUrl: '' }
			}
			return {
				file: await createFileFromPath(
					path,
					'modpack.mrpack',
					'application/x-modrinth-modpack+zip',
				),
				path,
				previewUrl: '',
			}
		},
	})

	return { instanceIconPickerModal }
}
