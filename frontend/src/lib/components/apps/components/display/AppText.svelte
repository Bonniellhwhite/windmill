<script lang="ts">
	import Tooltip from '$lib/components/Tooltip.svelte'
	import { Clipboard } from 'lucide-svelte'
	import { getContext } from 'svelte'
	import { twMerge } from 'tailwind-merge'
	import { copyToClipboard, isCodeInjection } from '../../../../utils'
	import Button from '../../../common/button/Button.svelte'
	import { initConfig, initOutput } from '../../editor/appUtils'
	import { components } from '../../editor/component'
	import type { AppInput } from '../../inputType'
	import type { AppViewerContext, ComponentCustomCSS, RichConfigurations } from '../../types'
	import AlignWrapper from '../helpers/AlignWrapper.svelte'
	import ResolveConfig from '../helpers/ResolveConfig.svelte'
	import RunnableWrapper from '../helpers/RunnableWrapper.svelte'
	import { initCss } from '../../utils'
	import ResolveStyle from '../helpers/ResolveStyle.svelte'

	export let id: string
	export let componentInput: AppInput | undefined
	export let horizontalAlignment: 'left' | 'center' | 'right' | undefined = 'left'
	export let verticalAlignment: 'top' | 'center' | 'bottom' | undefined = undefined
	export let configuration: RichConfigurations
	export let initializing: boolean | undefined = undefined
	export let customCss: ComponentCustomCSS<'textcomponent'> | undefined = undefined
	export let render: boolean
	export let editorMode: boolean = false

	let resolvedConfig = initConfig(
		components['textcomponent'].initialData.configuration,
		configuration
	)

	const { app, worldStore, mode, componentControl } =
		getContext<AppViewerContext>('AppViewerContext')

	let css = initCss($app.css?.textcomponent, customCss)

	let result: string | undefined = undefined

	if (
		componentInput?.type == 'template' ||
		(componentInput?.type == 'templatev2' && !isCodeInjection(componentInput.eval))
	) {
		result = componentInput.eval
		initializing = false
	}

	$componentControl[id] = {
		...$componentControl[id],
		setValue(value: string) {
			result = value
		}
	}

	const outputs = initOutput($worldStore, id, {
		result,
		loading: initializing
	})

	function getComponent() {
		switch (resolvedConfig.style) {
			case 'Title':
				return 'h1'
			case 'Subtitle':
				return 'h3'
			case 'Body':
				return 'p'
			case 'Caption':
				return 'p'
			case 'Label':
				return 'label'
			default:
				return 'p'
		}
	}

	function getClasses() {
		switch (resolvedConfig.style) {
			case 'Caption':
				return 'text-sm italic text-tertiary'
			case 'Label':
				return 'font-semibold text-sm'
			default:
				return ''
		}
	}

	function getClassesByType() {
		switch (resolvedConfig.style) {
			case 'Title':
				return 'h1-textarea'
			case 'Subtitle':
				return 'h3-textarea'
			case 'Body':
				return 'p-textarea'
			case 'Caption':
				return 'p-textarea'
			case 'Label':
				return 'label'
			default:
				return 'p-textarea'
		}
	}

	let component = 'p'
	let classes = ''

	function getHorizontalAlignement() {
		if (horizontalAlignment) {
			switch (horizontalAlignment) {
				case 'left':
					return '!text-left'
				case 'center':
					return '!text-center'
				case 'right':
					return '!text-right'
				default:
					return '!text-left'
			}
		}
	}

	$: resolvedConfig.style && (component = getComponent())
	$: resolvedConfig.style && (classes = getClasses())
	$: initialValue =
		componentInput?.type == 'template' || componentInput?.type == 'templatev2'
			? componentInput.eval
			: ''
	$: editableValue = initialValue ?? ''

	let rows = 1

	function onInput(e: Event) {
		const target = e.target as HTMLTextAreaElement

		if (target.value != undefined) {
			$componentControl[id]?.setCode?.(target.value)
			autosize()
		}
	}

	function autosize() {
		const el = document.getElementById(`text-${id}`)
		setTimeout(() => {
			if (el !== null) {
				el.style.cssText = 'height:auto; padding:0'
				el.style.cssText = 'height:' + el.scrollHeight + 'px'
			}
		}, 0)
	}
</script>

{#each Object.keys(components['textcomponent'].initialData.configuration) as key (key)}
	<ResolveConfig
		{id}
		{key}
		bind:resolvedConfig={resolvedConfig[key]}
		configuration={configuration[key]}
	/>
{/each}

{#each Object.keys(css ?? {}) as key (key)}
	<ResolveStyle
		{id}
		{customCss}
		{key}
		bind:css={css[key]}
		componentStyle={$app.css?.textcomponent}
	/>
{/each}

<RunnableWrapper {outputs} {render} {componentInput} {id} bind:initializing bind:result>
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div
		class={twMerge(
			'h-full w-full overflow-hidden',
			customCss?.container?.class,
			'wm-text-container'
		)}
		style={customCss?.container?.style}
		on:dblclick={() => {
			if (!editorMode) {
				editorMode = true
				document.getElementById(`text-${id}`)?.focus()
				autosize()
			}
		}}
		on:keydown|stopPropagation
	>
		{#if $mode == 'dnd' && editorMode && (componentInput?.type == 'template' || componentInput?.type == 'templatev2')}
			<AlignWrapper {horizontalAlignment} {verticalAlignment}>
				<textarea
					class={twMerge(
						'whitespace-pre-wrap !outline-none !border-0 !bg-transparent !resize-none !overflow-hidden !ring-0 !p-0 text-center',
						css?.text?.class,
						customCss?.text?.class,
						'wm-text',
						classes,
						getClasses(),
						getClassesByType(),
						getHorizontalAlignement()
					)}
					on:pointerdown|stopPropagation
					style={customCss?.text?.style}
					id={`text-${id}`}
					on:pointerenter={() => {
						const elem = document.getElementById(`text-${id}`)
						if (elem) {
							elem.focus()
						}
					}}
					{rows}
					on:input={onInput}
					value={editableValue}
				/>
			</AlignWrapper>
		{:else}
			<AlignWrapper {horizontalAlignment} {verticalAlignment}>
				{#if !result || result === ''}
					<div
						class="text-ternary bg-surface-primary flex justify-center items-center h-full w-full"
					>
						No text
					</div>
				{:else}
					<!-- svelte-ignore a11y-click-events-have-key-events -->
					<div
						class="flex flex-wrap gap-2 pb-0.5 {$mode === 'dnd' &&
						(componentInput?.type == 'template' || componentInput?.type == 'templatev2')
							? 'cursor-text'
							: ''}"
					>
						<svelte:element
							this={component}
							class={twMerge(
								'whitespace-pre-wrap',
								$app.css?.['textcomponent']?.['text']?.class,
								customCss?.text?.class,
								classes
							)}
							style={[$app.css?.['textcomponent']?.['text']?.style, customCss?.text?.style].join(
								';'
							)}
						>
							{String(result)}
						</svelte:element>

						{#if resolvedConfig.tooltip && resolvedConfig.tooltip != ''}
							<Tooltip>{resolvedConfig.tooltip}</Tooltip>
						{/if}
						{#if resolvedConfig.copyButton && result}
							<div class="flex">
								<Button
									title="Copy to clipboard"
									variant="border"
									size="xs"
									color="light"
									btnClasses="!p-1"
									on:click={() => copyToClipboard(result)}
								>
									<Clipboard size={14} strokeWidth={2} />
								</Button>
							</div>
						{/if}
					</div>
				{/if}
			</AlignWrapper>
		{/if}
	</div>
</RunnableWrapper>
