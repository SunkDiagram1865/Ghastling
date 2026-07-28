import type { Meta, StoryObj } from '@storybook/vue3-vite'
import { ref } from 'vue'

import ButtonStyled from '../../components/base/ButtonStyled.vue'
import type { CreationFlowContextValue } from '../../components/flows/creation-flow-modal/creation-flow-context'
import CreationFlowModal from '../../components/flows/creation-flow-modal/index.vue'
import { type PickedFile, provideFilePicker } from '../../providers'

const meta = {
	title: 'Servers/CreationFlowModal',
	component: CreationFlowModal,
	parameters: {
		layout: 'centered',
	},
} satisfies Meta<typeof CreationFlowModal>

export default meta
type Story = StoryObj<typeof meta>

// ============================================
// Create World (Hosting)
// ============================================

export const CreateWorld: Story = {
	name: 'Create World (Hosting)',
	render: () => ({
		components: { CreationFlowModal, ButtonStyled },
		setup() {
			const modalRef = ref<InstanceType<typeof CreationFlowModal> | null>(null)
			const lastEvent = ref('')
			const openModal = () => modalRef.value?.show()

			const onCreate = (config: CreationFlowContextValue) => {
				lastEvent.value = `create emitted — name: "${config.worldName.value}", mode: ${config.gamemode.value}`
			}
			return { modalRef, openModal, lastEvent, onCreate }
		},
		template: /*html*/ `
			<div class="flex flex-col gap-4 items-center">
				<ButtonStyled color="brand">
					<button @click="openModal">Create World</button>
				</ButtonStyled>
				<p v-if="lastEvent" class="text-sm text-secondary mt-2">Last event: {{ lastEvent }}</p>
				<CreationFlowModal
					ref="modalRef"
					type="world"
					:show-snapshot-toggle="true"
					@hide="() => {}"
					@browse-modpacks="() => console.log('browse-modpacks emitted')"
					@create="onCreate"
				/>
			</div>
		`,
	}),
}

// ============================================
// Server Setup (Legacy) (Hosting)
// ============================================

export const ServerOnboarding: Story = {
	name: 'Server Setup (Legacy) (Hosting)',
	render: () => ({
		components: { CreationFlowModal, ButtonStyled },
		setup() {
			const modalRef = ref<InstanceType<typeof CreationFlowModal> | null>(null)
			const lastEvent = ref('')
			const openModal = () => modalRef.value?.show()

			const onCreate = (config: CreationFlowContextValue) => {
				lastEvent.value = `create emitted — loader: ${config.selectedLoader.value}, version: ${config.selectedGameVersion.value}`
			}
			return { modalRef, openModal, lastEvent, onCreate }
		},
		template: /*html*/ `
			<div class="flex flex-col gap-4 items-center">
				<ButtonStyled color="brand">
					<button @click="openModal">Set Up Server</button>
				</ButtonStyled>
				<p v-if="lastEvent" class="text-sm text-secondary mt-2">Last event: {{ lastEvent }}</p>
				<CreationFlowModal
					ref="modalRef"
					type="server-onboarding"
					:show-snapshot-toggle="true"
					@hide="() => {}"
					@browse-modpacks="() => console.log('browse-modpacks emitted')"
					@create="onCreate"
				/>
			</div>
		`,
	}),
}

// ============================================
// Create Instance (App)
// ============================================

function createMockIcon(name: string): PickedFile {
	return {
		file: new File([''], `${name}.png`, { type: 'image/png' }),
		previewUrl:
			'data:image/svg+xml,%3Csvg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"%3E%3Crect width="64" height="64" rx="12" fill="%231bd96a"/%3E%3C/svg%3E',
	}
}

function renderInstanceStory(withDedicatedIconPicker: boolean) {
	return {
		components: { CreationFlowModal, ButtonStyled },
		setup() {
			provideFilePicker({
				pickImage: async () => createMockIcon('uploaded-icon'),
				...(withDedicatedIconPicker
					? { pickInstanceIcon: async () => createMockIcon('built-in-icon') }
					: {}),
				pickModpackFile: async () => null,
			})

			const modalRef = ref<InstanceType<typeof CreationFlowModal> | null>(null)
			const lastEvent = ref('')
			const openModal = () => modalRef.value?.show()

			const onCreate = (config: CreationFlowContextValue) => {
				lastEvent.value = `create emitted — loader: ${config.selectedLoader.value}, version: ${config.selectedGameVersion.value}`
			}
			return { modalRef, openModal, lastEvent, onCreate }
		},
		template: /*html*/ `
			<div class="flex flex-col gap-4 items-center">
				<ButtonStyled color="brand">
					<button @click="openModal">Create Instance</button>
				</ButtonStyled>
				<p v-if="lastEvent" class="text-sm text-secondary mt-2">Last event: {{ lastEvent }}</p>
				<CreationFlowModal
					ref="modalRef"
					type="instance"
					:show-snapshot-toggle="true"
					@hide="() => {}"
					@browse-modpacks="() => console.log('browse-modpacks emitted')"
					@create="onCreate"
				/>
			</div>
		`,
	}
}

export const Instance: Story = {
	name: 'Create Instance (App)',
	render: () => renderInstanceStory(true),
}

export const InstanceUploadFallback: Story = {
	name: 'Create Instance (Upload Fallback)',
	render: () => renderInstanceStory(false),
}
