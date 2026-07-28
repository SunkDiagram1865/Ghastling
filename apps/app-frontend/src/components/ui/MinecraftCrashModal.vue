<script setup lang="ts">
import { DownloadIcon, ExternalIcon } from '@modrinth/assets'
import { Admonition, ButtonStyled, defineMessages, NewModal, useVIntl } from '@modrinth/ui'
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useRouter } from 'vue-router'

import {
	clearCrashAnalysis,
	type CrashAnalysisResult,
	refreshCrashAnalysis,
} from '@/composables/useCrashAnalysis'
import type { MinecraftLaunchErrorPayload } from '@/composables/useMinecraftLaunchError'
import { process_listener } from '@/helpers/events.js'
import { get as getInstance } from '@/helpers/instance'
import { export_crash_context } from '@/helpers/logs.js'

interface CrashModalPayload extends MinecraftLaunchErrorPayload {
	title?: string
	summary?: string
	body?: string
	hint?: string
}

interface ProcessEvent {
	instance_id: string
	uuid: string
	event: 'launched' | 'finished'
	crashed?: boolean
}

interface CrashWarningPayload extends MinecraftLaunchErrorPayload {
	kind: 'minecraft_crash'
}

type Unlisten = () => void

const emit = defineEmits<{
	error: [error: unknown]
}>()

const router = useRouter()
const { formatMessage } = useVIntl()
const modal = ref<InstanceType<typeof NewModal>>()
const payload = ref<Partial<CrashModalPayload>>({})
const preview = ref(false)
const exporting = ref(false)
const analyzing = ref(false)
const activeRuns = new Map<string, string>()
const lastShownAt = new Map<string, number>()
let unlistenProcess: Unlisten | undefined
let mounted = false
let analysisVersion = 0

const messages = defineMessages({
	title: {
		id: 'app.minecraft-crash.title',
		defaultMessage: '{instanceName} crashed',
	},
	body: {
		id: 'app.minecraft-crash.body',
		defaultMessage:
			'Do not send a screenshot of this window when asking for help. Export the error report instead so the crash report, game logs, debug log, and JVM details can be checked together.',
	},
	summary: {
		id: 'app.minecraft-crash.summary',
		defaultMessage: 'Minecraft stopped unexpectedly.',
	},
	supportHint: {
		id: 'app.minecraft-crash.support-hint',
		defaultMessage:
			'When asking for help, send the exported ZIP. Do not send only a screenshot of this window because it does not contain the diagnostic evidence.',
	},
	exportContext: {
		id: 'app.minecraft-crash.export-context',
		defaultMessage: 'Export Minecraft error report',
	},
	viewLogs: {
		id: 'app.minecraft-crash.view-logs',
		defaultMessage: 'View logs and analysis',
	},
	previewInstance: {
		id: 'app.minecraft-crash.preview-instance',
		defaultMessage: 'Minecraft test instance',
	},
	launchFailedTitle: {
		id: 'app.minecraft-crash.launch-failed-title',
		defaultMessage: '{instanceName} could not start',
	},
	launchFailedSummary: {
		id: 'app.minecraft-crash.launch-failed-summary',
		defaultMessage: 'Minecraft failed during launch preparation.',
	},
	exitedBeforeInitialization: {
		id: 'app.minecraft-crash.exited-before-initialization',
		defaultMessage:
			'The Java process exited before it could connect to the launcher. The selected Java version is probably incompatible with this Minecraft or Mod loader version. Select the Java version required by the instance, then try again.',
	},
	initializationTimedOut: {
		id: 'app.minecraft-crash.initialization-timed-out',
		defaultMessage:
			'The Java process started but did not connect to the launcher within 15 seconds. Check the selected Java version and any wrapper command, then try again.',
	},
	preparationTimedOut: {
		id: 'app.minecraft-crash.preparation-timed-out',
		defaultMessage:
			'Launch preparation did not finish within 60 seconds and was cancelled. Check the Java path, launch hooks, wrapper command, and network connection, then try again.',
	},
	launchFailureHint: {
		id: 'app.minecraft-crash.launch-failure-hint',
		defaultMessage:
			'Open the Minecraft logs to view the captured Java output. When asking for help, export and send the complete Minecraft diagnostic package.',
	},
	analyzing: {
		id: 'app.minecraft-crash.analyzing',
		defaultMessage: 'Analyzing the logs from this launch...',
	},
	evidence: {
		id: 'app.minecraft-crash.evidence',
		defaultMessage: 'Reference evidence: {evidence}',
	},
	jvmArgumentsTitle: {
		id: 'app.minecraft-crash.diagnosis.jvm-arguments.title',
		defaultMessage: 'Possible issue: the JVM arguments are invalid',
	},
	jvmArgumentsAction: {
		id: 'app.minecraft-crash.diagnosis.jvm-arguments.action',
		defaultMessage:
			'You can try removing the JVM argument shown below from the instance settings, then launch again.',
	},
	javaTooNewTitle: {
		id: 'app.minecraft-crash.diagnosis.java-too-new.title',
		defaultMessage: 'Possible issue: the selected Java version is too new',
	},
	javaTooNewAction: {
		id: 'app.minecraft-crash.diagnosis.java-too-new.action',
		defaultMessage:
			'You can try selecting the Java major version required by this Minecraft and Mod loader version, then launch again.',
	},
	javaIncompatibleTitle: {
		id: 'app.minecraft-crash.diagnosis.java-incompatible.title',
		defaultMessage: 'Possible issue: the Java version is incompatible',
	},
	javaIncompatibleAction: {
		id: 'app.minecraft-crash.diagnosis.java-incompatible.action',
		defaultMessage:
			'You can try selecting the Java version requested in the error below, or using a compatible build of the affected Mod.',
	},
	java32BitTitle: {
		id: 'app.minecraft-crash.diagnosis.java-32bit.title',
		defaultMessage: 'Possible issue: 32-bit Java cannot allocate enough memory',
	},
	java32BitAction: {
		id: 'app.minecraft-crash.diagnosis.java-32bit.action',
		defaultMessage:
			'You can try installing and selecting a 64-bit Java runtime, then launch again.',
	},
	java11RequiredTitle: {
		id: 'app.minecraft-crash.diagnosis.java-11-required.title',
		defaultMessage: 'Possible issue: a Mod requires Java 11',
	},
	java11RequiredAction: {
		id: 'app.minecraft-crash.diagnosis.java-11-required.action',
		defaultMessage:
			'You can try selecting Java 11, or installing a build of the affected Mod that supports the current Java version.',
	},
	openJ9Title: {
		id: 'app.minecraft-crash.diagnosis.openj9.title',
		defaultMessage: 'Possible issue: OpenJ9 is not compatible with this instance',
	},
	openJ9Action: {
		id: 'app.minecraft-crash.diagnosis.openj9.action',
		defaultMessage:
			'You can try selecting a HotSpot-based Java runtime, such as the bundled Minecraft runtime or Eclipse Temurin.',
	},
	jdkRuntimeTitle: {
		id: 'app.minecraft-crash.diagnosis.jdk-runtime.title',
		defaultMessage: 'Possible issue: the selected JDK is not compatible',
	},
	jdkRuntimeAction: {
		id: 'app.minecraft-crash.diagnosis.jdk-runtime.action',
		defaultMessage:
			'You can try selecting a standard HotSpot Java runtime for this Minecraft version.',
	},
	forgeJavaTitle: {
		id: 'app.minecraft-crash.diagnosis.forge-java.title',
		defaultMessage: 'Possible issue: Forge is not compatible with the selected Java version',
	},
	forgeJavaAction: {
		id: 'app.minecraft-crash.diagnosis.forge-java.action',
		defaultMessage:
			'You can try using the Java version expected by this Forge release, or updating Forge.',
	},
	outOfMemoryTitle: {
		id: 'app.minecraft-crash.diagnosis.out-of-memory.title',
		defaultMessage: 'Possible issue: Minecraft ran out of memory',
	},
	outOfMemoryAction: {
		id: 'app.minecraft-crash.diagnosis.out-of-memory.action',
		defaultMessage:
			'You can try increasing the instance memory allocation, or removing memory-heavy Mods and resource packs.',
	},
	knownFailureTitle: {
		id: 'app.minecraft-crash.diagnosis.known-failure.title',
		defaultMessage: 'Possible issue: a specific launch problem was detected',
	},
	knownFailureAction: {
		id: 'app.minecraft-crash.diagnosis.known-failure.action',
		defaultMessage:
			'This is an automatic guess, not a guaranteed diagnosis. Open the log analysis for the full context before applying the suggested fix.',
	},
})

const diagnosisMessages = {
	jvm_arguments: [messages.jvmArgumentsTitle, messages.jvmArgumentsAction],
	java_too_new: [messages.javaTooNewTitle, messages.javaTooNewAction],
	java_incompatible: [messages.javaIncompatibleTitle, messages.javaIncompatibleAction],
	java_32bit: [messages.java32BitTitle, messages.java32BitAction],
	java_11_required: [messages.java11RequiredTitle, messages.java11RequiredAction],
	openj9: [messages.openJ9Title, messages.openJ9Action],
	jdk_runtime: [messages.jdkRuntimeTitle, messages.jdkRuntimeAction],
	forge_java_incompatible: [messages.forgeJavaTitle, messages.forgeJavaAction],
	out_of_memory: [messages.outOfMemoryTitle, messages.outOfMemoryAction],
} as const

const title = computed(
	() =>
		payload.value.title ||
		formatMessage(messages.title, {
			instanceName: payload.value.instance_name || 'Minecraft',
		}),
)
const summary = computed(() => payload.value.summary || formatMessage(messages.summary))
const body = computed(() => payload.value.body || formatMessage(messages.body))
const hint = computed(() => payload.value.hint || formatMessage(messages.supportHint))
const showSupportHint = computed(() => hint.value !== formatMessage(messages.supportHint))

function applyAnalysis(
	modalPayload: CrashModalPayload,
	analysis: CrashAnalysisResult | null,
): CrashModalPayload {
	const finding = analysis?.findings[0]
	if (!finding) return modalPayload

	const [titleMessage, actionMessage] = diagnosisMessages[
		finding.id as keyof typeof diagnosisMessages
	] ?? [messages.knownFailureTitle, messages.knownFailureAction]
	const evidence = finding.evidence[0]
	return {
		...modalPayload,
		summary: formatMessage(titleMessage),
		body: formatMessage(actionMessage),
		hint: evidence
			? formatMessage(messages.evidence, {
					evidence: `${evidence.filename}:${evidence.line} - ${evidence.text}`,
				})
			: modalPayload.hint,
	}
}

function show(modalPayload: CrashModalPayload, isPreview = false): boolean {
	if (!isPreview) {
		const now = Date.now()
		const lastShown = lastShownAt.get(modalPayload.instance_id) ?? 0
		if (now - lastShown < 5000) return false
		lastShownAt.set(modalPayload.instance_id, now)
	}
	analysisVersion += 1
	payload.value = modalPayload
	preview.value = isPreview
	modal.value?.show()
	return true
}

function launchErrorText(error: unknown): string {
	if (typeof error === 'string') return error
	if (error && typeof error === 'object') {
		const record = error as Record<string, unknown>
		const values = [record.message, record.error, record.cause]
			.filter((value): value is string => typeof value === 'string')
			.join('\n')
		if (values) return values
		try {
			return JSON.stringify(error)
		} catch {
			return ''
		}
	}
	return String(error)
}

function launchFailureBody(error: unknown): string | null {
	const errorText = launchErrorText(error)
	if (errorText.includes('Minecraft exited before launcher initialization completed')) {
		return formatMessage(messages.exitedBeforeInitialization)
	}
	if (errorText.includes('Minecraft launcher initialization did not respond')) {
		return formatMessage(messages.initializationTimedOut)
	}
	if (errorText.includes('Minecraft launch preparation timed out')) {
		return formatMessage(messages.preparationTimedOut)
	}
	return null
}

function isLaunchFailure(error: unknown): boolean {
	return launchFailureBody(error) !== null
}

async function analyzeAndUpdate(
	modalPayload: CrashModalPayload,
	fallbackHint?: string,
): Promise<CrashAnalysisResult | null> {
	const version = analysisVersion
	analyzing.value = true
	try {
		const analysis = await refreshCrashAnalysis(modalPayload.instance_id).catch((error) => {
			console.error('Failed to analyze Minecraft crash', error)
			return null
		})
		if (mounted && version === analysisVersion) {
			payload.value = applyAnalysis(modalPayload, analysis)
			if (!analysis?.findings.length && fallbackHint) payload.value.hint = fallbackHint
		}
		return analysis
	} finally {
		if (mounted && version === analysisVersion) analyzing.value = false
	}
}

async function handleLaunchError(
	error: unknown,
	launchPayload: MinecraftLaunchErrorPayload,
): Promise<boolean> {
	const failureBody = launchFailureBody(error)
	if (!failureBody) return false

	const instanceName = launchPayload.instance_name || 'Minecraft'
	const modalPayload: CrashModalPayload = {
		...launchPayload,
		title: formatMessage(messages.launchFailedTitle, { instanceName }),
		summary: formatMessage(messages.launchFailedSummary),
		body: failureBody,
		hint: formatMessage(messages.analyzing),
	}
	if (!show(modalPayload)) return true
	await analyzeAndUpdate(modalPayload, formatMessage(messages.launchFailureHint))
	return true
}

async function handleWarning(warning: CrashWarningPayload): Promise<void> {
	const modalPayload = { ...warning, hint: formatMessage(messages.analyzing) }
	if (!show(modalPayload)) return
	await analyzeAndUpdate(modalPayload)
}

function showPreview(): void {
	show(
		{
			instance_id: 'preview',
			instance_name: formatMessage(messages.previewInstance),
		},
		true,
	)
}

async function exportContext(): Promise<void> {
	const instanceId = payload.value.instance_id
	if (preview.value || !instanceId || exporting.value) return

	exporting.value = true
	try {
		await export_crash_context(instanceId, payload.value.instance_name || 'Minecraft')
	} catch (error) {
		emit('error', error)
	} finally {
		exporting.value = false
	}
}

async function openLogs(): Promise<void> {
	const instanceId = payload.value.instance_id
	if (preview.value || !instanceId) return
	modal.value?.hide()
	await router.push(`/instance/${encodeURIComponent(instanceId)}/logs`)
}

async function handleProcessEvent(event: ProcessEvent): Promise<void> {
	if (event.event === 'launched') {
		activeRuns.set(event.instance_id, event.uuid)
		clearCrashAnalysis(event.instance_id)
		return
	}
	if (event.event !== 'finished' || activeRuns.get(event.instance_id) !== event.uuid) return

	await new Promise((resolve) => setTimeout(resolve, 2000))
	if (!mounted || activeRuns.get(event.instance_id) !== event.uuid) return

	try {
		const analysis = await refreshCrashAnalysis(event.instance_id).catch((error) => {
			console.error('Failed to analyze finished Minecraft process', error)
			return null
		})
		if (!mounted || (!event.crashed && !analysis?.crashed)) return

		const instance = await getInstance(event.instance_id).catch(() => null)
		if (!mounted) return
		show(
			applyAnalysis(
				{
					instance_id: event.instance_id,
					instance_name: instance?.name || 'Minecraft',
				},
				analysis,
			),
		)
	} finally {
		if (activeRuns.get(event.instance_id) === event.uuid) activeRuns.delete(event.instance_id)
	}
}

onMounted(async () => {
	mounted = true
	const unlisten = await process_listener((event: ProcessEvent) => void handleProcessEvent(event))
	if (!mounted) {
		unlisten()
		return
	}
	unlistenProcess = unlisten
})

onUnmounted(() => {
	mounted = false
	analysisVersion += 1
	activeRuns.clear()
	unlistenProcess?.()
})

defineExpose({ handleLaunchError, handleWarning, isLaunchFailure, showPreview })
</script>

<template>
	<NewModal ref="modal" :header="title" fade="danger" max-width="560px">
		<div class="flex flex-col gap-4">
			<Admonition type="critical" :header="summary">
				{{ body }}
			</Admonition>
			<p class="m-0 text-secondary">
				{{ hint }}
			</p>
			<p v-if="showSupportHint" class="m-0 text-secondary">
				{{ formatMessage(messages.supportHint) }}
			</p>
		</div>
		<template #actions>
			<div class="flex flex-wrap justify-end gap-2">
				<ButtonStyled type="outlined">
					<button :disabled="analyzing" @click="openLogs">
						<ExternalIcon />
						{{ formatMessage(messages.viewLogs) }}
					</button>
				</ButtonStyled>
				<ButtonStyled color="brand">
					<button :disabled="analyzing || exporting" @click="exportContext">
						<DownloadIcon />
						{{ formatMessage(messages.exportContext) }}
					</button>
				</ButtonStyled>
			</div>
		</template>
	</NewModal>
</template>
