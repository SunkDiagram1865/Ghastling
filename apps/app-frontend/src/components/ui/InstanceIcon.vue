<script setup lang="ts">
import { Avatar } from '@modrinth/ui'
import { convertFileSrc } from '@tauri-apps/api/core'
import { computed } from 'vue'

import { getDefaultIconForLoader } from '@/helpers/instance-icons'
import { isBuiltInInstanceIcon } from '@/helpers/instance-icon-frame'

const props = withDefaults(
	defineProps<{
		iconPath?: string | null
		instanceId?: string | null
		loader?: string | null
	}>(),
	{
		iconPath: null,
		instanceId: null,
		loader: null,
	},
)

const iconUrl = computed(() => {
	if (props.iconPath) {
		return convertFileSrc(props.iconPath)
	}
	if (props.loader) {
		const defaultIcon = getDefaultIconForLoader(props.loader)
		if (defaultIcon) {
			return defaultIcon
		}
	}
	return null
})

const isFrameless = computed(() => {
	if (props.iconPath) {
		return isBuiltInInstanceIcon(props.iconPath)
	}
	return true
})
</script>

<template>
	<Avatar
		:src="iconUrl"
		:tint-by="instanceId"
		:class="{
			'!border-0 !rounded-none !bg-transparent !shadow-none': isFrameless,
		}"
	/>
</template>
