import { execFileSync } from 'node:child_process'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { Readable } from 'node:stream'
import { pipeline } from 'node:stream/promises'

const [command, tag, outputDirectory] = process.argv.slice(2)
const apiEndpoint = (process.env.CNB_API_ENDPOINT || 'https://api.cnb.cool').replace(/\/$/, '')
const repo = process.env.CNB_REPO_SLUG || 'ghastling/Ghastling'
const token = process.env.CNB_TOKEN
const tokenUser = process.env.CNB_TOKEN_USER_NAME || 'cnb'
const repoUrl = process.env.CNB_REPO_URL_HTTPS || `https://cnb.cool/${repo}.git`
const githubReleaseBaseUrl = (
	process.env.GITHUB_RELEASE_BASE_URL ||
	'https://github.com/SunkDiagram1865/Ghastling/releases/download'
).replace(/\/$/, '')
const githubApiBaseUrl = (
	process.env.GITHUB_API_BASE_URL || 'https://api.github.com/repos/SunkDiagram1865/Ghastling'
).replace(/\/$/, '')
const pollIntervalMs = Number(process.env.RELEASE_POLL_INTERVAL_MS || 30_000)

if (command !== 'finalize' || !tag || !outputDirectory || !token) {
	throw new Error('Usage: node cnb-release.mjs finalize <tag> <output-dir>; CNB_TOKEN is required')
}

const apiHeaders = {
	Accept: 'application/vnd.cnb.api+json',
	Authorization: `Bearer ${token}`,
}

async function apiRequest(url, options = {}, allowedStatuses = []) {
	const response = await fetch(url, {
		...options,
		headers: {
			...apiHeaders,
			...options.headers,
		},
	})
	if (!response.ok && !allowedStatuses.includes(response.status)) {
		throw new Error(
			`${options.method || 'GET'} ${url} failed (${response.status}): ${await response.text()}`,
		)
	}
	return response
}

async function getRelease() {
	const response = await apiRequest(
		`${apiEndpoint}/${repo}/-/releases/tags/${encodeURIComponent(tag)}`,
		{},
		[404],
	)
	return response.status === 404 ? null : await response.json()
}

async function ensureRelease() {
	const existing = await getRelease()
	if (existing) {
		return existing
	}

	const prerelease = tag.includes('-')
	const response = await apiRequest(
		`${apiEndpoint}/${repo}/-/releases`,
		{
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				tag_name: tag,
				target_commitish: process.env.CNB_COMMIT || tag,
				name: process.env.CNB_TAG_RELEASE_TITLE || `Ghastling Launcher ${tag}`,
				body: process.env.CNB_TAG_RELEASE_DESC || '',
				draft: true,
				prerelease,
				make_latest: 'false',
			}),
		},
		[409],
	)
	if (response.status !== 409) {
		return await response.json()
	}

	for (let attempt = 0; attempt < 10; attempt++) {
		await new Promise((resolve) => setTimeout(resolve, 1000))
		const release = await getRelease()
		if (release) {
			return release
		}
	}
	throw new Error(`Release ${tag} was created concurrently but could not be loaded`)
}

async function uploadAsset(release, filePath) {
	const assetName = path.basename(filePath)
	const size = fs.statSync(filePath).size
	const uploadResponse = await apiRequest(
		`${apiEndpoint}/${repo}/-/releases/${release.id}/asset-upload-url`,
		{
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ asset_name: assetName, size, overwrite: true, ttl: 0 }),
		},
	)
	const upload = await uploadResponse.json()
	const fileResponse = await fetch(upload.upload_url, {
		method: 'PUT',
		body: fs.createReadStream(filePath),
		duplex: 'half',
		headers: {
			'Content-Length': String(size),
			'Content-Type': 'application/octet-stream',
		},
	})
	if (!fileResponse.ok) {
		throw new Error(
			`Uploading ${assetName} failed (${fileResponse.status}): ${await fileResponse.text()}`,
		)
	}

	const verifyUrl = new URL(upload.verify_url, apiEndpoint).toString()
	await apiRequest(`${verifyUrl}${verifyUrl.includes('?') ? '&' : '?'}ttl=0`, { method: 'POST' })
	console.log(`Uploaded ${assetName}`)
}

async function waitForGithubManifest() {
	const manifestUrl = `${githubReleaseBaseUrl}/${encodeURIComponent(tag)}/latest.json`
	for (let attempt = 0; attempt < 180; attempt++) {
		let response
		try {
			response = await fetch(manifestUrl)
		} catch (error) {
			console.log(`GitHub release is not ready: ${error}`)
		}

		if (response?.ok) {
			const manifest = await response.json()
			if (manifest.version !== tag.replace(/^v/, '')) {
				throw new Error(`GitHub manifest version ${manifest.version} does not match ${tag}`)
			}
			return manifest
		}
		if (response && response.status !== 404 && response.status < 500) {
			throw new Error(
				`Downloading GitHub manifest failed (${response.status}): ${await response.text()}`,
			)
		}

		console.log(`Waiting for GitHub release manifest (${attempt + 1}/180)`)
		await new Promise((resolve) => setTimeout(resolve, pollIntervalMs))
	}
	throw new Error('Timed out waiting for the GitHub release manifest')
}

async function loadGithubRelease(url) {
	const response = await fetch(url, {
		headers: {
			Accept: 'application/vnd.github+json',
			'User-Agent': 'Ghastling-CNB-Release',
		},
	})
	if (!response.ok) {
		throw new Error(`Loading GitHub release failed (${response.status}): ${await response.text()}`)
	}
	return await response.json()
}

async function getGithubRelease() {
	return await loadGithubRelease(`${githubApiBaseUrl}/releases/tags/${encodeURIComponent(tag)}`)
}

async function getLatestGithubRelease() {
	return await loadGithubRelease(`${githubApiBaseUrl}/releases/latest`)
}

async function downloadFile(url, outputPath) {
	const response = await fetch(url)
	if (!response.ok || !response.body) {
		throw new Error(`Downloading ${url} failed (${response.status}): ${await response.text()}`)
	}
	await pipeline(Readable.fromWeb(response.body), fs.createWriteStream(outputPath))
}

async function mirrorGithubAssets(release, githubRelease) {
	const mirroredNames = new Set()
	for (const asset of githubRelease.assets || []) {
		if (asset.name === 'latest.json') {
			continue
		}
		if (!asset.name || !asset.browser_download_url || mirroredNames.has(asset.name)) {
			throw new Error(`Invalid or duplicate GitHub release asset: ${asset.name}`)
		}
		const outputPath = path.join(outputDirectory, asset.name)
		await downloadFile(asset.browser_download_url, outputPath)
		await uploadAsset(release, outputPath)
		mirroredNames.add(asset.name)
	}
	return mirroredNames
}

function createCnbManifest(githubManifest, mirroredNames) {
	const requiredPlatforms = [
		'darwin-aarch64',
		'darwin-x86_64',
		'linux-aarch64',
		'linux-x86_64',
		'windows-x86_64',
	]
	const platforms = {}
	for (const platform of requiredPlatforms) {
		const update = githubManifest.platforms?.[platform]
		if (!update || typeof update.signature !== 'string' || update.signature.trim().length < 32) {
			throw new Error(`Missing signed GitHub update for ${platform}`)
		}
		const filename = decodeURIComponent(path.posix.basename(new URL(update.url).pathname))
		if (!mirroredNames.has(filename)) {
			throw new Error(`GitHub release is missing updater artifact ${filename} for ${platform}`)
		}
		platforms[platform] = {
			...update,
			url: `https://cnb.cool/${repo}/-/releases/download/${encodeURIComponent(tag)}/${encodeURIComponent(filename)}`,
		}
	}

	return {
		...githubManifest,
		version: tag.replace(/^v/, ''),
		platforms,
	}
}

function publishUpdateBranch(manifestPath) {
	if (tag.includes('-')) {
		return
	}

	const directory = fs.mkdtempSync(path.join(os.tmpdir(), 'ghastling-cnb-update-'))
	const auth = Buffer.from(`${tokenUser}:${token}`).toString('base64')
	const git = (...args) => execFileSync('git', args, { cwd: directory, stdio: 'inherit' })
	git('init')
	git('config', 'user.name', 'Ghastling CNB Release')
	git('config', 'user.email', 'build@cnb.cool')
	git('checkout', '--orphan', 'update')
	fs.copyFileSync(manifestPath, path.join(directory, 'latest.json'))
	git('add', 'latest.json')
	git('commit', '-m', `Publish ${tag}`)
	git('remote', 'add', 'origin', repoUrl)
	git(
		'-c',
		`http.extraHeader=Authorization: Basic ${auth}`,
		'push',
		'--force',
		'origin',
		'HEAD:update',
	)
}

async function finalizeRelease() {
	fs.mkdirSync(outputDirectory, { recursive: true })
	const githubManifest = await waitForGithubManifest()
	const [githubRelease, latestGithubRelease] = await Promise.all([
		getGithubRelease(),
		getLatestGithubRelease(),
	])
	const isLatestRelease = githubRelease.tag_name === latestGithubRelease.tag_name
	const release = await ensureRelease()
	const mirroredNames = await mirrorGithubAssets(release, githubRelease)
	const manifest = createCnbManifest(githubManifest, mirroredNames)
	const manifestPath = path.join(outputDirectory, 'latest.json')
	fs.writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`)
	await uploadAsset(release, manifestPath)

	const prerelease = tag.includes('-')
	await apiRequest(`${apiEndpoint}/${repo}/-/releases/${release.id}`, {
		method: 'PATCH',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({
			draft: false,
			prerelease,
			make_latest: isLatestRelease ? 'true' : 'false',
		}),
	})
	if (isLatestRelease) {
		publishUpdateBranch(manifestPath)
	} else {
		console.log(`Skipped update branch for non-latest GitHub release ${tag}`)
	}
	console.log(`Published CNB release ${tag}`)
}

await finalizeRelease()
