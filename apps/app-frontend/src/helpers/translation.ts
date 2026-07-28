import { renderHighlightedString } from '@modrinth/utils'
import { configuredXss } from '@modrinth/utils/parse'
import { invoke } from '@tauri-apps/api/core'

import i18n from '@/i18n.config'

/** Minimal shape of a search hit object that has a translatable title and description. */
export interface TranslatableHit {
	/** Unique identifier — `project_id` on search hits, `id` on SearchResult. */
	project_id?: string
	id?: string
	title?: string
	description?: string
	/** Server search hits use `name` / `summary` instead of `title` / `description`. */
	name?: string
	summary?: string
}

export type TranslationProvider = 'microsoft' | 'google' | 'openai-compatible'
export type TranslationMode = 'bilingual' | 'translation-only'
export type TranslationStyle = 'default' | 'weakened' | 'brand' | 'border' | 'background'
export type TranslationTextFormat = 'plain' | 'html'
export type DescriptionSourceFormat = 'markdown' | 'html'

export interface TranslationSettings {
	provider: TranslationProvider
	target_language: string
	mode: TranslationMode
	auto_translate: boolean
	style: TranslationStyle
	openai_base_url: string
	openai_model: string
	openai_has_api_key: boolean
	openai_system_prompt: string
}

export interface TranslationSegment {
	id: string
	text: string
	format: TranslationTextFormat
}

export interface TranslationRequest {
	source_language: string
	target_language: string
	context: {
		title: string
		description: string
	}
	segments: TranslationSegment[]
}

export interface TranslationResponse {
	segments: Array<{ id: string; text: string }>
}

interface ProtectedElement {
	tagName: string
	attributes: Array<[string, string]>
	innerHtml?: string
}

export interface PreparedDescriptionBlock {
	id: string
	originalHtml: string
	translatable: boolean
	protectedElements: Record<string, ProtectedElement>
}

export interface PreparedDescription {
	blocks: PreparedDescriptionBlock[]
	segments: TranslationSegment[]
}

export async function getTranslationSettings(): Promise<TranslationSettings> {
	return await invoke('plugin:translation|translation_get_settings')
}

export async function updateTranslationSettings(settings: TranslationSettings): Promise<void> {
	await invoke('plugin:translation|translation_update_settings', { settings })
}

export async function setTranslationSecret(
	provider: TranslationProvider,
	secret: string | null,
): Promise<void> {
	await invoke('plugin:translation|translation_set_secret', { provider, secret })
}

export async function testTranslationProvider(provider: TranslationProvider): Promise<string> {
	return await invoke('plugin:translation|translation_test_provider', { provider })
}

export async function translate(request: TranslationRequest): Promise<TranslationResponse> {
	return await invoke('plugin:translation|translation_translate', { request })
}

export async function clearTranslationCache(): Promise<void> {
	await invoke('plugin:translation|translation_clear_cache')
}

export type TranslationErrorKind =
	| 'rate-limited'
	| 'authentication'
	| 'content-too-long'
	| 'network'
	| 'provider'

function translationErrorMessage(error: unknown): string {
	if (error instanceof Error) return error.message
	if (typeof error === 'string') return error
	if (
		typeof error === 'object' &&
		error !== null &&
		'message' in error &&
		typeof error.message === 'string'
	) {
		return error.message
	}
	return String(error)
}

export function getTranslationErrorKind(error: unknown): TranslationErrorKind {
	const message = translationErrorMessage(error)
	if (message.includes('TRANSLATION_RATE_LIMITED')) return 'rate-limited'
	if (message.includes('TRANSLATION_AUTHENTICATION_FAILED')) return 'authentication'
	if (message.includes('TRANSLATION_CONTENT_TOO_LONG')) return 'content-too-long'
	if (message.includes('TRANSLATION_NETWORK_FAILED')) return 'network'
	return 'provider'
}

function containsReadableText(element: Element): boolean {
	if (element.matches('pre, script, style, video, audio, iframe')) return false
	const clone = element.cloneNode(true) as Element
	clone.querySelectorAll('pre, code, script, style').forEach((node) => node.remove())
	clone.querySelectorAll('a').forEach((node) => {
		if (isUrlOnlyText(node.textContent ?? '')) node.remove()
	})
	return (clone.textContent ?? '').trim().length > 0
}

function isUrlOnlyText(value: string): boolean {
	return /^(?:https?:\/\/|www\.|mailto:)[^\s]+$/i.test(value.trim())
}

function protectElementAttributes(
	element: Element,
	blockIndex: number,
): Record<string, ProtectedElement> {
	const protectedElements: Record<string, ProtectedElement> = {}
	const elements = [element, ...Array.from(element.querySelectorAll('*'))]

	elements.forEach((current, elementIndex) => {
		const marker = `${blockIndex}-${elementIndex}`
		const attributes = Array.from(current.attributes).map(
			(attribute) => [attribute.name, attribute.value] as [string, string],
		)
		protectedElements[marker] = {
			tagName: current.tagName,
			attributes,
			...(current.matches('code, pre') ||
			(current.matches('a') && isUrlOnlyText(current.textContent ?? ''))
				? { innerHtml: current.innerHTML }
				: {}),
		}

		Array.from(current.attributes).forEach((attribute) => current.removeAttribute(attribute.name))
		current.setAttribute('data-ax-translation-attr', marker)
		if (protectedElements[marker].innerHtml !== undefined) current.setAttribute('translate', 'no')
	})

	return protectedElements
}

export function prepareDescription(
	description: string,
	sourceFormat: DescriptionSourceFormat = 'markdown',
): PreparedDescription {
	const renderedDescription =
		sourceFormat === 'html'
			? configuredXss.process(description ?? '')
			: renderHighlightedString(description ?? '')
	const document = new DOMParser().parseFromString(
		`<body>${renderedDescription}</body>`,
		'text/html',
	)
	const blocks: PreparedDescriptionBlock[] = []
	const segments: TranslationSegment[] = []

	Array.from(document.body.children).forEach((source, index) => {
		const id = `body-${index}`
		const originalHtml = configuredXss.process(source.outerHTML)
		const translatable = containsReadableText(source)
		const clone = source.cloneNode(true) as Element
		const protectedElements = translatable ? protectElementAttributes(clone, index) : {}

		blocks.push({ id, originalHtml, translatable, protectedElements })
		if (translatable) {
			segments.push({ id, text: clone.outerHTML, format: 'html' })
		}
	})

	return { blocks, segments }
}

function restoreTranslatedBlock(block: PreparedDescriptionBlock, translatedHtml: string): string {
	const document = new DOMParser().parseFromString(`<body>${translatedHtml}</body>`, 'text/html')
	const root = document.body.firstElementChild
	const translatedElements = document.body.querySelectorAll('*')
	if (
		!root ||
		document.body.children.length !== 1 ||
		translatedElements.length !== Object.keys(block.protectedElements).length ||
		Array.from(translatedElements).some(
			(element) => !element.hasAttribute('data-ax-translation-attr'),
		)
	) {
		throw new Error(`Translation markup changed for block ${block.id}`)
	}

	for (const [marker, protectedElement] of Object.entries(block.protectedElements)) {
		const matches = document.body.querySelectorAll(`[data-ax-translation-attr="${marker}"]`)
		if (matches.length !== 1 || matches[0].tagName !== protectedElement.tagName) {
			throw new Error(`Translation markup changed for block ${block.id}`)
		}
		const element = matches[0]
		Array.from(element.attributes).forEach((attribute) => element.removeAttribute(attribute.name))
		protectedElement.attributes.forEach(([name, value]) => element.setAttribute(name, value))
		if (protectedElement.innerHtml !== undefined) element.innerHTML = protectedElement.innerHtml
	}

	return configuredXss.process(root.outerHTML)
}

function translationStyleClass(style: TranslationStyle): string {
	return `ax-translation-style-${style}`
}

function restorePreparedDescription(
	prepared: PreparedDescription,
	translations: Record<string, string>,
): Map<string, string> {
	const restored = new Map<string, string>()
	for (const block of prepared.blocks) {
		if (!block.translatable) continue
		const translated = translations[block.id]
		if (!translated) throw new Error(`Missing translated block ${block.id}`)
		restored.set(block.id, restoreTranslatedBlock(block, translated))
	}
	return restored
}

export function validateTranslatedDescription(
	prepared: PreparedDescription,
	translations: Record<string, string>,
): void {
	restorePreparedDescription(prepared, translations)
}

export function renderTranslatedDescription(
	prepared: PreparedDescription,
	translations: Record<string, string>,
	mode: TranslationMode,
	style: TranslationStyle,
): string {
	let restored: Map<string, string>
	try {
		restored = restorePreparedDescription(prepared, translations)
	} catch {
		return prepared.blocks.map((block) => block.originalHtml).join('')
	}

	return prepared.blocks
		.map((block) => {
			if (!block.translatable) return block.originalHtml
			const translated = restored.get(block.id) ?? block.originalHtml
			if (mode === 'translation-only') return translated
			return `${block.originalHtml}<div class="ax-translation-block ${translationStyleClass(style)}">${translated}</div>`
		})
		.join('')
}

const translationCache = new Map<string, { title: string; description: string } | undefined>()

export async function translateSearchHits<T extends TranslatableHit>(
	hits: T[],
	force = false,
): Promise<T[]> {
	if (hits.length === 0) return hits

	const settings = await getTranslationSettings()
	if (!force && !settings.auto_translate) return hits

	const targetLanguage = settings.target_language || i18n.global.locale.value || 'en-US'
	if (!targetLanguage) return hits

	const hitsToTranslate: T[] = []
	const segments: TranslationSegment[] = []

	for (const hit of hits) {
		const key = hit.project_id ?? hit.id
		if (!key) continue
		const cached = translationCache.get(key)
		if (cached) {
			// Already cached — patch below.
			hitsToTranslate.push(hit)
			continue
		}
		const title = hit.title ?? hit.name ?? ''
		const description = hit.description ?? hit.summary ?? ''
		if (!title && !description) continue

		hitsToTranslate.push(hit)
		segments.push(
			{ id: `title:${key}`, text: title, format: 'plain' },
			{ id: `description:${key}`, text: description, format: 'plain' },
		)
	}

	if (segments.length === 0 && hitsToTranslate.length > 0) {
		// All hits are already in cache — still return a new array so callers
		// can detect that translation is active (translated !== hits).
		return hits.map(applyCachedTranslation)
	}
	if (segments.length === 0) return hits

	const response = await translate({
		source_language: 'auto',
		target_language: targetLanguage,
		context: { title: '', description: '' },
		segments,
	}).catch(() => null)

	if (!response) return hits

	const translatedMap = new Map<string, { title: string; description: string }>()
	for (const seg of response.segments) {
		const [, projectId] = seg.id.split(':', 2) as [string, string]
		const field = seg.id.startsWith('title:') ? 'title' : 'description'
		if (!translatedMap.has(projectId)) {
			translatedMap.set(projectId, { title: '', description: '' })
		}
		const entry = translatedMap.get(projectId)!
		entry[field] = seg.text
	}

	// Write cache
	for (const [projectId, translated] of translatedMap) {
		translationCache.set(projectId, translated)
	}

	return hits.map(applyCachedTranslation)

	/** Apply cached translation to a single hit, falling back to original fields. */
	function applyCachedTranslation<T extends TranslatableHit>(hit: T): T {
		const key = hit.project_id ?? hit.id
		if (!key) return hit
		const translated = translationCache.get(key)
		if (!translated) return hit
		return {
			...hit,
			title: translated.title || hit.title,
			description: translated.description || hit.description,
			name: translated.title || hit.name || hit.title,
			summary: translated.description || hit.summary || hit.description,
		}
	}
}
