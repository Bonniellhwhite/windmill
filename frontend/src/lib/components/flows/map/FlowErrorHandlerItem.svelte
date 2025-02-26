<script lang="ts">
	import type { FlowEditorContext } from '../types'
	import { getContext } from 'svelte'
	import Icon from 'svelte-awesome'
	import { faBug } from '@fortawesome/free-solid-svg-icons'
	import { classNames, emptySchema } from '$lib/utils'
	import type { FlowModuleState } from '../flowState'
	import Toggle from '$lib/components/Toggle.svelte'
	import { NEVER_TESTED_THIS_FAR } from '../models'
	import type { FlowCopilotContext } from '$lib/components/copilot/flow'
	import { fade } from 'svelte/transition'

	const { selectedId, flowStateStore, flowStore } =
		getContext<FlowEditorContext>('FlowEditorContext')

	function onToggle() {
		if ($flowStore?.value?.failure_module) {
			$flowStore.value.failure_module = undefined
			// By default, we return to settings when disabling the failure module
			$selectedId = 'settings-metadata'
		} else {
			const failureModule: FlowModuleState = {
				schema: emptySchema(),
				previewResult: NEVER_TESTED_THIS_FAR
			}

			$flowStore.value.failure_module = {
				id: 'failure',
				value: { type: 'identity' }
			}
			$flowStateStore['failure'] = failureModule

			$selectedId = 'failure'
			$flowStore = $flowStore
		}
	}

	const { currentStepStore: copilotCurrentStepStore } =
		getContext<FlowCopilotContext | undefined>('FlowCopilotContext') || {}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
	on:click={() => {
		if ($copilotCurrentStepStore !== undefined) return
		if ($flowStore?.value?.failure_module) {
			$selectedId = 'failure'
		} else {
			onToggle()
		}
	}}
	class={classNames(
		$copilotCurrentStepStore !== undefined ? 'border-gray-500/75' : 'cursor-pointer',
		'border transition-colors duration-[400ms] ease-linear mx-auto rounded-sm px-2 py-1 bg-surface text-sm flex justify-between items-center flex-row overflow-x-hidden relative',
		$selectedId?.includes('failure') ? 'outline outline-offset-1 outline-2 outline-slate-900' : ''
	)}
	style="min-width: 275px"
>
	{#if $copilotCurrentStepStore !== undefined}
		<div transition:fade class="absolute inset-0 bg-gray-500 bg-opacity-75 z-[900]" />
	{/if}
	<div class=" flex justify-between items-center flex-wrap">
		<div>
			<Icon data={faBug} class="mr-2" />
			<span class="font-bold text-xs">Error Handler</span>
		</div>
	</div>

	<div class=" items-center truncate flex text-xs">
		{#if Boolean($flowStore?.value?.failure_module)}
			<span>
				{$flowStore.value.failure_module?.summary ||
					($flowStore.value.failure_module?.value.type === 'rawscript'
						? `${$flowStore.value.failure_module?.value.language}`
						: 'TBD')}
			</span>
		{/if}
	</div>
	<div class="-my-1">
		<Toggle
			checked={Boolean($flowStore?.value?.failure_module)}
			on:change={onToggle}
			id="error-handler-toggle"
		/>
	</div>
</div>
