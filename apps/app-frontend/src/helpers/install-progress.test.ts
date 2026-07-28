import assert from 'node:assert/strict'
import test from 'node:test'

import {
	effectiveInstallProgress,
	hasDeterminateInstallProgress,
	installProgressFraction,
} from './install-progress.ts'

test('clears completed content progress when the next phase has no progress', () => {
	const completed = {
		phase: 'downloading_content',
		progress: { current: 10, total: 10 },
	}
	assert.equal(installProgressFraction(completed), 1)

	const nextPhase = {
		phase: 'downloading_minecraft',
		progress: null,
	}
	assert.equal(effectiveInstallProgress(nextPhase), null)
	assert.equal(installProgressFraction(nextPhase), null)
})

test('treats zero and non-finite totals as indeterminate', () => {
	for (const total of [0, Number.NaN, Number.POSITIVE_INFINITY]) {
		const progress = { current: 1, total }
		assert.equal(hasDeterminateInstallProgress(progress), false)
		assert.equal(installProgressFraction({ phase: 'downloading_minecraft', progress }), null)
	}
})
