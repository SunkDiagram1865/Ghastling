export type AnnouncementChangeType =
	| 'added'
	| 'changed'
	| 'deprecated'
	| 'removed'
	| 'fixed'
	| 'security'

export type AnnouncementChange = string

export type LauncherAnnouncement = {
	readonly id: string
	readonly version: string
	readonly publishedAt: string
	readonly title: string
	readonly changes: Readonly<Partial<Record<AnnouncementChangeType, readonly AnnouncementChange[]>>>
	readonly notes?: string
	readonly externalUrl?: string
}

export const ANNOUNCEMENT_CHANGE_TYPES: readonly AnnouncementChangeType[] = [
	'added',
	'changed',
	'deprecated',
	'removed',
	'fixed',
	'security',
]

export const launcherAnnouncements: readonly LauncherAnnouncement[] = [
	{
		id: 'launcher-0.0.4·',
		version: '0.0.4',
		publishedAt: '2026-08-10',
		title: 'Ghastling Launcher 0.0.4',
		changes: {
			added: [
				'同步 Axolotl 系统代理功能。',
				'离线账号自定义 UUID 功能。',
				'复制账户 UUID 功能。',
			],
			changed: [
				'导览流程更新。',
				'优化 Ghastling 在系统托盘逻辑。',
				'汉化游玩时长单位显示。',
				'修正 Modrinth 托管部分汉化。'
			],
			fixed: [
				'修复发现页小窗标题显示异常问题。',
				'修复小窗主界面显示异常问题。',
			],
		},
	},
]

export function getAnnouncementByVersion(version: string | null | undefined) {
	if (!version) return undefined
	return launcherAnnouncements.find((announcement) => announcement.version === version)
}

export function getAnnouncements(): readonly LauncherAnnouncement[] {
	return launcherAnnouncements
}

export function getAnnouncementById(id: string) {
	return launcherAnnouncements.find((announcement) => announcement.id === id)
}

export function getLocalizedAnnouncementText(text: string): string {
	return text
}
