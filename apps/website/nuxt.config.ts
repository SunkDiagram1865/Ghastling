import svgLoader from 'vite-svg-loader'

const SITE_URL = 'https://sunkdiagram1865.github.io/Ghastling/'

export default defineNuxtConfig({
	srcDir: 'src/',
	app: {
		head: {
			htmlAttrs: {
				class: 'accent-pink dark-mode',
				lang: 'zh-CN',
			},
			title: 'Ghastling Launcher - 免费开源的 Minecraft 启动器',
			link: [
				{ rel: 'icon', type: 'image/png', href: '/ghastling.png' },
				{ rel: 'apple-touch-icon', type: 'image/png', href: '/ghastling.png' },
			],
		},
	},
	runtimeConfig: {
		public: {
			siteUrl: SITE_URL,
		},
	},
	vite: {
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
		prerender: {
			crawlLinks: false,
			routes: ['/', '/changelog'],
		},
	},
	routeRules: {
		'/': { static: true },
		'/changelog': { static: true },
	},
	typescript: {
		shim: false,
		strict: true,
		typeCheck: false,
	},
	compatibilityDate: '2025-01-01',
	telemetry: false,
})
