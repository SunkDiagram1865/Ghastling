const trimTrailingSlash = (url: string) => url.replace(/\/$/, '')

export const GhastlingBrandConfig = Object.freeze({
	productName: 'Ghastling Launcher',
	shortProductName: 'Ghastling',
	organizationName: 'Coffeepop Studio',
	shortOrganizationName: 'GHS',
	developerName: 'SunkDiagram1865',
	website: 'https://sunkdiagram1865.github.io/Ghastling/',
	sourceUrl: 'https://www.ghs.red',
	supportUrl: 'https://github.com/SunkDiagram1865/Ghastling/issues',
	qqGroupNumber: '208375315',
	sponsorUrl: 'https://afdian.com/a/cysunk',
	bundleIdentifier: 'com.cysunk.ghastling',
	deepLinkScheme: 'ghastling',
	userAgent: (version: string, os: string) => `garbage-human-studio/ghastling/${version} (${os})`,
	capabilities: Object.freeze({
		publicModrinthApi: true,
		privateModrinthServices: true,
		ghsTelemetry: false,
	}),
})

const siteUrl = trimTrailingSlash(import.meta.env.MODRINTH_URL || 'https://modrinth.com')
const officialLabrinthBaseUrl = trimTrailingSlash(
	import.meta.env.MODRINTH_API_BASE_URL || 'https://api.modrinth.com',
)
const archonBaseUrl = trimTrailingSlash(
	import.meta.env.MODRINTH_ARCHON_BASE_URL || 'https://archon.modrinth.com',
)
const sharedInstancesBaseUrl = trimTrailingSlash(
	import.meta.env.SHARED_INSTANCES_API_BASE_URL || 'https://shared-instances.modrinth.com',
)
export const MODRINTH_MIRROR_BASE_URL = 'https://mod.mcimirror.top/modrinth'
type DownloadSourceMode = 'auto' | 'official_only' | 'mirror_preferred'

let modrinthSourceMode: DownloadSourceMode = 'auto'

function autoPrefersMirror() {
	if (typeof navigator === 'undefined') return false

	const languages = [...(navigator.languages ?? []), navigator.language]
	const usesMainlandChinese = languages.some((language) => {
		const normalized = language.toLowerCase().replace('_', '-')
		return normalized.startsWith('zh-cn') || normalized.startsWith('zh-hans')
	})
	const timeZone = Intl.DateTimeFormat().resolvedOptions().timeZone?.toLowerCase()
	const usesMainlandTimeZone = [
		'asia/shanghai',
		'asia/chongqing',
		'asia/harbin',
		'asia/urumqi',
	].includes(timeZone ?? '')

	return usesMainlandTimeZone || (!timeZone && usesMainlandChinese)
}

export function setModrinthSourceMode(sourceMode: DownloadSourceMode) {
	modrinthSourceMode = sourceMode
}

export function setModrinthMirrorEnabled(enabled: boolean) {
	setModrinthSourceMode(enabled ? 'mirror_preferred' : 'official_only')
}

export function getOfficialLabrinthBaseUrl() {
	return officialLabrinthBaseUrl
}

export function getLabrinthBaseUrl() {
	const useMirror =
		modrinthSourceMode === 'mirror_preferred' ||
		(modrinthSourceMode === 'auto' && autoPrefersMirror())
	return useMirror ? MODRINTH_MIRROR_BASE_URL : officialLabrinthBaseUrl
}

export const config = {
	siteUrl,
	stripePublishableKey:
		import.meta.env.VITE_STRIPE_PUBLISHABLE_KEY ||
		'pk_test_51JbFxJJygY5LJFfKV50mnXzz3YLvBVe2Gd1jn7ljWAkaBlRz3VQdxN9mXcPSrFbSqxwAb0svte9yhnsmm7qHfcWn00R611Ce7b',
	labrinthBaseUrl: getLabrinthBaseUrl,
	archonBaseUrl,
	sharedInstancesBaseUrl,
}
