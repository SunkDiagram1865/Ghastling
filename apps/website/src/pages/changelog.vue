<script setup lang="ts">
import { CalendarIcon, HistoryIcon } from '@modrinth/assets'
import Accordion from '@modrinth/ui/src/components/base/Accordion.vue'
import ButtonStyled from '@modrinth/ui/src/components/base/ButtonStyled.vue'
import TagItem from '@modrinth/ui/src/components/base/TagItem.vue'
import { defineMessages, useVIntl } from '@modrinth/ui/src/composables/i18n.ts'

import {
	ANNOUNCEMENT_CHANGE_TYPES,
	type AnnouncementChangeType,
	getLocalizedAnnouncementText,
	launcherAnnouncements,
} from '../../../app-frontend/src/announcements/catalog'

type GitHubRelease = {
	draft: boolean
	prerelease: boolean
	tag_name: string
}

const GITHUB_RELEASES_URL = 'https://api.github.com/repos/SunkDiagram1865/Ghastling/releases'

const { formatMessage, locale } = useVIntl()

const messages = defineMessages({
	seoTitle: {
		id: 'ghastling-site.changelog.seo.title',
		defaultMessage: 'Changelog - Ghastling Launcher',
	},
	seoDescription: {
		id: 'ghastling-site.changelog.seo.description',
		defaultMessage: 'See what changed in each public Ghastling Launcher release.',
	},
	eyebrow: { id: 'ghastling-site.changelog.eyebrow', defaultMessage: 'Release history' },
	title: { id: 'ghastling-site.changelog.title', defaultMessage: 'Changelog' },
	description: {
		id: 'ghastling-site.changelog.description',
		defaultMessage: 'Browse features, changes, and fixes in every public release.',
	},
	loading: {
		id: 'ghastling-site.changelog.loading',
		defaultMessage: 'Checking published releases…',
	},
	errorTitle: {
		id: 'ghastling-site.changelog.error.title',
		defaultMessage: 'Changelog is temporarily unavailable',
	},
	errorDescription: {
		id: 'ghastling-site.changelog.error.description',
		defaultMessage: 'Try again shortly, or visit GitHub to browse releases.',
	},
	retry: { id: 'ghastling-site.changelog.retry', defaultMessage: 'Retry' },
	empty: {
		id: 'ghastling-site.changelog.empty',
		defaultMessage: 'No public release notes are available yet.',
	},
	added: { id: 'ghastling-site.changelog.category.added', defaultMessage: 'Added' },
	changed: { id: 'ghastling-site.changelog.category.changed', defaultMessage: 'Changed' },
	deprecated: {
		id: 'ghastling-site.changelog.category.deprecated',
		defaultMessage: 'Deprecated',
	},
	removed: { id: 'ghastling-site.changelog.category.removed', defaultMessage: 'Removed' },
	fixed: { id: 'ghastling-site.changelog.category.fixed', defaultMessage: 'Fixed' },
	security: { id: 'ghastling-site.changelog.category.security', defaultMessage: 'Security' },
})

const {
	data: releases,
	error,
	status,
	refresh,
} = await useFetch<GitHubRelease[]>(GITHUB_RELEASES_URL, {
	server: false,
	query: { per_page: 100 },
	transform: (data) => data.filter((release) => !release.draft && !release.prerelease),
})

const categoryClasses: Record<AnnouncementChangeType, string> = {
	added: 'bg-brand-green',
	changed: 'bg-brand-blue',
	deprecated: 'bg-brand-orange',
	removed: 'bg-brand-red',
	fixed: 'bg-brand-purple',
	security: 'bg-brand-orange',
}

const publishedTags = computed(
	() => new Set(releases.value?.map((release) => release.tag_name) ?? []),
)
const releaseOrder = computed(
	() =>
		new Map<string, number>(
			(releases.value ?? []).map((release, index) => [release.tag_name, index] as const),
		),
)
const announcements = computed(() => {
	return [...launcherAnnouncements]
		.filter((announcement) => publishedTags.value.has(`v${announcement.version}`))
		.sort(
			(left, right) =>
				(releaseOrder.value.get(`v${left.version}`) ?? Infinity) -
				(releaseOrder.value.get(`v${right.version}`) ?? Infinity),
		)
})
const isLoading = computed(() => status.value === 'idle' || status.value === 'pending')
const seoTitle = computed(() => formatMessage(messages.seoTitle))
const seoDescription = computed(() => formatMessage(messages.seoDescription))

useSeoMeta({
	title: () => seoTitle.value,
	description: () => seoDescription.value,
	ogTitle: () => seoTitle.value,
	ogDescription: () => seoDescription.value,
	ogType: 'website',
	ogUrl: 'https://sunkdiagram1865.github.io/Ghastling/changelog',
	robots: 'index, follow',
})

useHead({
	link: [{ rel: 'canonical', href: 'https://sunkdiagram1865.github.io/Ghastling/changelog' }],
})
</script>

<template>
	<section class="changelog-page">
		<header class="changelog-header">
			<span class="section-eyebrow">{{ formatMessage(messages.eyebrow) }}</span>
			<h1>{{ formatMessage(messages.title) }}</h1>
			<p>{{ formatMessage(messages.description) }}</p>
		</header>

		<div v-if="isLoading" class="status-panel" role="status">
			<div class="loading-indicator" aria-hidden="true" />
			{{ formatMessage(messages.loading) }}
		</div>

		<div v-else-if="error" class="status-panel error-panel" role="alert">
			<div>
				<h2>{{ formatMessage(messages.errorTitle) }}</h2>
				<p>{{ formatMessage(messages.errorDescription) }}</p>
			</div>
			<ButtonStyled color="brand" type="outlined">
				<button type="button" @click="refresh()">{{ formatMessage(messages.retry) }}</button>
			</ButtonStyled>
		</div>

		<p v-else-if="announcements.length === 0" class="status-panel">
			{{ formatMessage(messages.empty) }}
		</p>

		<div v-else class="announcement-list">
			<Accordion
				v-for="(announcement, index) in announcements"
				:key="announcement.id"
				:open-by-default="index === 0"
				class="announcement"
				button-class="group flex w-full cursor-pointer items-center gap-4 border-0 bg-transparent px-5 py-4 text-left"
			>
				<template #title>
					<div class="announcement-heading">
						<div class="announcement-title-row">
							<h2>{{ getLocalizedAnnouncementText(announcement.title, locale) }}</h2>
							<TagItem>v{{ announcement.version }}</TagItem>
						</div>
						<div class="announcement-date">
							<CalendarIcon aria-hidden="true" />
							<time :datetime="announcement.publishedAt">{{ announcement.publishedAt }}</time>
						</div>
					</div>
				</template>

				<div class="announcement-content">
					<section
						v-for="type in ANNOUNCEMENT_CHANGE_TYPES"
						v-show="announcement.changes[type]?.length"
						:key="type"
						class="change-group"
					>
						<h3>
							<span :class="categoryClasses[type]" aria-hidden="true" />
							{{ formatMessage(messages[type]) }}
						</h3>
						<ul>
							<li v-for="change in announcement.changes[type]" :key="change['en-US']">
								{{ getLocalizedAnnouncementText(change, locale) }}
							</li>
						</ul>
					</section>
				</div>
			</Accordion>
		</div>

		<div class="changelog-footer">
			<HistoryIcon aria-hidden="true" />
			<a
				href="https://github.com/SunkDiagram1865/Ghastling/releases"
				target="_blank"
				rel="noopener"
			>
				GitHub Releases
			</a>
		</div>
	</section>
</template>

<style scoped lang="scss">
.changelog-page {
	width: min(52rem, calc(100% - 2rem));
	margin: 0 auto;
	padding: 4rem 0 5rem;
}

.changelog-header {
	max-width: 40rem;
	margin-bottom: 2.5rem;

	h1 {
		margin: 0.5rem 0 0;
		color: var(--color-contrast);
		font-size: 2.25rem;
		line-height: 1.15;
	}

	p {
		margin: 1rem 0 0;
		color: var(--color-secondary);
		line-height: 1.65;
	}
}

.announcement-list {
	display: flex;
	flex-direction: column;
	gap: 0.75rem;
}

.announcement {
	overflow: hidden;
	border: 1px solid var(--surface-5);
	border-radius: 0.5rem;
	background: var(--surface-4);
}

.announcement-heading {
	display: flex;
	min-width: 0;
	flex: 1;
	align-items: center;
	justify-content: space-between;
	gap: 1rem;
}

.announcement-title-row,
.announcement-date,
.changelog-footer {
	display: flex;
	align-items: center;
}

.announcement-title-row {
	min-width: 0;
	gap: 0.75rem;

	h2 {
		margin: 0;
		overflow: hidden;
		color: var(--color-contrast);
		font-size: 1rem;
		font-weight: 600;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
}

.announcement-date {
	flex-shrink: 0;
	gap: 0.35rem;
	color: var(--color-secondary);
	font-size: 0.8125rem;

	svg {
		width: 1rem;
		height: 1rem;
	}
}

.announcement-content {
	padding: 0 1.25rem 0.5rem;
	border-top: 1px solid var(--surface-5);
	background: var(--surface-3);
}

.change-group {
	display: grid;
	grid-template-columns: 7rem minmax(0, 1fr);
	gap: 1.25rem;
	padding: 1rem 0;
	border-top: 1px solid var(--surface-5);

	&:first-child {
		border-top: 0;
	}

	h3 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin: 0;
		color: var(--color-secondary);
		font-size: 0.875rem;
		font-weight: 600;

		span {
			width: 0.5rem;
			height: 0.5rem;
			border-radius: 50%;
		}
	}

	ul {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		margin: 0;
		padding-left: 1.25rem;
		color: var(--color-base);
		line-height: 1.6;
	}
}

.status-panel {
	display: flex;
	align-items: center;
	justify-content: center;
	gap: 0.75rem;
	margin: 0;
	padding: 2rem;
	border: 1px solid var(--surface-5);
	border-radius: 0.5rem;
	background: var(--surface-4);
	color: var(--color-secondary);
	text-align: center;
}

.error-panel {
	justify-content: space-between;
	text-align: left;

	h2,
	p {
		margin: 0;
	}

	h2 {
		color: var(--color-contrast);
		font-size: 1rem;
	}

	p {
		margin-top: 0.25rem;
	}
}

.loading-indicator {
	width: 1rem;
	height: 1rem;
	border: 2px solid var(--surface-5);
	border-top-color: var(--color-brand);
	border-radius: 50%;
	animation: spin 700ms linear infinite;
}

.changelog-footer {
	justify-content: center;
	gap: 0.5rem;
	margin-top: 2rem;
	color: var(--color-secondary);
	font-size: 0.875rem;

	svg {
		width: 1rem;
		height: 1rem;
	}

	a {
		color: inherit;
	}
}

@keyframes spin {
	to {
		transform: rotate(1turn);
	}
}

@media (max-width: 600px) {
	.changelog-page {
		padding: 2.5rem 0 3rem;
	}

	.changelog-header h1 {
		font-size: 1.875rem;
	}

	.announcement-heading,
	.error-panel {
		align-items: flex-start;
		flex-direction: column;
	}

	.change-group {
		grid-template-columns: 1fr;
		gap: 0.5rem;
	}
}
</style>
