import { invoke } from '@tauri-apps/api/core'

export type HongshiStatus =
	| 'unsupported'
	| 'idle'
	| 'waiting_for_port'
	| 'downloading'
	| 'selecting_node'
	| 'starting'
	| 'open'
	| 'closed'
	| 'error'

export type HongshiErrorType =
	| 'unsupported'
	| 'node_list'
	| 'node_unavailable'
	| 'invalid_port'
	| 'install'
	| 'kernel_start'
	| 'kernel_exit'
	| 'status_file'
	| 'unknown'

export interface HongshiNode {
	name: string
	address: string
	latency_ms: number | null
	reachable: boolean
	cached: boolean
}

export interface DetectedLanPort {
	instance_id: string
	instance_name: string
	process_id: string
	port: number
	detected_at: string
}

export interface HongshiState {
	supported: boolean
	status: HongshiStatus
	local_port: number | null
	node: HongshiNode | null
	public_address: string | null
	created_at: string | null
	last_exit_code: number | null
	error_type: HongshiErrorType | null
	error_message: string | null
	bound_instance_id: string | null
	port_changed: boolean
	binary_installed: boolean
	download_progress: number | null
}

export function validLocalPort(value: string): number | null {
	if (!/^\d+$/.test(value)) return null
	const port = Number(value)
	return Number.isInteger(port) && port >= 1 && port <= 65535 ? port : null
}

export function selectedDetectedInstance(current: string, ports: DetectedLanPort[]): string {
	if (current !== 'manual' && ports.some((entry) => entry.instance_id === current)) {
		return current
	}
	return ports.length === 1 ? ports[0].instance_id : 'manual'
}

export function selectedNodePreference(current: string, nodes: HongshiNode[]): string {
	return current === 'auto' || nodes.some((node) => node.name === current) ? current : 'auto'
}

const command = (name: string) => `plugin:hongshi|${name}`

export const hongshi = {
	getState: () => invoke<HongshiState>(command('hongshi_get_state')),
	getNodes: (forceRefresh = false) =>
		invoke<HongshiNode[]>(command('hongshi_get_nodes'), { forceRefresh }),
	getDetectedPorts: () => invoke<DetectedLanPort[]>(command('hongshi_get_detected_ports')),
	download: () => invoke<void>(command('hongshi_download')),
	host: (localPort: number, nodeName: string | null, instanceId: string | null) =>
		invoke<void>(command('hongshi_host'), {
			localPort,
			nodeName,
			instanceId,
		}),
	stop: () => invoke<void>(command('hongshi_stop')),
	reset: () => invoke<void>(command('hongshi_reset')),
	openLogs: () => invoke<void>(command('hongshi_open_logs')),
}
