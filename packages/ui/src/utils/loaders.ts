import type { Archon } from '@modrinth/api-client'

export type ServerLoader = Archon.Servers.v0.Loader | 'Bukkit'

export const loaderDisplayNames: Record<string, string> = {
	fabric: 'Fabric',
	neoforge: 'NeoForge',
	neo_forge: 'NeoForge',
	forge: 'Forge',
	quilt: 'Quilt',
	paper: 'Paper',
	spigot: 'Spigot',
	purpur: 'Purpur',
	bukkit: 'Bukkit',
	vanilla: 'Vanilla',
}

export const loaderMessages: Record<string, { id: string; defaultMessage: string }> = {
	vanilla: {
		id: 'loader.vanilla',
		defaultMessage: 'None',
	},
}

/** Maps loader IDs to built-in instance icon IDs */
export const loaderIconMap: Record<string, string> = {
	vanilla: 'grass-block',
	fabric: 'fabric',
	forge: 'anvil',
	neoforge: 'neoforge',
	quilt: 'quilt',
}

export const formatLoaderLabel = (
	item: string,
	formatMessage?: (msg: { id: string; defaultMessage: string }) => string,
) => {
	if (formatMessage && loaderMessages[item]) {
		return formatMessage(loaderMessages[item])
	}
	return loaderDisplayNames[item] ?? item.charAt(0).toUpperCase() + item.slice(1)
}
