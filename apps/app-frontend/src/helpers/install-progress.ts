export interface ProgressValue {
	current: number
	total: number
	secondary?: ProgressValue | null
}

export interface ProgressSnapshot {
	phase: string
	progress?: ProgressValue | null
}

export function effectiveInstallProgress(
	snapshot: ProgressSnapshot,
): ProgressValue | null | undefined {
	if (snapshot.phase === 'downloading_content' && snapshot.progress?.secondary) {
		return snapshot.progress.secondary
	}

	return snapshot.progress
}

export function hasDeterminateInstallProgress(
	progress: ProgressValue | null | undefined,
): progress is ProgressValue {
	return (
		progress != null &&
		Number.isFinite(progress.current) &&
		Number.isFinite(progress.total) &&
		progress.current >= 0 &&
		progress.total > 0
	)
}

export function installProgressFraction(snapshot: ProgressSnapshot): number | null {
	const progress = effectiveInstallProgress(snapshot)
	if (!hasDeterminateInstallProgress(progress)) return null

	return Math.max(0, Math.min(1, progress.current / progress.total))
}
