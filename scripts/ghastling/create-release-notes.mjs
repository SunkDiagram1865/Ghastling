import fs from 'node:fs/promises'
import { createRequire } from 'node:module'

const require = createRequire(new URL('../../apps/app-frontend/package.json', import.meta.url))
const ts = require('typescript')

const [tag, outputPath] = process.argv.slice(2)
const version = tag?.replace(/^v/, '')

if (!version || !outputPath) {
	throw new Error(
		'Usage: node scripts/ghastling/create-release-notes.mjs <version-tag> <output-path>',
	)
}

const catalogSource = await fs.readFile('apps/app-frontend/src/announcements/catalog.ts', 'utf8')
const catalogModule = await import(
	`data:text/javascript;base64,${Buffer.from(
		ts.transpileModule(catalogSource, {
			compilerOptions: {
				module: ts.ModuleKind.ESNext,
				target: ts.ScriptTarget.ES2022,
			},
		}).outputText,
	).toString('base64')}`
)

const announcement = catalogModule.getAnnouncementByVersion(version)
if (!announcement) {
	throw new Error(`No bundled announcement found for release ${version}`)
}

const categoryLabels = {
	added: '新增',
	changed: '变更',
	deprecated: '弃用',
	removed: '移除',
	fixed: 'Bug 修复',
	security: '安全修复',
}

const lines = [
	`# ${announcement.title['zh-CN']}`,
	'',
	`发布日期：${announcement.publishedAt}`,
	'',
]

for (const type of catalogModule.ANNOUNCEMENT_CHANGE_TYPES) {
	const changes = announcement.changes[type]
	if (!changes?.length) continue

	lines.push(`## ${categoryLabels[type]}`, '')
	for (const change of changes) {
		lines.push(`- ${change['zh-CN']}`)
	}
	lines.push('')
}

if (announcement.notes) {
	lines.push('## 说明', '', announcement.notes['zh-CN'], '')
}

await fs.writeFile(outputPath, `${lines.join('\n').replace(/\n+$/, '')}\n`)
console.log(`Generated release notes for ${version} from the launcher announcement catalog.`)
