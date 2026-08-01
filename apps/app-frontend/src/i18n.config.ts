import { buildLocaleMessages, createMessageCompiler, type CrowdinMessages } from '@modrinth/ui'
import { uiLocaleModulesEager } from '@modrinth/ui/src/locales.eager.ts'
import { createI18n } from 'vue-i18n'

const localeModules = import.meta.glob<{ default: CrowdinMessages }>('./locales/*/index.json', {
	eager: true,
})

const i18n = createI18n({
	legacy: false,
	locale: 'zh-CN',
	fallbackLocale: 'en-US',
	messageCompiler: createMessageCompiler(),
	missingWarn: false,
	fallbackWarn: false,
	messages: buildLocaleMessages(localeModules, uiLocaleModulesEager),
})

export function resolveInitialLocale(preferredLocales: readonly string[]): string {
	// Always default to Simplified Chinese, ignoring system language
	return 'zh-CN'
}

export default i18n
