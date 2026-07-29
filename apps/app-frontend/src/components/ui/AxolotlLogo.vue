<template>
	<div aria-label="Ghastling" class="flex h-full items-center gap-2 font-extrabold text-contrast">
		<img aria-hidden="true" class="aspect-square h-full object-contain" :src="ghastlingVisual" />
		<span v-if="!iconOnly" class="hidden text-sm tracking-wide xl:inline">Ghastling</span>
	</div>
</template>

<script setup lang="ts">
import ghastlingVisual from '@/assets/axolotl.png'

defineProps<{
	iconOnly?: boolean
}>()
</script>
