<script setup lang="ts">
import { defineMessages, useVIntl } from '@modrinth/ui/src/composables/i18n.ts'

import AxolotlWordmark from '~/components/brand/AxolotlWordmark.vue'

const { formatMessage } = useVIntl()

const messages = defineMessages({
	home: { id: 'axolotl-site.navigation.home', defaultMessage: 'Ghastling Launcher 首页' },
	description: {
			id: 'axolotl-site.footer.description',
		defaultMessage: '一款适用于 Windows x64 的自由开源 Minecraft 启动器。',
	},
	qqGroup: {
			id: 'axolotl-site.footer.qq-group',
		defaultMessage: '启动器官方 QQ 群',
	},
	openSourcePrefix: {
			id: 'axolotl-site.footer.open-source-prefix',
		defaultMessage: 'Ghastling 是',
	},
	openSourceLink: {
			id: 'axolotl-site.footer.open-source-link',
		defaultMessage: '开源软件',
	},
	disclaimer: {
		id: 'axolotl-site.footer.minecraft-disclaimer',
		defaultMessage:
			'非 Minecraft 官方服务。未经 Mojang 或 Microsoft 批准，亦与其无关联。',
	},
})
</script>

<template>
	<footer class="site-footer">
		<div class="footer-shell">
			<div class="footer-notice">
				Ghastling 是基于
				<a
					href="https://github.com/SunkDiagram1865/Ghastling/blob/main/COPYING.md"
					target="_blank"
					rel="noopener"
				>
					GPL-3.0
				</a>
				发布的自由软件。安装前请阅读
				<a
					href="https://github.com/SunkDiagram1865/Ghastling#readme"
					target="_blank"
					rel="noopener"
				>
					项目文档
				</a>
				。
			</div>

			<div class="footer-main">
				<div class="footer-brand">
					<NuxtLink to="/" :aria-label="formatMessage(messages.home)" class="button-animation">
						<AxolotlWordmark />
					</NuxtLink>
					<p>{{ formatMessage(messages.description) }}</p>
				</div>

				<a
					class="qq-group"
					href="https://qm.qq.com/q/L1aiQTdReY"
					target="_blank"
					rel="noopener"
				>
					<span>{{ formatMessage(messages.qqGroup) }}</span>
					<strong>208375315</strong>
				</a>
			</div>

			<div class="footer-bottom">
				<p>© {{ new Date().getFullYear() }} SunkDiagram1865</p>
				<p class="open-source">
					{{ formatMessage(messages.openSourcePrefix) }}
					<a href="https://github.com/SunkDiagram1865/Ghastling" target="_blank" rel="noopener">
						{{ formatMessage(messages.openSourceLink) }} </a
					>.
				</p>
				<p class="disclaimer">{{ formatMessage(messages.disclaimer) }}</p>
			</div>
		</div>
	</footer>
</template>

<style scoped lang="scss">
.site-footer {
	position: relative;
	border-top: 1px solid var(--color-divider);
	background: color-mix(in srgb, var(--surface-1) 96%, var(--color-brand));
	color: var(--color-base);

	&::before {
		position: absolute;
		top: -1px;
		left: 50%;
		width: min(28rem, 60vw);
		height: 1px;
		transform: translateX(-50%);
		background: linear-gradient(90deg, transparent, var(--color-brand), transparent);
		content: '';
		opacity: 0.6;
	}
}

.footer-shell {
	width: min(80rem, calc(100% - 3rem));
	margin: 0 auto;
	padding: 3.25rem 0 1.5rem;
}

.footer-notice {
	text-align: center;
	font-size: 24px;
	color: var(--color-contrast);
	margin-bottom: 2.5rem;

	a {
		color: var(--color-brand);
		text-decoration: underline;
		text-underline-offset: 0.15rem;
	}
}

.footer-main {
	display: grid;
	align-items: center;
	grid-template-columns: minmax(0, 1fr) auto;
	gap: 3rem;
}

.footer-brand {
	display: flex;
	align-items: flex-start;
	flex-direction: column;
	gap: 0.7rem;

	p {
		max-width: 30rem;
		margin: 0;
		color: var(--color-secondary);
		font-size: 0.875rem;
		line-height: 1.6;
	}
}

.qq-group {
	display: flex;
	align-items: center;
	gap: 0.5rem;
	padding: 0.55rem 1rem;
	border: 1px solid color-mix(in srgb, var(--color-brand) 35%, var(--color-divider));
	border-radius: 0.9rem;
	background: color-mix(in srgb, var(--color-brand) 8%, var(--surface-3));
	font-size: 0.8125rem;
	text-decoration: none;
	transition:
		background 120ms ease,
		color 120ms ease,
		box-shadow 120ms ease;

	&:hover {
		background: var(--surface-4);
		color: var(--color-contrast);
		box-shadow: 0 0.5rem 1.5rem color-mix(in srgb, var(--color-brand) 16%, transparent);
	}

	span {
		color: var(--color-secondary);
	}

	strong {
		color: var(--color-contrast);
		font-variant-numeric: tabular-nums;
		user-select: all;
	}
}

.footer-bottom {
	display: grid;
	align-items: center;
	grid-template-columns: minmax(0, 1fr) auto;
	gap: 0.55rem 2rem;
	margin-top: 2.5rem;
	padding-top: 1.25rem;
	border-top: 1px solid var(--color-divider);
	color: var(--color-secondary);
	font-size: 0.72rem;

	p {
		margin: 0;
	}

	a {
		color: var(--color-base);
		text-underline-offset: 0.15rem;
	}
}

.open-source {
	text-align: right;
}

.disclaimer {
	grid-column: 1 / -1;
	font-size: 0.64rem;
	letter-spacing: 0.015em;
	opacity: 0.45;
}

@media (max-width: 700px) {
	.footer-shell {
		width: calc(100% - 2rem);
		padding-top: 2rem;
	}

	.footer-notice {
		font-size: 18px;
		margin-bottom: 2rem;
	}

	.footer-main {
		align-items: flex-start;
		grid-template-columns: 1fr;
		gap: 2rem;
	}

	.footer-bottom {
		grid-template-columns: 1fr;
		margin-top: 2rem;
		gap: 0.55rem;
	}

	.open-source,
	.disclaimer {
		grid-column: 1;
		text-align: left;
	}
}
</style>
