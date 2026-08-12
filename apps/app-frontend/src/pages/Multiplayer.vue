<script setup lang="ts">
import {
	ArrowLeftIcon,
	BinaryIcon,
	CheckCircleIcon,
	DownloadIcon,
	LogInIcon,
	LogOutIcon,
	PlayIcon,
	RefreshCwIcon,
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
	NavTabs,
	ProgressBar,
	StyledInput,
	TagItem,
	useVIntl,
} from '@modrinth/ui'
import { computed, ref } from 'vue'

import { useTerracottaSession } from '@/composables/useTerracottaSession'
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
	poweredByTerracotta: {
		id: 'app.multiplayer.powered-by-terracotta',
		defaultMessage: 'Powered by Terracotta | 陶瓦联机',
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
})

const tabIndex = ref(0)
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
					<div
						class="flex size-10 shrink-0 items-center justify-center rounded-xl bg-highlight-orange text-orange"
					>
						<BinaryIcon class="size-5" />
					</div>
					<div class="min-w-0">
						<h2 class="m-0 text-lg font-semibold text-contrast">
							{{ formatMessage(messages.downloadTerracotta) }}
						</h2>
						<p class="mb-0 mt-1 text-secondary">
							{{ formatMessage(messages.notRunning) }}
						</p>
					</div>
				</div>

				<Admonition type="warning" :header="formatMessage(messages.binaryNotFound)">
					<div class="flex flex-col gap-2">
						<span>{{ formatMessage(messages.platformInfo, { platform: platformKey }) }}</span>
						<code class="w-fit max-w-full select-all break-all rounded-lg bg-surface-3 px-2 py-1">
							{{ binaryPathHint }}
						</code>
					</div>
				</Admonition>

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
			<div class="flex flex-col gap-5">
				<div class="flex items-start gap-3">
					<div
						class="flex size-10 shrink-0 items-center justify-center rounded-xl bg-brand-highlight text-brand"
					>
						<UsersIcon class="size-5" />
					</div>
					<div class="min-w-0">
						<h2 class="m-0 text-lg font-semibold text-contrast">
							{{ formatMessage(messages.notRunningTitle) }}
						</h2>
						<p class="mb-0 mt-1 text-secondary">
							{{ formatMessage(messages.notRunning) }}
						</p>
					</div>
				</div>
				<div class="flex flex-wrap gap-2">
					<ButtonStyled color="brand">
						<button type="button" :disabled="isActionPending" @click="startTerracotta">
							<PlayIcon />
							{{ formatMessage(messages.startTerracotta) }}
						</button>
					</ButtonStyled>
				</div>
			</div>
		</Card>

		<div class="mt-auto pt-6 text-center text-xs text-secondary">
			{{ formatMessage(messages.poweredByTerracotta) }}
		</div>
	</div>
</template>
