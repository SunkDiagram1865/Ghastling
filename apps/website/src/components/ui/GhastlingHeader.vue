<script setup lang="ts">
import GithubIcon from '@modrinth/assets/external/github.svg?component'
import DownloadIcon from '@modrinth/assets/icons/download.svg?component'
import HamburgerIcon from '@modrinth/assets/icons/hamburger.svg?component'
import SettingsIcon from '@modrinth/assets/icons/settings.svg?component'
import XIcon from '@modrinth/assets/icons/x.svg?component'
import ButtonStyled from '@modrinth/ui/src/components/base/ButtonStyled.vue'
import { defineMessages, useVIntl } from '@modrinth/ui/src/composables/i18n.ts'

import GhastlingWordmark from '~/components/brand/GhastlingWordmark.vue'

const emit = defineEmits<{
	openSettings: []
}>()

const mobileMenuOpen = ref(false)
const { formatMessage } = useVIntl()

const messages = defineMessages({
	home: { id: 'ghastling-site.navigation.home', defaultMessage: 'Ghastling Launcher home' },
	primary: { id: 'ghastling-site.navigation.primary', defaultMessage: 'Primary navigation' },
	mobile: { id: 'ghastling-site.navigation.mobile', defaultMessage: 'Mobile navigation' },
	features: { id: 'ghastling-site.navigation.features', defaultMessage: 'Features' },
	faq: { id: 'ghastling-site.navigation.faq', defaultMessage: 'FAQ' },
	changelog: { id: 'ghastling-site.navigation.changelog', defaultMessage: 'Changelog' },
	openSource: { id: 'ghastling-site.navigation.open-source', defaultMessage: 'Open source' },
	download: { id: 'ghastling-site.navigation.download', defaultMessage: 'Download' },
	openSettings: {
		id: 'ghastling-site.navigation.open-settings',
		defaultMessage: 'Open display settings',
	},
	openMenu: { id: 'ghastling-site.navigation.open-menu', defaultMessage: 'Open navigation' },
	closeMenu: { id: 'ghastling-site.navigation.close-menu', defaultMessage: 'Close navigation' },
})

function openSettings() {
	mobileMenuOpen.value = false
	emit('openSettings')
}
</script>

<template>
	<header class="site-header">
		<div class="header-inner">
			<NuxtLink
				to="/"
				:aria-label="formatMessage(messages.home)"
				class="brand-link button-animation"
			>
				<GhastlingWordmark />
			</NuxtLink>

			<nav class="desktop-navigation" :aria-label="formatMessage(messages.primary)">
				<ButtonStyled type="transparent">
					<NuxtLink to="/#features">{{ formatMessage(messages.features) }}</NuxtLink>
				</ButtonStyled>
				<ButtonStyled type="transparent">
					<NuxtLink to="/#faq">{{ formatMessage(messages.faq) }}</NuxtLink>
				</ButtonStyled>
				<ButtonStyled type="transparent">
					<NuxtLink to="/changelog">{{ formatMessage(messages.changelog) }}</NuxtLink>
				</ButtonStyled>
				<ButtonStyled type="transparent">
					<a href="https://github.com/SunkDiagram1865/Ghastling" target="_blank" rel="noopener">
						<GithubIcon aria-hidden="true" />
						{{ formatMessage(messages.openSource) }}
					</a>
				</ButtonStyled>
			</nav>

			<div class="header-actions">
				<ButtonStyled class="desktop-download" color="brand">
					<a href="#download">
						<DownloadIcon aria-hidden="true" />
						{{ formatMessage(messages.download) }}
					</a>
				</ButtonStyled>
				<ButtonStyled circular type="transparent">
					<button :aria-label="formatMessage(messages.openSettings)" @click="openSettings">
						<SettingsIcon aria-hidden="true" />
					</button>
				</ButtonStyled>
				<ButtonStyled class="mobile-menu-button" circular type="transparent">
					<button
						:aria-label="formatMessage(mobileMenuOpen ? messages.closeMenu : messages.openMenu)"
						:aria-expanded="mobileMenuOpen"
						@click="mobileMenuOpen = !mobileMenuOpen"
					>
						<XIcon v-if="mobileMenuOpen" aria-hidden="true" />
						<HamburgerIcon v-else aria-hidden="true" />
					</button>
				</ButtonStyled>
			</div>
		</div>

		<Transition name="mobile-menu">
			<nav
				v-if="mobileMenuOpen"
				class="mobile-navigation"
				:aria-label="formatMessage(messages.mobile)"
			>
				<NuxtLink to="/#features" @click="mobileMenuOpen = false">
					{{ formatMessage(messages.features) }}
				</NuxtLink>
				<NuxtLink to="/#faq" @click="mobileMenuOpen = false">
					{{ formatMessage(messages.faq) }}
				</NuxtLink>
				<NuxtLink to="/changelog" @click="mobileMenuOpen = false">
					{{ formatMessage(messages.changelog) }}
				</NuxtLink>
				<a
					href="https://github.com/SunkDiagram1865/Ghastling"
					target="_blank"
					rel="noopener"
					@click="mobileMenuOpen = false"
				>
					{{ formatMessage(messages.openSource) }}
				</a>
				<a class="mobile-download" href="#download" @click="mobileMenuOpen = false">
					<DownloadIcon aria-hidden="true" />
					{{ formatMessage(messages.download) }}
				</a>
			</nav>
		</Transition>
	</header>
</template>

<style scoped lang="scss">
.site-header {
	position: relative;
	z-index: 40;
}

.header-inner {
	display: grid;
	grid-template-columns: 1fr auto;
	align-items: center;
	gap: 0.5rem;
	max-width: 1280px;
	margin: 0 auto;
	padding: 1rem 1.5rem;
}

.brand-link {
	width: fit-content;
	text-decoration: none;
}

.desktop-navigation,
.header-actions {
	display: flex;
	align-items: center;
	gap: 0.25rem;
}

.desktop-navigation {
	grid-column: 1 / -1;
	grid-row: 2;
	justify-content: center;
}

.header-actions {
	grid-column: 2;
	grid-row: 1;
	justify-content: flex-end;
}

.mobile-menu-button,
.mobile-navigation {
	display: none;
}

.mobile-navigation {
	position: absolute;
	top: 100%;
	right: 1rem;
	width: min(22rem, calc(100% - 2rem));
	flex-direction: column;
	gap: 0.25rem;
	padding: 0.75rem;
	border: 1px solid var(--color-divider);
	border-radius: 1rem;
	background: color-mix(in srgb, var(--color-raised-bg) 92%, transparent);
	box-shadow: 0 1.25rem 3rem rgb(0 0 0 / 22%);
	backdrop-filter: blur(20px) saturate(150%);

	a {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 0.875rem;
		border-radius: var(--radius-md);
		color: var(--color-base);
		font-weight: 600;
		text-decoration: none;

		&:hover {
			background: var(--color-button-bg);
		}
	}

	.mobile-download {
		justify-content: center;
		margin-top: 0.25rem;
		background: var(--color-brand);
		color: var(--color-on-brand);
	}
}

.mobile-menu-enter-active,
.mobile-menu-leave-active {
	transition: 160ms ease;
}

.mobile-menu-enter-from,
.mobile-menu-leave-to {
	transform: translateY(-0.5rem) scale(0.98);
	opacity: 0;
}

@media (min-width: 1024px) {
	.header-inner {
		grid-template-columns: auto 1fr auto;
	}

	.desktop-navigation {
		grid-column: 2;
		grid-row: 1;
	}

	.header-actions {
		grid-column: 3;
	}
}

@media (max-width: 760px) {
	.header-inner {
		grid-template-columns: 1fr auto;
		padding: 0.875rem 1rem;
	}

	.desktop-navigation,
	.desktop-download {
		display: none;
	}

	.mobile-menu-button,
	.mobile-navigation {
		display: flex;
	}
}
</style>
