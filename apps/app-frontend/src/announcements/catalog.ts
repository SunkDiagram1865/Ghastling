export type AnnouncementChangeType =
	| 'added'
	| 'changed'
	| 'deprecated'
	| 'removed'
	| 'fixed'
	| 'security'

export type LocalizedAnnouncementText = Readonly<{ 'zh-CN': string }>

export type AnnouncementChange = LocalizedAnnouncementText

export type LauncherAnnouncement = {
	readonly id: string
	readonly version: string
	readonly publishedAt: string
	readonly title: LocalizedAnnouncementText
	readonly changes: Readonly<Partial<Record<AnnouncementChangeType, readonly AnnouncementChange[]>>>
	readonly notes?: LocalizedAnnouncementText
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
		id: 'launcher-0.0.1',
		version: '0.0.1',
		publishedAt: '2026-07-30',
		title: {
			'zh-CN': 'Ghastling Launcher 0.0.1',
		},
		changes: {
			added: [
				{
					'zh-CN': 'Ghastling Launcher 首个版本发布，基于 Modrinth 和 Axolotl 进行二次开发的 Minecraft 启动器。',
				},
				{
					'zh-CN': '便携模式：数据存储在可执行文件所在目录 (./com.cysunk.ghestling)。',
				},
				{
					'zh-CN': '自定义强调色和 Ghastling 品牌定制。',
				},
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

export function getLocalizedAnnouncementText(
	text: LocalizedAnnouncementText,
	_locale: string,
): string {
	return text['zh-CN']
}
