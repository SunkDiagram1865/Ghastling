import { type Ref, ref } from 'vue'

import { analyze_crash } from '@/helpers/logs'

export interface CrashAnalysisEvidence {
	filename: string
	line: number
	text: string
}

export interface CrashAnalysisFinding {
	id: string
	confidence: 'high' | 'medium'
	evidence: CrashAnalysisEvidence[]
}

export interface CrashAnalysisMod {
	file_name: string
	id?: string
	name?: string
	matched_class?: string
}

export interface CrashAnalysisResult {
	ruleset: string
	crashed: boolean
	sources: Array<{
		filename: string
		source_type: string
		modified_at: number
		line_count: number
		content: string
	}>
	findings: CrashAnalysisFinding[]
	mods: CrashAnalysisMod[]
	combined_log: string
}

interface CrashAnalysisState {
	analysis: Ref<CrashAnalysisResult | null>
	loading: Ref<boolean>
	error: Ref<unknown>
	pending?: Promise<CrashAnalysisResult | null>
}

const states = new Map<string, CrashAnalysisState>()

function getState(instanceId: string): CrashAnalysisState {
	let state = states.get(instanceId)
	if (!state) {
		state = {
			analysis: ref(null),
			loading: ref(false),
			error: ref(null),
		}
		states.set(instanceId, state)
	}
	return state
}

export async function refreshCrashAnalysis(
	instanceId: string,
): Promise<CrashAnalysisResult | null> {
	const state = getState(instanceId)
	if (state.pending) return state.pending
	state.loading.value = true
	state.error.value = null
	state.pending = analyze_crash(instanceId)
		.then((analysis: CrashAnalysisResult) => {
			state.analysis.value = analysis
			return analysis
		})
		.catch((error) => {
			state.error.value = error
			throw error
		})
		.finally(() => {
			state.loading.value = false
			state.pending = undefined
		})
	return state.pending
}

export function clearCrashAnalysis(instanceId: string): void {
	const state = getState(instanceId)
	state.analysis.value = null
	state.error.value = null
}

export function useCrashAnalysis(instanceId: string) {
	const state = getState(instanceId)
	return {
		analysis: state.analysis,
		loading: state.loading,
		error: state.error,
		refresh: () => refreshCrashAnalysis(instanceId),
		clear: () => clearCrashAnalysis(instanceId),
	}
}
