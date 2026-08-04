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
		id: 'launcher-0.0.3',
		version: '0.0.3',
		publishedAt: '2026-08-04',
		title: {
			'zh-CN': 'Ghastling Launcher 0.0.3',
		},
		changes: {
			added: [
				{
					'zh-CN': '高级设置调试功能。',
				},
				{
					'zh-CN': '添加启动器至系统托盘。',
				},
				{
					'zh-CN': '增加下载 Java 下载图标',
				},
				{
					'zh-CN': '增加 Modrinth 服务器界面',
				},
				{
					'zh-CN': '可以自定义开关拖放功能。',
				},
				{
					'zh-CN': '主页界面更新。',
				},
			],
			fixed: [
				{
					'zh-CN': '资源下载页服务器与其他界面 UI 对齐。',
				},
				{
					'zh-CN': '修复实例设置 Java 与内存分配路径过长导致文字超出 UI 范围。',
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
