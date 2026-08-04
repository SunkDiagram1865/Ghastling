<script setup lang="ts">
import { computed, onMounted } from 'vue'

import { useSystemInfo } from '@/composables/useSystemInfo'

const { info, start, error } = useSystemInfo(2000)

onMounted(() => {
	start()
})

function formatBytes(bytes: number): string {
	if (bytes === 0) return '0 B'
	const units = ['B', 'KB', 'MB', 'GB', 'TB']
	const k = 1024
	const i = Math.floor(Math.log(bytes) / Math.log(k))
	const unitIndex = Math.min(i, units.length - 1)
	return parseFloat((bytes / Math.pow(k, unitIndex)).toFixed(2)) + ' ' + units[unitIndex]
}

function formatBytesShort(bytes: number): string {
	if (bytes === 0) return '0 B'
	const units = ['B', 'KB', 'MB', 'GB', 'TB']
	const k = 1024
	const i = Math.floor(Math.log(bytes) / Math.log(k))
	const unitIndex = Math.min(i, units.length - 1)
	const val = bytes / Math.pow(k, unitIndex)
	return (val >= 10 ? val.toFixed(0) : val.toFixed(1)) + ' ' + units[unitIndex]
}

const cpuPercent = computed(() => {
	if (!info.value) return 0
	return Math.round(info.value.cpu_usage * 100) / 100
})

const memoryPercent = computed(() => {
	if (!info.value || info.value.memory_total === 0) return 0
	return Math.round((info.value.memory_used / info.value.memory_total) * 10000) / 100
})

const diskPercent = computed(() => {
	if (!info.value || info.value.disk_total === 0) return 0
	return Math.round((info.value.disk_used / info.value.disk_total) * 10000) / 100
})

const gpuPercent = computed(() => {
	if (!info.value) return 0
	return Math.round(info.value.gpu_usage * 100) / 100
})

const memoryUsedFormatted = computed(() => {
	if (!info.value) return '0'
	return formatBytesShort(info.value.memory_used)
})

const memoryTotalFormatted = computed(() => {
	if (!info.value) return '0'
	return formatBytesShort(info.value.memory_total)
})

const diskUsedFormatted = computed(() => {
	if (!info.value) return '0'
	return formatBytesShort(info.value.disk_used)
})

const diskTotalFormatted = computed(() => {
	if (!info.value) return '0'
	return formatBytesShort(info.value.disk_total)
})

const cpuCoresText = computed(() => {
	if (!info.value) return ''
	return `${info.value.cpu_cores} Core${info.value.cpu_cores > 1 ? 's' : ''}`
})
</script>

<template>
	<div v-if="error" class="text-red-500 text-sm py-2">
		系统信息获取失败
	</div>
	<div v-else-if="info" class="flex items-end justify-center gap-12 py-4">
		<div class="flex flex-col items-center">
			<div class="relative w-36 h-24 overflow-hidden">
				<svg class="w-36 h-24" viewBox="0 0 80 48" preserveAspectRatio="xMidYMid meet">
					<path
						d="M 6 46 A 34 34 0 0 1 74 46"
						fill="none"
						class="stroke-divider"
						stroke-width="5"
						stroke-linecap="round"
					/>
					<path
						d="M 6 46 A 34 34 0 0 1 74 46"
						fill="none"
						class="stroke-brand transition-all duration-500"
						stroke-width="5"
						stroke-linecap="round"
						:stroke-dasharray="`${(cpuPercent / 100) * 106.8} 106.8`"
						:style="{ stroke: cpuPercent > 80 ? '#ef4444' : cpuPercent > 60 ? '#f59e0b' : '#22c55e' }"
					/>
				</svg>
				<div class="absolute inset-x-0 bottom-1 text-center">
					<span class="text-lg font-bold" :style="{ color: cpuPercent > 80 ? '#ef4444' : cpuPercent > 60 ? '#f59e0b' : '#22c55e' }">
						{{ cpuPercent.toFixed(1) }}%
					</span>
				</div>
			</div>
			<div class="mt-1 text-center">
				<span class="text-sm text-secondary">CPU: {{ cpuCoresText }}</span>
			</div>
		</div>

		<div class="flex flex-col items-center">
			<div class="relative w-36 h-24 overflow-hidden">
				<svg class="w-36 h-24" viewBox="0 0 80 48" preserveAspectRatio="xMidYMid meet">
					<path
						d="M 6 46 A 34 34 0 0 1 74 46"
						fill="none"
						class="stroke-divider"
						stroke-width="5"
						stroke-linecap="round"
					/>
					<path
						d="M 6 46 A 34 34 0 0 1 74 46"
						fill="none"
						class="stroke-brand transition-all duration-500"
						stroke-width="5"
						stroke-linecap="round"
						:stroke-dasharray="`${(memoryPercent / 100) * 106.8} 106.8`"
						:style="{ stroke: memoryPercent > 80 ? '#ef4444' : memoryPercent > 60 ? '#f59e0b' : '#3b82f6' }"
					/>
				</svg>
				<div class="absolute inset-x-0 bottom-1 text-center">
					<span class="text-lg font-bold" :style="{ color: memoryPercent > 80 ? '#ef4444' : memoryPercent > 60 ? '#f59e0b' : '#3b82f6' }">
						{{ memoryPercent.toFixed(1) }}%
					</span>
				</div>
			</div>
			<div class="mt-1 text-center">
				<span class="text-sm text-secondary">内存: {{ memoryUsedFormatted }} / {{ memoryTotalFormatted }}</span>
			</div>
		</div>

		<div class="flex flex-col items-center">
			<div class="relative w-36 h-24 overflow-hidden">
				<svg class="w-36 h-24" viewBox="0 0 80 48" preserveAspectRatio="xMidYMid meet">
					<path
						d="M 6 46 A 34 34 0 0 1 74 46"
						fill="none"
						class="stroke-divider"
						stroke-width="5"
						stroke-linecap="round"
					/>
					<path
						d="M 6 46 A 34 34 0 0 1 74 46"
						fill="none"
						class="stroke-brand transition-all duration-500"
						stroke-width="5"
						stroke-linecap="round"
						:stroke-dasharray="`${(diskPercent / 100) * 106.8} 106.8`"
						:style="{ stroke: diskPercent > 80 ? '#ef4444' : diskPercent > 60 ? '#f59e0b' : '#a855f7' }"
					/>
				</svg>
				<div class="absolute inset-x-0 bottom-1 text-center">
					<span class="text-lg font-bold" :style="{ color: diskPercent > 80 ? '#ef4444' : diskPercent > 60 ? '#f59e0b' : '#a855f7' }">
						{{ diskPercent.toFixed(1) }}%
					</span>
				</div>
			</div>
			<div class="mt-1 text-center">
				<span class="text-sm text-secondary">存储: {{ diskUsedFormatted }} / {{ diskTotalFormatted }}</span>
			</div>
		</div>

		<div class="flex flex-col items-center">
			<div class="relative w-36 h-24 overflow-hidden">
				<svg class="w-36 h-24" viewBox="0 0 80 48" preserveAspectRatio="xMidYMid meet">
					<path
						d="M 6 46 A 34 34 0 0 1 74 46"
						fill="none"
						class="stroke-divider"
						stroke-width="5"
						stroke-linecap="round"
					/>
					<path
						d="M 6 46 A 34 34 0 0 1 74 46"
						fill="none"
						class="stroke-brand transition-all duration-500"
						stroke-width="5"
						stroke-linecap="round"
						:stroke-dasharray="`${(gpuPercent / 100) * 106.8} 106.8`"
						:style="{ stroke: gpuPercent > 80 ? '#ef4444' : gpuPercent > 60 ? '#f59e0b' : '#eab308' }"
					/>
				</svg>
				<div class="absolute inset-x-0 bottom-1 text-center">
					<span class="text-lg font-bold" :style="{ color: gpuPercent > 80 ? '#ef4444' : gpuPercent > 60 ? '#f59e0b' : '#eab308' }">
						{{ gpuPercent.toFixed(1) }}%
					</span>
				</div>
			</div>
			<div class="mt-1 text-center">
				<span class="text-sm text-secondary">GPU</span>
			</div>
		</div>
	</div>
	<div v-else class="flex items-center justify-center h-32">
		<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-brand"></div>
	</div>
</template>
