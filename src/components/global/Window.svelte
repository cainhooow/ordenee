<script lang="ts">
	import { page } from '$app/stores';
	import FastAccess from './Window/FastAccess.svelte';
	import SearchBar from './Window/SearchBar.svelte';
	import { handleCommand } from '../../utils/actions/Commands';
	import ActionHandled from './Dialogs/ActionHandled.dialog.svelte';
	import { ordeneeIADialog } from '../../store';

	let dialogVisible = false;

	ordeneeIADialog.subscribe((vs) => (dialogVisible = vs));

	function currentRoute(route: String) {
		return route == $page.route.id;
	}

	// Matchers: [route1, route2]
	function routeMatch(routes: string[]) {
		return routes.find((route) => route == $page.route.id);
	}

	function closeApp(ev: MouseEvent) {
		ev.preventDefault();
		handleCommand('close-app', ev);
	}
</script>

{#if dialogVisible}
	<ActionHandled />
{/if}

<div
	class="relative flex items-center justify-between w-full top-0 border-b p-[0.3rem] border-b-zinc-700"
	data-tauri-drag-region="true"
>
	<div class="flex gap-3 items-center">
		<a class="text-2xl border-r border-r-zinc-700 pr-2" href="/">Ordenee</a>
		<div class="flex gap-[0.1rem] items-center">
			<div
				class="relative px-2 ps-2 border border-transparent rounded group hover:bg-zinc-800/50 hover:border-zinc-700"
				role="button"
			>
				<FastAccess />
			</div>
		</div>
	</div>
	<SearchBar />
	<div class="mx-2">
		<button
			on:click={closeApp}
			class="transition-all ease-in delay-[10ms] hover:bg-red-400 hover:text-red-900 rounded-full px-1"
		>
			<i class="ri-close-line"></i>
		</button>
	</div>
</div>
