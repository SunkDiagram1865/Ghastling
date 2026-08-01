<script setup lang="ts">
import IssuesIcon from '@modrinth/assets/icons/issues.svg?component'
import { defineMessages, useVIntl } from '@modrinth/ui/src/composables/i18n.ts'

const { formatMessage } = useVIntl()

const messages = defineMessages({
	title: {
		id: 'axolotl-site.update-rescue.title',
		defaultMessage: 'Unable to check for updates through CNB on v1.2.2?',
	},
	description: {
		id: 'axolotl-site.update-rescue.description',
		defaultMessage:
			'Open Settings → Updates, switch the channel to GitHub, and check again manually. After upgrading to v1.2.3, you can continue using the CNB channel normally.',
	},
	action: {
		id: 'axolotl-site.update-rescue.action',
		defaultMessage: 'View the latest release',
	},
})
</script>

<template>
	<aside class="update-rescue-banner" role="alert">
		<div class="banner-inner">
			<IssuesIcon aria-hidden="true" />
			<div class="banner-copy">
				<strong>{{ formatMessage(messages.title) }}</strong>
				<span>{{ formatMessage(messages.description) }}</span>
			</div>
			<a
				href="https://github.com/Mystic-Stars/Axolotl/releases/latest"
				target="_blank"
				rel="noopener"
			>
				{{ formatMessage(messages.action) }}
			</a>
		</div>
	</aside>
</template>

<style scoped lang="scss">
.update-rescue-banner {
	position: relative;
	z-index: 50;
	border-bottom: 1px solid var(--banner-error-border);
	background: var(--banner-error-bg);
	color: var(--banner-error-text);
}

.banner-inner {
	display: flex;
	align-items: center;
	gap: 0.625rem;
	width: 100%;
	max-width: 1600px;
	margin: 0 auto;
	padding: 0.625rem 1rem;

	> svg {
		width: 1.25rem;
		height: 1.25rem;
		flex-shrink: 0;
	}

	a {
		flex-shrink: 0;
		margin-left: auto;
		padding: 0.45rem 0.7rem;
		border: 1px solid currentColor;
		border-radius: var(--radius-md);
		color: inherit;
		font-size: 0.8125rem;
		font-weight: 700;
		text-decoration: none;
		transition: background-color 120ms ease;

		&:hover {
			background: color-mix(in srgb, currentColor 10%, transparent);
		}
	}
}

.banner-copy {
	display: flex;
	align-items: baseline;
	flex: 1;
	gap: 0.5rem;
	min-width: 0;
	font-size: clamp(0.75rem, 0.9vw, 0.875rem);
	line-height: 1.4;

	strong {
		flex-shrink: 0;
	}
}

@media (min-width: 1280px) {
	.banner-copy {
		white-space: nowrap;
	}
}

@media (max-width: 800px) {
	.banner-inner {
		align-items: flex-start;
		padding: 0.75rem 1rem;
	}

	.banner-copy {
		align-items: flex-start;
		flex-direction: column;
		gap: 0.15rem;
	}
}

@media (max-width: 560px) {
	.banner-inner {
		flex-wrap: wrap;

		a {
			margin-left: 2.125rem;
		}
	}
}
</style>
