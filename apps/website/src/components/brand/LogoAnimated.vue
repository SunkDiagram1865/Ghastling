<template>
	<div class="axolotl-logo">
		<img src="/axolotl.png" alt="" />
	</div>
</template>

<style lang="scss" scoped>
.axolotl-logo {
	display: flex;
	align-items: center;
	justify-content: center;
	height: 5rem;
	margin-top: 1rem;

	img {
		width: 5rem;
		height: 5rem;
		object-fit: contain;
		filter: drop-shadow(0 0.75rem 1.5rem rgb(245 139 174 / 28%));
		animation: float 3.5s ease-in-out infinite;
	}
}

@keyframes float {
	0%,
	100% {
		transform: translateY(0) rotate(-2deg);
	}
	50% {
		transform: translateY(-0.4rem) rotate(2deg);
	}
}
</style>
