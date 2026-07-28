<template>
	<span class="wordmark">
		<img src="/axolotl.png" alt="" />
		<span>Axolotl</span>
	</span>
</template>

<style scoped lang="scss">
.wordmark {
	display: inline-flex;
	align-items: center;
	gap: 0.625rem;
	color: var(--color-contrast);
	font-size: 1.375rem;
	font-weight: 800;
	letter-spacing: -0.035em;
	line-height: 1;

	img {
		width: 2rem;
		height: 2rem;
		object-fit: contain;
		filter: drop-shadow(0 0.35rem 0.7rem rgb(245 139 174 / 22%));
	}
}
</style>
