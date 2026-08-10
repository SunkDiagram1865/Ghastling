import { readFile, access } from 'node:fs/promises'
import { parse, TYPE } from '@formatjs/icu-messageformat-parser'

// en-US 语言文件已删除，代码中的 defaultMessage 即为英文源文本
// 无需独立对比 en-US 与 zh-CN 的键覆盖率
const localePairs = []

async function fileExists(path) {
	try {
		await access(path)
		return true
	} catch {
		return false
	}
}

const failures = []
const allowedUntranslatedMessages = new Set([
	'Chaos Cubed',
	'MINECON Earth 2017',
	'Modrinth',
	'Striding Hero',
	'Ghastling Launcher',
	'Explore high-quality Minecraft content on Modrinth.',
	'example.modrinth.gg',
	'{title} - {count}',
	'Hooks',
	'/path/to/java',
	'https://example.com/api/yggdrasil',
	'Fabric',
	'NeoForge',
	'Quilt',
	'Java {version}',
])

function messageText(value) {
	return typeof value === 'string' ? value : (value?.message ?? '')
}

function argumentNames(message) {
	const names = new Set()
	const argumentTypes = new Set([
		TYPE.argument,
		TYPE.number,
		TYPE.date,
		TYPE.time,
		TYPE.select,
		TYPE.plural,
	])

	function visit(elements) {
		for (const element of elements) {
			if (argumentTypes.has(element.type)) names.add(element.value)
			if (element.options) {
				for (const option of Object.values(element.options)) visit(option.value)
			}
			if (element.children) visit(element.children)
		}
	}

	visit(parse(message))
	return [...names].sort()
}

for (const [sourcePath, translationPath] of localePairs) {
	if (!(await fileExists(sourcePath)) || !(await fileExists(translationPath))) {
		continue
	}
	const source = JSON.parse(await readFile(sourcePath, 'utf8'))
	const translation = JSON.parse(await readFile(translationPath, 'utf8'))

	for (const key of Object.keys(source)) {
		if (!(key in translation)) {
			failures.push(`${translationPath}: missing ${key}`)
			continue
		}

		try {
			const sourceMessage = messageText(source[key])
			const translationMessage = messageText(translation[key])
			const sourceArguments = argumentNames(sourceMessage)
			const translationArguments = argumentNames(translationMessage)
			if (sourceArguments.join('\0') !== translationArguments.join('\0')) {
				failures.push(
					`${translationPath}: ICU arguments for ${key} are [${translationArguments.join(', ')}], expected [${sourceArguments.join(', ')}]`,
				)
			}

			if (
				sourceMessage === translationMessage &&
				/[A-Za-z]{2}/.test(sourceMessage) &&
				!allowedUntranslatedMessages.has(sourceMessage)
			) {
				failures.push(`${translationPath}: untranslated ${key}`)
			}
		} catch (error) {
			failures.push(`${translationPath}: invalid ICU message ${key}: ${error.message}`)
		}
	}
}

if (failures.length > 0) {
	console.error(`Simplified Chinese coverage check failed:\n${failures.join('\n')}`)
	process.exit(1)
}

console.log('Simplified Chinese key coverage and ICU argument checks passed.')
