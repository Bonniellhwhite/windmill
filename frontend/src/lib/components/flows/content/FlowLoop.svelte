<script lang="ts">
	import { getContext } from 'svelte'
	import FlowCard from '../common/FlowCard.svelte'
	import type { FlowEditorContext } from '../types'
	import Toggle from '$lib/components/Toggle.svelte'
	import SimpleEditor from '$lib/components/SimpleEditor.svelte'
	import Tooltip from '$lib/components/Tooltip.svelte'
	import PropPickerWrapper from '../propPicker/PropPickerWrapper.svelte'
	import FlowModuleEarlyStop from './FlowModuleEarlyStop.svelte'
	import FlowModuleSuspend from './FlowModuleSuspend.svelte'
	// import FlowRetries from './FlowRetries.svelte'
	import { Button, Tab, TabContent, Tabs } from '$lib/components/common'
	import type { FlowModule } from '$lib/gen/models/FlowModule'
	import { Pane, Splitpanes } from 'svelte-splitpanes'
	import { getStepPropPicker } from '../previousResults'

	import FlowModuleSleep from './FlowModuleSleep.svelte'
	import FlowModuleMock from './FlowModuleMock.svelte'

	const { previewArgs, flowStateStore, flowStore } =
		getContext<FlowEditorContext>('FlowEditorContext')

	export let mod: FlowModule
	export let parentModule: FlowModule | undefined
	export let previousModule: FlowModule | undefined

	let editor: SimpleEditor | undefined = undefined
	let selected: string = 'early-stop'

	$: stepPropPicker = getStepPropPicker(
		$flowStateStore,
		parentModule,
		previousModule,
		mod.id,
		$flowStore,
		$previewArgs,
		false
	)
</script>

<div class="h-full flex flex-col">
	<FlowCard title="For loop">
		<div slot="header" class="grow">
			<input bind:value={mod.summary} placeholder={'Summary'} />
		</div>
		<Splitpanes horizontal class="!max-h-[calc(100%-48px)]">
			<Pane size={60} minSize={20} class="p-4">
				{#if mod.value.type === 'forloopflow'}
					<div class="flex flex-row gap-8 mt-2 mb-6">
						<div>
							<div class="mb-2 text-sm font-bold"
								>Skip failures <Tooltip
									>If disabled, the flow will fail as soon as one of the iteration fail. Otherwise,
									the error will be collected as the result of the iteration. Regardless of this
									setting, if an error handler is defined, it will process the error.</Tooltip
								></div
							>
							<Toggle
								bind:checked={mod.value.skip_failures}
								options={{
									right: 'Skip failures'
								}}
							/>
						</div>
						<div>
							<div class="mb-2 text-sm font-bold">Run in parallel</div>
							<Toggle
								bind:checked={mod.value.parallel}
								options={{
									right: 'All iterations run in parallel'
								}}
							/>
						</div>
						<div>
							<div class="mb-2 text-sm font-bold">Parallelism</div>
							<input
								type="number"
								disabled={!mod.value.parallel}
								bind:value={mod.value.parallelism}
							/>
						</div>
					</div>
					<div class="my-2 text-sm font-bold">
						Iterator expression
						<Tooltip>
							List to iterate over. For more information see the
							<a href="https://www.windmill.dev/docs/flows/flow_loops">docs.</a>
						</Tooltip>
					</div>
					{#if mod.value.iterator.type == 'javascript'}
						<div class="border w-full" id="flow-editor-iterator-expression">
							<PropPickerWrapper
								notSelectable
								pickableProperties={stepPropPicker.pickableProperties}
								on:select={({ detail }) => {
									editor?.insertAtCursor(detail)
									editor?.focus()
								}}
							>
								<SimpleEditor
									bind:this={editor}
									lang="javascript"
									bind:code={mod.value.iterator.expr}
									class="small-editor"
									shouldBindKey={false}
									extraLib={stepPropPicker.extraLib}
								/>
							</PropPickerWrapper>
						</div>
					{:else}
						<Button
							on:click={() => {
								if (mod.value.type === 'forloopflow') mod.value.iterator.type = 'javascript'
							}}
						/>
					{/if}
				{/if}
			</Pane>
			<Pane size={40} minSize={20} class="flex flex-col flex-1">
				<Tabs bind:selected>
					<!-- <Tab value="retries">Retries</Tab> -->
					<Tab value="early-stop">Early Stop/Break</Tab>
					<Tab value="suspend">Suspend/Approval</Tab>
					<Tab value="sleep">Sleep</Tab>
					<Tab value="mock">Mock</Tab>

					<svelte:fragment slot="content">
						<div class="overflow-hidden bg-surface" style="height:calc(100% - 32px);">
							<!-- <TabContent value="retries" class="flex flex-col flex-1 h-full">
								<div class="p-4 overflow-y-auto">
									<FlowRetries bind:flowModule={mod} />
								</div>
							</TabContent> -->

							<TabContent value="early-stop" class="flex flex-col flex-1 h-full">
								<div class="p-4 overflow-y-auto">
									<FlowModuleEarlyStop bind:flowModule={mod} />
								</div>
							</TabContent>

							<TabContent value="suspend" class="flex flex-col flex-1 h-full">
								<div class="p-4 overflow-y-auto">
									<FlowModuleSuspend previousModuleId={previousModule?.id} bind:flowModule={mod} />
								</div>
							</TabContent>
							<TabContent value="sleep" class="flex flex-col flex-1 h-full">
								<div class="p-4 overflow-y-auto">
									<FlowModuleSleep previousModuleId={previousModule?.id} bind:flowModule={mod} />
								</div>
							</TabContent>
							<TabContent value="mock" class="flex flex-col flex-1 h-full">
								<div class="p-4 overflow-y-auto">
									<FlowModuleMock bind:flowModule={mod} />
								</div>
							</TabContent>
						</div>
					</svelte:fragment>
				</Tabs>
			</Pane>
		</Splitpanes>
	</FlowCard>
</div>
