import { invoke } from '@tauri-apps/api/core'

export type TerracottaStatus =
	| 'idle'
	| 'starting'
	| 'downloading'
	| 'waiting'
	| 'host_scanning'
	| 'host_starting'
	| 'host_ready'
	| 'guest_connecting'
	| 'guest_starting'
	| 'guest_ready'
	| 'error'
	| 'fatal'

export type TerracottaDownloadStage =
	| 'preparing'
	| 'downloading'
	| 'verifying'
	| 'extracting'
	| 'installing'
	| 'complete'

export type TerracottaErrorType = 'os' | 'network' | 'install' | 'terracotta' | 'unknown'

export interface TerracottaPlayer {
	machine_id: string
	name: string
	vendor: string
	kind: 'HOST' | 'GUEST' | 'UNKNOWN'
}

export interface TerracottaState {
	status: TerracottaStatus
	http_port: number | null
	room_code: string | null
	server_port: number | null
	players: TerracottaPlayer[]
	download_progress: number | null
	download_stage: TerracottaDownloadStage | null
	binary_installed: boolean
	error_type: TerracottaErrorType | null
	error_message: string | null
	profile_index: number | null
}

const command = (name: string) => `plugin:terracotta|${name}`

export const terracotta = {
	getState: () => invoke<TerracottaState>(command('terracotta_get_state')),
	getPlatformKey: () => invoke<string>(command('terracotta_get_platform_key')),
	getPlayerName: () => invoke<string>(command('terracotta_get_player_name')),
	start: () => invoke<void>(command('terracotta_start'), { autoDownload: true }),
	host: (playerName: string) =>
		invoke<void>(command('terracotta_host'), { playerName: playerName.trim() }),
	join: (playerName: string, roomCode: string) =>
		invoke<void>(command('terracotta_join'), {
			playerName: playerName.trim(),
			roomCode: roomCode.trim(),
		}),
	reset: () => invoke<void>(command('terracotta_reset')),
	stop: () => invoke<void>(command('terracotta_stop')),
	download: () => invoke<void>(command('terracotta_download')),
}
