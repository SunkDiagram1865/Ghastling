<script setup lang="ts">
const emit = defineEmits<{
	(e: 'click'): void
}>()

withDefaults(
	defineProps<{
		tooltip?: string
		highlighted?: boolean
		disabled?: boolean
	}>(),
	{
		tooltip: undefined,
		highlighted: false,
		disabled: false,
	},
)
</script>

<template>
	<button
		v-tooltip="tooltip"
		type="button"
		class="group block aspect-[31/40] m-0 p-0 border-0 bg-transparent cursor-pointer"
		:class="{ 'pointer-events-none opacity-65': disabled }"
		:aria-label="tooltip"
		:aria-pressed="highlighted"
		:disabled="disabled"
		@click="emit('click')"
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
				class="relative z-10 flex items-center justify-center h-full w-full text-primary"
			>
				<span class="flex flex-col items-center justify-center text-center gap-1">
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
