<script setup lang="ts">
import { PaintbrushIcon } from '@modrinth/assets'
import SettingsIcon from '@modrinth/assets/icons/settings.svg?component'
import XIcon from '@modrinth/assets/icons/x.svg?component'
import ButtonStyled from '@modrinth/ui/src/components/base/ButtonStyled.vue'
import Toggle from '@modrinth/ui/src/components/base/Toggle.vue'
import ThemeSelector from '@modrinth/ui/src/components/settings/ThemeSelector.vue'
import { defineMessages, useVIntl } from '@modrinth/ui/src/composables/i18n.ts'

type Theme = 'system' | 'light' | 'dark' | 'oled'
type SettingsTab = 'appearance' | 'about'

const open = defineModel<boolean>({ required: true })
const { formatMessage } = useVIntl()

const activeTab = ref<SettingsTab>('appearance')
const preferredTheme = ref<Theme>('system')
const systemTheme = ref<'light' | 'dark'>('dark')
const advancedRendering = ref(true)
const reduceMotion = ref(false)
const externalLinksNewTab = ref(true)
const themeOptions = ['dark', 'light', 'oled', 'system'] as const

// About tab state
const copied = ref(false)
const ghastlingVersion = 'Website 2026'
const qqGroupNumber = '208375315'
const sponsorUrl = 'https://afdian.com/a/cysunk'
const websiteUrl = 'https://sunkdiagram1865.github.io/Ghastling/'

async function copyQqGroupNumber() {
	try {
		await navigator.clipboard.writeText(qqGroupNumber)
	} catch {
		// clipboard API may not be available; fallback is fine for display
	}
	copied.value = true
	setTimeout(() => {
		copied.value = false
	}, 3000)
}

const messages = defineMessages({
	title: { id: 'axolotl-settings.title', defaultMessage: '设置' },
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
	// About tab
	aboutTitle: { id: 'axolotl-settings.about.title', defaultMessage: '关于' },
	aboutProductTitle: {
		id: 'axolotl-settings.about.product-title',
		defaultMessage: '关于 Ghastling Launcher',
	},
	aboutVersion: {
		id: 'axolotl-settings.about.version',
		defaultMessage: '版本 {version}',
	},
	aboutDeveloper: {
		id: 'axolotl-settings.about.developer',
		defaultMessage: '由 Coffeepop Studio 的 SunkDiagram1865 开发。',
	},
	aboutAttribution: {
		id: 'axolotl-settings.about.attribution',
		defaultMessage: '本应用是基于开源 Modrinth 项目的二次开发版本。',
	},
	aboutContentSearchAttribution: {
		id: 'axolotl-settings.about.content-search-attribution',
		defaultMessage: '中文内容搜索使用了来自 Plain Craft Launcher 和 MC 百科的项目名称的数据。',
	},
	aboutCommunitySupport: {
		id: 'axolotl-settings.about.community-support',
		defaultMessage: '社区与支持',
	},
	aboutQqGroup: {
		id: 'axolotl-settings.about.qq-group',
		defaultMessage: '玩家 QQ 群',
	},
	aboutCopyQqGroup: {
		id: 'axolotl-settings.about.copy-qq-group',
		defaultMessage: '复制群号',
	},
	aboutCopiedQqGroup: {
		id: 'axolotl-settings.about.copied-qq-group',
		defaultMessage: '群号已复制',
	},
	aboutAfdian: {
		id: 'axolotl-settings.about.afdian',
		defaultMessage: '在爱发电赞助',
	},
	aboutAfdianDescription: {
		id: 'axolotl-settings.about.afdian-description',
		defaultMessage: '帮助支持持续开发',
	},
	aboutOriginalSource: {
		id: 'axolotl-settings.about.original-source',
		defaultMessage: '查看原始 Modrinth 源代码',
	},
	aboutProjectWebsite: {
		id: 'axolotl-settings.about.project-website',
		defaultMessage: '访问项目官网',
	},
	aboutPclSource: {
		id: 'axolotl-settings.about.pcl-source',
		defaultMessage: '查看 Plain Craft Launcher 源代码与许可证',
	},
	aboutMcModWebsite: {
		id: 'axolotl-settings.about.mcmod-website',
		defaultMessage: '访问 MC 百科',
	},
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
	if (isOpen) activeTab.value = 'appearance'
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
									:class="{ selected: activeTab === 'appearance' }"
									@click="activeTab = 'appearance'"
								>
									<PaintbrushIcon aria-hidden="true" />
									{{ formatMessage(messages.appearanceTitle) }}
								</button>
								<button
									:class="{ selected: activeTab === 'about' }"
									@click="activeTab = 'about'"
								>
									<svg aria-hidden="true" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>
									{{ formatMessage(messages.aboutTitle) }}
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
							<!-- Appearance tab -->
							<section v-if="activeTab === 'appearance'" class="settings-pane">
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

							<!-- About tab -->
							<section v-else-if="activeTab === 'about'" class="settings-pane about-pane">
								<div class="about-header">
									<img class="about-logo" src="/ghastling.png" alt="" />
									<div>
										<h3 class="about-product-title">
											{{ formatMessage(messages.aboutProductTitle) }}
										</h3>
										<p class="about-version">
											{{ formatMessage(messages.aboutVersion, { version: ghastlingVersion }) }}
										</p>
									</div>
								</div>

								<div class="about-card">
									<p>{{ formatMessage(messages.aboutDeveloper) }}</p>
									<p>{{ formatMessage(messages.aboutAttribution) }}</p>
									<p>{{ formatMessage(messages.aboutContentSearchAttribution) }}</p>
								</div>

								<div class="about-section">
									<h4 class="about-section-title">
										{{ formatMessage(messages.aboutCommunitySupport) }}
									</h4>
									<div class="about-community-grid">
										<button
											type="button"
											:disabled="copied"
											:aria-label="copied ? formatMessage(messages.aboutCopiedQqGroup) : formatMessage(messages.aboutCopyQqGroup)"
											class="about-community-card"
											@click="copyQqGroupNumber"
										>
											<span class="about-community-icon qq-icon">
												<svg viewBox="0 0 448 512" aria-hidden="true"><path fill="currentColor" d="M433.754 420.445c-11.526 1.393-44.86-52.741-44.86-52.741c0 31.345-16.136 72.247-51.051 101.786c16.842 5.192 54.843 19.167 45.803 34.421c-7.316 12.343-125.51 7.881-159.632 4.037c-34.122 3.844-152.316 8.306-159.632-4.037c-9.045-15.25 28.918-29.214 45.783-34.415c-34.92-29.539-51.059-70.445-51.059-101.792c0 0-33.334 54.134-44.859 52.741c-5.37-.65-12.424-29.644 9.347-99.704c10.261-33.024 21.995-60.478 40.144-105.779C60.683 98.063 108.982.006 224 0c113.737.006 163.156 96.133 160.264 214.963c18.118 45.223 29.912 72.85 40.144 105.778c21.768 70.06 14.716 99.053 9.346 99.704"/></svg>
											</span>
											<span class="about-community-text">
												<strong>{{ formatMessage(messages.aboutQqGroup) }}</strong>
												<span>{{ qqGroupNumber }}</span>
											</span>
											<span class="about-community-action" aria-live="polite">
												<svg v-if="copied" class="check-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
												<svg v-else class="copy-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
											</span>
										</button>

										<a
											:href="sponsorUrl"
											target="_blank"
											rel="noopener noreferrer"
											class="about-community-card"
										>
											<span class="about-community-icon afdian-icon">
												<img src="~/assets/images/external/afdian.png" alt="" />
											</span>
											<span class="about-community-text">
												<strong>{{ formatMessage(messages.aboutAfdian) }}</strong>
												<span>{{ formatMessage(messages.aboutAfdianDescription) }}</span>
											</span>
											<svg class="about-community-external" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>
										</a>
									</div>
								</div>

								<div class="about-links">
									<a href="https://github.com/modrinth/code" target="_blank" rel="noopener noreferrer">
										{{ formatMessage(messages.aboutOriginalSource) }}
										<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>
									</a>
									<a :href="websiteUrl" target="_blank" rel="noopener noreferrer">
										{{ formatMessage(messages.aboutProjectWebsite) }}
										<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>
									</a>
									<a href="https://github.com/Meloong-Git/PCL/tree/fd7b722346523d9574678a8a4a02928d31cd1e0c" target="_blank" rel="noopener noreferrer">
										{{ formatMessage(messages.aboutPclSource) }}
										<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>
									</a>
									<a href="https://www.mcmod.cn/" target="_blank" rel="noopener noreferrer">
										{{ formatMessage(messages.aboutMcModWebsite) }}
										<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>
									</a>
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

/* About tab styles */
.about-pane {
	display: flex;
	flex-direction: column;
	gap: 1.5rem;
}

.about-header {
	display: flex;
	align-items: center;
	gap: 1rem;

	.about-logo {
		width: 5rem;
		height: 5rem;
		object-fit: contain;
		flex-shrink: 0;
	}

	.about-product-title {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--color-contrast);
	}

	.about-version {
		margin: 0.25rem 0 0;
		color: var(--color-secondary);
	}
}

.about-card {
	display: flex;
	flex-direction: column;
	gap: 0.75rem;
	padding: 1rem;
	border-radius: 0.75rem;
	background: var(--color-bg);

	p {
		margin: 0;
		color: var(--color-base);
		font-size: 0.9rem;
		line-height: 1.5;
	}
}

.about-section-title {
	margin: 0 0 0.75rem;
	font-size: 1rem;
	font-weight: 700;
	color: var(--color-contrast);
}

.about-community-grid {
	display: grid;
	gap: 0.75rem;
	grid-template-columns: repeat(auto-fill, minmax(min(100%, 16rem), 1fr));
}

.about-community-card {
	display: flex;
	align-items: center;
	gap: 0.75rem;
	padding: 1rem;
	border: 0;
	border-radius: 0.75rem;
	background: var(--color-bg);
	color: inherit;
	font: inherit;
	text-align: left;
	text-decoration: none;
	transition: background 120ms ease;
	cursor: pointer;

	&:hover {
		background: var(--color-button-bg);
	}

	&:disabled {
		cursor: default;
	}
}

.about-community-icon {
	display: flex;
	align-items: center;
	justify-content: center;
	width: 2.5rem;
	height: 2.5rem;
	flex-shrink: 0;
	border-radius: 0.75rem;
	background: var(--color-raised-bg);

	svg {
		width: 1.5rem;
		height: 1.5rem;
	}
}

.qq-icon {
	color: var(--color-contrast);
}

.afdian-icon {
	img {
		width: 1.75rem;
		height: 1.75rem;
		object-fit: contain;
	}
}

.about-community-text {
	display: flex;
	min-width: 0;
	flex: 1;
	flex-direction: column;
	gap: 0.15rem;

	strong {
		color: var(--color-contrast);
		font-size: 0.9rem;
	}

	span {
		color: var(--color-secondary);
		font-size: 0.8rem;
	}
}

.about-community-action {
	flex-shrink: 0;

	.check-icon {
		width: 1.25rem;
		height: 1.25rem;
		color: var(--color-green);
	}

	.copy-icon {
		width: 1.25rem;
		height: 1.25rem;
		color: var(--color-secondary);
	}
}

.about-community-external {
	width: 1.25rem;
	height: 1.25rem;
	flex-shrink: 0;
	color: var(--color-secondary);
}

.about-links {
	display: flex;
	flex-direction: column;
	align-items: flex-start;
	gap: 0.75rem;

	a {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		font-weight: 600;
		font-size: 0.9rem;
		color: var(--color-brand);
		text-decoration: none;

		&:hover {
			text-decoration: underline;
		}

		svg {
			width: 1rem;
			height: 1rem;
		}
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
