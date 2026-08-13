<script setup lang="ts">
import {
	ArrowLeftIcon,
	BinaryIcon,
	CheckCircleIcon,
	DownloadIcon,
	GlobeIcon,
	LogInIcon,
	LogOutIcon,
	PlayIcon,
	RefreshCwIcon,
	ServerIcon,
	SpinnerIcon,
	UserIcon,
	UsersIcon,
} from '@modrinth/assets'
import {
	Admonition,
	ButtonStyled,
	Card,
	CopyCode,
	defineMessages,
	DropdownSelect,
	NavTabs,
	ProgressBar,
	StyledInput,
	TagItem,
	useVIntl,
} from '@modrinth/ui'
import { computed, ref, watch } from 'vue'

import hongshiIcon from '@/assets/multiplayer/hongshi.png'
import terracottaIcon from '@/assets/multiplayer/terracotta.png'
import { useHongshiSession } from '@/composables/useHongshiSession'
import { useTerracottaSession } from '@/composables/useTerracottaSession'
import {
	type DetectedLanPort,
	type HongshiNode,
	selectedDetectedInstance,
	selectedNodePreference,
	validLocalPort,
} from '@/helpers/hongshi'
import type { TerracottaPlayer, TerracottaStatus } from '@/helpers/terracotta'

const { formatMessage } = useVIntl()
const messages = defineMessages({
	title: { id: 'app.multiplayer.title', defaultMessage: 'Multiplayer' },
	host: { id: 'app.multiplayer.host', defaultMessage: 'Host' },
	join: { id: 'app.multiplayer.join', defaultMessage: 'Join' },
	hostDescription: {
		id: 'app.multiplayer.host-description',
		defaultMessage: 'Create a virtual LAN room so friends can connect directly to your game.',
	},
	lanHint: {
		id: 'app.multiplayer.lan-hint',
		defaultMessage:
			'Open your Minecraft world, then press Esc → Open to LAN → choose a port. Terracotta will detect it automatically.',
	},
	joinDescription: {
		id: 'app.multiplayer.join-description',
		defaultMessage: "Enter a room code to join a friend's virtual LAN room.",
	},
	playerName: {
		id: 'app.multiplayer.player-name',
		defaultMessage: 'Player name',
	},
	roomCode: {
		id: 'app.multiplayer.room-code',
		defaultMessage: 'Room code',
	},
	roomCodePlaceholder: {
		id: 'app.multiplayer.room-code-placeholder',
		defaultMessage: 'e.g. U/ABCD-EFGH-IJKL-MNOP',
	},
	startHosting: {
		id: 'app.multiplayer.start-hosting',
		defaultMessage: 'Start hosting',
	},
	joinRoom: {
		id: 'app.multiplayer.join-room',
		defaultMessage: 'Join room',
	},
	copyRoomCode: {
		id: 'app.multiplayer.copy-room-code',
		defaultMessage: 'Copy room code',
	},
	back: {
		id: 'app.multiplayer.back',
		defaultMessage: 'Back',
	},
	disconnect: {
		id: 'app.multiplayer.disconnect',
		defaultMessage: 'Disconnect',
	},
	statusIdle: {
		id: 'app.multiplayer.status.idle',
		defaultMessage: 'Not connected',
	},
	statusStarting: {
		id: 'app.multiplayer.status.starting',
		defaultMessage: 'Starting...',
	},
	statusWaiting: {
		id: 'app.multiplayer.status.waiting',
		defaultMessage: 'Waiting...',
	},
	statusHostScanning: {
		id: 'app.multiplayer.status.host-scanning',
		defaultMessage: 'Creating room...',
	},
	statusHostStarting: {
		id: 'app.multiplayer.status.host-starting',
		defaultMessage: 'Starting host...',
	},
	statusHostReady: {
		id: 'app.multiplayer.status.host-ready',
		defaultMessage: 'Room ready',
	},
	statusGuestConnecting: {
		id: 'app.multiplayer.status.guest-connecting',
		defaultMessage: 'Joining room...',
	},
	statusGuestStarting: {
		id: 'app.multiplayer.status.guest-starting',
		defaultMessage: 'Connecting as guest...',
	},
	statusGuestReady: {
		id: 'app.multiplayer.status.guest-ready',
		defaultMessage: 'Connected to room',
	},
	statusError: {
		id: 'app.multiplayer.status.error',
		defaultMessage: 'Error',
	},
	statusFatal: {
		id: 'app.multiplayer.status.fatal',
		defaultMessage: 'Fatal error',
	},
	statusDownloading: {
		id: 'app.multiplayer.status.downloading',
		defaultMessage: 'Downloading...',
	},
	players: {
		id: 'app.multiplayer.players',
		defaultMessage: 'Players',
	},
	playersInRoom: {
		id: 'app.multiplayer.players-in-room',
		defaultMessage: '{count} player(s) in room',
	},
	notRunning: {
		id: 'app.multiplayer.not-running',
		defaultMessage: 'Multiplayer service is not running. Start hosting or join a room to begin.',
	},
	notRunningTitle: {
		id: 'app.multiplayer.not-running-title',
		defaultMessage: 'Start a multiplayer session',
	},
	shareCode: {
		id: 'app.multiplayer.share-code',
		defaultMessage: 'Share this code with friends to let them join:',
	},
	serverAddress: {
		id: 'app.multiplayer.server-address',
		defaultMessage: 'Backup connection address',
	},
	hostLabel: {
		id: 'app.multiplayer.host-label',
		defaultMessage: 'Host',
	},
	guestLabel: {
		id: 'app.multiplayer.guest-label',
		defaultMessage: 'Guest',
	},
	unknownPlayerRole: {
		id: 'app.multiplayer.unknown-player-role',
		defaultMessage: 'Unknown role',
	},
	platformInfo: {
		id: 'app.multiplayer.platform-info',
		defaultMessage: 'Current platform: {platform}',
	},
	binaryNotFound: {
		id: 'app.multiplayer.binary-not-found',
		defaultMessage: 'Terracotta binary not found. Please download it and place it at:',
	},
	downloadTerracotta: {
		id: 'app.multiplayer.download-terracotta',
		defaultMessage: 'Download Terracotta',
	},
	retry: {
		id: 'app.multiplayer.retry',
		defaultMessage: 'Retry',
	},
	checkNetwork: {
		id: 'app.multiplayer.check-network',
		defaultMessage: 'Check your network connection',
	},
	downloadProgress: {
		id: 'app.multiplayer.download-progress',
		defaultMessage: 'Download progress',
	},
	verifying: {
		id: 'app.multiplayer.verifying',
		defaultMessage: 'Verifying...',
	},
	extracting: {
		id: 'app.multiplayer.extracting',
		defaultMessage: 'Extracting...',
	},
	installing: {
		id: 'app.multiplayer.installing',
		defaultMessage: 'Installing...',
	},
	connecting: {
		id: 'app.multiplayer.connecting',
		defaultMessage: 'Connecting...',
	},
	errorNetwork: {
		id: 'app.multiplayer.error.network',
		defaultMessage: 'Network error',
	},
	errorInstall: {
		id: 'app.multiplayer.error.install',
		defaultMessage: 'Installation error',
	},
	errorTerracotta: {
		id: 'app.multiplayer.error.terracotta',
		defaultMessage: 'Terracotta error',
	},
	errorUnknown: {
		id: 'app.multiplayer.error.unknown',
		defaultMessage: 'Unknown error',
	},
	errorOs: {
		id: 'app.multiplayer.error.os',
		defaultMessage: 'System error',
	},
	startTerracotta: {
		id: 'app.multiplayer.start-terracotta',
		defaultMessage: 'Start Terracotta',
	},
	exitTerracotta: {
		id: 'app.multiplayer.exit-terracotta',
		defaultMessage: 'Exit multiplayer',
	},
	startDescription: {
		id: 'app.multiplayer.start-description',
		defaultMessage: "Start the multiplayer service to host games or join friends' rooms.",
	},
	loading: {
		id: 'app.multiplayer.loading',
		defaultMessage: 'Initializing...',
	},
	noPlayers: {
		id: 'app.multiplayer.no-players',
		defaultMessage: 'No players in room',
	},
	poweredByTerracotta: {
		id: 'app.multiplayer.powered-by-terracotta',
		defaultMessage: '由 陶瓦联机 强力驱动',
	},
	poweredByHongshi: {
		id: 'app.multiplayer.powered-by-hongshi',
		defaultMessage: '由 红石联机 强力驱动',
	},
	hongshiTitle: {
		id: 'app.multiplayer.hongshi.title',
		defaultMessage: '开始红石联机',
	},
	hongshiNotRunning: {
		id: 'app.multiplayer.hongshi.not-running',
		defaultMessage: '联机服务未运行。请创建房间或加入房间以开始联机。',
	},
	hongshiStart: {
		id: 'app.multiplayer.hongshi.start',
		defaultMessage: '启动红石联机',
	},
	hongshiDownload: {
		id: 'app.multiplayer.hongshi.download',
		defaultMessage: '下载红石联机',
	},
	hongshiBinaryMissing: {
		id: 'app.multiplayer.hongshi.binary-missing',
		defaultMessage: '请先下载红石联机内核，然后创建房间。',
	},
	hongshiLocalPort: {
		id: 'app.multiplayer.hongshi.local-port',
		defaultMessage: '本地 Minecraft 端口',
	},
	hongshiDetectedPort: {
		id: 'app.multiplayer.hongshi.detected-port',
		defaultMessage: '{instance} — 端口 {port}',
	},
	hongshiManualPort: {
		id: 'app.multiplayer.hongshi.manual-port',
		defaultMessage: '手动输入端口',
	},
	hongshiPortHint: {
		id: 'app.multiplayer.hongshi.port-hint',
		defaultMessage: '在游戏中开放局域网，Ghastling 会自动检测端口；外部游戏可使用手动端口。',
	},
	hongshiNode: {
		id: 'app.multiplayer.hongshi.node',
		defaultMessage: '中继节点',
	},
	hongshiAutoNode: {
		id: 'app.multiplayer.hongshi.node-auto',
		defaultMessage: '自动 — 最低延迟',
	},
	hongshiNodeLabel: {
		id: 'app.multiplayer.hongshi.node-label',
		defaultMessage: '{name} — {latency} 毫秒{cached}',
	},
	hongshiCachedNode: {
		id: 'app.multiplayer.hongshi.cached',
		defaultMessage: '（缓存）',
	},
	hongshiUnreachableNode: {
		id: 'app.multiplayer.hongshi.unreachable',
		defaultMessage: '不可达',
	},
	hongshiRefreshNodes: {
		id: 'app.multiplayer.hongshi.refresh-nodes',
		defaultMessage: '刷新节点',
	},
	hongshiCreateTunnel: {
		id: 'app.multiplayer.hongshi.create',
		defaultMessage: '创建公共房间',
	},
	hongshiCreatingTunnel: {
		id: 'app.multiplayer.hongshi.creating',
		defaultMessage: '正在创建隧道...',
	},
	hongshiSelectingNode: {
		id: 'app.multiplayer.hongshi.selecting-node',
		defaultMessage: '正在选择最佳中继节点...',
	},
	hongshiPublicAddress: {
		id: 'app.multiplayer.hongshi.public-address',
		defaultMessage: '公共地址',
	},
	hongshiPublicAddressHint: {
		id: 'app.multiplayer.hongshi.public-address-hint',
		defaultMessage: '好友可以在 Minecraft 中直接输入此地址，无需安装红石联机。',
	},
	hongshiLimits: {
		id: 'app.multiplayer.hongshi.limits',
		defaultMessage:
			'隧道在无玩家 10 分钟或总计 6 小时后关闭。最多 10 名玩家，共享 10 Mbps 带宽。',
	},
	hongshiPortChanged: {
		id: 'app.multiplayer.hongshi.port-changed',
		defaultMessage: 'Minecraft 开放了不同的局域网端口，请重启隧道后再分享地址。',
	},
	hongshiRestartTunnel: {
		id: 'app.multiplayer.hongshi.restart',
		defaultMessage: '重启隧道',
	},
	hongshiOpenLogs: {
		id: 'app.multiplayer.hongshi.open-logs',
		defaultMessage: '打开红石联机日志',
	},
	hongshiClosedTunnel: {
		id: 'app.multiplayer.hongshi.closed',
		defaultMessage: '红石联机房间已关闭，请创建新房间以获取新地址。',
	},
	hongshiUnsupported: {
		id: 'app.multiplayer.hongshi.unsupported',
		defaultMessage: '红石联机不支持当前操作系统或架构。',
	},
})

const tabIndex = ref(0)
const currentView = ref<'home' | 'hongshi'>('home')
const {
	download: downloadTerracotta,
	host: hostGame,
	isActionPending,
	join: joinGame,
	platformKey,
	playerName,
	reset: resetState,
	roomCodeInput,
	start: startTerracotta,
	state,
	stop: stopTerracotta,
} = useTerracottaSession()
const {
	detectedPorts,
	download: downloadHongshi,
	host: hostHongshi,
	isActionPending: isHongshiActionPending,
	isNodesLoading,
	nodes,
	openLogs: openHongshiLogs,
	refreshNodes,
	reset: resetHongshi,
	state: hongshiState,
	stop: stopHongshi,
} = useHongshiSession()

const nodeStorageKey = 'ghastling-hongshi-node'
const selectedNodeName = ref(localStorage.getItem(nodeStorageKey) ?? 'auto')
const selectedInstanceId = ref('manual')
const manualPort = ref('25565')
const hasLoadedNodes = ref(false)

const hongshiSupported = computed(() => hongshiState.value?.supported ?? false)
const detectedPortOptions = computed(() => [
	'manual',
	...detectedPorts.value.map((entry) => entry.instance_id),
])
const nodeOptions = computed(() => ['auto', ...nodes.value.map((node) => node.name)])
const selectedDetectedPort = computed(() =>
	detectedPorts.value.find((entry) => entry.instance_id === selectedInstanceId.value),
)
const effectiveLocalPort = computed(() => {
	if (selectedDetectedPort.value) return selectedDetectedPort.value.port
	return validLocalPort(manualPort.value)
})
const isHongshiBusy = computed(() =>
	['downloading', 'selecting_node', 'starting', 'waiting_for_port'].includes(
		hongshiState.value?.status ?? '',
	),
)
const selectedNode = computed(() =>
	selectedNodeName.value === 'auto'
		? null
		: (nodes.value.find((node) => node.name === selectedNodeName.value) ?? null),
)

watch(
	detectedPorts,
	(ports) => {
		selectedInstanceId.value = selectedDetectedInstance(selectedInstanceId.value, ports)
	},
	{ immediate: true },
)

watch(nodes, (value) => {
	selectedNodeName.value = selectedNodePreference(selectedNodeName.value, value)
	if (value.length > 0) hasLoadedNodes.value = true
})

watch(selectedNodeName, (value) => localStorage.setItem(nodeStorageKey, value))

watch(currentView, (view) => {
	if (view === 'hongshi' && hongshiSupported.value && !hasLoadedNodes.value) {
		void refreshNodes()
	}
})

function detectedPortLabel(value: string) {
	if (value === 'manual') return formatMessage(messages.hongshiManualPort)
	const entry = detectedPorts.value.find((port) => port.instance_id === value)
	return entry
		? formatMessage(messages.hongshiDetectedPort, {
				instance: entry.instance_name,
				port: entry.port,
			})
		: value
}

function nodeOptionLabel(value: string) {
	if (value === 'auto') return formatMessage(messages.hongshiAutoNode)
	const node = nodes.value.find((entry) => entry.name === value)
	if (!node) return value
	if (!node.reachable) {
		return `${node.name} — ${formatMessage(messages.hongshiUnreachableNode)}${
			node.cached ? formatMessage(messages.hongshiCachedNode) : ''
		}`
	}
	return formatMessage(messages.hongshiNodeLabel, {
		name: node.name,
		latency: node.latency_ms,
		cached: node.cached ? formatMessage(messages.hongshiCachedNode) : '',
	})
}

async function startHongshiTunnel() {
	if (!effectiveLocalPort.value) return
	await hostHongshi(
		effectiveLocalPort.value,
		selectedNodeName.value === 'auto' ? null : selectedNodeName.value,
		selectedInstanceId.value === 'manual' ? null : selectedInstanceId.value,
	)
}

async function restartHongshiTunnel() {
	if (!(await stopHongshi())) return
	await startHongshiTunnel()
}

function exitHongshiView() {
	currentView.value = 'home'
}

const tabLinks = computed(() => [
	{ label: formatMessage(messages.host), href: 'host', icon: UsersIcon },
	{ label: formatMessage(messages.join), href: 'join', icon: LogInIcon },
])

const isRunning = computed(() => !!state.value?.http_port)
const isSessionReady = computed(
	() => state.value?.status === 'host_ready' || state.value?.status === 'guest_ready',
)
const isHostSession = computed(() => state.value?.status === 'host_ready')
const canSubmitSession = computed(
	() =>
		playerName.value.trim().length > 0 &&
		(tabIndex.value === 0 || roomCodeInput.value.trim().length > 0),
)
const guestServerAddress = computed(() =>
	state.value?.server_port ? `127.0.0.1:${state.value.server_port}` : '',
)

const statusText = computed(() => {
	if (!state.value) return ''
	const statusMap = {
		idle: messages.statusIdle,
		starting: messages.statusStarting,
		waiting: messages.statusWaiting,
		host_scanning: messages.statusHostScanning,
		host_starting: messages.statusHostStarting,
		host_ready: messages.statusHostReady,
		guest_connecting: messages.statusGuestConnecting,
		guest_starting: messages.statusGuestStarting,
		guest_ready: messages.statusGuestReady,
		error: messages.statusError,
		fatal: messages.statusFatal,
		downloading: messages.statusDownloading,
	} satisfies Record<TerracottaStatus, (typeof messages)[keyof typeof messages]>
	return formatMessage(statusMap[state.value.status])
})

const playerCount = computed(() => state.value?.players?.length ?? 0)

function playerRoleMessage(kind: TerracottaPlayer['kind']) {
	if (kind === 'HOST') return messages.hostLabel
	if (kind === 'GUEST') return messages.guestLabel
	return messages.unknownPlayerRole
}

const binaryPathHint = computed(() => {
	const name = platformKey.value?.includes('windows') ? 'terracotta.exe' : 'terracotta'
	return `com.cysunk.ghestling/terracotta/${name}`
})

const downloadStageText = computed(() => {
	if (state.value?.download_stage) {
		if (state.value.download_stage === 'downloading')
			return formatMessage(messages.downloadProgress)
		if (state.value.download_stage === 'verifying') return formatMessage(messages.verifying)
		if (state.value.download_stage === 'extracting') return formatMessage(messages.extracting)
		if (state.value.download_stage === 'installing') return formatMessage(messages.installing)
		if (state.value.download_stage === 'complete') return ''
		if (state.value.download_stage === 'preparing') return formatMessage(messages.connecting)
	}
	if (state.value?.status === 'downloading') {
		if (state.value.download_progress === null || state.value.download_progress === 0)
			return formatMessage(messages.connecting)
		if (state.value.download_progress! < 100) return formatMessage(messages.downloadProgress)
		return formatMessage(messages.verifying)
	}
	return ''
})

const errorTypeLabel = computed(() => {
	const et = state.value?.error_type
	switch (et) {
		case 'network':
			return formatMessage(messages.errorNetwork)
		case 'install':
			return formatMessage(messages.errorInstall)
		case 'terracotta':
			return formatMessage(messages.errorTerracotta)
		case 'os':
			return formatMessage(messages.errorOs)
		default:
			return formatMessage(messages.errorUnknown)
	}
})

const isRecoverable = computed(() => {
	const et = state.value?.error_type
	if (!et) return state.value?.status === 'error'
	return et !== 'os'
})
</script>

<template>
	<div class="box-border flex min-h-full w-full flex-col gap-3 p-6">
		<h1 class="m-0 text-2xl font-semibold text-contrast">
			{{ formatMessage(messages.title) }}
		</h1>

		<template v-if="currentView === 'hongshi'">
			<Card v-if="!hongshiState" class="!m-0">
				<div class="flex items-center gap-3">
					<SpinnerIcon class="size-8 animate-spin text-brand" />
					<h2 class="m-0 text-lg font-semibold text-contrast">
						{{ formatMessage(messages.loading) }}
					</h2>
				</div>
			</Card>

			<Card v-else-if="!hongshiSupported" class="!m-0">
				<Admonition type="warning" :header="formatMessage(messages.hongshiTitle)">
					{{ formatMessage(messages.hongshiUnsupported) }}
				</Admonition>
				<div class="mt-4 flex flex-wrap gap-2">
					<ButtonStyled type="outlined">
						<button type="button" @click="exitHongshiView">
							<ArrowLeftIcon />
							{{ formatMessage(messages.back) }}
						</button>
					</ButtonStyled>
				</div>
			</Card>

			<Card v-else-if="!hongshiState.binary_installed" class="!m-0">
				<div class="flex flex-col gap-5">
					<div class="flex items-start gap-3">
						<img :src="hongshiIcon" class="size-10 shrink-0 rounded-xl" alt="Hongshi" />
						<div class="min-w-0 flex-1">
							<h2 class="m-0 text-lg font-semibold text-contrast">
								{{ formatMessage(messages.hongshiDownload) }}
							</h2>
							<p class="mb-0 mt-1 text-secondary">
								{{ formatMessage(messages.hongshiBinaryMissing) }}
							</p>
						</div>
					</div>

					<ProgressBar
						v-if="hongshiState.status === 'downloading'"
						full-width
						:progress="hongshiState.download_progress ?? 0"
						:max="100"
						:waiting="
							hongshiState.download_progress === null || hongshiState.download_progress === 0
						"
						:label="formatMessage(messages.statusDownloading)"
						show-progress
					/>

					<div v-else class="flex flex-wrap gap-2">
						<ButtonStyled color="brand">
							<button type="button" :disabled="isHongshiActionPending" @click="downloadHongshi">
								<DownloadIcon />
								{{ formatMessage(messages.hongshiDownload) }}
							</button>
						</ButtonStyled>
						<ButtonStyled type="outlined">
							<button type="button" @click="exitHongshiView">
								<ArrowLeftIcon />
								{{ formatMessage(messages.back) }}
							</button>
						</ButtonStyled>
					</div>
				</div>
			</Card>

			<Card v-else-if="hongshiState.status === 'open'" class="!m-0">
				<div class="flex flex-col gap-5">
					<div class="flex flex-wrap items-start justify-between gap-3">
						<div class="flex items-center gap-3">
							<CheckCircleIcon class="size-7 shrink-0 text-green" />
							<div>
								<h2 class="m-0 text-lg font-semibold text-contrast">
									{{ formatMessage(messages.statusHostReady) }}
								</h2>
								<p class="mb-0 mt-1 text-sm text-secondary">
									{{ formatMessage(messages.hongshiPublicAddressHint) }}
								</p>
							</div>
						</div>
						<TagItem v-if="hongshiState.node">
							<ServerIcon />
							{{ hongshiState.node.name }}
						</TagItem>
					</div>

					<div
						v-if="hongshiState.public_address"
						class="flex flex-wrap items-center justify-between gap-3 rounded-xl bg-surface-2 p-4"
					>
						<div class="min-w-0">
							<div class="font-semibold text-contrast">
								{{ formatMessage(messages.hongshiPublicAddress) }}
							</div>
							<div class="mt-1 text-sm text-secondary">
								{{ hongshiState.node?.name }} · 127.0.0.1:{{ hongshiState.local_port }}
							</div>
						</div>
						<CopyCode :text="hongshiState.public_address" />
					</div>

					<Admonition
						v-if="hongshiState.port_changed"
						type="warning"
						:header="formatMessage(messages.hongshiLocalPort)"
					>
						{{ formatMessage(messages.hongshiPortChanged) }}
						<template #actions>
							<ButtonStyled color="orange">
								<button
									type="button"
									:disabled="isHongshiActionPending"
									@click="restartHongshiTunnel"
								>
									<RefreshCwIcon />
									{{ formatMessage(messages.hongshiRestartTunnel) }}
								</button>
							</ButtonStyled>
						</template>
					</Admonition>

					<Admonition type="info" :header="formatMessage(messages.hongshiTitle)">
						{{ formatMessage(messages.hongshiLimits) }}
					</Admonition>

					<div class="flex flex-wrap gap-2">
						<ButtonStyled color="red" type="outlined">
							<button type="button" :disabled="isHongshiActionPending" @click="stopHongshi">
								<LogOutIcon />
								{{ formatMessage(messages.disconnect) }}
							</button>
						</ButtonStyled>
						<ButtonStyled type="outlined">
							<button type="button" :disabled="isHongshiActionPending" @click="openHongshiLogs">
								<BinaryIcon />
								{{ formatMessage(messages.hongshiOpenLogs) }}
							</button>
						</ButtonStyled>
						<ButtonStyled type="outlined">
							<button type="button" @click="exitHongshiView">
								<ArrowLeftIcon />
								{{ formatMessage(messages.back) }}
							</button>
						</ButtonStyled>
					</div>
				</div>
			</Card>

			<Card v-else-if="isHongshiBusy" class="!m-0">
				<div class="flex flex-col gap-5">
					<div class="flex items-center gap-3">
						<SpinnerIcon class="size-6 shrink-0 animate-spin text-orange" />
						<h2 class="m-0 text-lg font-semibold text-contrast">
							{{
								formatMessage(
									hongshiState.status === 'selecting_node'
										? messages.hongshiSelectingNode
										: hongshiState.status === 'downloading'
											? messages.statusDownloading
											: messages.hongshiCreatingTunnel,
								)
							}}
						</h2>
					</div>
					<ProgressBar
						v-if="hongshiState.status === 'downloading'"
						full-width
						:progress="hongshiState.download_progress ?? 0"
						:max="100"
						:waiting="
							hongshiState.download_progress === null || hongshiState.download_progress === 0
						"
						:label="formatMessage(messages.statusDownloading)"
						show-progress
					/>
					<div class="flex flex-wrap gap-2">
						<ButtonStyled color="red" type="outlined">
							<button type="button" :disabled="isHongshiActionPending" @click="stopHongshi">
								<LogOutIcon />
								{{ formatMessage(messages.disconnect) }}
							</button>
						</ButtonStyled>
					</div>
				</div>
			</Card>

			<Card v-else class="!m-0">
				<div class="flex flex-col gap-5">
					<Admonition
						v-if="hongshiState.status === 'error'"
						type="critical"
						:header="formatMessage(messages.errorNetwork)"
					>
						{{ hongshiState.error_message || formatMessage(messages.checkNetwork) }}
						<template #actions>
							<ButtonStyled type="outlined">
								<button type="button" @click="openHongshiLogs">
									<BinaryIcon />
									{{ formatMessage(messages.hongshiOpenLogs) }}
								</button>
							</ButtonStyled>
						</template>
					</Admonition>

					<Admonition
						v-else-if="hongshiState.status === 'closed'"
						type="warning"
						:header="formatMessage(messages.statusIdle)"
					>
						{{ formatMessage(messages.hongshiClosedTunnel) }}
					</Admonition>

					<div>
						<h2 class="m-0 text-lg font-semibold text-contrast">
							{{ formatMessage(messages.hongshiTitle) }}
						</h2>
						<p class="mb-0 mt-1 text-secondary">
							{{ formatMessage(messages.hongshiPortHint) }}
						</p>
					</div>

					<div class="grid gap-4 md:grid-cols-2">
						<div class="flex min-w-0 flex-col gap-2">
							<span class="font-semibold text-contrast">{{
								formatMessage(messages.hongshiLocalPort)
							}}</span>
							<DropdownSelect
								v-model="selectedInstanceId"
								class="!w-full"
								:options="detectedPortOptions"
								:display-name="detectedPortLabel"
								name="Hongshi local port source"
							/>
						</div>

						<div class="flex min-w-0 flex-col gap-2">
							<span class="font-semibold text-contrast">{{
								formatMessage(messages.hongshiNode)
							}}</span>
							<DropdownSelect
								v-model="selectedNodeName"
								class="!w-full"
								:options="nodeOptions"
								:display-name="nodeOptionLabel"
								name="Hongshi relay node"
							/>
						</div>

						<label
							v-if="selectedInstanceId === 'manual'"
							class="flex min-w-0 flex-col gap-2"
							for="hongshi-local-port"
						>
							<span class="font-semibold text-contrast">{{
								formatMessage(messages.hongshiManualPort)
							}}</span>
							<StyledInput
								id="hongshi-local-port"
								v-model="manualPort"
								:icon="ServerIcon"
								inputmode="numeric"
								placeholder="25565"
							/>
						</label>
					</div>

					<Admonition type="info" :header="formatMessage(messages.hongshiTitle)">
						{{ formatMessage(messages.hongshiLimits) }}
					</Admonition>

					<div class="flex flex-wrap gap-2">
						<ButtonStyled color="brand">
							<button
								type="button"
								:disabled="
									!effectiveLocalPort ||
									nodes.length === 0 ||
									isHongshiActionPending ||
									isNodesLoading ||
									(selectedNode && !selectedNode.reachable)
								"
								@click="startHongshiTunnel"
							>
								<GlobeIcon />
								{{ formatMessage(messages.hongshiCreateTunnel) }}
							</button>
						</ButtonStyled>
						<ButtonStyled type="outlined">
							<button type="button" :disabled="isNodesLoading" @click="refreshNodes(true)">
								<RefreshCwIcon :class="{ 'animate-spin': isNodesLoading }" />
								{{ formatMessage(messages.hongshiRefreshNodes) }}
							</button>
						</ButtonStyled>
						<ButtonStyled type="outlined">
							<button type="button" @click="exitHongshiView">
								<ArrowLeftIcon />
								{{ formatMessage(messages.back) }}
							</button>
						</ButtonStyled>
					</div>
				</div>
			</Card>

		<div class="mt-auto pt-6 text-center text-xs text-secondary">
			{{ formatMessage(messages.poweredByHongshi) }}
		</div>
	</template>

		<template v-else>
		<Card v-if="!state" class="!m-0">
			<div class="flex items-center gap-3">
				<SpinnerIcon class="size-8 animate-spin text-brand" />
				<h2 class="m-0 text-lg font-semibold text-contrast">
					{{ formatMessage(messages.loading) }}
				</h2>
			</div>
		</Card>

		<Card v-else-if="!state.binary_installed" class="!m-0">
			<div class="flex flex-col gap-5">
				<div class="flex items-start gap-3">
					<img :src="terracottaIcon" class="size-10 shrink-0 rounded-xl" alt="Terracotta" />
					<div class="min-w-0">
					<h2 class="m-0 text-lg font-semibold text-contrast">
						{{ formatMessage(messages.downloadTerracotta) }}
					</h2>
					<p class="mb-0 mt-1 text-secondary">
						{{ formatMessage(messages.notRunning) }}
					</p>
				</div>
			</div>

				<ProgressBar
					v-if="state.status === 'downloading'"
					full-width
					:progress="state.download_progress ?? 0"
					:max="100"
					:waiting="state.download_progress === null || state.download_progress === 0"
					:label="downloadStageText || statusText"
					show-progress
				/>

				<div v-else class="flex flex-wrap gap-2">
					<ButtonStyled color="brand">
						<button type="button" :disabled="isActionPending" @click="downloadTerracotta">
							<DownloadIcon />
							{{ formatMessage(messages.downloadTerracotta) }}
						</button>
					</ButtonStyled>
				</div>
			</div>
		</Card>

		<Card v-else-if="state.status === 'starting' || state.status === 'downloading'" class="!m-0">
			<div class="flex flex-col gap-5">
				<div class="flex items-center gap-3">
					<SpinnerIcon class="size-6 shrink-0 animate-spin text-orange" />
					<h2 class="m-0 text-lg font-semibold text-contrast">{{ statusText }}</h2>
				</div>
				<ProgressBar
					v-if="state.status === 'downloading'"
					full-width
					:progress="state.download_progress ?? 0"
					:max="100"
					:waiting="state.download_progress === null"
					:label="downloadStageText"
					show-progress
				/>
			</div>
		</Card>

		<Card
			v-else-if="isRunning && (state.status === 'idle' || state.status === 'waiting')"
			class="!m-0"
		>
			<div class="flex flex-col gap-5">
				<div class="flex flex-wrap items-center justify-between gap-3">
					<NavTabs
						mode="local"
						:active-index="tabIndex"
						:links="tabLinks"
						@tab-click="tabIndex = $event"
					/>
					<ButtonStyled color="red" type="outlined">
						<button type="button" :disabled="isActionPending" @click="stopTerracotta">
							<LogOutIcon />
							{{ formatMessage(messages.exitTerracotta) }}
						</button>
					</ButtonStyled>
				</div>

				<div>
					<h2 class="m-0 text-lg font-semibold text-contrast">
						{{ formatMessage(tabIndex === 0 ? messages.host : messages.join) }}
					</h2>
					<p class="mb-0 mt-1 text-secondary">
						{{
							formatMessage(tabIndex === 0 ? messages.hostDescription : messages.joinDescription)
						}}
					</p>
				</div>

				<div class="grid gap-4 md:grid-cols-2">
					<label class="flex min-w-0 flex-col gap-2" for="multiplayer-player-name">
						<span class="font-semibold text-contrast">
							{{ formatMessage(messages.playerName) }}
						</span>
						<StyledInput
							id="multiplayer-player-name"
							v-model="playerName"
							:icon="UserIcon"
							:placeholder="formatMessage(messages.playerName)"
							autocomplete="off"
						/>
					</label>

					<label
						v-if="tabIndex === 1"
						class="flex min-w-0 flex-col gap-2"
						for="multiplayer-room-code"
					>
						<span class="font-semibold text-contrast">
							{{ formatMessage(messages.roomCode) }}
						</span>
						<StyledInput
							id="multiplayer-room-code"
							v-model="roomCodeInput"
							:icon="UsersIcon"
							:placeholder="formatMessage(messages.roomCodePlaceholder)"
							autocomplete="off"
							:spellcheck="false"
						/>
					</label>
				</div>

				<div class="flex flex-wrap gap-2">
					<ButtonStyled color="brand">
						<button
							v-if="tabIndex === 0"
							type="button"
							:disabled="!canSubmitSession || isActionPending"
							@click="hostGame"
						>
							<PlayIcon />
							{{ formatMessage(messages.startHosting) }}
						</button>
						<button
							v-else
							type="button"
							:disabled="!canSubmitSession || isActionPending"
							@click="joinGame"
						>
							<LogInIcon />
							{{ formatMessage(messages.joinRoom) }}
						</button>
					</ButtonStyled>
				</div>
			</div>
		</Card>

		<Card
			v-else-if="state.status === 'host_scanning' || state.status === 'host_starting'"
			class="!m-0"
		>
			<div class="flex flex-col gap-5">
				<div class="flex items-center gap-3">
					<SpinnerIcon class="size-6 shrink-0 animate-spin text-orange" />
					<h2 class="m-0 text-lg font-semibold text-contrast">{{ statusText }}</h2>
				</div>
				<Admonition type="info" :header="formatMessage(messages.host)">
					{{ formatMessage(messages.lanHint) }}
				</Admonition>
				<div class="flex flex-wrap gap-2">
					<ButtonStyled type="outlined">
						<button type="button" :disabled="isActionPending" @click="resetState">
							<ArrowLeftIcon />
							{{ formatMessage(messages.back) }}
						</button>
					</ButtonStyled>
				</div>
			</div>
		</Card>

		<Card v-else-if="isSessionReady" class="!m-0">
			<div class="flex flex-col gap-5">
				<div class="flex flex-wrap items-start justify-between gap-3">
					<div class="flex items-center gap-3">
						<CheckCircleIcon class="size-7 shrink-0 text-green" />
						<div>
							<h2 class="m-0 text-lg font-semibold text-contrast">{{ statusText }}</h2>
							<p class="mb-0 mt-1 text-sm text-secondary">
								{{ formatMessage(messages.playersInRoom, { count: playerCount }) }}
							</p>
						</div>
					</div>
					<TagItem>
						<UsersIcon v-if="isHostSession" />
						<LogInIcon v-else />
						{{ formatMessage(isHostSession ? messages.hostLabel : messages.guestLabel) }}
					</TagItem>
				</div>

				<div
					v-if="isHostSession && state.room_code"
					class="flex flex-wrap items-center justify-between gap-3 rounded-xl bg-surface-2 p-4"
				>
					<div class="min-w-0">
						<div class="font-semibold text-contrast">{{ formatMessage(messages.roomCode) }}</div>
						<div class="mt-1 text-sm text-secondary">
							{{ formatMessage(messages.shareCode) }}
						</div>
					</div>
					<CopyCode :text="state.room_code" />
				</div>

				<div
					v-if="!isHostSession && guestServerAddress"
					class="flex flex-wrap items-center justify-between gap-3 rounded-xl bg-surface-2 p-4"
				>
					<div class="min-w-0">
						<div class="font-semibold text-contrast">
							{{ formatMessage(messages.serverAddress) }}
						</div>
					</div>
					<CopyCode :text="guestServerAddress" />
				</div>

				<section class="flex flex-col gap-3">
					<div class="flex items-center justify-between gap-3">
						<h3 class="m-0 text-base font-semibold text-contrast">
							{{ formatMessage(messages.players) }}
						</h3>
						<TagItem>
							<UsersIcon />
							{{ playerCount }}
						</TagItem>
					</div>

					<div
						v-if="state.players.length > 0"
						class="overflow-hidden rounded-xl border border-solid border-surface-5"
					>
						<div
							v-for="(player, index) in state.players"
							:key="player.machine_id || index"
							class="flex min-w-0 items-center gap-3 border-0 border-b border-solid border-divider bg-surface-2 px-4 py-3 last:border-b-0"
						>
							<div
								class="flex size-9 shrink-0 items-center justify-center rounded-full bg-highlight-green text-green"
							>
								<UserIcon class="size-4" />
							</div>
							<span class="min-w-0 flex-1 truncate font-medium text-contrast">
								{{ player.name }}
							</span>
							<TagItem>
								{{ formatMessage(playerRoleMessage(player.kind)) }}
							</TagItem>
						</div>
					</div>
					<div
						v-else
						class="flex items-center gap-2 rounded-xl bg-surface-2 px-4 py-5 text-secondary"
					>
						<UsersIcon class="size-5" />
						{{ formatMessage(messages.noPlayers) }}
					</div>
				</section>

				<div class="flex flex-wrap gap-2">
					<ButtonStyled color="red" type="outlined">
						<button type="button" :disabled="isActionPending" @click="resetState">
							<LogOutIcon />
							{{ formatMessage(messages.disconnect) }}
						</button>
					</ButtonStyled>
				</div>
			</div>
		</Card>

		<Card
			v-else-if="state.status === 'guest_connecting' || state.status === 'guest_starting'"
			class="!m-0"
		>
			<div class="flex flex-col gap-5">
				<div class="flex items-center gap-3">
					<SpinnerIcon class="size-6 shrink-0 animate-spin text-orange" />
					<h2 class="m-0 text-lg font-semibold text-contrast">{{ statusText }}</h2>
				</div>
				<div class="flex flex-wrap gap-2">
					<ButtonStyled type="outlined">
						<button type="button" :disabled="isActionPending" @click="resetState">
							<ArrowLeftIcon />
							{{ formatMessage(messages.back) }}
						</button>
					</ButtonStyled>
				</div>
			</div>
		</Card>

		<Card v-else-if="state.status === 'error' || state.status === 'fatal'" class="!m-0">
			<Admonition type="critical" :header="errorTypeLabel">
				{{ state.error_message || formatMessage(messages.checkNetwork) }}
				<template v-if="isRecoverable" #actions>
					<ButtonStyled color="red" type="outlined">
						<button type="button" :disabled="isActionPending" @click="resetState">
							<RefreshCwIcon />
							{{ formatMessage(messages.retry) }}
						</button>
					</ButtonStyled>
				</template>
			</Admonition>
		</Card>

		<Card v-else-if="!isRunning" class="!m-0">
			<div class="flex items-start gap-3">
				<img :src="terracottaIcon" class="size-10 shrink-0 rounded-xl" alt="Terracotta" />
				<div class="min-w-0 flex-1">
					<h2 class="m-0 text-lg font-semibold text-contrast">
						{{ formatMessage(messages.notRunningTitle) }}
					</h2>
					<p class="mb-0 mt-1 text-secondary">
						{{ formatMessage(messages.notRunning) }}
					</p>
				</div>
				<ButtonStyled color="brand">
					<button type="button" :disabled="isActionPending" @click="startTerracotta">
						<PlayIcon />
						{{ formatMessage(messages.startTerracotta) }}
					</button>
				</ButtonStyled>
			</div>
		</Card>

		<Card v-if="!isRunning && hongshiState && hongshiState.binary_installed" class="!m-0">
			<div class="flex items-start gap-3">
				<img :src="hongshiIcon" class="size-10 shrink-0 rounded-xl" alt="Hongshi" />
				<div class="min-w-0 flex-1">
					<h2 class="m-0 text-lg font-semibold text-contrast">
						{{ formatMessage(messages.hongshiTitle) }}
					</h2>
					<p class="mb-0 mt-1 text-secondary">
						{{ formatMessage(messages.hongshiNotRunning) }}
					</p>
				</div>
				<ButtonStyled color="brand">
					<button type="button" @click="currentView = 'hongshi'">
						<PlayIcon />
						{{ formatMessage(messages.hongshiStart) }}
					</button>
				</ButtonStyled>
			</div>
		</Card>

		<Card v-else-if="!isRunning && hongshiState && hongshiState.status === 'downloading'" class="!m-0">
			<div class="flex flex-col gap-5">
				<div class="flex items-start gap-3">
					<img :src="hongshiIcon" class="size-10 shrink-0 rounded-xl" alt="Hongshi" />
					<div class="min-w-0 flex-1">
						<h2 class="m-0 text-lg font-semibold text-contrast">
							{{ formatMessage(messages.hongshiDownload) }}
						</h2>
						<p class="mb-0 mt-1 text-secondary">
							{{ formatMessage(messages.hongshiBinaryMissing) }}
						</p>
					</div>
				</div>

				<ProgressBar
					full-width
					:progress="hongshiState.download_progress ?? 0"
					:max="100"
					:waiting="
						hongshiState.download_progress === null || hongshiState.download_progress === 0
					"
					:label="formatMessage(messages.statusDownloading)"
					show-progress
				/>
			</div>
		</Card>

		<Card v-else-if="!isRunning && hongshiState && !hongshiState.binary_installed" class="!m-0">
			<div class="flex flex-col gap-5">
				<div class="flex items-start gap-3">
					<img :src="hongshiIcon" class="size-10 shrink-0 rounded-xl" alt="Hongshi" />
					<div class="min-w-0 flex-1">
						<h2 class="m-0 text-lg font-semibold text-contrast">
							{{ formatMessage(messages.hongshiDownload) }}
						</h2>
						<p class="mb-0 mt-1 text-secondary">
							{{ formatMessage(messages.hongshiBinaryMissing) }}
						</p>
					</div>
				</div>

				<div class="flex flex-wrap gap-2">
					<ButtonStyled color="brand">
						<button type="button" :disabled="isHongshiActionPending" @click="downloadHongshi">
							<DownloadIcon />
							{{ formatMessage(messages.hongshiDownload) }}
						</button>
					</ButtonStyled>
				</div>
			</div>
		</Card>

		<Card v-else-if="!isRunning" class="!m-0">
			<div class="flex items-start gap-3">
				<img :src="hongshiIcon" class="size-10 shrink-0 rounded-xl" alt="Hongshi" />
				<div class="min-w-0 flex-1">
					<h2 class="m-0 text-lg font-semibold text-contrast">
						{{ formatMessage(messages.hongshiTitle) }}
					</h2>
					<p class="mb-0 mt-1 text-secondary">
						{{ formatMessage(messages.hongshiNotRunning) }}
					</p>
				</div>
				<ButtonStyled color="brand">
					<button type="button" @click="currentView = 'hongshi'">
						<PlayIcon />
						{{ formatMessage(messages.hongshiStart) }}
					</button>
				</ButtonStyled>
			</div>
		</Card>

		<div v-if="isRunning" class="mt-auto pt-6 text-center text-xs text-secondary">
			{{ formatMessage(messages.poweredByTerracotta) }}
		</div>
	</template>
	</div>
</template>
