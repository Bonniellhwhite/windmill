<script lang="ts">
	import { push } from '$lib/history'
	import { classNames } from '$lib/utils'
	import { createEventDispatcher, getContext, onDestroy } from 'svelte'
	import { twMerge } from 'tailwind-merge'
	import { columnConfiguration, isFixed, toggleFixed } from '../gridUtils'
	import Grid from '../svelte-grid/Grid.svelte'
	import type { AppEditorContext, AppViewerContext, GridItem } from '../types'
	import { expandGriditem, findGridItem, selectId } from './appUtils'
	import Component from './component/Component.svelte'
	import ComponentWrapper from './component/ComponentWrapper.svelte'
	import GridViewer from './GridViewer.svelte'

	export let containerHeight: number | undefined = undefined
	export let containerWidth: number | undefined = undefined
	let classes = ''
	export { classes as class }
	export let style = ''
	export let noPadding = false
	export let noYPadding = false
	export let subGridId: string
	export let visible: boolean = true
	export let id: string
	export let shouldHighlight: boolean = true

	const dispatch = createEventDispatcher()

	const {
		app,
		connectingInput,
		selectedComponent,
		focusedGrid,
		mode,
		parentWidth,
		breakpoint,
		allIdsInPath
	} = getContext<AppViewerContext>('AppViewerContext')

	const editorContext = getContext<AppEditorContext>('AppEditorContext')

	let isActive = false
	let sber = editorContext?.componentActive?.subscribe((x) => (isActive = x))

	onDestroy(() => {
		sber?.()
	})
	$: highlight = id === $focusedGrid?.parentComponentId && shouldHighlight

	const onpointerdown = (e) => {
		dispatch('focus')
	}

	function selectComponent(e: PointerEvent, id: string) {
		if (!$connectingInput.opened) {
			selectId(e, id, selectedComponent, $app)
		}
	}

	function lock(dataItem: GridItem) {
		let fComponent = findGridItem($app, dataItem.id)
		if (fComponent) {
			fComponent = toggleFixed(fComponent)
		}
		$app = $app
	}

	let container: HTMLElement | undefined = undefined
</script>

<div
	class="translate-x-0 translate-y-0 w-full subgrid {visible
		? 'visible'
		: 'invisible h-0 overflow-hidden'}"
	bind:this={container}
	on:pointerdown={onpointerdown}
>
	<div
		class={twMerge(
			$allIdsInPath.includes(id) ? 'overflow-visible' : 'overflow-auto',
			noYPadding ? '' : 'py-2',
			classes ?? '',
			noPadding ? 'px-0' : 'px-2'
		)}
		style="{containerHeight ? `height: ${containerHeight - 2}px;` : ''} {style ?? ''}"
	>
		{#if $mode !== 'preview'}
			<div
				class={highlight
					? `animate-border border-dashed border-2 min-h-full ${
							isActive && !$selectedComponent?.includes(id)
								? 'border-orange-600'
								: 'border-gray-600'
					  }`
					: ''}
			>
				<Grid
					allIdsInPath={$allIdsInPath}
					items={$app.subgrids?.[subGridId] ?? []}
					on:redraw={(e) => {
						push(editorContext?.history, $app)
						if ($app.subgrids) {
							$app.subgrids[subGridId] = e.detail
						}
					}}
					selectedIds={$selectedComponent}
					let:dataItem
					rowHeight={36}
					cols={columnConfiguration}
					fastStart={true}
					gap={[4, 2]}
					scroller={container}
					parentWidth={$parentWidth - 17}
					{containerWidth}
				>
					<ComponentWrapper
						id={dataItem.id}
						type={dataItem.data.type}
						class={classNames(
							'h-full w-full center-center',
							$selectedComponent?.includes(dataItem.id) ? 'active-grid-item' : '',
							'top-0'
						)}
					>
						<Component
							render={visible}
							component={dataItem.data}
							selected={Boolean($selectedComponent?.includes(dataItem.id))}
							locked={isFixed(dataItem)}
							on:lock={() => lock(dataItem)}
							on:expand={() => {
								const parentGridItem = findGridItem($app, id)

								if (!parentGridItem) {
									return
								}

								$selectedComponent = [dataItem.id]
								push(editorContext?.history, $app)

								expandGriditem(
									$app.subgrids?.[subGridId] ?? [],
									dataItem.id,
									$breakpoint,
									parentGridItem
								)
								$app = $app
							}}
						/>
					</ComponentWrapper>
				</Grid>
			</div>
		{:else}
			<GridViewer
				allIdsInPath={$allIdsInPath}
				items={$app.subgrids?.[subGridId] ?? []}
				let:dataItem
				rowHeight={36}
				cols={columnConfiguration}
				gap={[4, 2]}
				parentWidth={$parentWidth - 17}
				{containerWidth}
			>
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<div
					on:pointerdown|stopPropagation={(e) => selectComponent(e, dataItem.id)}
					class={classNames('h-full w-full center-center', 'top-0')}
				>
					<Component
						render={visible}
						component={dataItem.data}
						selected={Boolean($selectedComponent?.includes(dataItem.id))}
						locked={isFixed(dataItem)}
					/>
				</div>
			</GridViewer>
		{/if}
	</div>
</div>
