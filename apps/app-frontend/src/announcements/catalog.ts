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
		id: 'launcher-0.0.5·',
		version: '0.0.5',
		publishedAt: '2026-08-12',
		title: 'Ghastling Launcher 0.0.5',
		changes: {
			added: ['陶瓦联机进程结束按钮。'],
			changed: ['设置界面的调整（Java 管理，资源管理，更新）。'],
		},
	},
	{
		id: 'launcher-0.0.4·',
		version: '0.0.4',
		publishedAt: '2026-08-11',
		title: 'Ghastling Launcher 0.0.4',
		changes: {
			added: ['同步 Axolotl 系统代理功能。', '离线账号自定义 UUID 功能。', '复制账户 UUID 功能。'],
			changed: [
				'导览流程更新。',
				'优化 Ghastling 在系统托盘逻辑。',
				'汉化游玩时长单位显示。',
				'修正 Modrinth 托管部分汉化。',
			],
			fixed: ['修复发现页小窗标题显示异常问题。', '修复小窗主界面显示异常问题。'],
		},
	},
	{
		id: 'launcher-0.0.3·',
		version: '0.0.3',
		publishedAt: '2026-08-04',
		title: 'Ghastling Launcher 0.0.3',
		changes: {
			added: [
				'高级设置调试功能。',
				'添加启动器至系统托盘。',
				'增加下载 Java 下载图标。',
				'增加 Modrinth 服务器界面。',
				'可以自定义开关拖放功能。',
				'主页界面更新。',
			],
			fixed: [
				'资源下载页服务器与其他界面 UI 对齐。',
				'修复实例设置 Java 与内存分配路径过长导致文字超出 UI 范围。',
			],
		},
	},
	{
		id: 'launcher-0.0.2·',
		version: '0.0.2',
		publishedAt: '2026-08-02',
		title: 'Ghastling Launcher 0.0.2',
		changes: {
			added: [
				'启动器账号分类，不同账号登录方式的分组。',
				'增加陶瓦联机功能，实现 P2P 联机功能。',
				'增加下载 Java 下载图标。',
				'启动器开屏界面添加 Ghastling 音效。',
				'皮肤管理页添加披风切换功能。',
				'侧边栏实例数量显示，可以设置成 0 隐藏。',
			],
			removed: ['移除多语言支持，仅支持中文。', '移除 Axolotl 首次开屏弹窗。'],
			fixed: [
				'下载游戏版本未检测到 Java 在尝试下载 Java 时校验导致失败的 bug。',
				'修复缩略图无法正常应用的 bug。',
				'修复 Minecraft 进程正常结束启动器显示崩溃的 bug。',
			],
		},
	},
	{
		id: 'launcher-0.0.1·',
		version: '0.0.1',
		publishedAt: '2026-07-30',
		title: 'Ghastling Launcher 0.0.1',
		changes: {
			added: [
				'Ghastling Launcher 首个版本发布，基于 Modrinth 和 Axolotl 进行二次开发的 Minecraft 启动器。',
				'便携模式：数据存储在可执行文件所在目录 (./com.cysunk.ghestling)。',
				'自定义强调色和 Ghastling 品牌定制。',
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
