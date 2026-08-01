interface GitHubRelease {
	tag_name: string
}

// GitHub 最新 release 接口（未认证，限速 60 次/小时/IP）
const GITHUB_API_URL = 'https://api.github.com/repos/SunkDiagram1865/Ghastling/releases/latest'
// 拉取失败时的兜底链接，指向最新 release 页面
const FALLBACK_URL = 'https://github.com/SunkDiagram1865/Ghastling/releases/latest'
// 下载直链模板：{version} 会被替换为不带前缀 v 的最新版本号
const DOWNLOAD_URL_TEMPLATE =
	'https://github.com/SunkDiagram1865/Ghastling/releases/latest/download/Ghastling_{version}_x64.zip'

export function useLatestRelease() {
	const downloadUrl = useState<string>('ghastling-latest-download-url', () => FALLBACK_URL)
	const version = useState<string | undefined>('ghastling-latest-version', () => undefined)
	const fetched = useState<boolean>('ghastling-latest-fetched', () => false)

	async function fetchLatest() {
		if (fetched.value) return
		fetched.value = true
		try {
			const release = await $fetch<GitHubRelease>(GITHUB_API_URL, {
				headers: { Accept: 'application/vnd.github+json' },
			})
			const tag = release.tag_name
			if (tag) {
				const ver = tag.replace(/^v/, '')
				version.value = ver
				downloadUrl.value = DOWNLOAD_URL_TEMPLATE.replace('{version}', ver)
			}
		} catch {
			// 拉取失败时保留兜底链接，允许下次挂载时重试
			fetched.value = false
		}
	}

	onMounted(() => {
		fetchLatest()
	})

	return {
		downloadUrl,
		version,
		fetchLatest,
	}
}
