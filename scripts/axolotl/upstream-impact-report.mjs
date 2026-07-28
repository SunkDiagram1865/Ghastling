import { execFileSync } from 'node:child_process'
import { writeFile } from 'node:fs/promises'

const base = process.argv[2] ?? 'origin/main'
const head = process.argv[3] ?? 'HEAD'
const output = process.argv[4] ?? 'upstream-impact.md'

function git(...args) {
	return execFileSync('git', args, { encoding: 'utf8' }).trim()
}

const files = git('diff', '--name-only', `${base}...${head}`).split(/\r?\n/).filter(Boolean)

const categories = [
	['Desktop source', /^(apps\/app(?:-frontend)?|packages\/app-lib)\//],
	['Branding and assets', /(icon|logo|brand|tauri[^/]*\.conf|Info\.plist|COPYING|README)/i],
	['Ads and telemetry', /(ads?|aditude|posthog|analytics|telemetry|promotion)/i],
	['API and service endpoints', /(api|client|fetch|auth|friends|hosting|archon|intercom)/i],
	['Translations', /(locales|i18n|crowdin)/i],
	['Database migrations', /packages\/app-lib\/migrations\//],
	['Tauri permissions and updater', /(capabilities|permissions|updater|tauri-release)/i],
]

const sections = categories.map(([title, pattern]) => [
	title,
	files.filter((file) => pattern.test(file)),
])
const addedSensitiveLines = git(
	'diff',
	'--unified=0',
	`${base}...${head}`,
	'--',
	'apps/app',
	'apps/app-frontend',
	'packages/app-lib',
)
	.split(/\r?\n/)
	.filter(
		(line) =>
			line.startsWith('+') &&
			!/^[+]{3}/.test(line) &&
			/(Modrinth App|modrinth:\/\/|ads?|aditude|posthog|telemetry|updater|archon|intercom|friends|https?:\/\/)/i.test(
				line,
			),
	)
	.slice(0, 200)

const lines = [
	'# Axolotl upstream impact report',
	'',
	`Compared \`${base}\` with \`${head}\`. ${files.length} file(s) changed.`,
	'',
]

for (const [title, matches] of sections) {
	lines.push(`## ${title}`, '')
	lines.push(matches.length > 0 ? matches.map((file) => `- \`${file}\``).join('\n') : '- None')
	lines.push('')
}

lines.push('## Sensitive added lines', '')
lines.push(
	addedSensitiveLines.length > 0
		? ['```diff', ...addedSensitiveLines, '```'].join('\n')
		: '- No matching added lines.',
)
lines.push('', 'The Axolotl brand and localization guards must pass before merge.', '')

await writeFile(output, lines.join('\n'))
console.log(`Wrote ${output}`)
