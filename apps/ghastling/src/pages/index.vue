<script setup lang="ts">
import { DownloadIcon } from '@modrinth/assets'
import CompassIcon from '@modrinth/assets/icons/compass.svg?component'
import EyeOffIcon from '@modrinth/assets/icons/eye-off.svg?component'
import LanguagesIcon from '@modrinth/assets/icons/languages.svg?component'
import SearchIcon from '@modrinth/assets/icons/search.svg?component'
import TrashIcon from '@modrinth/assets/icons/trash.svg?component'
import Avatar from '@modrinth/ui/src/components/base/Avatar.vue'
import ButtonStyled from '@modrinth/ui/src/components/base/ButtonStyled.vue'
import Checkbox from '@modrinth/ui/src/components/base/Checkbox.vue'
import IntlFormatted from '@modrinth/ui/src/components/base/IntlFormatted.vue'
import { defineMessages, useVIntl } from '@modrinth/ui/src/composables/i18n.ts'
import { useLatestRelease } from '~/composables/use-latest-release'

const { downloadUrl: latestDownloadUrl } = useLatestRelease()

const modManagementData = [
	{
		id: 'P7dR8mSH',
		name: 'Fabric API',
		author: 'modmuss50',
		version: '0.86.1+1.20.1',
		iconUrl: 'https://cdn.modrinth.com/data/P7dR8mSH/icon.png',
	},
	{
		id: 'AANobbMI',
		name: 'Sodium',
		author: 'jellysquid3',
		version: 'mc1.20.1-0.5.0',
		iconUrl: 'https://cdn.modrinth.com/data/AANobbMI/icon.png',
	},
	{
		id: 'YL57xq9U',
		name: 'Iris Shaders',
		author: 'coderbot',
		version: '1.6.5+1.20.1',
		iconUrl: 'https://cdn.modrinth.com/data/YL57xq9U/dc558eece920db435f9823ce86de0c4cde89800b.png',
	},
	{
		id: 'gvQqBUqZ',
		name: 'Lithium',
		author: 'jellysquid3',
		version: 'mc1.20.1-0.11.2',
		iconUrl: 'https://cdn.modrinth.com/data/gvQqBUqZ/icon.png',
	},
	{
		id: 'mOgUt4GM',
		name: 'Mod Menu',
		author: 'Prospector',
		version: '7.2.1',
		iconUrl: 'https://cdn.modrinth.com/data/mOgUt4GM/5a20ed1450a0e1e79a1fe04e61bb4e5878bf1d20.png',
	},
	{
		id: '9s6osm5g',
		name: 'Cloth Config API',
		author: 'shedaniel',
		version: '11.1.106+fabric',
		iconUrl: 'https://cdn.modrinth.com/data/9s6osm5g/icon.png',
	},
	{
		id: 'lhGA9TYQ',
		name: 'Architectury API',
		author: 'shedaniel',
		version: '9.1.12+fabric',
		iconUrl: 'https://cdn.modrinth.com/data/lhGA9TYQ/icon.png',
	},
	{
		id: 'nrJ2NpD0',
		name: 'Craftify',
		author: 'ThatGravyBoat',
		version: '8.5.2023',
		iconUrl: 'https://cdn.modrinth.com/data/nrJ2NpD0/4f21214db060ed4542b1f3983c4113d293480a1b.webp',
	},
]

const newProjects = Array.from({ length: 40 }, (_, index) => {
	const project = modManagementData[index % modManagementData.length]

	return {
		id: `${project.id}-${index}`,
		icon_url: project.iconUrl,
		title: project.name,
		description: `${project.name} is ready to explore on Modrinth.`,
	}
})
const val = Math.ceil(newProjects.length / 6)
const rows = [
	newProjects.slice(0, val),
	newProjects.slice(val, val * 2),
	newProjects.slice(val * 2, val * 3),
	newProjects.slice(val * 3, val * 4),
	newProjects.slice(val * 4, val * 5),
]

const { formatMessage, locale } = useVIntl()

const messages = defineMessages({
	openSourceBadge: {
		id: 'app-marketing.features.open-source.title',
		defaultMessage: '开放源代码',
	},
	oneLauncher: {
		id: 'ghastling-marketing.demo.one-launcher',
		defaultMessage: '一个启动器，承载每一个世界。',
	},
	everythingTogether: {
		id: 'ghastling-marketing.demo.everything-together',
		defaultMessage: '配置、模组、存档和设置始终井然有序。',
	},
	includedMods: {
		id: 'ghastling-marketing.demo.included-mods',
		defaultMessage: '包含的模组',
	},
	downloadAxolotlForWindows: {
		id: 'ghastling-marketing.hero.download-for-windows',
		defaultMessage: '适用于 Windows 的 Ghastling Launcher',
	},
	description: {
		id: 'app-marketing.hero.description',
		defaultMessage:
			'基于 Modrinth, Axolotl 进行二次开发，Ghastling 提供更纯粹、无广告的 Minecraft 体验，让探索、翻译与游玩都按照你的方式进行。',
	},
	heroDownload: {
		id: 'ghastling-marketing.hero.download-button',
		defaultMessage: '下载 Ghastling Launcher',
	},
	heroScreenshotAlt: {
		id: 'ghastling-marketing.hero.screenshot-alt',
		defaultMessage: 'Ghastling Launcher 主界面。',
	},
	builtOnModrinth: {
		id: 'ghastling-marketing.highlights.eyebrow',
		defaultMessage: '基于 Modrinth, Axolotl 进行二次开发',
	},
	highlightsTitle: {
		id: 'ghastling-marketing.highlights.title',
		defaultMessage: '基于 Modrinth 的与众不同启动器，带来前所未有的体验。',
	},
	highlightsDescription: {
		id: 'ghastling-marketing.highlights.description',
		defaultMessage:
			'通过 Modrinth 探索 Minecraft，再用围绕玩家打造的贴心功能，让启动器真正成为你的空间。',
	},
	modrinthNative: {
		id: 'ghastling-marketing.highlights.modrinth.title',
		defaultMessage: '以 Modrinth 为核心',
	},
	modrinthNativeDescription: {
		id: 'ghastling-marketing.highlights.modrinth.description',
		defaultMessage:
			'无需离开启动器，即可从 Modrinth 生态中探索模组、整合包、资源包、光影及更多内容。',
	},
	adFree: {
		id: 'ghastling-marketing.highlights.ad-free.title',
		defaultMessage: '没有广告，没有干扰',
	},
	adFreeDescription: {
		id: 'ghastling-marketing.highlights.ad-free.description',
		defaultMessage: '每一处界面都留给你的世界，而不是广告。',
	},
	localized: {
		id: 'ghastling-marketing.highlights.localized.title',
		defaultMessage: '真正面向玩家的本地化',
	},
	localizedDescription: {
		id: 'ghastling-marketing.highlights.localized.description',
		defaultMessage: '接近 99% 的 Ghastling 界面已完成本地化，提供完整中文体验。',
	},
	offlineAccounts: {
		id: 'ghastling-marketing.showcase.offline.title',
		defaultMessage: '无需正版账号，也可从离线账户开始',
	},
	offlineAccountsDescription: {
		id: 'ghastling-marketing.showcase.offline.description',
		defaultMessage: '创建本地身份；无需正版账号也能开始游玩。',
	},
	offlineLabel: { id: 'ghastling-marketing.showcase.offline.label', defaultMessage: '离线账户' },
	themes: {
		id: 'ghastling-marketing.showcase.themes.title',
		defaultMessage: '每套设备，都有适合的色彩主题',
	},
	themesDescription: {
		id: 'ghastling-marketing.showcase.themes.description',
		defaultMessage: '选择你自己心仪的强调色',
	},
	personalizeLabel: {
		id: 'ghastling-marketing.showcase.themes.label',
		defaultMessage: '个性化',
	},
	translation: {
		id: 'ghastling-marketing.showcase.translation.title',
		defaultMessage: '探索时，直接翻译 Modrinth 内容',
	},
	translationDescription: {
		id: 'ghastling-marketing.showcase.translation.description',
		defaultMessage: '让探索不被语言阻隔',
	},
	translateLabel: {
		id: 'ghastling-marketing.showcase.translation.label',
		defaultMessage: '翻译',
	},
	offlineScreenshotAlt: {
		id: 'ghastling-marketing.showcase.offline.alt',
		defaultMessage: 'Ghastling Launcher 的离线账户添加窗口。',
	},
	themesScreenshotAlt: {
		id: 'ghastling-marketing.showcase.themes.alt',
		defaultMessage: 'Ghastling Launcher 的主题自定义设置。',
	},
	translationScreenshotAlt: {
		id: 'ghastling-marketing.showcase.translation.alt',
		defaultMessage: 'Ghastling Launcher 的 Modrinth 翻译设置。',
	},
	unlikeAnyLauncher: {
		id: 'app-marketing.features.unlike-any-launcher',
		defaultMessage: '与众不同的启动器',
	},
	youveUsedBefore: {
		id: 'app-marketing.features.youve-used-before',
		defaultMessage: '带来前所未有的体验',
	},
	installedMods: {
		id: 'app-marketing.features.mod-management.installed-mods',
		defaultMessage: '已安装模组',
	},
	searchMods: {
		id: 'app-marketing.features.mod-management.search-mods',
		defaultMessage: '搜索模组',
	},
	name: {
		id: 'app-marketing.features.mod-management.name',
		defaultMessage: '名称',
	},
	version: {
		id: 'app-marketing.features.mod-management.version',
		defaultMessage: '版本',
	},
	actions: {
		id: 'app-marketing.features.mod-management.actions',
		defaultMessage: '操作',
	},
	byAuthor: {
		id: 'app-marketing.features.mod-management.byAuthor',
		defaultMessage: '作者：{author}',
	},
	modManagement: {
		id: 'app-marketing.features.mod-management.title',
		defaultMessage: '模组管理',
	},
	modManagementDescription: {
		id: 'app-marketing.features.mod-management.description',
		defaultMessage: 'Ghastling 将模组、版本和实例整理得井井有条，让安装、移除与更新内容变得轻松。',
	},
	playWithFavoriteMods: {
		id: 'app-marketing.features.play.title',
		defaultMessage: '畅玩你喜爱的模组',
	},
	playWithFavoriteModsDescription: {
		id: 'app-marketing.features.play.description',
		defaultMessage: '创建实例、加入喜爱的内容，只需几次点击即可进入 Minecraft。',
	},
	shareModpacks: {
		id: 'app-marketing.features.sharing.title',
		defaultMessage: '分享整合包',
	},
	shareModpacksDescription: {
		id: 'app-marketing.features.sharing.description',
		defaultMessage: '轻松创建、导出并分享整合包，同时完整保留依赖与版本信息。',
	},
	share: {
		id: 'app-marketing.features.sharing.share-button',
		defaultMessage: '分享',
	},
	modpack: {
		id: 'app-marketing.features.sharing.modpack',
		defaultMessage: '整合包',
	},
	activityMonitor: {
		id: 'app-marketing.features.performance.activity-monitor',
		defaultMessage: '活动监视器',
	},
	goodPerformance: {
		id: 'app-marketing.features.performance.good-performance',
		defaultMessage: '性能良好',
	},
	processName: {
		id: 'app-marketing.features.performance.process-name',
		defaultMessage: '进程名称',
	},
	cpuPercent: {
		id: 'app-marketing.features.performance.cpu-percent',
		defaultMessage: 'CPU 占用',
	},
	ram: {
		id: 'app-marketing.features.performance.ram',
		defaultMessage: '内存',
	},
	axolotlApp: {
		id: 'ghastling-marketing.features.performance.ghastling-app',
		defaultMessage: 'Ghastling Launcher',
	},
	small: {
		id: 'app-marketing.features.performance.small',
		defaultMessage: '轻量',
	},
	lessThan150MB: {
		id: 'app-marketing.features.performance.less-than-150mb',
		defaultMessage: '低于 150 MB',
	},
	googleChrome: {
		id: 'app-marketing.features.performance.google-chrome',
		defaultMessage: 'Google Chrome',
	},
	discord: {
		id: 'app-marketing.features.performance.discord',
		defaultMessage: 'Discord',
	},
	infiniteMB: {
		id: 'app-marketing.features.performance.infinite-mb',
		defaultMessage: '∞ MB',
	},
	oneBillionPercent: {
		id: 'app-marketing.features.performance.one-billion-percent',
		defaultMessage: '10 亿 %',
	},
	infiniteTimesInfiniteMB: {
		id: 'app-marketing.features.performance.infinite-times-infinite-mb',
		defaultMessage: '∞ × ∞ MB',
	},
	performant: {
		id: 'app-marketing.features.performance.title',
		defaultMessage: '高性能',
	},
	performantDescription: {
		id: 'app-marketing.features.performance.description',
		defaultMessage: 'Ghastling 以灵敏的界面与轻量的桌面核心保持安静、高效，不打扰你的游戏。',
	},
	websiteIntegration: {
		id: 'app-marketing.features.website.title',
		defaultMessage: '网站集成',
	},
	websiteIntegrationDescription: {
		id: 'app-marketing.features.website.description',
		defaultMessage: '直接在启动器内浏览社区内容，从发现模组到创建可玩的实例一气呵成。',
	},
	profileImporting: {
		id: 'app-marketing.features.importing.title',
		defaultMessage: '导入配置',
	},
	profileImportingDescription: {
		id: 'app-marketing.features.importing.description',
		defaultMessage: '从常用启动器导入现有配置，无需手动重建即可继续游玩。',
	},
	openSource: {
		id: 'app-marketing.features.open-source.title',
		defaultMessage: '开放源代码',
	},
	openSourceDescription: {
		id: 'app-marketing.features.open-source.description',
		defaultMessage:
			'Ghastling 完全开源。你可以在 <github-link>GitHub</github-link> 查看代码、关注开发或参与贡献。',
	},
	offlineMode: {
		id: 'app-marketing.features.offline.title',
		defaultMessage: '离线模式',
	},
	offlineModeDescription: {
		id: 'app-marketing.features.offline.description',
		defaultMessage: '无论是否连接网络，都可以畅玩你的模组。',
	},
	followProjects: {
		id: 'app-marketing.features.follow.title',
		defaultMessage: '关注项目',
	},
	followProjectsDescription: {
		id: 'app-marketing.features.follow.description',
		defaultMessage: '收藏喜爱的内容，一键获取更新。',
	},
	windows: {
		id: 'app-marketing.download.windows',
		defaultMessage: 'Windows',
	},
	seoTitle: {
		id: 'ghastling-site.seo.title',
		defaultMessage: 'Ghastling Launcher - 免费开源的 Minecraft 启动器',
	},
	seoDescription: {
		id: 'ghastling-site.seo.description',
		defaultMessage:
			'下载 Ghastling Launcher：适用于 Windows 的免费开源、无广告 Minecraft（我的世界）启动器，支持 Modrinth 内容管理、中文翻译与离线账户。',
	},
	socialImageAlt: {
		id: 'ghastling-site.seo.social-image-alt',
		defaultMessage: 'Ghastling Launcher 正在展示一个 Minecraft 实例及其已安装内容。',
	},
	faqEyebrow: {
		id: 'ghastling-site.faq.eyebrow',
		defaultMessage: '常见问题',
	},
	faqTitle: {
		id: 'ghastling-site.faq.title',
		defaultMessage: '关于 Ghastling Launcher，你需要了解的一切',
	},
	faqDescription: {
		id: 'ghastling-site.faq.description',
		defaultMessage: '了解支持平台、账户类型、内容来源与官方下载方式。',
	},
	faqPlatformsQuestion: {
		id: 'ghastling-site.faq.platforms.question',
		defaultMessage: 'Ghastling Launcher 支持哪些操作系统？',
	},
	faqPlatformsAnswer: {
		id: 'ghastling-site.faq.platforms.answer',
		defaultMessage: 'Ghastling Launcher 仅支持 Windows 10/11 64位操作系统。',
	},
	faqFreeQuestion: {
		id: 'ghastling-site.faq.free.question',
		defaultMessage: 'Ghastling Launcher 免费且开源吗？',
	},
	faqFreeAnswer: {
		id: 'ghastling-site.faq.free.answer',
		defaultMessage:
			'是。Ghastling Launcher 是基于 GPL-3.0 发布的自由软件，源代码和版本历史均在 GitHub 上公开。',
	},
	faqAccountsQuestion: {
		id: 'ghastling-site.faq.accounts.question',
		defaultMessage: '可以使用 Microsoft 正版账户、离线账户与 little skin 账户吗？',
	},
	faqAccountsAnswer: {
		id: 'ghastling-site.faq.accounts.answer',
		defaultMessage:
			'可以。Ghastling 同时支持 Microsoft Minecraft 正版账户、离线账户与第三方账户，你可以根据自己的游玩方式选择账户类型。',
	},
	faqContentQuestion: {
		id: 'ghastling-site.faq.content.question',
		defaultMessage: 'Ghastling 从哪里获取模组和其他 Minecraft 内容？',
	},
	faqContentAnswer: {
		id: 'ghastling-site.faq.content.answer',
		defaultMessage:
			'Ghastling 使用 Modrinth 与 CurseForge 公共 API，帮助你直接在启动器中发现并管理模组、整合包、资源包、光影及其他社区内容。',
	},
	faqDownloadQuestion: {
		id: 'ghastling-site.faq.download.question',
		defaultMessage: '应该从哪里下载 Ghastling Launcher？',
	},
	faqDownloadAnswer: {
		id: 'ghastling-site.faq.download.answer',
		defaultMessage: '请点击本官方网站的下载按钮，亦或者在 GitHub 上获取。',
	},
	appScreenshotAlt: {
		id: 'app-marketing.hero.app-screenshot-alt',
		defaultMessage: `Ghastling Launcher 实例内容预览。`,
	},
	gdlauncherAlt: {
		id: 'app-marketing.features.importing.gdlauncher-alt',
		defaultMessage: 'GDLauncher',
	},
	multimcAlt: {
		id: 'app-marketing.features.importing.multimc-alt',
		defaultMessage: 'MultiMC',
	},
})

const config = useRuntimeConfig()
const siteUrl = config.public.siteUrl
const canonicalUrl = `${siteUrl}/`
const socialImageUrl = `${siteUrl}/showcase/home.png`
const githubUrl = 'https://github.com/SunkDiagram1865/Ghastling'
const licenseUrl = `${githubUrl}/blob/main/LICENSE`

const title = computed(() => formatMessage(messages.seoTitle))
const description = computed(() => formatMessage(messages.seoDescription))
const socialImageAlt = computed(() => formatMessage(messages.socialImageAlt))
const faqItems = computed(() => [
	{
		question: formatMessage(messages.faqPlatformsQuestion),
		answer: formatMessage(messages.faqPlatformsAnswer),
	},
	{
		question: formatMessage(messages.faqFreeQuestion),
		answer: formatMessage(messages.faqFreeAnswer),
	},
	{
		question: formatMessage(messages.faqAccountsQuestion),
		answer: formatMessage(messages.faqAccountsAnswer),
	},
	{
		question: formatMessage(messages.faqContentQuestion),
		answer: formatMessage(messages.faqContentAnswer),
	},
	{
		question: formatMessage(messages.faqDownloadQuestion),
		answer: formatMessage(messages.faqDownloadAnswer),
	},
])

const structuredData = computed(() => ({
	'@context': 'https://schema.org',
	'@graph': [
		{
			'@type': 'WebSite',
			'@id': `${canonicalUrl}#website`,
			url: canonicalUrl,
			name: 'Ghastling Launcher',
			description: description.value,
			inLanguage: locale.value,
			publisher: { '@id': `${canonicalUrl}#organization` },
		},
		{
			'@type': 'Organization',
			'@id': `${canonicalUrl}#organization`,
			name: 'Mystic Stars',
			url: canonicalUrl,
			logo: {
				'@type': 'ImageObject',
				url: `${siteUrl}/ghastling.png`,
				width: 256,
				height: 256,
			},
			sameAs: [githubUrl],
		},
		{
			'@type': 'SoftwareApplication',
			'@id': `${canonicalUrl}#software`,
			name: 'Ghastling Launcher',
			description: description.value,
			url: canonicalUrl,
			downloadUrl: `${canonicalUrl}#download`,
			image: socialImageUrl,
			applicationCategory: 'GameApplication',
			applicationSubCategory: 'Minecraft Launcher',
			operatingSystem: 'Windows 10/11',
			isAccessibleForFree: true,
			license: licenseUrl,
			softwareHelp: `${githubUrl}#readme`,
			author: { '@id': `${canonicalUrl}#organization` },
			inLanguage: ['zh-CN', 'en-US'],
		},
		{
			'@type': 'FAQPage',
			'@id': `${canonicalUrl}#faq`,
			inLanguage: locale.value,
			mainEntity: faqItems.value.map((item) => ({
				'@type': 'Question',
				name: item.question,
				acceptedAnswer: {
					'@type': 'Answer',
					text: item.answer,
				},
			})),
		},
	],
}))

useSeoMeta({
	title: () => title.value,
	description: () => description.value,
	robots: 'index, follow, max-image-preview:large, max-snippet:-1, max-video-preview:-1',
	author: 'Mystic Stars',
	applicationName: 'Ghastling Launcher',
	themeColor: '#39c5bb',
	colorScheme: 'dark light',
	ogTitle: () => title.value,
	ogDescription: () => description.value,
	ogType: 'website',
	ogUrl: canonicalUrl,
	ogSiteName: 'Ghastling Launcher',
	ogLocale: () => locale.value.replace('-', '_'),
	ogLocaleAlternate: () => (locale.value === 'zh-CN' ? 'en_US' : 'zh_CN'),
	ogImage: socialImageUrl,
	ogImageAlt: () => socialImageAlt.value,
	ogImageWidth: 1600,
	ogImageHeight: 871,
	twitterCard: 'summary_large_image',
	twitterTitle: () => title.value,
	twitterDescription: () => description.value,
	twitterImage: socialImageUrl,
	twitterImageAlt: () => socialImageAlt.value,
})

useHead(() => ({
	link: [{ rel: 'canonical', href: canonicalUrl }],
	script: [
		{
			key: 'ghastling-structured-data',
			type: 'application/ld+json',
			innerHTML: JSON.stringify(structuredData.value).replace(/</g, '\\u003c'),
		},
	],
}))
</script>

<template>
	<div>
		<div class="landing-hero">
			<div
				class="hero-kicker relative mt-4 h-fit w-fit rounded-full px-3 py-1 text-sm font-bold text-brand backdrop-blur-lg"
			>
				{{ formatMessage(messages.openSourceBadge) }}
			</div>
			<h1 class="main-header max-w-[60rem]">
				{{ formatMessage(messages.downloadAxolotlForWindows) }}
			</h1>
			<p class="main-subheader">
				{{ formatMessage(messages.description) }}
			</p>

			<a
			class="hero-download"
			:href="latestDownloadUrl"
			target="_blank"
			rel="noopener"
		>
			<DownloadIcon aria-hidden="true" />
			{{ formatMessage(messages.heroDownload) }}
		</a>

			<img
				class="hero-screenshot"
				src="/showcase/home.png"
				:alt="formatMessage(messages.heroScreenshotAlt)"
				width="1600"
				height="871"
				decoding="async"
				fetchpriority="high"
			/>
			<div class="bottom-transition" />
		</div>
		<section id="features" class="ghastling-highlights" aria-labelledby="ghastling-highlights-title">
			<div class="highlights-intro">
				<span class="section-eyebrow">{{ formatMessage(messages.builtOnModrinth) }}</span>
				<h2 id="ghastling-highlights-title">{{ formatMessage(messages.highlightsTitle) }}</h2>
				<p>{{ formatMessage(messages.highlightsDescription) }}</p>
			</div>

			<div class="modrinth-feature-grid">
				<article class="feature gradient-border promise-card">
					<div class="promise-meta"><CompassIcon /><span>01</span></div>
					<h3>{{ formatMessage(messages.modrinthNative) }}</h3>
					<p>{{ formatMessage(messages.modrinthNativeDescription) }}</p>
				</article>
				<article class="feature gradient-border promise-card">
					<div class="promise-meta"><EyeOffIcon /><span>02</span></div>
					<h3>{{ formatMessage(messages.adFree) }}</h3>
					<p>{{ formatMessage(messages.adFreeDescription) }}</p>
				</article>
				<article class="feature gradient-border promise-card">
					<div class="promise-meta"><LanguagesIcon /><span>03</span></div>
					<h3>{{ formatMessage(messages.localized) }}</h3>
					<p>{{ formatMessage(messages.localizedDescription) }}</p>
				</article>
				<article class="feature gradient-border showcase-card showcase-card-wide">
					<div class="showcase-copy">
						<span>{{ formatMessage(messages.offlineLabel) }}</span>
						<h3>{{ formatMessage(messages.offlineAccounts) }}</h3>
						<p>{{ formatMessage(messages.offlineAccountsDescription) }}</p>
					</div>
					<div class="showcase-image-frame">
						<img
							class="showcase-image light-shot"
							src="/showcase/offline.png"
							:alt="formatMessage(messages.offlineScreenshotAlt)"
							width="1600"
							height="871"
							decoding="async"
							loading="lazy"
						/>
						<img
							class="showcase-image dark-shot"
							src="/showcase/offline.png"
							:alt="formatMessage(messages.offlineScreenshotAlt)"
							width="1600"
							height="871"
							decoding="async"
							loading="lazy"
						/>
					</div>
				</article>

				<article class="feature gradient-border showcase-card">
					<div class="showcase-copy">
						<span>{{ formatMessage(messages.personalizeLabel) }}</span>
						<h3>{{ formatMessage(messages.themes) }}</h3>
						<p>{{ formatMessage(messages.themesDescription) }}</p>
					</div>
					<div class="showcase-image-frame">
						<img
							class="showcase-image light-shot"
							src="/showcase/theme.png"
							:alt="formatMessage(messages.themesScreenshotAlt)"
							width="1600"
							height="871"
							decoding="async"
							loading="lazy"
						/>
						<img
							class="showcase-image dark-shot"
							src="/showcase/theme.png"
							:alt="formatMessage(messages.themesScreenshotAlt)"
							width="1600"
							height="871"
							decoding="async"
							loading="lazy"
						/>
					</div>
				</article>

				<article class="feature gradient-border showcase-card">
					<div class="showcase-copy">
						<span>{{ formatMessage(messages.translateLabel) }}</span>
						<h3>{{ formatMessage(messages.translation) }}</h3>
						<p>{{ formatMessage(messages.translationDescription) }}</p>
					</div>
					<div class="showcase-image-frame">
						<img
							class="showcase-image light-shot"
							src="/showcase/translation.png"
							:alt="formatMessage(messages.translationScreenshotAlt)"
							width="1600"
							height="871"
							decoding="async"
							loading="lazy"
						/>
						<img
							class="showcase-image dark-shot"
							src="/showcase/translation.png"
							:alt="formatMessage(messages.translationScreenshotAlt)"
							width="1600"
							height="871"
							decoding="async"
							loading="lazy"
						/>
					</div>
				</article>
			</div>
		</section>
		<div id="features" class="features">
			<div class="feature-grid">
				<div class="feature gradient-border mods">
					<div class="search-bar">
						<h4>{{ formatMessage(messages.installedMods) }}</h4>
						<div class="mini-input">
							<SearchIcon aria-hidden="true" />
							<div class="search">{{ formatMessage(messages.searchMods) }}</div>
						</div>
					</div>
					<div class="header row">
						<div />
						<div class="cell">{{ formatMessage(messages.name) }}</div>
						<div class="cell">{{ formatMessage(messages.version) }}</div>
						<div class="cell">{{ formatMessage(messages.actions) }}</div>
					</div>
					<div class="table">
						<div
							v-for="(mod, index) in modManagementData"
							:key="mod.id"
							:class="['row', { first: index === 0 }]"
						>
							<div class="cell">
								<Avatar size="sm" :src="mod.iconUrl" />
							</div>
							<div class="cell">
								<div class="name">{{ mod.name }}</div>
								<div class="description">
									{{ formatMessage(messages.byAuthor, { author: mod.author }) }}
								</div>
							</div>
							<div class="cell">{{ mod.version }}</div>
							<div class="cell check">
								<Checkbox :model-value="true" tabindex="-1" />
								<ButtonStyled circular type="transparent">
									<button tabindex="-1">
										<TrashIcon />
									</button>
								</ButtonStyled>
							</div>
						</div>
					</div>
					<h3>{{ formatMessage(messages.modManagement) }}</h3>
					<p>
						{{ formatMessage(messages.modManagementDescription) }}
					</p>
				</div>
				<div class="feature gradient-border website">
					<img class="website-logo" src="/ghastling.png" alt="" aria-hidden="true" />
					<div class="projects-showcase">
						<div v-for="(row, index) in rows" :key="index" class="row">
							<div v-for="n in 2" :key="n" class="row__content" :class="{ offset: index % 2 }">
								<div
									v-for="project in row"
									:key="project.id"
									class="project button-animation gradient-border"
								>
									<Avatar :src="project.icon_url!" alt="" size="sm" />
									<div class="project-info">
										<span class="title">
											{{ project.title }}
										</span>
										<span class="description">
											{{ project.description }}
										</span>
									</div>
								</div>
							</div>
						</div>
					</div>
					<h3>{{ formatMessage(messages.websiteIntegration) }}</h3>
					<p>
						{{ formatMessage(messages.websiteIntegrationDescription) }}
					</p>
				</div>
			</div>
			<div class="feature-row">
				<div class="point">
					<div class="title">
						<svg
							class="dark-icon"
							xmlns="http://www.w3.org/2000/svg"
							width="24"
							height="24"
							viewBox="0 0 24 24"
							fill="none"
						>
							<path
								d="M21 8.25C21 8.70312 20.9258 9.13281 20.7773 9.53906C20.6289 9.94531 20.4141 10.3125 20.1328 10.6406C19.8516 10.9688 19.5273 11.2461 19.1602 11.4727C18.793 11.6992 18.3828 11.8516 17.9297 11.9297C17.8438 12.375 17.6914 12.7852 17.4727 13.1602C17.2539 13.5352 16.9766 13.8594 16.6406 14.1328C16.3047 14.4062 15.9375 14.6172 15.5391 14.7656C15.1406 14.9141 14.7109 14.9922 14.25 15H9.75C9.5 15 9.26172 15.0391 9.03516 15.1172C8.80859 15.1953 8.60156 15.3047 8.41406 15.4453C8.22656 15.5859 8.0625 15.7539 7.92188 15.9492C7.78125 16.1445 7.67578 16.3633 7.60547 16.6055C8.02734 16.707 8.41406 16.8711 8.76562 17.0977C9.11719 17.3242 9.42188 17.6016 9.67969 17.9297C9.9375 18.2578 10.1367 18.6172 10.2773 19.0078C10.418 19.3984 10.4922 19.8125 10.5 20.25C10.5 20.7656 10.4023 21.25 10.207 21.7031C10.0117 22.1562 9.74219 22.5547 9.39844 22.8984C9.05469 23.2422 8.65625 23.5117 8.20312 23.707C7.75 23.9023 7.26562 24 6.75 24C6.23438 24 5.75 23.9023 5.29688 23.707C4.84375 23.5117 4.44531 23.2461 4.10156 22.9102C3.75781 22.5742 3.48828 22.1758 3.29297 21.7148C3.09766 21.2539 3 20.7656 3 20.25C3 19.8047 3.07422 19.3789 3.22266 18.9727C3.37109 18.5664 3.57812 18.2031 3.84375 17.8828C4.10938 17.5625 4.42578 17.2852 4.79297 17.0508C5.16016 16.8164 5.5625 16.6602 6 16.582V7.41797C5.5625 7.33203 5.16016 7.17578 4.79297 6.94922C4.42578 6.72266 4.10938 6.44922 3.84375 6.12891C3.57812 5.80859 3.37109 5.44141 3.22266 5.02734C3.07422 4.61328 3 4.1875 3 3.75C3 3.23438 3.09766 2.75 3.29297 2.29688C3.48828 1.84375 3.75391 1.44922 4.08984 1.11328C4.42578 0.777344 4.82422 0.507812 5.28516 0.304688C5.74609 0.101562 6.23438 0 6.75 0C7.26562 0 7.75 0.0976563 8.20312 0.292969C8.65625 0.488281 9.05078 0.757812 9.38672 1.10156C9.72266 1.44531 9.99219 1.84375 10.1953 2.29688C10.3984 2.75 10.5 3.23438 10.5 3.75C10.5 4.19531 10.4258 4.62109 10.2773 5.02734C10.1289 5.43359 9.92188 5.79688 9.65625 6.11719C9.39062 6.4375 9.07422 6.71484 8.70703 6.94922C8.33984 7.18359 7.9375 7.33984 7.5 7.41797V14.2734C7.82812 14.0234 8.18359 13.832 8.56641 13.6992C8.94922 13.5664 9.34375 13.5 9.75 13.5H14.25C14.5 13.5 14.7383 13.4609 14.9648 13.3828C15.1914 13.3047 15.3984 13.1953 15.5859 13.0547C15.7734 12.9141 15.9375 12.7461 16.0781 12.5508C16.2188 12.3555 16.3242 12.1367 16.3945 11.8945C15.9727 11.793 15.5859 11.6289 15.2344 11.4023C14.8828 11.1758 14.5781 10.9023 14.3203 10.582C14.0625 10.2617 13.8633 9.90234 13.7227 9.50391C13.582 9.10547 13.5078 8.6875 13.5 8.25C13.5 7.73438 13.5977 7.25 13.793 6.79688C13.9883 6.34375 14.2539 5.94922 14.5898 5.61328C14.9258 5.27734 15.3242 5.00781 15.7852 4.80469C16.2461 4.60156 16.7344 4.5 17.25 4.5C17.7656 4.5 18.25 4.59766 18.7031 4.79297C19.1562 4.98828 19.5508 5.25781 19.8867 5.60156C20.2227 5.94531 20.4922 6.34375 20.6953 6.79688C20.8984 7.25 21 7.73438 21 8.25ZM4.5 3.75C4.5 4.0625 4.55859 4.35547 4.67578 4.62891C4.79297 4.90234 4.95312 5.14062 5.15625 5.34375C5.35938 5.54688 5.59766 5.70703 5.87109 5.82422C6.14453 5.94141 6.4375 6 6.75 6C7.0625 6 7.35547 5.94141 7.62891 5.82422C7.90234 5.70703 8.14062 5.54688 8.34375 5.34375C8.54688 5.14062 8.70703 4.90234 8.82422 4.62891C8.94141 4.35547 9 4.0625 9 3.75C9 3.4375 8.94141 3.14453 8.82422 2.87109C8.70703 2.59766 8.54688 2.35938 8.34375 2.15625C8.14062 1.95312 7.90234 1.79297 7.62891 1.67578C7.35547 1.55859 7.0625 1.5 6.75 1.5C6.4375 1.5 6.14453 1.55859 5.87109 1.67578C5.59766 1.79297 5.35938 1.95312 5.15625 2.15625C4.95312 2.35938 4.79297 2.59766 4.67578 2.87109C4.55859 3.14453 4.5 3.4375 4.5 3.75ZM9 20.25C9 19.9375 8.94141 19.6445 8.82422 19.3711C8.70703 19.0977 8.54688 18.8594 8.34375 18.6562C8.14062 18.4531 7.90234 18.293 7.62891 18.1758C7.35547 18.0586 7.0625 18 6.75 18C6.4375 18 6.14453 18.0586 5.87109 18.1758C5.59766 18.293 5.35938 18.4531 5.15625 18.6562C4.95312 18.8594 4.79297 19.0977 4.67578 19.3711C4.55859 19.6445 4.5 19.9375 4.5 20.25C4.5 20.5625 4.55859 20.8555 4.67578 21.1289C4.79297 21.4023 4.95312 21.6406 5.15625 21.8438C5.35938 22.0469 5.59766 22.207 5.87109 22.3242C6.14453 22.4414 6.4375 22.5 6.75 22.5C7.0625 22.5 7.35547 22.4414 7.62891 22.3242C7.90234 22.207 8.14062 22.0469 8.34375 21.8438C8.54688 21.6406 8.70703 21.4023 8.82422 21.1289C8.94141 20.8555 9 20.5625 9 20.25ZM17.25 10.5C17.5625 10.5 17.8555 10.4414 18.1289 10.3242C18.4023 10.207 18.6406 10.0469 18.8438 9.84375C19.0469 9.64062 19.207 9.40234 19.3242 9.12891C19.4414 8.85547 19.5 8.5625 19.5 8.25C19.5 7.9375 19.4414 7.64453 19.3242 7.37109C19.207 7.09766 19.0469 6.85938 18.8438 6.65625C18.6406 6.45312 18.4023 6.29297 18.1289 6.17578C17.8555 6.05859 17.5625 6 17.25 6C16.9375 6 16.6445 6.05859 16.3711 6.17578C16.0977 6.29297 15.8594 6.45312 15.6562 6.65625C15.4531 6.85938 15.293 7.09766 15.1758 7.37109C15.0586 7.64453 15 7.9375 15 8.25C15 8.5625 15.0586 8.85547 15.1758 9.12891C15.293 9.40234 15.4531 9.64062 15.6562 9.84375C15.8594 10.0469 16.0977 10.207 16.3711 10.3242C16.6445 10.4414 16.9375 10.5 17.25 10.5Z"
								fill="currentColor"
							/>
						</svg>
						<svg
							class="light-icon"
							xmlns="http://www.w3.org/2000/svg"
							width="24"
							height="24"
							viewBox="0 0 24 24"
							fill="none"
						>
							<path
								d="M21 8.25C21 8.70312 20.9258 9.13281 20.7773 9.53906C20.6289 9.94531 20.4141 10.3125 20.1328 10.6406C19.8516 10.9688 19.5273 11.2461 19.1602 11.4727C18.793 11.6992 18.3828 11.8516 17.9297 11.9297C17.8438 12.375 17.6914 12.7852 17.4727 13.1602C17.2539 13.5352 16.9766 13.8594 16.6406 14.1328C16.3047 14.4062 15.9375 14.6172 15.5391 14.7656C15.1406 14.9141 14.7109 14.9922 14.25 15H9.75C9.5 15 9.26172 15.0391 9.03516 15.1172C8.80859 15.1953 8.60156 15.3047 8.41406 15.4453C8.22656 15.5859 8.0625 15.7539 7.92188 15.9492C7.78125 16.1445 7.67578 16.3633 7.60547 16.6055C8.02734 16.707 8.41406 16.8711 8.76562 17.0977C9.11719 17.3242 9.42188 17.6016 9.67969 17.9297C9.9375 18.2578 10.1367 18.6172 10.2773 19.0078C10.418 19.3984 10.4922 19.8125 10.5 20.25C10.5 20.7656 10.4023 21.25 10.207 21.7031C10.0117 22.1562 9.74219 22.5547 9.39844 22.8984C9.05469 23.2422 8.65625 23.5117 8.20312 23.707C7.75 23.9023 7.26562 24 6.75 24C6.23438 24 5.75 23.9023 5.29688 23.707C4.84375 23.5117 4.44531 23.2461 4.10156 22.9102C3.75781 22.5742 3.48828 22.1758 3.29297 21.7148C3.09766 21.2539 3 20.7656 3 20.25C3 19.8047 3.07422 19.3789 3.22266 18.9727C3.37109 18.5664 3.57812 18.2031 3.84375 17.8828C4.10938 17.5625 4.42578 17.2852 4.79297 17.0508C5.16016 16.8164 5.5625 16.6602 6 16.582V7.41797C5.5625 7.33203 5.16016 7.17578 4.79297 6.94922C4.42578 6.72266 4.10938 6.44922 3.84375 6.12891C3.57812 5.80859 3.37109 5.44141 3.22266 5.02734C3.07422 4.61328 3 4.1875 3 3.75C3 3.23438 3.09766 2.75 3.29297 2.29688C3.48828 1.84375 3.75391 1.44922 4.08984 1.11328C4.42578 0.777344 4.82422 0.507812 5.28516 0.304688C5.74609 0.101562 6.23438 0 6.75 0C7.26562 0 7.75 0.0976563 8.20312 0.292969C8.65625 0.488281 9.05078 0.757812 9.38672 1.10156C9.72266 1.44531 9.99219 1.84375 10.1953 2.29688C10.3984 2.75 10.5 3.23438 10.5 3.75C10.5 4.19531 10.4258 4.62109 10.2773 5.02734C10.1289 5.43359 9.92188 5.79688 9.65625 6.11719C9.39062 6.4375 9.07422 6.71484 8.70703 6.94922C8.33984 7.18359 7.9375 7.33984 7.5 7.41797V14.2734C7.82812 14.0234 8.18359 13.832 8.56641 13.6992C8.94922 13.5664 9.34375 13.5 9.75 13.5H14.25C14.5 13.5 14.7383 13.4609 14.9648 13.3828C15.1914 13.3047 15.3984 13.1953 15.5859 13.0547C15.7734 12.9141 15.9375 12.7461 16.0781 12.5508C16.2188 12.3555 16.3242 12.1367 16.3945 11.8945C15.9727 11.793 15.5859 11.6289 15.2344 11.4023C14.8828 11.1758 14.5781 10.9023 14.3203 10.582C14.0625 10.2617 13.8633 9.90234 13.7227 9.50391C13.582 9.10547 13.5078 8.6875 13.5 8.25C13.5 7.73438 13.5977 7.25 13.793 6.79688C13.9883 6.34375 14.2539 5.94922 14.5898 5.61328C14.9258 5.27734 15.3242 5.00781 15.7852 4.80469C16.2461 4.60156 16.7344 4.5 17.25 4.5C17.7656 4.5 18.25 4.59766 18.7031 4.79297C19.1562 4.98828 19.5508 5.25781 19.8867 5.60156C20.2227 5.94531 20.4922 6.34375 20.6953 6.79688C20.8984 7.25 21 7.73438 21 8.25ZM4.5 3.75C4.5 4.0625 4.55859 4.35547 4.67578 4.62891C4.79297 4.90234 4.95312 5.14062 5.15625 5.34375C5.35938 5.54688 5.59766 5.70703 5.87109 5.82422C6.14453 5.94141 6.4375 6 6.75 6C7.0625 6 7.35547 5.94141 7.62891 5.82422C7.90234 5.70703 8.14062 5.54688 8.34375 5.34375C8.54688 5.14062 8.70703 4.90234 8.82422 4.62891C8.94141 4.35547 9 4.0625 9 3.75C9 3.4375 8.94141 3.14453 8.82422 2.87109C8.70703 2.59766 8.54688 2.35938 8.34375 2.15625C8.14062 1.95312 7.90234 1.79297 7.62891 1.67578C7.35547 1.55859 7.0625 1.5 6.75 1.5C6.4375 1.5 6.14453 1.55859 5.87109 1.67578C5.59766 1.79297 5.35938 1.95312 5.15625 2.15625C4.95312 2.35938 4.79297 2.59766 4.67578 2.87109C4.55859 3.14453 4.5 3.4375 4.5 3.75ZM9 20.25C9 19.9375 8.94141 19.6445 8.82422 19.3711C8.70703 19.0977 8.54688 18.8594 8.34375 18.6562C8.14062 18.4531 7.90234 18.293 7.62891 18.1758C7.35547 18.0586 7.0625 18 6.75 18C6.4375 18 6.14453 18.0586 5.87109 18.1758C5.59766 18.293 5.35938 18.4531 5.15625 18.6562C4.95312 18.8594 4.79297 19.0977 4.67578 19.3711C4.55859 19.6445 4.5 19.9375 4.5 20.25C4.5 20.5625 4.55859 20.8555 4.67578 21.1289C4.79297 21.4023 4.95312 21.6406 5.15625 21.8438C5.35938 22.0469 5.59766 22.207 5.87109 22.3242C6.14453 22.4414 6.4375 22.5 6.75 22.5C7.0625 22.5 7.35547 22.4414 7.62891 22.3242C7.90234 22.207 8.14062 22.0469 8.34375 21.8438C8.54688 21.6406 8.70703 21.4023 8.82422 21.1289C8.94141 20.8555 9 20.5625 9 20.25ZM17.25 10.5C17.5625 10.5 17.8555 10.4414 18.1289 10.3242C18.4023 10.207 18.6406 10.0469 18.8438 9.84375C19.0469 9.64062 19.207 9.40234 19.3242 9.12891C19.4414 8.85547 19.5 8.5625 19.5 8.25C19.5 7.9375 19.4414 7.64453 19.3242 7.37109C19.207 7.09766 19.0469 6.85938 18.8438 6.65625C18.6406 6.45312 18.4023 6.29297 18.1289 6.17578C17.8555 6.05859 17.5625 6 17.25 6C16.9375 6 16.6445 6.05859 16.3711 6.17578C16.0977 6.29297 15.8594 6.45312 15.6562 6.65625C15.4531 6.85938 15.293 7.09766 15.1758 7.37109C15.0586 7.64453 15 7.9375 15 8.25C15 8.5625 15.0586 8.85547 15.1758 9.12891C15.293 9.40234 15.4531 9.64062 15.6562 9.84375C15.8594 10.0469 16.0977 10.207 16.3711 10.3242C16.6445 10.4414 16.9375 10.5 17.25 10.5Z"
								fill="currentColor"
							/>
						</svg>
						<h3>{{ formatMessage(messages.openSource) }}</h3>
					</div>
					<div class="description">
						<IntlFormatted :message-id="messages.openSourceDescription">
							<template #github-link="{ children }">
								<a
									href="https://github.com/SunkDiagram1865/Ghastling"
									rel="noopener"
									target="_blank"
								>
									<component :is="() => children" />
								</a>
							</template>
						</IntlFormatted>
					</div>
				</div>
				<div class="point">
					<div class="title">
						<svg
							class="dark-icon"
							xmlns="http://www.w3.org/2000/svg"
							width="24"
							height="24"
							viewBox="0 0 24 24"
							fill="none"
						>
							<g clip-path="url(#clip0_897_3802)">
								<path
									d="M4.39313 8.58984C2.31984 9.28359 0.75 10.9266 0.75 13.5C0.75 16.5938 3.28125 18.75 6.375 18.75H15.0173M21.9291 17.7066C22.7456 17.0297 23.25 16.013 23.25 14.625C23.25 11.8209 20.7656 10.605 18.75 10.5C18.3333 6.30281 15.4219 3.75 12 3.75C10.7738 3.75 9.71297 4.07484 8.83125 4.60031"
									stroke="currentColor"
									stroke-width="2.34783"
									stroke-linecap="round"
									stroke-linejoin="round"
								/>
								<path
									d="M21 21L3 3"
									stroke="currentColor"
									stroke-width="2.34783"
									stroke-miterlimit="10"
									stroke-linecap="round"
								/>
							</g>
							<defs>
								<clipPath id="clip0_897_3802">
									<rect width="24" height="24" fill="white" />
								</clipPath>
							</defs>
						</svg>
						<svg
							class="light-icon"
							xmlns="http://www.w3.org/2000/svg"
							width="26"
							height="24"
							viewBox="0 0 26 24"
							fill="none"
						>
							<path
								d="M5.39313 8.58984C3.31984 9.28359 1.75 10.9266 1.75 13.5C1.75 16.5938 4.28125 18.75 7.375 18.75H16.0173M22.9291 17.7066C23.7456 17.0297 24.25 16.013 24.25 14.625C24.25 11.8209 21.7656 10.605 19.75 10.5C19.3333 6.30281 16.4219 3.75 13 3.75C11.7738 3.75 10.713 4.07484 9.83125 4.60031"
								stroke="currentColor"
								stroke-width="2.34783"
								stroke-linecap="round"
								stroke-linejoin="round"
							/>
							<path
								d="M22 21L4 3"
								stroke="currentColor"
								stroke-width="2.34783"
								stroke-miterlimit="10"
								stroke-linecap="round"
							/>
						</svg>
						<h3>{{ formatMessage(messages.offlineMode) }}</h3>
					</div>
					<div class="description">
						{{ formatMessage(messages.offlineModeDescription) }}
					</div>
				</div>
				<div class="point">
					<div class="title">
						<svg
							class="dark-icon"
							xmlns="http://www.w3.org/2000/svg"
							width="24"
							height="24"
							viewBox="0 0 24 24"
							fill="none"
						>
							<path
								d="M22.8533 11.7089C22.8205 11.6379 22.0264 9.94977 20.2611 8.25801C17.9089 6.00383 14.938 4.8125 11.668 4.8125C8.39796 4.8125 5.42702 6.00383 3.07483 8.25801C1.30952 9.94977 0.51171 11.6406 0.482647 11.7089C0.440003 11.8008 0.417969 11.9003 0.417969 12.0009C0.417969 12.1015 0.440003 12.201 0.482647 12.2929C0.51546 12.3639 1.30952 14.0511 3.07483 15.7429C5.42702 17.9962 8.39796 19.1875 11.668 19.1875C14.938 19.1875 17.9089 17.9962 20.2611 15.7429C22.0264 14.0511 22.8205 12.3639 22.8533 12.2929C22.8959 12.201 22.918 12.1015 22.918 12.0009C22.918 11.9003 22.8959 11.8008 22.8533 11.7089ZM11.668 17.75C8.78234 17.75 6.2614 16.7446 4.17452 14.7627C3.31825 13.9466 2.58976 13.0161 2.01171 12C2.58961 10.9838 3.31811 10.0532 4.17452 9.2373C6.2614 7.25535 8.78234 6.25 11.668 6.25C14.5536 6.25 17.0745 7.25535 19.1614 9.2373C20.0193 10.053 20.7494 10.9836 21.3289 12C20.653 13.2093 17.7083 17.75 11.668 17.75ZM11.668 7.6875C10.7779 7.6875 9.90792 7.94042 9.16789 8.41429C8.42787 8.88815 7.8511 9.56167 7.5105 10.3497C7.16991 11.1377 7.08079 12.0048 7.25443 12.8413C7.42806 13.6779 7.85664 14.4463 8.48598 15.0494C9.11532 15.6525 9.91714 16.0632 10.7901 16.2296C11.663 16.396 12.5678 16.3106 13.39 15.9842C14.2123 15.6578 14.9151 15.1051 15.4096 14.3959C15.904 13.6867 16.168 12.8529 16.168 12C16.1667 10.8566 15.6922 9.76041 14.8486 8.95192C14.0049 8.14342 12.8611 7.68869 11.668 7.6875ZM11.668 14.875C11.0746 14.875 10.4946 14.7064 10.0012 14.3905C9.5079 14.0746 9.12338 13.6256 8.89632 13.1002C8.66926 12.5749 8.60985 11.9968 8.7256 11.4391C8.84136 10.8814 9.12708 10.3691 9.54664 9.96707C9.9662 9.56499 10.5007 9.29117 11.0827 9.18024C11.6646 9.06931 12.2678 9.12624 12.816 9.34385C13.3642 9.56145 13.8327 9.92994 14.1624 10.4027C14.492 10.8755 14.668 11.4314 14.668 12C14.668 12.7625 14.3519 13.4938 13.7893 14.0329C13.2267 14.5721 12.4636 14.875 11.668 14.875Z"
								fill="currentColor"
							/>
						</svg>
						<svg
							class="light-icon"
							xmlns="http://www.w3.org/2000/svg"
							width="24"
							height="24"
							viewBox="0 0 24 24"
							fill="none"
						>
							<path
								d="M22.8533 11.7089C22.8205 11.6379 22.0264 9.94977 20.2611 8.25801C17.9089 6.00383 14.938 4.8125 11.668 4.8125C8.39796 4.8125 5.42702 6.00383 3.07483 8.25801C1.30952 9.94977 0.51171 11.6406 0.482647 11.7089C0.440003 11.8008 0.417969 11.9003 0.417969 12.0009C0.417969 12.1015 0.440003 12.201 0.482647 12.2929C0.51546 12.3639 1.30952 14.0511 3.07483 15.7429C5.42702 17.9962 8.39796 19.1875 11.668 19.1875C14.938 19.1875 17.9089 17.9962 20.2611 15.7429C22.0264 14.0511 22.8205 12.3639 22.8533 12.2929C22.8959 12.201 22.918 12.1015 22.918 12.0009C22.918 11.9003 22.8959 11.8008 22.8533 11.7089ZM11.668 17.75C8.78234 17.75 6.2614 16.7446 4.17452 14.7627C3.31825 13.9466 2.58976 13.0161 2.01171 12C2.58961 10.9838 3.31811 10.0532 4.17452 9.2373C6.2614 7.25535 8.78234 6.25 11.668 6.25C14.5536 6.25 17.0745 7.25535 19.1614 9.2373C20.0193 10.053 20.7494 10.9836 21.3289 12C20.653 13.2093 17.7083 17.75 11.668 17.75ZM11.668 7.6875C10.7779 7.6875 9.90792 7.94042 9.16789 8.41429C8.42787 8.88815 7.8511 9.56167 7.5105 10.3497C7.16991 11.1377 7.08079 12.0048 7.25443 12.8413C7.42806 13.6779 7.85664 14.4463 8.48598 15.0494C9.11532 15.6525 9.91714 16.0632 10.7901 16.2296C11.663 16.396 12.5678 16.3106 13.39 15.9842C14.2123 15.6578 14.9151 15.1051 15.4096 14.3959C15.904 13.6867 16.168 12.8529 16.168 12C16.1667 10.8566 15.6922 9.76041 14.8486 8.95192C14.0049 8.14342 12.8611 7.68869 11.668 7.6875ZM11.668 14.875C11.0746 14.875 10.4946 14.7064 10.0012 14.3905C9.5079 14.0746 9.12338 13.6256 8.89632 13.1002C8.66926 12.5749 8.60985 11.9968 8.7256 11.4391C8.84136 10.8814 9.12708 10.3691 9.54664 9.96707C9.9662 9.56499 10.5007 9.29117 11.0827 9.18024C11.6646 9.06931 12.2678 9.12624 12.816 9.34385C13.3642 9.56145 13.8327 9.92994 14.1624 10.4027C14.492 10.8755 14.668 11.4314 14.668 12C14.668 12.7625 14.3519 13.4938 13.7893 14.0329C13.2267 14.5721 12.4636 14.875 11.668 14.875Z"
								fill="currentColor"
							/>
						</svg>
						<h3>{{ formatMessage(messages.followProjects) }}</h3>
					</div>
					<div class="description">{{ formatMessage(messages.followProjectsDescription) }}</div>
				</div>
			</div>
		</div>
		<section id="faq" class="faq-section" aria-labelledby="faq-title">
			<div class="faq-intro">
				<span class="section-eyebrow">{{ formatMessage(messages.faqEyebrow) }}</span>
				<h2 id="faq-title">{{ formatMessage(messages.faqTitle) }}</h2>
				<p>{{ formatMessage(messages.faqDescription) }}</p>
			</div>
			<div class="faq-list">
				<details v-for="item in faqItems" :key="item.question" class="faq-item">
					<summary>{{ item.question }}</summary>
					<p>{{ item.answer }}</p>
				</details>
			</div>
		</section>
	</div>
</template>

<style scoped lang="scss">
.faq-section {
	display: grid;
	grid-template-columns: minmax(0, 0.8fr) minmax(0, 1.2fr);
	gap: 4rem;
	width: min(76rem, calc(100% - 3rem));
	margin: 0 auto;
	padding: 7rem 0;
}

.faq-intro {
	h2 {
		margin: 0.75rem 0 1rem;
		color: var(--color-contrast);
		font-size: clamp(2rem, 4vw, 3.25rem);
		line-height: 1.08;
	}

	p {
		max-width: 32rem;
		margin: 0;
		color: var(--color-secondary);
		font-size: 1.05rem;
		line-height: 1.7;
	}
}

.faq-list {
	display: flex;
	flex-direction: column;
	gap: 0.75rem;
}

.faq-item {
	border: 1px solid var(--color-divider);
	border-radius: 1rem;
	background: var(--surface-2);

	summary {
		padding: 1.15rem 1.25rem;
		color: var(--color-contrast);
		font-weight: 700;
		line-height: 1.4;
		cursor: pointer;
	}

	p {
		margin: 0;
		padding: 0 1.25rem 1.25rem;
		color: var(--color-secondary);
		line-height: 1.7;
	}
}

@media (max-width: 800px) {
	.faq-section {
		grid-template-columns: 1fr;
		gap: 2rem;
		width: calc(100% - 2rem);
		padding: 5rem 0;
	}
}

.landing-hero {
	position: relative;
	background:
		radial-gradient(circle at 18% 20%, rgb(57 197 187 / 26%), transparent 28rem),
		radial-gradient(circle at 82% 36%, rgb(102 204 255 / 16%), transparent 32rem),
		linear-gradient(180deg, #17121d 0%, #11111d 58%, #101018 100%);
	padding: 6rem 1rem 12rem;
	margin-top: -5rem;
	display: flex;
	justify-content: center;
	align-items: center;
	text-align: center;
	flex-direction: column;
	isolation: isolate;

	.hero-kicker {
		border: 1px solid color-mix(in srgb, var(--color-brand) 28%, transparent);
		background: color-mix(in srgb, var(--color-brand) 12%, transparent);
		box-shadow: 0 0.75rem 2.5rem color-mix(in srgb, var(--color-brand) 12%, transparent);
	}

	.main-subheader {
		font-size: 1.625rem;
		line-height: 125%;
		margin: 0 0 1.625rem;
		font-weight: 400;
		line-break: loose;
		color: var(--landing-color-subheading);
		max-width: 1096px;
		mask-image: none;
	}

	.hero-download {
		display: inline-flex;
		align-items: center;
		gap: 0.625rem;
		padding: 0.875rem 2rem;
		margin-bottom: 2rem;
		border: 0;
		border-radius: 0.875rem;
		background: var(--color-brand);
		color: var(--color-brand-inverted);
		font-size: 1.125rem;
		font-weight: 700;
		text-decoration: none;
		transition: 120ms ease;

		&:hover {
			background: color-mix(in srgb, var(--color-brand) 85%, white);
			box-shadow: 0 0.75rem 2.5rem color-mix(in srgb, var(--color-brand) 28%, transparent);
			transform: translateY(-0.125rem);
		}

		svg {
			width: 1.25rem;
			height: 1.25rem;
			flex: 0 0 auto;
		}
	}

	img {
		width: 100%;
		max-width: 65rem;
		height: auto;
		mask-image: none;
		z-index: 1;
	}

	.hero-screenshot {
		margin: 3rem auto 0;
		border-radius: 1rem;
	}

	.bottom-transition {
		z-index: -1;
	}
}

.ghastling-highlights {
	position: relative;
	padding: 1rem 1.5rem;
	background: var(--landing-transition-gradient-end);
	scroll-margin-top: 4rem;
}

.highlights-intro {
	max-width: 48rem;
	margin: 0 auto 2.75rem;
	text-align: center;

	h2 {
		margin: 0.7rem 0 1rem;
		color: var(--color-contrast);
		font-size: clamp(2.25rem, 5vw, 4.25rem);
		font-weight: 650;
		letter-spacing: -0.045em;
		line-height: 1.04;
	}

	p {
		margin: 0;
		color: var(--color-secondary);
		font-size: 1.05rem;
		line-height: 1.65;
	}
}

.section-eyebrow {
	color: var(--color-brand);
	font-size: 0.75rem;
	font-weight: 800;
	letter-spacing: 0.1em;
	text-transform: uppercase;
}

.modrinth-feature-grid {
	width: min(100%, 68.5rem);
	margin: 0 auto;
	display: grid;
	grid-template-columns: repeat(6, minmax(0, 1fr));
	gap: 1rem;
}

.modrinth-feature-grid .feature {
	padding: var(--gap-xl);
	z-index: 1;
	background: radial-gradient(
		50% 50% at 50% 50%,
		rgba(44, 48, 79, 0.35) 0%,
		rgba(32, 35, 50, 0.27) 100%
	);
	box-shadow:
		2px 2px 12px 0 rgba(0, 0, 0, 0.16),
		2px 2px 64px 0 rgba(57, 61, 94, 0.45) inset;
	backdrop-filter: blur(6px);
	-webkit-backdrop-filter: blur(6px);
	overflow: hidden;
}

.promise-card {
	grid-column: span 2;
	min-height: 12.5rem;
	padding: 1.5rem;

	.promise-meta {
		display: flex;
		align-items: center;
		justify-content: space-between;
		color: var(--color-brand);

		svg {
			width: 1.35rem;
			height: 1.35rem;
		}

		span {
			color: var(--color-brand);
			font-size: 0.75rem;
			font-weight: 800;
			letter-spacing: 0.09em;
		}
	}

	h3 {
		margin: 2.2rem 0 0.65rem;
		color: var(--color-contrast);
		font-size: 1.2rem;
		letter-spacing: -0.025em;
	}

	p {
		margin: 0;
		color: var(--color-secondary);
		font-size: 0.9rem;
		line-height: 1.6;
	}
}

.showcase-card {
	display: flex;
	grid-column: span 3;
	min-width: 0;
	flex-direction: column;
	overflow: hidden;
}

.showcase-card-wide {
	grid-column: 1 / -1;
	display: grid;
	grid-template-columns: minmax(17rem, 0.78fr) minmax(0, 1.22fr);
	align-items: center;
}

.showcase-copy {
	padding: 1.75rem 1.75rem 1.5rem;

	span {
		color: var(--color-brand);
		font-size: 0.72rem;
		font-weight: 800;
		letter-spacing: 0.1em;
		text-transform: uppercase;
	}

	h3 {
		margin: 0.55rem 0 0.65rem;
		color: var(--color-contrast);
		font-size: clamp(1.35rem, 2.4vw, 1.8rem);
		letter-spacing: -0.035em;
		line-height: 1.1;
	}

	p {
		margin: 0;
		color: var(--color-secondary);
		font-size: 0.9rem;
		line-height: 1.6;
	}
}

.showcase-image-frame {
	position: relative;
	min-width: 0;
	padding: 0 1rem 1rem;
}

.showcase-card-wide .showcase-image-frame {
	padding: 1rem 1rem 1rem 0;
}

.showcase-image {
	display: block;
	width: 100%;
	height: auto;
	border: 1px solid color-mix(in srgb, var(--color-contrast) 12%, transparent);
	border-radius: 0.85rem;
	box-shadow: 0 1rem 2.5rem rgb(0 0 0 / 15%);
}

.dark-shot {
	display: none;
}

:global(html.dark-mode) .light-shot,
:global(html.oled-mode) .light-shot {
	display: none;
}

:global(html.dark-mode) .dark-shot,
:global(html.oled-mode) .dark-shot {
	display: block;
}

:global(html.light-mode) .ghastling-highlights {
	background: #f8f7f8;
}

.main-header {
	font-size: 5.25rem;
	font-weight: 600;
	line-height: 100%;
	margin: 1rem 0 2rem;
}

.subheader {
	font-size: 3.5rem;
	font-weight: 600;
	line-height: 100%;
	margin: 0 auto;
	padding: 0 4rem 4rem;
}

.features {
	position: relative;
	width: 100%;
	background: var(--landing-transition-gradient-end);
	align-content: center;
	justify-content: center;
	display: flex;
	flex-direction: column;

	h3 {
		font-weight: 500;
		font-size: var(--font-size-xl) !important;
	}

	p {
		font-size: var(--font-size-md) !important;
	}

	.feature-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		grid-template-rows: minmax(33rem, auto);
		max-width: 68.5rem;
		width: min(calc(100% - 3rem), 68.5rem);
		gap: 1rem;
		margin: 0 auto;
		padding: 0;

		.mods {
			grid-column: 1 / 2;
			grid-row: 1 / 2;

			.table {
				margin-bottom: 1rem;
				overflow: hidden;
				max-height: 32rem;
			}

			h3,
			p {
				text-align: center;
			}

			h4 {
				margin: 0;
				color: var(--color-contrast);
			}

			.search-bar {
				width: 100%;
				padding: var(--gap-sm);
				display: flex;
				flex-direction: row;
				justify-content: space-between;
				align-items: center;
				border-radius: var(--radius-md);
				border: 1px solid var(--landing-border-color);
				background: linear-gradient(0deg, #3b3f55 0%, #3b3f55 100%), rgba(59, 63, 85, 0.15);
				margin-bottom: 0.5rem;
				white-space: nowrap;
				font-size: var(--font-size-sm);

				.mini-input {
					display: flex;
					flex-direction: row;
					align-items: center;
					gap: 0.5rem;
					padding: var(--gap-sm) var(--gap-md);
					border-radius: var(--radius-sm);
					background-color: #1e202f;
					flex-grow: 1;
					max-width: 12rem;
				}

				h4 {
					font-weight: normal;
					margin-left: 0.5rem;
				}
			}

			.row {
				display: grid;
				grid-template-columns: 3rem 2fr 1fr 3.75rem;
				padding: 0 var(--gap-sm);
				gap: 1rem;

				.cell {
					display: flex;
					flex-direction: column;
					justify-content: center;
					padding: var(--gap-sm) 0;
					font-size: var(--font-size-sm);

					.name {
						color: var(--color-contrast);
					}

					.description {
						font-size: var(--font-size-xs);
					}

					&.last {
						align-items: flex-end;
					}

					&.check {
						align-items: center;
						flex-direction: row;
					}
				}
			}

			.header {
				.cell {
					color: var(--color-base);
				}
			}
		}

		.playing {
			grid-column: 1 / 2;
			grid-row: 4 / 5;
			position: relative;
			min-height: 23rem;

			.launcher-preview {
				position: absolute;
				inset: var(--gap-xl) var(--gap-xl) 7.5rem;
				display: flex;
				align-items: center;
				gap: var(--gap-lg);
				padding: var(--gap-xl);
				border: 1px solid rgb(245 139 174 / 24%);
				border-radius: 1.25rem;
				background: linear-gradient(135deg, rgb(245 139 174 / 14%), rgb(136 103 255 / 8%));
				text-align: left;

				img {
					width: 7rem;
					height: 7rem;
					object-fit: contain;
				}

				div {
					display: flex;
					flex-direction: column;
					gap: var(--gap-sm);
				}

				strong {
					font-size: 1.25rem;
				}

				span {
					color: var(--color-secondary);
				}
			}

			.text {
				position: absolute;
				left: 50%;
				bottom: 0;
				transform: translateX(-50%);
				display: flex;
				flex-direction: column;
				justify-content: center;
				align-items: center;
				color: var(--color-contrast);
				text-align: center;
				padding: var(--gap-xl);
				mask-image: none;
				z-index: 10;
				width: 100%;
			}
		}

		.sharing {
			grid-column: 2 / 3;
			grid-row: 1 / 2;

			.table {
				height: 11rem;
				overflow: hidden;
				margin-bottom: 1rem;
			}

			h3,
			p {
				text-align: center;
			}

			.row {
				display: grid;
				grid-template-columns: 3rem 1fr 2fr;
				padding: 0 var(--gap-sm);
				gap: 1rem;

				.cell {
					display: flex;
					align-items: center;
					padding: var(--gap-sm) 0;

					.description {
						display: -webkit-box;
						-webkit-line-clamp: 2;
						-webkit-box-orient: vertical;
						overflow: hidden;
					}
				}
			}

			.header {
				display: flex;
				flex-direction: row;
				gap: 0.25rem;
				padding: var(--gap-sm) var(--gap-sm);
			}

			.export-card {
				position: absolute;
				right: 1rem;
				top: 1.5rem;
				display: flex;
				flex-direction: row;
				align-items: center;
				padding: var(--gap-sm);
				gap: var(--gap-md);
				border-radius: var(--radius-md);
				background: linear-gradient(0deg, #3b3f55 0%, #3b3f55 100%), rgba(59, 63, 85, 0.15);
				border: 1px solid var(--landing-border-color);
				margin-bottom: 1rem;
				width: 20rem;
				font-size: var(--font-size-sm);

				img {
					width: 4.5rem;
					height: 4.5rem;
				}

				.info {
					width: 100%;
					display: flex;
					flex-direction: column;
					align-items: flex-start;
					gap: 0.25rem;

					.exporting {
						display: flex;
						flex-direction: row;
						align-items: center;
						justify-content: space-between;
						gap: 0.5rem;
						width: 100%;

						.tag {
							display: flex;
							flex-direction: row;
							align-items: center;
							gap: 0.25rem;
							border-radius: var(--radius-sm);
						}

						.small-button {
							display: flex;
							flex-direction: row;
							align-items: center;
							gap: 0.25rem;
							padding: var(--gap-xs) var(--gap-md);
							border-radius: var(--radius-lg);
							border: 1px solid var(--landing-border-color);
							font-size: var(--font-size-xs);
						}
					}

					.name {
						font-size: var(--font-size-md);
						color: var(--color-contrast);
						margin: 0;
					}
				}
			}
		}

		.performance {
			grid-column: 1 / 2;
			grid-row: 3 / 4;

			.title {
				display: flex;
				justify-content: space-between;
				align-items: baseline;
				margin-bottom: 0.5rem;

				h3 {
					margin: 0;
					font-size: var(--font-size-lg);
				}
			}

			h3,
			h4,
			p {
				text-align: center;
				margin: 0;
			}

			h4,
			h3 {
				color: var(--color-contrast);
				margin-top: var(--gap-lg) !important;
			}

			h4 {
				font-size: var(--font-size-md);
				font-weight: normal;
			}

			.table {
				height: 10rem;
				overflow: hidden;
			}

			.row {
				display: grid;
				grid-template-columns: 3rem 2fr 1fr 1fr;
				padding: 0 var(--gap-sm);
				gap: 1rem;

				.cell {
					display: flex;
					align-items: center;
					padding: var(--gap-sm) 0;
					color: var(--color-gray);

					&.important {
						color: var(--color-contrast);
						font-weight: 500;
					}
				}
			}

			.header {
				font-size: var(--font-size-sm);
				.cell {
					color: var(--color-base);
				}
			}

			.icon-logo {
				width: 3rem;
				height: 3rem;
				padding: 0.5rem;
				border-radius: var(--radius-sm);
				background-color: var(--color-button-bg);

				&.ghastling {
					background-color: var(--color-brand) !important;
					border: 1px solid var(--color-accent-contrast) !important;

					.icon {
						width: 2rem;
						height: 2rem;
						z-index: 0;

						:deep(svg) {
							width: 2rem;
							height: 2rem;
						}

						:deep(path) {
							fill: var(--color-accent-contrast) !important;
						}
					}
				}
			}

			.app-icon {
				width: 2.25rem;
				height: 2.25rem;
				object-fit: contain;
			}
		}

		.importing {
			position: relative;
			grid-column: 2 / 3;
			grid-row: 1 / 2;

			.importing-app-icon {
				position: absolute;
				top: 4rem;
				left: 50%;
				transform: translate(-50%, -50%);
				width: 4rem;
				height: 4rem;
				object-fit: contain;
			}

			.inner-circle {
				position: absolute;
				inset: 0 0 4rem;
			}

			.launcher-badge {
				position: absolute;
				background-color: var(--landing-transition-gradient-end);
				width: 3rem;
				height: 3rem;
				padding: 0.5rem;
				border-radius: 100%;
				z-index: 10;
				user-select: none;

				&:hover {
					cursor: default;
				}

				&.top-left {
					top: calc(15% - 3rem);
					left: calc(58% - 11rem);
				}

				&.top-right {
					top: calc(15% - 3rem);
					right: calc(58% - 11rem);
				}

				&.bottom-left {
					bottom: calc(41% + 3rem);
					left: calc(58% - 11rem);
				}

				&.bottom-right {
					bottom: calc(41% + 3rem);
					right: calc(58% - 11rem);
				}

				&.bottom-middle {
					bottom: calc(22% + 3rem);
					left: 50%;
					transform: translateX(-50%);
				}

				&.center {
					top: calc(50% - 3rem);
					left: 50%;
					transform: translate(-50%, -50%);
				}

				svg {
					width: 2rem;
					height: 2rem;
				}

				img {
					width: 2rem;
					height: 2rem;
				}
			}

			.text {
				position: absolute;
				text-align: center;
				left: 50%;
				bottom: 0;
				transform: translateX(-50%);
				width: 100%;
				padding: var(--gap-xl);
			}
		}

		.website {
			grid-column: 2 / 3;
			grid-row: 1 / 2;
			text-align: center;
			padding: 0 !important;

			position: relative;

			.projects-showcase {
				margin: calc(5rem + var(--gap-xl)) 0 var(--gap-xl);
				z-index: 3;
				text-align: left;

				.row {
					--gap: var(--gap-md);

					width: 100vw;
					gap: var(--gap);
					margin-bottom: var(--gap);
					display: flex;
					overflow: hidden;
					user-select: none;

					.row__content {
						flex-shrink: 0;
						display: flex;
						min-width: 100%;
						gap: var(--gap);
						transform: translateX(-15%);

						&.offset {
							transform: translateX(-130%);
						}
					}

					.project {
						position: relative;
						display: flex;

						cursor: pointer;
						padding: 1rem;
						gap: 1rem;
						border-radius: 1rem;
						border: 1px solid var(--landing-border-color);
						transition:
							background 0.5s ease-in-out,
							transform 0.05s ease-in-out;
						// Removed due to lag on mobile :(
						background: var(--landing-blob-gradient);

						img {
							height: 3rem;
						}

						.project-info {
							box-sizing: border-box;
						}

						.title {
							color: var(--landing-color-heading);
							max-width: 13.75rem;
							overflow: hidden;
							white-space: nowrap;
							text-overflow: ellipsis;
							margin: 0;
							font-weight: 600;
							font-size: 1.25rem;
							line-height: 110%;
							display: block;
						}

						.description {
							width: 13.75rem;

							display: -webkit-box;
							-webkit-line-clamp: 2;
							-webkit-box-orient: vertical;
							overflow: hidden;

							font-weight: 500;
							font-size: 0.875rem;
							line-height: 125%;
							margin: 0.25rem 0 0;
						}
					}
				}
			}

			.website-logo {
				position: absolute;
				top: 1.5rem;
				left: 50%;
				width: 3.5rem;
				height: 3.5rem;
				transform: translateX(-50%);
				z-index: 4;
				object-fit: contain;
			}

			p {
				padding: var(--gap-xl);
				padding-top: 0;
			}
		}

		.feature {
			padding: var(--gap-xl);
			z-index: 1;
			background: radial-gradient(
				50% 50% at 50% 50%,
				rgba(44, 48, 79, 0.35) 0%,
				rgba(32, 35, 50, 0.27) 100%
			);
			box-shadow:
				2px 2px 12px 0 rgba(0, 0, 0, 0.16),
				2px 2px 64px 0 rgba(57, 61, 94, 0.45) inset;
			backdrop-filter: blur(6px);
			-webkit-backdrop-filter: blur(6px);
			max-width: 540px;
			width: 100%;
			overflow: hidden;

			.additional-label {
				width: fit-content;
				padding: 0.5rem 0.75rem;
				margin-bottom: 0.5rem;
				background: var(--landing-blue-label-bg);
				color: var(--landing-blue-label);
				border-radius: 6px;
				font-weight: 700;
				font-size: 1rem;
			}

			h3,
			p {
				margin: 0;
			}

			h3 {
				font-size: var(--font-size-xl);
				color: var(--landing-color-heading);
				margin-bottom: 0.375rem;
			}

			p {
				color: var(--landing-color-subheading);
			}
		}
	}

	.feature-row {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: var(--gap-lg);
		max-width: 1096px;
		margin: 0 auto;
		padding: calc(var(--gap-xl) * 2) 1rem;

		@media (max-width: 1024px) {
			grid-template-columns: repeat(1, 1fr);

			.point {
				text-align: center;

				.title {
					justify-content: center;
				}
			}
		}

		.point {
			display: flex;
			flex-direction: column;
			gap: var(--gap-md);
			padding: 1rem 0;

			svg {
				width: 1.5rem;
				height: 1.5rem;
				color: var(--color-brand);
			}

			.title {
				display: flex;
				align-items: center;
				gap: 0.5rem;
			}

			h3 {
				font-size: var(--font-size-lg);
				font-weight: normal;
				color: var(--landing-color-heading);
				margin: 0;
			}

			p {
				color: var(--landing-color-subheading);
				margin: 0;
			}

			a {
				text-decoration: underline;
			}
		}
	}
}

.table {
	display: grid;
	border: 1px solid rgba(#a8b1ddbf, 0.25);
	gap: 0.25rem;
	overflow: hidden;
	font-size: var(--font-size-sm);
	background: rgba(59, 63, 85, 0.15);
	box-shadow: 2px 2px 12px 0px rgba(0, 0, 0, 0.16);

	button {
		&:hover {
			cursor: default !important;
		}
	}

	.first {
		border-top: none !important;
	}

	.row {
		&:not(.header) {
			border-top: 1px solid rgba(#a8b1ddbf, 0.25);
		}
	}
}

.row,
.header,
.table,
.project,
.export-card {
	user-select: none;

	&:hover {
		cursor: default;
	}
}

.gradient-border {
	position: relative;
	border-radius: var(--radius-lg);

	&:before {
		content: '';
		position: absolute;
		inset: 0;
		padding: 1px;
		z-index: -1;
		border-radius: 1rem;
		background: var(--landing-border-gradient);

		-webkit-mask:
			linear-gradient(#fff 0 0) content-box,
			linear-gradient(#fff 0 0);
		mask:
			linear-gradient(#fff 0 0) content-box,
			linear-gradient(#fff 0 0);
		-webkit-mask-composite: xor;
		mask-composite: exclude;
	}
}

.bottom-transition {
	position: absolute;
	bottom: 0;
	width: 100%;
	height: 30rem;
	background: linear-gradient(
		0deg,
		var(--landing-transition-gradient-end) 0%,
		var(--landing-transition-gradient-start) 100%
	);
}

@media screen and (max-width: 1024px) {
	.feature-grid {
		grid-template-columns: 1fr !important;
		grid-template-rows: none !important;
		gap: var(--gap-lg);
		margin: 0 auto;
		align-content: center;

		.feature {
			width: 100% !important;
			margin: 0 auto;
		}

		.mods,
		.website {
			grid-row: auto !important;
			grid-column: 1 / 2 !important;
		}
	}

	.main-header {
		font-size: 4rem !important;
	}

	.subheader {
		font-size: 1.5rem !important;
	}

	.main-subheader {
		font-size: 1.25rem !important;
	}
}

@media screen and (max-width: 746px) {
	.ghastling-highlights {
		padding: 0 1rem 1rem;
	}

	.highlights-intro {
		margin-bottom: 2rem;

		p {
			font-size: 0.95rem;
		}
	}

	.modrinth-feature-grid {
		grid-template-columns: 1fr;
	}

	.features .feature-grid {
		width: min(calc(100% - 2rem), 68.5rem);
		gap: 1rem;
	}

	.promise-card,
	.showcase-card {
		grid-column: auto;
	}

	.promise-card {
		min-height: auto;
		padding: 1.25rem;

		h3 {
			margin-top: 1.5rem;
		}
	}

	.showcase-card-wide {
		grid-column: auto;
		grid-template-columns: 1fr;
	}

	.showcase-copy {
		padding: 1.35rem 1.25rem 1.15rem;
	}

	.showcase-image-frame,
	.showcase-card-wide .showcase-image-frame {
		padding: 0 0.75rem 0.75rem;
	}

	.main-header {
		font-size: 3rem !important;
	}

	.subheader {
		font-size: 1.25rem !important;
	}

	.main-subheader {
		font-size: 1.1rem !important;
	}

	.logo-banner {
		padding: 3rem 1rem 3.75rem 1rem;

		.overlay {
			bottom: 3.5rem;
		}
	}
}

.dark-icon {
	display: block;
}

.light-icon {
	display: none;
}

.light-mode {
	.features {
		background: #f8f7f8;
	}

	.bottom-transition {
		background: linear-gradient(rgba(#f8f7f8, 0) 0%, #f8f7f8 100%);
	}

	.dark-icon {
		display: none;
	}

	.light-icon {
		display: block;
	}

	.feature {
		background: radial-gradient(
			50% 50% at 50% 50%,
			rgba(255, 255, 255, 0.35) 0%,
			rgba(255, 255, 255, 0.27) 100%
		) !important;
		box-shadow:
			2px 2px 64px 0px rgba(255, 255, 255, 0.45) inset,
			2px 2px 12px 0px rgba(0, 0, 0, 0.16) !important;
		border: none !important;
	}

	.gradient-border {
		&:before {
			background: var(--landing-border-gradient-light);
		}
	}

	.search-bar {
		background: var(--color-raised-bg) !important;
		border: 2px solid var(--color-brand) !important;

		.mini-input {
			background: var(--color-raised-bg) !important;
			border: 2px solid var(--color-bg);
		}
	}

	.ellipsis {
		opacity: 0.75;
		background: linear-gradient(
			180deg,
			rgba(57, 197, 187, 0.14) 0%,
			rgba(57, 197, 187, 0) 100%
		) !important;
	}

	.landing-hero {
		background:
			radial-gradient(circle at 18% 20%, rgb(57 197 187 / 20%), transparent 28rem),
			radial-gradient(circle at 82% 36%, rgb(102 204 255 / 11%), transparent 32rem),
			linear-gradient(180deg, #fff9fc 0%, #faf6fa 58%, #f8f4f7 100%);

		.hero-wordmark {
			color: rgb(105 73 88 / 8%);
		}
	}

	.table {
		background: white;
	}

	.project {
		background: rgba(255, 255, 255, 0.8) !important;
	}

	.export-card {
		background: white !important;
	}
}

.main-header-strong {
	font-weight: 600;
	background-color: var(--color-brand);
	background-image: linear-gradient(180deg, #b8e6e2 0%, var(--color-brand) 64%);
	background-size: 100%;
	background-clip: text;
	-webkit-text-fill-color: transparent;
	-moz-text-fill-color: transparent;
	color: transparent;
}

.logo-banner {
	position: relative;
	display: flex;
	align-items: center;
	justify-content: center;
	padding: 4rem 1rem 6.75rem 1rem;
	overflow: hidden;
	width: 100%;
	background-color: var(--color-accent-contrast);

	svg {
		z-index: 2;
		width: auto;
		height: 32rem;
	}

	.overlay {
		z-index: 3;
		position: absolute;
		bottom: 5rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
		gap: 1.5rem;
		white-space: nowrap;
	}
}
</style>
