<script lang="ts">
	import { Job, JobService, type Flow, type FlowModule } from '$lib/gen'
	import { workspaceStore } from '$lib/stores'
	import { faClose, faPlay, faRefresh } from '@fortawesome/free-solid-svg-icons'
	import { Button, Drawer, Kbd } from './common'
	import { createEventDispatcher, getContext } from 'svelte'
	import Icon from 'svelte-awesome'
	import type { FlowEditorContext } from './flows/types'
	import { runFlowPreview } from './flows/utils'
	import SchemaForm from './SchemaForm.svelte'
	import FlowStatusViewer from '../components/FlowStatusViewer.svelte'
	import FlowProgressBar from './flows/FlowProgressBar.svelte'
	import CapturePayload from './flows/content/CapturePayload.svelte'
	import { Loader2 } from 'lucide-svelte'
	import { getModifierKey } from '$lib/utils'
	import DrawerContent from './common/drawer/DrawerContent.svelte'
	import SavedInputs from './SavedInputs.svelte'
	import { dfs } from './flows/dfs'

	let capturePayload: CapturePayload
	export let previewMode: 'upTo' | 'whole'
	export let open: boolean

	export let jobId: string | undefined = undefined
	export let job: Job | undefined = undefined
	let isRunning: boolean = false
	let jobProgressReset: () => void

	const { selectedId, previewArgs, flowStateStore, flowStore, initialPath } =
		getContext<FlowEditorContext>('FlowEditorContext')
	const dispatch = createEventDispatcher()

	function sliceModules(modules: FlowModule[], upTo: number, idOrders: string[]): FlowModule[] {
		return modules
			.filter((x) => idOrders.indexOf(x.id) <= upTo)
			.map((m) => {
				if (idOrders.indexOf(m.id) == upTo) {
					return m
				}
				if (m.value.type === 'forloopflow') {
					m.value.modules = sliceModules(m.value.modules, upTo, idOrders)
				} else if (m.value.type === 'branchone') {
					m.value.branches = m.value.branches.map((b) => {
						b.modules = sliceModules(b.modules, upTo, idOrders)
						return b
					})
					m.value.default = sliceModules(m.value.default, upTo, idOrders)
				} else if (m.value.type === 'branchall') {
					m.value.branches = m.value.branches.map((b) => {
						b.modules = sliceModules(b.modules, upTo, idOrders)
						return b
					})
				}
				return m
			})
	}

	function extractFlow(previewMode: 'upTo' | 'whole'): Flow {
		if (previewMode === 'whole') {
			return $flowStore
		} else {
			const flow: Flow = JSON.parse(JSON.stringify($flowStore))
			const idOrders = dfs(flow.value.modules, (x) => x.id)
			let upToIndex = idOrders.indexOf($selectedId)

			if (upToIndex != -1) {
				flow.value.modules = sliceModules(flow.value.modules, upToIndex, idOrders)
			}
			return flow
		}
	}

	export async function runPreview(args: Record<string, any>) {
		jobProgressReset()
		const newFlow = extractFlow(previewMode)
		jobId = await runFlowPreview(args, newFlow)
		isRunning = true
	}

	function onKeyDown(event: KeyboardEvent) {
		if (open) {
			switch (event.key) {
				case 'Enter':
					if (event.ctrlKey || event.metaKey) {
						event.preventDefault()
						runPreview($previewArgs)
					}
					break
			}
		}
	}

	$: if (job?.type === 'CompletedJob') {
		isRunning = false
	}

	let inputLibraryDrawer: Drawer
</script>

<CapturePayload bind:this={capturePayload} />

<svelte:window on:keydown={onKeyDown} />

<Drawer bind:this={inputLibraryDrawer}>
	<DrawerContent title="Input library {initialPath}" on:close={inputLibraryDrawer?.toggleDrawer}>
		<SavedInputs
			flowPath={initialPath}
			isValid={true}
			args={$previewArgs}
			on:selected_args={(e) => {
				$previewArgs = JSON.parse(JSON.stringify(e.detail))
				inputLibraryDrawer?.closeDrawer()
			}}
		/>
	</DrawerContent>
</Drawer>

<div
	class="flex divide-y flex-col space-y-2 h-screen bg-surface px-6 py-2 w-full"
	id="flow-preview-content"
>
	<div class="flex flex-row justify-between w-full items-center gap-x-2">
		<div class="w-8">
			<button
				on:click={() => dispatch('close')}
				class="hover:bg-surface-hover bg-surface-secondary rounded-full w-8 h-8 flex items-center justify-center transition-all"
			>
				<Icon data={faClose} class="text-secondary" />
			</button>
		</div>

		{#if isRunning}
			<Button
				color="red"
				on:click={async () => {
					isRunning = false
					try {
						jobId &&
							(await JobService.cancelQueuedJob({
								workspace: $workspaceStore ?? '',
								id: jobId,
								requestBody: {}
							}))
					} catch {}
				}}
				size="sm"
				btnClasses="w-full max-w-lg"
			>
				<Loader2 size={18} class="animate-spin mr-2" />
				Cancel
			</Button>
		{:else}
			<Button
				variant="contained"
				startIcon={{ icon: isRunning ? faRefresh : faPlay }}
				color="dark"
				size="sm"
				btnClasses="w-full max-w-lg"
				on:click={() => runPreview($previewArgs)}
				id="flow-editor-test-flow-drawer"
			>
				Test flow &nbsp;<Kbd small isModifier>{getModifierKey()}</Kbd>
				<Kbd small><span class="text-lg font-bold">⏎</span></Kbd>
			</Button>
		{/if}
		<div class="flex gap-2">
			{#if initialPath != ''}
				<Button
					btnClasses="h-full truncate"
					size="sm"
					variant="border"
					on:click={() => {
						inputLibraryDrawer?.openDrawer()
					}}>Past Runs/Input library</Button
				>
			{/if}

			<Button
				btnClasses="h-full truncate"
				size="sm"
				variant="border"
				on:click={() => {
					capturePayload.openDrawer()
				}}>Fill args from a request</Button
			>
		</div>
	</div>
	<FlowProgressBar {job} bind:reset={jobProgressReset} />

	<div class="overflow-y-auto grow divide-y divide-gray-600 pr-4">
		<div class="max-h-1/2 overflow-auto border-b border-gray-700">
			<SchemaForm
				noVariablePicker
				compact
				class="py-4 max-w-3xl"
				schema={$flowStore.schema}
				bind:args={$previewArgs}
			/>
		</div>
		<div class="pt-4 grow">
			{#if jobId}
				<FlowStatusViewer
					{flowStateStore}
					{jobId}
					on:jobsLoaded={({ detail }) => {
						job = detail
					}}
				/>
			{:else}
				<div class="italic text-tertiary h-full grow"> Flow status will be displayed here </div>
			{/if}
		</div>
	</div>
</div>
