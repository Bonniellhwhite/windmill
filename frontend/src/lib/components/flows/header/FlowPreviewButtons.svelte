<script lang="ts">
	import { Badge } from '$lib/components/common'
	import Button from '$lib/components/common/button/Button.svelte'

	import Drawer from '$lib/components/common/drawer/Drawer.svelte'
	import FlowPreviewContent from '$lib/components/FlowPreviewContent.svelte'
	import type { Job } from '$lib/gen'
	import { faPlay } from '@fortawesome/free-solid-svg-icons'

	import { getContext } from 'svelte'
	import type { FlowEditorContext } from '../types'
	const { selectedId } = getContext<FlowEditorContext>('FlowEditorContext')
	let previewOpen = false
	let previewMode: 'upTo' | 'whole' = 'whole'

	let jobId: string | undefined = undefined
	let job: Job | undefined = undefined

	$: upToDisabled =
		$selectedId == undefined ||
		[
			'settings',
			'settings-metadata',
			'settings-schedule',
			'settings-retries',
			'settings-same-worker',
			'settings-graph',
			'settings-worker-group',
			'settings-cache',
			'settings-concurrency',
			'inputs',
			'schedules',
			'failure',
			'constants'
		].includes($selectedId) ||
		$selectedId?.includes('branch')
</script>

{#if !upToDisabled}
	<Button
		size="xs"
		disabled={upToDisabled}
		color="light"
		variant="border"
		on:click={() => {
			previewMode = 'upTo'
			previewOpen = !previewOpen
		}}
		startIcon={{ icon: faPlay }}
	>
		Test up to
		<Badge baseClass="ml-1" color="indigo">
			{$selectedId}
		</Badge>
	</Button>
{/if}

<Button
	color="dark"
	size="xs"
	on:click={() => {
		previewMode = 'whole'
		previewOpen = !previewOpen
	}}
	startIcon={{ icon: faPlay }}
	id="flow-editor-test-flow"
>
	Test flow
</Button>

<Drawer bind:open={previewOpen} alwaysOpen size="75%">
	<FlowPreviewContent
		open={previewOpen}
		bind:previewMode
		bind:job
		bind:jobId
		on:close={() => {
			previewOpen = false
		}}
	/>
</Drawer>
