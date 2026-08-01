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
		id: 'launcher-0.0.2',
		version: '0.0.2',
		publishedAt: '2026-08-01',
		title: {
			'zh-CN': 'Ghastling Launcher 0.0.2',
		},
		changes: {
			added: [
				{
					'zh-CN': '启动器账号分类，不同账号登录方式的分组。',
				},
				{
					'zh-CN': '增加陶瓦联机功能，实现 P2P 联机功能。',
				},
				{
					'zh-CN': '增加下载 Java 下载图标',
				},
				{
					'zh-CN': '启动器开屏界面添加 Ghastling 音效',
				},
				{
					'zh-CN': '皮肤管理页添加披风切换功能。',
				},
				{
					'zh-CN': '侧边栏实例数量显示，可以设置成 0 隐藏。',
				},
			],
			fixed: [
				{
					'zh-CN': '下载游戏版本未检测到 Java 在尝试下载 Java 时校验导致失败的 bug。',
				},
				{
					'zh-CN': '修复缩略图无法正常应用的 bug。',
				},
				{
					'zh-CN': '修复 Minecraft 进程正常结束启动器显示崩溃的 bug。',
				},
			],
			removed: [
				{
					'zh-CN': '移除多语言支持，仅支持中文。',
				},
				{
					'zh-CN': '移除 Axolotl 首次开屏弹窗。',
				}
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
