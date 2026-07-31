<script setup lang="ts">
import { PaintbrushIcon } from '@modrinth/assets'
import SettingsIcon from '@modrinth/assets/icons/settings.svg?component'
import XIcon from '@modrinth/assets/icons/x.svg?component'
import ButtonStyled from '@modrinth/ui/src/components/base/ButtonStyled.vue'
import Toggle from '@modrinth/ui/src/components/base/Toggle.vue'
import ThemeSelector from '@modrinth/ui/src/components/settings/ThemeSelector.vue'
import { defineMessages, useVIntl } from '@modrinth/ui/src/composables/i18n.ts'

type Theme = 'system' | 'light' | 'dark' | 'oled'
type SettingsTab = 'appearance'

const open = defineModel<boolean>({ required: true })
const { formatMessage } = useVIntl()

const preferredTheme = ref<Theme>('system')
const systemTheme = ref<'light' | 'dark'>('dark')
const advancedRendering = ref(true)
const reduceMotion = ref(false)
const externalLinksNewTab = ref(true)
const themeOptions = ['dark', 'light', 'oled', 'system'] as const

const messages = defineMessages({
	title: { id: 'axolotl-settings.title', defaultMessage: '显示设置' },
	description: {
			id: 'axolotl-settings.description',
		defaultMessage: '自定义 Ghastling 在此设备上的显示方式。',
	},
	close: { id: 'axolotl-settings.close', defaultMessage: '关闭设置' },
	appearanceTitle: { id: 'axolotl-settings.appearance.title', defaultMessage: '外观' },
	themeTitle: { id: 'axolotl-settings.theme.title', defaultMessage: '颜色主题' },
	themeDescription: {
			id: 'axolotl-settings.theme.description',
		defaultMessage: '选择此设备上的 Ghastling 颜色主题。',
	},
	interfaceTitle: { id: 'axolotl-settings.interface.title', defaultMessage: '界面' },
	interfaceDescription: {
			id: 'axolotl-settings.interface.description',
		defaultMessage: '启用或停用此设备上的视觉效果。',
	},
	advancedRenderingTitle: {
			id: 'axolotl-settings.advanced-rendering.title',
		defaultMessage: '高级渲染',
	},
	advancedRenderingDescription: {
			id: 'axolotl-settings.advanced-rendering.description',
		defaultMessage: '使用模糊、渐变和增强背景效果。',
	},
	reduceMotionTitle: {
			id: 'axolotl-settings.reduce-motion.title',
		defaultMessage: '减少动态效果',
	},
	reduceMotionDescription: {
			id: 'axolotl-settings.reduce-motion.description',
		defaultMessage: '停用装饰动画和过渡效果。',
	},
	externalLinksTitle: {
			id: 'axolotl-settings.external-links.title',
		defaultMessage: '在新标签页打开外部链接',
	},
	externalLinksDescription: {
			id: 'axolotl-settings.external-links.description',
		defaultMessage: '访问其他网站时保留当前下载页面。',
	},
	done: { id: 'axolotl-settings.done', defaultMessage: '完成' },
})

let systemThemeQuery: MediaQueryList | undefined

function applyTheme() {
	if (!import.meta.client) return

	const resolvedTheme = preferredTheme.value === 'system' ? systemTheme.value : preferredTheme.value
	document.documentElement.classList.remove('light-mode', 'dark-mode', 'oled-mode')
	document.documentElement.classList.add(`${resolvedTheme}-mode`, 'accent-pink')
	document.documentElement.style.colorScheme = resolvedTheme === 'light' ? 'light' : 'dark'
	localStorage.setItem('axolotl-theme', preferredTheme.value)
}

function updateColorTheme(theme: Theme) {
	preferredTheme.value = theme
	applyTheme()
}

function applyRenderingPreferences() {
	if (!import.meta.client) return
	document.documentElement.classList.toggle('reduced-effects', !advancedRendering.value)
	document.documentElement.classList.toggle('reduced-motion', reduceMotion.value)
	localStorage.setItem('axolotl-advanced-rendering', String(advancedRendering.value))
	localStorage.setItem('axolotl-reduce-motion', String(reduceMotion.value))
	localStorage.setItem('axolotl-external-links-new-tab', String(externalLinksNewTab.value))
}

function handleSystemTheme(event: MediaQueryListEvent) {
	systemTheme.value = event.matches ? 'dark' : 'light'
	if (preferredTheme.value === 'system') applyTheme()
}

function handleKeyDown(event: KeyboardEvent) {
	if (event.key === 'Escape') open.value = false
}

function handleExternalLink(event: MouseEvent) {
	if (!externalLinksNewTab.value || !(event.target instanceof Element)) return
	const anchor = event.target.closest<HTMLAnchorElement>('a[href]')
	if (!anchor) return

	const destination = new URL(anchor.href, window.location.href)
	if (destination.origin !== window.location.origin) {
		anchor.target = '_blank'
		anchor.rel = 'noopener'
	}
}

watch(open, (isOpen) => {
	if (!import.meta.client) return
	document.body.style.overflow = isOpen ? 'hidden' : ''
})

watch([advancedRendering, reduceMotion, externalLinksNewTab], applyRenderingPreferences)

onMounted(() => {
	systemThemeQuery = window.matchMedia('(prefers-color-scheme: dark)')
	systemTheme.value = systemThemeQuery.matches ? 'dark' : 'light'
	preferredTheme.value = (localStorage.getItem('axolotl-theme') as Theme | null) ?? 'system'
	advancedRendering.value = localStorage.getItem('axolotl-advanced-rendering') !== 'false'
	reduceMotion.value = localStorage.getItem('axolotl-reduce-motion') === 'true'
	externalLinksNewTab.value = localStorage.getItem('axolotl-external-links-new-tab') !== 'false'
	applyTheme()
	applyRenderingPreferences()
	systemThemeQuery.addEventListener('change', handleSystemTheme)
	window.addEventListener('keydown', handleKeyDown)
	window.addEventListener('click', handleExternalLink, true)
})

onBeforeUnmount(() => {
	document.body.style.overflow = ''
	systemThemeQuery?.removeEventListener('change', handleSystemTheme)
	window.removeEventListener('keydown', handleKeyDown)
	window.removeEventListener('click', handleExternalLink, true)
})
</script>

<template>
	<Teleport to="body">
		<Transition name="settings-modal">
			<div v-if="open" class="settings-backdrop" @click.self="open = false">
				<section
					class="settings-panel"
					role="dialog"
					aria-modal="true"
					aria-labelledby="settings-title"
				>
					<header class="settings-header">
						<h2 id="settings-title">
							<SettingsIcon aria-hidden="true" />
							{{ formatMessage(messages.title) }}
						</h2>
						<ButtonStyled circular type="transparent">
							<button :aria-label="formatMessage(messages.close)" @click="open = false">
								<XIcon aria-hidden="true" />
							</button>
						</ButtonStyled>
					</header>

					<div class="settings-body">
						<aside class="settings-sidebar">
							<nav :aria-label="formatMessage(messages.title)">
								<button
									class="selected"
								>
									<PaintbrushIcon aria-hidden="true" />
									{{ formatMessage(messages.appearanceTitle) }}
								</button>
							</nav>

							<div class="settings-brand">
								<img src="/ghastling.png" alt="" />
								<div>
									<strong>Ghastling Launcher</strong>
									<span>Website 2026</span>
								</div>
							</div>
						</aside>

						<div class="settings-main">
							<section class="settings-pane">
								<div class="settings-section">
									<h3>{{ formatMessage(messages.themeTitle) }}</h3>
									<p>{{ formatMessage(messages.themeDescription) }}</p>
									<ThemeSelector
										:update-color-theme="updateColorTheme"
										:current-theme="preferredTheme"
										:theme-options="themeOptions"
										:system-theme-color="systemTheme"
									/>
								</div>

								<div class="settings-section interface-section">
									<h3>{{ formatMessage(messages.interfaceTitle) }}</h3>
									<p>{{ formatMessage(messages.interfaceDescription) }}</p>
									<div class="settings-toggles">
										<div class="setting-row">
											<label for="advanced-rendering">
												<strong>{{ formatMessage(messages.advancedRenderingTitle) }}</strong>
												<span>{{ formatMessage(messages.advancedRenderingDescription) }}</span>
											</label>
											<Toggle id="advanced-rendering" v-model="advancedRendering" />
										</div>
										<div class="setting-row">
											<label for="reduce-motion">
												<strong>{{ formatMessage(messages.reduceMotionTitle) }}</strong>
												<span>{{ formatMessage(messages.reduceMotionDescription) }}</span>
											</label>
											<Toggle id="reduce-motion" v-model="reduceMotion" />
										</div>
										<div class="setting-row">
											<label for="external-links">
												<strong>{{ formatMessage(messages.externalLinksTitle) }}</strong>
												<span>{{ formatMessage(messages.externalLinksDescription) }}</span>
											</label>
											<Toggle id="external-links" v-model="externalLinksNewTab" />
										</div>
									</div>
								</div>
							</section>
						</div>
					</div>
				</section>
			</div>
		</Transition>
	</Teleport>
</template>

<style scoped lang="scss">
.settings-backdrop {
	position: fixed;
	inset: 0;
	z-index: 100;
	display: flex;
	align-items: center;
	justify-content: center;
	padding: 1rem;
	background: rgb(10 12 18 / 58%);
	backdrop-filter: blur(8px) saturate(90%);
}

.settings-panel {
	display: flex;
	flex-direction: column;
	width: min(60rem, 100%);
	height: min(40rem, calc(100vh - 2rem));
	overflow: hidden;
	border: 1px solid var(--color-divider);
	border-radius: 1.25rem;
	background: var(--color-raised-bg);
	box-shadow: 0 2rem 6rem rgb(0 0 0 / 42%);
}

.settings-header {
	display: flex;
	flex: 0 0 auto;
	align-items: center;
	justify-content: space-between;
	min-height: 5.25rem;
	padding: 0 1.5rem;
	border-bottom: 1px solid var(--color-divider);
	background: var(--color-raised-bg);

	h2 {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		margin: 0;
		color: var(--color-contrast);
		font-size: 1.125rem;
		font-weight: 800;
	}

	svg {
		width: 1.125rem;
		height: 1.125rem;
	}
}

.settings-body {
	display: grid;
	grid-template-columns: 15.5rem minmax(0, 1fr);
	min-height: 0;
	flex: 1;
	padding: 1.5rem 0 0 1.5rem;
}

.settings-sidebar {
	display: flex;
	min-height: 0;
	flex-direction: column;
	padding: 0 1rem 0.75rem 0;
	border-right: 1px solid var(--color-divider);

	nav {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	button {
		display: flex;
		align-items: center;
		gap: 0.625rem;
		width: 100%;
		padding: 0.55rem 1rem;
		border: 0;
		border-radius: 0.75rem;
		background: transparent;
		color: var(--color-base);
		font: inherit;
		font-weight: 650;
		text-align: left;
		transition: 120ms ease;

		&:hover {
			background: var(--color-button-bg);
			color: var(--color-contrast);
			cursor: pointer;
		}

		&.selected {
			background: var(--color-brand);
			color: var(--color-brand-inverted);
			box-shadow: 0 0.5rem 1.5rem color-mix(in srgb, var(--color-brand) 16%, transparent);
		}

		svg {
			width: 1rem;
			height: 1rem;
			flex: 0 0 auto;
		}
	}
}

.settings-brand {
	display: flex;
	align-items: center;
	gap: 0.75rem;
	margin-top: auto;
	padding: 1rem 0.25rem 0;
	color: var(--color-secondary);
	font-size: 0.8rem;

	img {
		width: 2.25rem;
		height: 2.25rem;
		object-fit: contain;
	}

	div {
		display: flex;
		min-width: 0;
		flex-direction: column;
		gap: 0.15rem;
	}

	strong {
		color: var(--color-base);
		font-size: 0.875rem;
	}

	span {
		overflow: hidden;
		white-space: nowrap;
		text-overflow: ellipsis;
	}
}

.settings-main {
	position: relative;
	min-width: 0;
	min-height: 0;
	overflow-y: auto;
	scrollbar-color: var(--color-scrollbar) transparent;
}

.settings-main::after {
	position: sticky;
	bottom: 0;
	display: block;
	height: 2.5rem;
	margin-top: -2.5rem;
	background: linear-gradient(transparent, var(--color-raised-bg));
	content: '';
	pointer-events: none;
}

.settings-pane {
	padding: 0 2rem 3rem 1.5rem;
}

.settings-section {
	h3,
	p {
		margin: 0;
	}

	h3 {
		color: var(--color-contrast);
		font-size: 1.125rem;
		font-weight: 700;
	}

	p {
		margin-top: 0.25rem;
		color: var(--color-base);
	}
}

.interface-section {
	margin-top: 1.75rem;
	padding-top: 1.5rem;
	border-top: 1px solid var(--color-divider);
}

.settings-toggles {
	display: flex;
	flex-direction: column;
	margin-top: 0.75rem;
}

.setting-row {
	display: flex;
	align-items: center;
	justify-content: space-between;
	gap: 1.5rem;
	padding: 1rem 0;
	border-bottom: 1px solid color-mix(in srgb, var(--color-divider) 68%, transparent);

	label {
		display: flex;
		min-width: 0;
		flex: 1;
		flex-direction: column;
		gap: 0.25rem;
	}

	strong {
		color: var(--color-contrast);
		font-size: 0.95rem;
	}

	span {
		color: var(--color-secondary);
		font-size: 0.85rem;
		line-height: 1.45;
	}
}

.settings-modal-enter-active,
.settings-modal-leave-active {
	transition: opacity 180ms ease;

	.settings-panel {
		transition: transform 180ms ease;
	}
}

.settings-modal-enter-from,
.settings-modal-leave-to {
	opacity: 0;

	.settings-panel {
		transform: translateY(0.75rem) scale(0.985);
	}
}

@media (max-width: 700px) {
	.settings-backdrop {
		align-items: flex-end;
		padding: 0;
	}

	.settings-panel {
		height: 94vh;
		border-radius: 1.25rem 1.25rem 0 0;
	}

	.settings-header {
		min-height: 4.5rem;
		padding: 0 1rem;
	}

	.settings-body {
		display: flex;
		flex-direction: column;
		padding: 0;
	}

	.settings-sidebar {
		padding: 0.75rem 1rem;
		border-right: 0;
		border-bottom: 1px solid var(--color-divider);

		nav {
			flex-direction: row;
		}

		button {
			justify-content: center;
		}
	}

	.settings-brand {
		display: none;
	}

	.settings-pane {
		padding: 1.25rem 1rem 3rem;
	}

	.setting-row {
		align-items: flex-start;
	}
}
</style>
