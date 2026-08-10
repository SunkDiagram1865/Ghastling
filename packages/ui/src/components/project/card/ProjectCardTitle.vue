<script setup lang="ts">
defineProps<{
	title: string
	compact?: boolean
}>()
</script>

<template>
	<!-- the weird padding and negative margin are to include the potential hover underline in bounding box which affects rendering on firefox -->
	<span
		class="project-card-title inline-block max-w-full truncate pb-[2px] mb-[-2px] font-semibold text-contrast m-0 leading-none smart-clickable:underline-on-hover whitespace-nowrap overflow-hidden"
		:class="compact ? 'text-lg' : 'text-xl'"
	>
		{{ title }}
	</span>
</template>
<style scoped>
@container (width < 550px) {
	.project-card-title {
		@apply text-base;
	}
}
</style>
