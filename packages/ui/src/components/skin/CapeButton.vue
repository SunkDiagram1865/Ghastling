<script setup lang="ts">
import { computed } from 'vue'

const emit = defineEmits<{
	(e: 'select'): void
}>()

const props = withDefaults(
	defineProps<{
		name: string | undefined
		id: string
		texture: string
		isEquipped?: boolean
		selected?: boolean
		faded?: boolean
		disabled?: boolean
	}>(),
	{
		isEquipped: false,
		selected: undefined,
		faded: false,
		disabled: false,
	},
)

const highlighted = computed(() => props.selected ?? props.isEquipped)
</script>

<template>
	<button
		v-tooltip="name"
		class="group block aspect-[31/40] m-0 p-0 border-0 bg-transparent cursor-pointer"
		:class="{ 'pointer-events-none opacity-65': disabled }"
		:aria-label="name"
		:aria-pressed="highlighted"
		:disabled="disabled"
		@click="emit('select')"
	>
		<span
			class="relative block h-full w-full shrink-0 rounded-[20px] border border-solid transition-[border-color,background-color,transform] duration-200 p-1 group-active:scale-95"
			:class="
				highlighted
					? 'border-brand bg-brand-highlight'
					: 'border-surface-4 bg-surface-3 hover:border-surface-5 hover:bg-surface-4'
			"
		>
			<span
				class="relative z-10 flex items-center justify-center h-full w-full"
			>
				<span
					class="block magical-cape-transform rounded-[16px] overflow-hidden"
					:class="{
						'brightness-[0.3] contrast-[0.8]': faded,
					}"
				>
					<img :src="texture" alt="" />
				</span>
				<span
					v-if="$slots.default || $slots.icon"
					class="absolute inset-0 flex flex-col items-center justify-center text-primary font-medium gap-1"
				>
					<span class="flex items-center justify-center leading-none">
						<slot name="icon"></slot>
					</span>
					<span class="block text-xs leading-none">
						<slot></slot>
					</span>
				</span>
			</span>
		</span>
	</button>
</template>
<style lang="scss" scoped>
.magical-cape-transform {
	aspect-ratio: 10 / 16;
	position: relative;
	overflow: hidden;
	height: 90%;
	width: auto;
}

.magical-cape-transform img {
	position: absolute;
	object-fit: cover;
	image-rendering: pixelated;

	// scales image up so that the target area of the texture (10x16) is 100% of the container
	width: calc(64 / 10 * 100%);
	height: calc(32 / 16 * 100%);

	// offsets the image so that the target area is in the container
	left: calc(1 / 10 * -100%);
	top: calc(1 / 16 * -100%);

	// scale the image up a little bit to avoid edges from the surrounding texture due to rounding
	scale: 1.01;
	transform-origin: calc(10 / 2 / 64 * 100%) calc(16 / 2 / 32 * 100%);
}
</style>
