<script lang="ts">
	import { goto } from '$app/navigation'
	import { logout } from '$lib/logout'
	import {
		userStore,
		usersWorkspaceStore,
		superadmin,
		usageStore,
		premiumStore,
		workspaceStore
	} from '$lib/stores'
	import { classNames } from '$lib/utils'
	import { faCog, faCrown, faHardHat, faSignOut, faUser } from '@fortawesome/free-solid-svg-icons'
	import Icon from 'svelte-awesome'
	import Menu from '../common/menu/Menu.svelte'
	import { SUPERADMIN_SETTINGS_HASH, USER_SETTINGS_HASH } from './settings'
	import { isCloudHosted } from '$lib/cloud'
	import { switchWorkspace } from '$lib/storeUtils'

	export let isCollapsed: boolean = false
</script>

<Menu let:close placement="bottom-start">
	<button
		title="User Menu"
		slot="trigger"
		type="button"
		class={classNames(
			'group w-full flex items-center text-white hover:bg-gray-50 hover:text-gray-900  focus:outline-none  px-2 py-2 text-sm font-medium rounded-md h-8 '
		)}
	>
		<Icon
			data={faUser}
			class={classNames('flex-shrink-0 h-4 w-4', isCollapsed ? '-mr-1' : 'mr-2')}
		/>

		{#if !isCollapsed}
			<span class={classNames('whitespace-pre truncate')}>
				{$userStore?.username ?? ($superadmin ? $superadmin : '___')}
				{#if $userStore?.is_admin}
					<Icon data={faCrown} scale={0.6} />
				{:else if $userStore?.operator}
					<Icon class="ml-2" data={faHardHat} scale={0.8} />
				{/if}
			</span>
		{/if}
	</button>

	<div class="divide-y">
		<div class="px-4 py-3" role="none">
			<p class="text-sm font-medium text-primary truncate" role="none">
				{$usersWorkspaceStore?.email}
			</p>
			<span class="text-xs text-tertiary">
				{#if $userStore?.is_admin}
					Admin of this workspace <Icon data={faCrown} scale={0.6} />
				{:else if $userStore?.operator}
					Operator in this workspace <Icon class="ml-2" data={faHardHat} scale={0.8} />
				{/if}
			</span>
		</div>

		<div class="py-1" role="none">
			<a
				href={USER_SETTINGS_HASH}
				class="text-secondary block px-4 py-2 text-sm hover:bg-surface-hover hover:text-primary"
				role="menuitem"
				tabindex="-1"
			>
				<Icon class="pr-0.5" data={faCog} /> Account settings
			</a>
		</div>
		{#if $superadmin}
			<div class="py-1" role="none">
				<a
					href={SUPERADMIN_SETTINGS_HASH}
					class="text-secondary block px-4 py-2 text-sm hover:bg-surface-hover hover:text-primary"
					role="menuitem"
					tabindex="-1"
				>
					<Icon class="pr-0.5" data={faCog} /> Superadmin settings
				</a>
			</div>
			<div class="py-1" role="none">
				<button
					on:click={() => {
						if ($workspaceStore === 'admins') {
							return
						}
						switchWorkspace('admins')
						close()
					}}
					class="text-secondary block text-left px-4 py-2 font-normal text-sm hover:bg-surface-hover hover:text-primary w-full"
					role="menuitem"
					tabindex="-1"
				>
					<Icon class="pr-0.5" data={faCog} /> Superadmin workspace
				</button>
			</div>
		{/if}
		<div class="py-1" role="none">
			<button
				type="button"
				class="text-secondary block w-full text-left px-4 py-2 text-sm hover:bg-surface-hover hover:text-primary"
				role="menuitem"
				tabindex="-1"
				on:click={() => logout()}
			>
				<Icon class="pr-0.5" data={faSignOut} /> Sign out
			</button>
		</div>
		{#if isCloudHosted() && $premiumStore}
			{#if !$premiumStore.premium}
				<div class="py-1" role="none">
					<span class="text-secondary block w-full text-left px-4 py-2 text-sm"
						>{$usageStore}/1000 free-tier executions</span
					>
					<div class="w-full bg-gray-200 h-1">
						<div class="bg-blue-400 h-1" style="width: {Math.min($usageStore, 1000) / 10}%" />
					</div>
					{#if $userStore?.is_admin}
						<button
							type="button"
							class="text-secondary block font-normal w-full text-left px-4 py-2 text-sm hover:bg-gray-100 hover:text-gray-900"
							role="menuitem"
							tabindex="-1"
							on:click={() => {
								close()
								goto('/workspace_settings?tab=premium')
							}}
						>
							Upgrade
						</button>
					{/if}
				</div>
			{:else}
				<div class="py-1" role="none">
					<button
						type="button"
						class="text-secondary block font-normal w-full text-left px-4 py-2 text-sm hover:bg-gray-100 hover:text-gray-900"
						role="menuitem"
						tabindex="-1"
						on:click={() => {
							close()
							goto('/workspace_settings?tab=premium')
						}}>Premium plan</button
					>
				</div>
			{/if}
		{/if}
	</div>
</Menu>
