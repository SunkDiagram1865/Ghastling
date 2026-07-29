<script setup lang="ts">
import ghastlingMascot from '@/assets/ghastling.png?url'

defineProps<{
	alt: string
}>()
</script>

<template>
	<img class="onboarding-mascot" :src="ghastlingMascot" :alt="alt" />
</template>

<style scoped lang="scss">
.onboarding-mascot {
	width: auto;
	height: clamp(5rem, 10vw, 8rem);
	max-width: 9rem;
	object-fit: contain;
}
</style>
