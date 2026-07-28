<script setup lang="ts">
import {
	CheckCircleIcon,
	FolderSearchIcon,
	RefreshCwIcon,
	SearchIcon,
	SpinnerIcon,
	XCircleIcon,
} from '@modrinth/assets'
import {
	ButtonStyled,
	Checkbox,
	defineMessages,
	injectNotificationManager,
	Slider,
	StyledInput,
	useVIntl,
} from '@modrinth/ui'
import { open } from '@tauri-apps/plugin-dialog'
import { computed, readonly, ref, watch } from 'vue'

import JavaDetectionModal from '@/components/ui/JavaDetectionModal.vue'
import MemoryAllocationDisplay from '@/components/ui/MemoryAllocationDisplay.vue'
import useJavaTest from '@/composables/useJavaTest'
import useMemorySlider from '@/composables/useMemorySlider'
import { trackEvent } from '@/helpers/analytics'
import { edit, get_optimal_jre_key } from '@/helpers/instance'
import { get_java_versions } from '@/helpers/jre'
import { get } from '@/helpers/settings'
import { injectInstanceSettings } from '@/providers/instance-settings'

import type { AppSettings } from '../../../helpers/types'

const { handleError } = injectNotificationManager()
const { formatMessage } = useVIntl()
const messages = defineMessages({
	javaInstallation: { id: 'instance.settings.tabs.java.java-installation', defaultMessage: 'Java installation' },
	autoLabel: { id: 'instance.settings.tabs.java.auto-label', defaultMessage: 'Auto (recommended)' },
	customLabel: { id: 'instance.settings.tabs.java.custom-label', defaultMessage: 'Custom path...' },
	javaPathPlaceholder: { id: 'instance.settings.tabs.java.java-path-placeholder', defaultMessage: '/path/to/java' },
	javaMemory: { id: 'instance.settings.tabs.java.java-memory', defaultMessage: 'Memory allocated' },
	customMemoryAllocation: { id: 'instance.settings.tabs.java.custom-memory-allocation', defaultMessage: 'Custom memory allocation' },
	automaticMemory: { id: 'instance.settings.tabs.java.automatic-memory', defaultMessage: 'Automatically allocate memory at launch' },
	javaArguments: { id: 'instance.settings.tabs.java.java-arguments', defaultMessage: 'Java arguments' },
	customJavaArguments: { id: 'instance.settings.tabs.java.custom-java-arguments', defaultMessage: 'Custom Java arguments' },
	enterJavaArguments: { id: 'instance.settings.tabs.java.enter-java-arguments', defaultMessage: 'Enter Java arguments...' },
	javaEnvironmentVariables: { id: 'instance.settings.tabs.java.environment-variables', defaultMessage: 'Environment variables' },
	customEnvironmentVariables: { id: 'instance.settings.tabs.java.custom-environment-variables', defaultMessage: 'Custom environment variables' },
	enterEnvironmentVariables: { id: 'instance.settings.tabs.java.enter-environment-variables', defaultMessage: 'Enter environmental variables...' },
	detect: { id: 'app.java.detect', defaultMessage: 'Detect' },
	browse: { id: 'app.java.browse', defaultMessage: 'Browse' },
})
const { instance } = injectInstanceSettings()

const globalSettings = (await get().catch(handleError)) as unknown as AppSettings
const optimalJava = readonly(await get_optimal_jre_key(instance.value.id).catch(handleError))
const allJavaVersions = ref(await get_java_versions().catch(handleError))

const SELECT_AUTO = '__auto__'
const SELECT_CUSTOM = '__custom__'

const selectedVersion = ref<string>(
	instance.value.java_path
		? (allJavaVersions.value?.some((j: any) => j.path === instance.value.java_path) ? instance.value.java_path : SELECT_CUSTOM)
		: SELECT_AUTO
)

const customPath = ref(instance.value.java_path ?? '')

const activePath = computed(() => {
	if (selectedVersion.value === SELECT_AUTO) return optimalJava?.path ?? ''
	if (selectedVersion.value === SELECT_CUSTOM) return customPath.value
	return selectedVersion.value
})

const effectiveJavaVersion = computed(() => {
	if (selectedVersion.value === SELECT_AUTO) return optimalJava?.parsed_version ?? 0
	if (selectedVersion.value === SELECT_CUSTOM) return optimalJava?.parsed_version ?? 0
	return allJavaVersions.value?.find((j: any) => j.path === selectedVersion.value)?.parsed_version ?? 0
})

const { testingJava, javaTestResult, testJavaInstallationDebounced, testJavaInstallation } =
	useJavaTest()

const hoveringTest = ref(false)
let hasInitialized = false

watch(activePath, (newPath) => {
	if (newPath && effectiveJavaVersion.value) {
		if (!hasInitialized) {
			testJavaInstallation(newPath, effectiveJavaVersion.value, false)
			hasInitialized = true
		} else {
			testJavaInstallationDebounced(newPath, effectiveJavaVersion.value)
		}
	}
}, { immediate: true })

const javaDetectionModal = ref<{ show: (version: number, current: object) => void } | null>(null)

async function handleBrowseJava() {
	const result = await open({ multiple: false })
	if (result) {
		selectedVersion.value = SELECT_CUSTOM
		customPath.value = result.path ?? result
		trackEvent('JavaManualSelect', { source: 'instance_settings' })
	}
}

function handleDetectJava() {
	javaDetectionModal.value?.show(effectiveJavaVersion.value, { path: activePath.value })
}

const tableData = computed(() => {
	const rows: any[] = []

	rows.push({
		_select: SELECT_AUTO,
		parsed_version: optimalJava?.parsed_version ?? null,
		distribution: formatMessage(messages.autoLabel),
		path: optimalJava?.path ?? '',
		_isAuto: true,
	})

	const seenVersions = new Set<number>()
	if (optimalJava?.parsed_version) seenVersions.add(optimalJava.parsed_version)

	if (allJavaVersions.value) {
		const sorted = [...allJavaVersions.value].sort((a: any, b: any) => b.parsed_version - a.parsed_version)
		for (const java of sorted) {
			const jv = java as any
			rows.push({
				_select: jv.path,
				parsed_version: jv.parsed_version,
				distribution: jv.distribution || null,
				path: jv.path || '',
				_isAuto: false,
			})
		}
	}

	rows.push({
		_select: SELECT_CUSTOM,
		parsed_version: null,
		distribution: formatMessage(messages.customLabel),
		path: selectedVersion.value === SELECT_CUSTOM ? customPath.value : '',
		_isCustom: true,
	})

	return rows
})

function onSelectRow(row: any) {
	if (row._isCustom) {
		selectedVersion.value = SELECT_CUSTOM
		if (!customPath.value) customPath.value = ''
	} else if (row._isAuto) {
		selectedVersion.value = SELECT_AUTO
	} else {
		selectedVersion.value = row._select
	}
}

const overrideJavaArgs = ref((instance.value.extra_launch_args?.length ?? 0) > 0)
const javaArgs = ref((instance.value.extra_launch_args ?? globalSettings?.extra_launch_args ?? []).join(' '))

const overrideEnvVars = ref((instance.value.custom_env_vars?.length ?? 0) > 0)
const envVars = ref((instance.value.custom_env_vars ?? globalSettings?.custom_env_vars ?? []).map((x: string[]) => x.join('=')).join(' '))

const overrideMemorySettings = ref(!!instance.value.memory)
const memory = ref(instance.value.memory ?? globalSettings?.memory ?? { maximum: 2048, automatic: true })
const effectiveMemory = computed(() => overrideMemorySettings.value ? memory.value : (globalSettings?.memory ?? { maximum: 2048, automatic: true }))
const memData = await useMemorySlider().catch(() => ({ maxMemory: ref(4096), snapPoints: computed(() => []) }))
const maxMemory = memData.maxMemory
const snapPoints = memData.snapPoints

const editInstanceObject = computed(() => ({
	java_path: selectedVersion.value === SELECT_AUTO
		? null
		: (activePath.value ? activePath.value.replace('java.exe', 'javaw.exe') : null),
	extra_launch_args: overrideJavaArgs.value ? javaArgs.value.trim().split(/\s+/).filter(Boolean) : null,
	custom_env_vars: overrideEnvVars.value ? envVars.value.trim().split(/\s+/).filter(Boolean).map((x: string) => x.split('=').filter(Boolean)) : null,
	memory: overrideMemorySettings.value ? memory.value : null,
}))

watch([selectedVersion, customPath, overrideJavaArgs, javaArgs, overrideEnvVars, envVars, overrideMemorySettings, memory], async () => {
	await edit(instance.value.id, editInstanceObject.value).catch(handleError)
}, { deep: true })

</script>

<template>
	<div>
		<JavaDetectionModal ref="javaDetectionModal" @submit="(val) => { selectedVersion = SELECT_CUSTOM; customPath = val.path }" />
		<h2 class="m-0 mb-3 text-base font-extrabold text-contrast block">
			{{ formatMessage(messages.javaInstallation) }}
		</h2>

		<div class="flex flex-col gap-1">
			<div
				v-for="row in tableData"
				:key="row._select"
				class="flex items-start gap-2.5 px-3 py-2 rounded-lg cursor-pointer transition-colors border-l-[3px]"
				:class="selectedVersion === row._select
					? 'border-accent bg-accent/5'
					: 'border-transparent hover:bg-button-bg'"
				@click="onSelectRow(row)"
			>
				<div class="flex items-center justify-center mt-0.5 shrink-0">
					<div
						role="radio"
						:aria-checked="selectedVersion === row._select"
						class="w-4 h-4 rounded-full border-2 flex items-center justify-center transition-colors"
						:class="selectedVersion === row._select
							? 'border-accent bg-accent'
							: 'border-button-border group-hover:border-accent'"
					>
						<div v-if="selectedVersion === row._select" class="w-2 h-2 rounded-full bg-white" />
					</div>
				</div>
				<div class="flex-1 min-w-0">
					<div class="flex items-center gap-2">
						<span
							v-if="row._isAuto"
							class="text-sm font-semibold text-accent"
						>
							{{ formatMessage(messages.autoLabel) }}
						</span>
						<span
							v-else-if="row._isCustom"
							class="text-sm font-semibold italic text-secondary"
						>
							{{ formatMessage(messages.customLabel) }}
						</span>
						<span v-else class="text-sm font-semibold tabular-nums">
							Java {{ row.parsed_version }}
						</span>
						<span v-if="!row._isAuto && !row._isCustom && row.distribution" class="text-xs text-secondary">
							{{ row.distribution }}
						</span>
					</div>
					<div v-if="row._isAuto" class="text-xs text-secondary mt-0.5">
						Java {{ row.parsed_version }}
						<span v-if="row.distribution"> — {{ row.distribution }}</span>
					</div>
					<div v-else-if="row.path" v-tooltip="row.path" class="text-xs text-secondary font-mono truncate mt-0.5">
						{{ row.path }}
					</div>
				</div>
			</div>
		</div>

		<div v-if="selectedVersion === SELECT_CUSTOM" class="flex flex-col gap-2 p-2 bg-bg rounded-lg border border-button-border mt-2">
			<div class="flex gap-2 items-center">
				<StyledInput
					v-model="customPath"
					autocomplete="off"
					:placeholder="formatMessage(messages.javaPathPlaceholder)"
					wrapper-class="flex-1 min-w-0"
				/>
				<ButtonStyled
					:color="!hoveringTest && !testingJava ? (javaTestResult === true ? 'green' : 'red') : 'standard'"
					color-fill="text"
				>
					<button aria-label="Test Java path" :disabled="testingJava" @click="testJavaInstallation(activePath, effectiveJavaVersion, true)" @mouseenter="hoveringTest = true" @mouseleave="hoveringTest = false">
						<SpinnerIcon v-if="testingJava" class="animate-spin h-4 w-4" />
						<CheckCircleIcon v-else-if="javaTestResult === true && !hoveringTest" class="h-4 w-4" />
						<XCircleIcon v-else-if="javaTestResult !== true && !hoveringTest" class="h-4 w-4" />
						<RefreshCwIcon v-else class="h-4 w-4" />
					</button>
				</ButtonStyled>
			</div>
			<div class="flex gap-2">
				<ButtonStyled><button aria-label="Detect Java installations" @click="handleDetectJava"><SearchIcon /> {{ formatMessage(messages.detect) }}</button></ButtonStyled>
				<ButtonStyled><button @click="handleBrowseJava"><FolderSearchIcon /> {{ formatMessage(messages.browse) }}</button></ButtonStyled>
			</div>
		</div>

		<h2 class="mt-4 mb-1 text-base font-extrabold text-contrast block">{{ formatMessage(messages.javaMemory) }}</h2>
		<Checkbox v-model="overrideMemorySettings" :label="formatMessage(messages.customMemoryAllocation)" class="mb-2" />
		<Checkbox v-if="overrideMemorySettings" v-model="memory.automatic" :label="formatMessage(messages.automaticMemory)" class="mb-2" />
		<Slider id="max-memory" v-model="memory.maximum" :disabled="!overrideMemorySettings || memory.automatic" :min="512" :max="maxMemory" :step="64" :snap-points="snapPoints" :snap-range="512" unit="MB" />
		<MemoryAllocationDisplay :instance-id="instance.id" :memory="effectiveMemory" />
		<h2 class="mt-4 mb-1 text-base font-extrabold text-contrast block">{{ formatMessage(messages.javaArguments) }}</h2>
		<Checkbox v-model="overrideJavaArgs" :label="formatMessage(messages.customJavaArguments)" class="my-1" />
		<StyledInput id="java-args" v-model="javaArgs" autocomplete="off" :disabled="!overrideJavaArgs" :placeholder="formatMessage(messages.enterJavaArguments)" wrapper-class="w-full" />
		<h2 class="mt-4 mb-1 text-base font-extrabold text-contrast block">{{ formatMessage(messages.javaEnvironmentVariables) }}</h2>
		<Checkbox v-model="overrideEnvVars" :label="formatMessage(messages.customEnvironmentVariables)" class="mb-2" />
		<StyledInput id="env-vars" v-model="envVars" autocomplete="off" :disabled="!overrideEnvVars" :placeholder="formatMessage(messages.enterEnvironmentVariables)" wrapper-class="w-full" />
	</div>
</template>
