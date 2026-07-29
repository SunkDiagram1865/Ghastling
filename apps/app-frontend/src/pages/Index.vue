<script setup lang="ts">
import { LanguagesIcon, SpinnerIcon } from '@modrinth/assets'
import { ButtonStyled, defineMessages, injectNotificationManager, useVIntl } from '@modrinth/ui'
import type { SearchResult } from '@modrinth/utils'
import dayjs from 'dayjs'
import { computed, onUnmounted, ref, shallowRef, watch } from 'vue'
import { useRoute } from 'vue-router'

import RowDisplay from '@/components/RowDisplay.vue'
import RecentWorldsList from '@/components/ui/world/RecentWorldsList.vue'
import { useNetworkStatus } from '@/composables/useNetworkStatus'
import { useTranslationToggle } from '@/composables/useTranslationToggle'
import { get_search_results } from '@/helpers/cache.js'
import { instance_listener } from '@/helpers/events'
import { list } from '@/helpers/instance'
import { translateSearchHits } from '@/helpers/translation'
import type { GameInstance } from '@/helpers/types'
import { useBreadcrumbs } from '@/store/breadcrumbs'

const { handleError } = injectNotificationManager()
const route = useRoute()
const breadcrumbs = useBreadcrumbs()
const { formatMessage } = useVIntl()

const messages = defineMessages({
	home: { id: 'app.home.breadcrumb', defaultMessage: 'Home' },
	welcomeBack: { id: 'app.home.welcome-back', defaultMessage: 'Welcome back!' },
	welcome: {
		id: 'app.home.welcome',
		defaultMessage: 'Welcome to Ghastling Launcher!',
	},
	discoverModpack: {
		id: 'app.home.discover-modpack',
		defaultMessage: 'Discover a modpack',
	},
	discoverMods: { id: 'app.home.discover-mods', defaultMessage: 'Discover mods' },
	translateProject: {
		id: 'app.project.translation.translate',
		defaultMessage: 'Translate',
	},
	showOriginal: {
		id: 'app.project.translation.show-original',
		defaultMessage: 'Show original',
	},
	translating: {
		id: 'app.project.translation.translating',
		defaultMessage: 'Translating…',
	},
})

breadcrumbs.setRootContext({ name: formatMessage(messages.home), link: route.path })

const instances = ref<GameInstance[]>([])

const featuredModpacks = ref<SearchResult[]>([])
const featuredMods = ref<SearchResult[]>([])
const installedModpacksFilter = ref('')
const originalFeaturedModpacks = shallowRef<SearchResult[]>([])
const originalFeaturedMods = shallowRef<SearchResult[]>([])
const {
	translationActive,
	translationLoading,
	start: startTranslation,
	isStale,
	done: doneTranslation,
	toggle,
	cancel: cancelTranslation,
} = useTranslationToggle()

const recentInstances = computed(() =>
	instances.value
		.filter((x) => x.last_played)
		.slice()
		.sort((a, b) => dayjs(b.last_played).diff(dayjs(a.last_played))),
)

const hasFeaturedProjects = computed(
	() => (featuredModpacks.value?.length ?? 0) + (featuredMods.value?.length ?? 0) > 0,
)

const { offline } = useNetworkStatus()

async function fetchInstances() {
	instances.value = await list().catch(handleError)

	const filters = []
	for (const instance of instances.value) {
		if (instance.link && instance.link.project_id) {
			filters.push(`NOT"project_id"="${instance.link.project_id}"`)
		}
	}
	installedModpacksFilter.value = filters.join(' AND ')
}

async function fetchFeaturedModpacks() {
	const response = await get_search_results(
		`?facets=[["project_type:modpack"]]&limit=10&index=follows&filters=${installedModpacksFilter.value}`,
	)

	if (response) {
		featuredModpacks.value = response.result.hits
	} else {
		featuredModpacks.value = []
	}
}

async function fetchFeaturedMods() {
	const response = await get_search_results('?facets=[["project_type:mod"]]&limit=10&index=follows')

	if (response) {
		featuredMods.value = response.result.hits
	} else {
		featuredModpacks.value = []
	}
}

async function refreshFeaturedProjects() {
	cancelTranslation()

	await Promise.all([fetchFeaturedModpacks(), fetchFeaturedMods()])

	// Save pristine copies for the toggle-to-original flow.
	originalFeaturedModpacks.value = featuredModpacks.value
	originalFeaturedMods.value = featuredMods.value

	// Auto-translate if the user has enabled the setting.
	try {
		const [translatedModpacks, translatedMods] = await Promise.all([
			translateSearchHits(featuredModpacks.value),
			translateSearchHits(featuredMods.value),
		])
		if (translatedModpacks !== featuredModpacks.value) {
			featuredModpacks.value = translatedModpacks
		}
		if (translatedMods !== featuredMods.value) {
			featuredMods.value = translatedMods
		}
		if (translatedModpacks !== featuredModpacks.value || translatedMods !== featuredMods.value) {
			translationActive.value = true
		}
	} catch {
		// Translation errors are non-critical; keep original content.
	}
}

async function translateFeaturedProjects() {
	const version = startTranslation()
	try {
		const [translatedModpacks, translatedMods] = await Promise.all([
			translateSearchHits(originalFeaturedModpacks.value, true),
			translateSearchHits(originalFeaturedMods.value, true),
		])
		if (isStale(version)) return
		if (translatedModpacks !== originalFeaturedModpacks.value) {
			featuredModpacks.value = translatedModpacks
		}
		if (translatedMods !== originalFeaturedMods.value) {
			featuredMods.value = translatedMods
		}
		translationActive.value = true
	} finally {
		doneTranslation(version)
	}
}

function toggleTranslation() {
	toggle(
		() => {
			featuredModpacks.value = originalFeaturedModpacks.value
			featuredMods.value = originalFeaturedMods.value
		},
		() => void translateFeaturedProjects(),
	)
}

await fetchInstances()
if (!offline.value) await refreshFeaturedProjects()

watch(offline, (isOffline) => {
	if (isOffline) {
		featuredModpacks.value = []
		featuredMods.value = []
	} else {
		void refreshFeaturedProjects()
	}
})

const unlistenInstance = await instance_listener(
	async (e: { event: string; instance_id: string }) => {
		await fetchInstances()

		if (!offline.value && (e.event === 'added' || e.event === 'created' || e.event === 'removed')) {
			await refreshFeaturedProjects()
		}
	},
)

onUnmounted(() => {
	unlistenInstance()
})
</script>

<template>
	<div class="p-6 flex flex-col gap-2">
		<div class="flex items-center justify-between">
			<h1 v-if="recentInstances?.length > 0" class="m-0 text-2xl font-extrabold">
				{{ formatMessage(messages.welcomeBack) }}
			</h1>
			<h1 v-else class="m-0 text-2xl font-extrabold">
				{{ formatMessage(messages.welcome) }}
			</h1>
			<ButtonStyled size="large" type="transparent">
				<button :disabled="translationLoading" @click="toggleTranslation">
					<SpinnerIcon v-if="translationLoading" class="animate-spin" />
					<LanguagesIcon v-else />
					{{
						formatMessage(
							translationLoading
								? messages.translating
								: translationActive
									? messages.showOriginal
									: messages.translateProject,
						)
					}}
				</button>
			</ButtonStyled>
		</div>
		<div data-onboarding-id="home-recent">
			<RecentWorldsList :recent-instances="recentInstances" />
		</div>
		<div data-onboarding-id="home-featured">
			<RowDisplay
				v-if="hasFeaturedProjects"
				:instances="[
					{
						label: formatMessage(messages.discoverModpack),
						route: '/browse/modpack',
						instances: featuredModpacks,
						downloaded: false,
					},
					{
						label: formatMessage(messages.discoverMods),
						route: '/browse/mod',
						instances: featuredMods,
						downloaded: false,
					},
				]"
				:can-paginate="true"
			/>
		</div>
	</div>
</template>
