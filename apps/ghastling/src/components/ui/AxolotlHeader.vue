<script setup lang="ts">
import { DownloadIcon } from '@modrinth/assets'
import GithubIcon from '@modrinth/assets/external/github.svg?component'
import HamburgerIcon from '@modrinth/assets/icons/hamburger.svg?component'
import SettingsIcon from '@modrinth/assets/icons/settings.svg?component'
import XIcon from '@modrinth/assets/icons/x.svg?component'
import ButtonStyled from '@modrinth/ui/src/components/base/ButtonStyled.vue'
import { defineMessages, useVIntl } from '@modrinth/ui/src/composables/i18n.ts'

import AxolotlWordmark from '~/components/brand/AxolotlWordmark.vue'

const emit = defineEmits<{
	openSettings: []
}>()

const mobileMenuOpen = ref(false)
const { formatMessage } = useVIntl()

const messages = defineMessages({
	home: { id: 'axolotl-site.navigation.home', defaultMessage: 'Ghastling Launcher 首页' },
	primary: { id: 'axolotl-site.navigation.primary', defaultMessage: '主导航' },
	mobile: { id: 'axolotl-site.navigation.mobile', defaultMessage: '移动端导航' },
	features: { id: 'axolotl-site.navigation.features', defaultMessage: '功能' },
	faq: { id: 'axolotl-site.navigation.faq', defaultMessage: '常见问题' },
	changelog: { id: 'axolotl-site.navigation.changelog', defaultMessage: '更新记录' },
	openSource: { id: 'axolotl-site.navigation.open-source', defaultMessage: '开放源代码' },
	openSettings: {
			id: 'axolotl-site.navigation.open-settings',
		defaultMessage: '打开显示设置',
	},
	openMenu: { id: 'axolotl-site.navigation.open-menu', defaultMessage: '打开导航' },
	closeMenu: { id: 'axolotl-site.navigation.close-menu', defaultMessage: '关闭导航' },
	download: { id: 'axolotl-site.navigation.download', defaultMessage: '下载' },
})

const base = import.meta.env.BASE_URL
const sounds = [
	`${base}sounds/Ghastling_ambient1.ogg`,
	`${base}sounds/Ghastling_ambient2.ogg`,
	`${base}sounds/Ghastling_ambient3.ogg`,
	`${base}sounds/Ghastling_ambient4.ogg`,
	`${base}sounds/Ghastling_ambient5.ogg`,
	`${base}sounds/Ghastling_ambient6.ogg`,
	`${base}sounds/Ghastling_ambient7.ogg`,
]

function playRandomSound() {
	if (!import.meta.client) return
	const randomIndex = Math.floor(Math.random() * sounds.length)
	const audio = new Audio(sounds[randomIndex])
	audio.volume = 0.5
	audio.play().catch(() => {
		// 忽略播放失败（如用户未与页面交互）
	})
}

function handleLogoClick(event: MouseEvent) {
	playRandomSound()
}

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
				@click="handleLogoClick"
			>
				<AxolotlWordmark />
			</NuxtLink>

			<nav class="desktop-navigation" :aria-label="formatMessage(messages.primary)">
				<ButtonStyled type="transparent">
					<NuxtLink to="/#features">{{ formatMessage(messages.features) }}</NuxtLink>
				</ButtonStyled>
				<ButtonStyled type="transparent">
					<NuxtLink to="/#faq">{{ formatMessage(messages.faq) }}</NuxtLink>
				</ButtonStyled>
				<ButtonStyled type="transparent">
					<a href="https://github.com/SunkDiagram1865/Ghastling/releases" target="_blank" rel="noopener">
						{{ formatMessage(messages.changelog) }}
					</a>
				</ButtonStyled>
				<ButtonStyled type="transparent">
					<a href="https://github.com/SunkDiagram1865/Ghastling" target="_blank" rel="noopener">
						<GithubIcon aria-hidden="true" />
						{{ formatMessage(messages.openSource) }}
					</a>
				</ButtonStyled>
			</nav>

			<div class="header-actions">
				<ButtonStyled color="brand" color-fill="background">
					<a
						href="https://github.com/SunkDiagram1865/Ghastling/releases/latest/download/Ghastling%20Launcher.exe"
						target="_blank"
						rel="noopener"
					>
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
				<a
					href="https://github.com/SunkDiagram1865/Ghastling/releases"
					target="_blank"
					rel="noopener"
					@click="mobileMenuOpen = false"
				>
					{{ formatMessage(messages.changelog) }}
				</a>
				<a
					href="https://github.com/SunkDiagram1865/Ghastling"
					target="_blank"
					rel="noopener"
					@click="mobileMenuOpen = false"
				>
					{{ formatMessage(messages.openSource) }}
				</a>
			</nav>
		</Transition>
	</header>
</template>

<style scoped lang="scss">
.site-header {
	position: sticky;
	top: 0;
	z-index: 40;
	background: color-mix(in srgb, var(--surface-1) 88%, transparent);
	backdrop-filter: blur(16px) saturate(120%);
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

	.desktop-navigation {
		display: none;
	}

	.mobile-menu-button,
	.mobile-navigation {
		display: flex;
	}
}
</style>
