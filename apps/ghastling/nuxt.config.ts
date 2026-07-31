import svgLoader from 'vite-svg-loader'

const SITE_URL = 'https://sunkdiagram1865.github.io/Ghastling/'

export default defineNuxtConfig({
	srcDir: 'src/',
	ssr: false,
	app: {
		baseURL: '/Ghastling/',
		buildAssetsDir: '_nuxt',
		head: {
			htmlAttrs: {
				class: 'accent-pink dark-mode',
				lang: 'zh-CN',
			},
			title: 'Ghastling Launcher - 免费开源的 Minecraft 启动器',
			link: [
				{ rel: 'icon', type: 'image/png', href: '/Ghastling/ghastling.png' },
				{ rel: 'apple-touch-icon', type: 'image/png', href: '/Ghastling/ghastling.png' },
			],
		},
	},
	runtimeConfig: {
		public: {
			siteUrl: SITE_URL,
		},
	},
	vite: {
		base: '/Ghastling/',
		css: {
			preprocessorOptions: {
				scss: {
					silenceDeprecations: ['import'],
				},
			},
		},
		resolve: {
			dedupe: ['vue'],
		},
		plugins: [
			svgLoader({
				svgoConfig: {
					plugins: [
						{
							name: 'preset-default',
							params: {
								overrides: {
									removeViewBox: false,
									cleanupIds: { minify: false },
								},
							},
						},
					],
				},
			}),
		],
	},
	css: ['~/assets/styles/tailwind.css'],
	postcss: {
		plugins: {
			tailwindcss: {},
			autoprefixer: {},
		},
	},
	nitro: {
	},
	typescript: {
		shim: false,
		strict: true,
		typeCheck: false,
	},
	experimental: {
		appManifest: false,
	},
	compatibilityDate: '2025-01-01',
	telemetry: false,
})
