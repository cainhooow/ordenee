<script lang="ts">
	import type { Action } from '../../../types/Command/Action';
	import { commands, handleCommand } from '../../../utils/actions/Commands';
	import Input from '../../ui/Input/Input.svelte';

	let searchText = '';

	//NOTE - ENDING FOR ACTIONS;
	//NOTE - Handle actions for commands
	function handleEvent(ev: MouseEvent, action: Action) {
		ev.preventDefault();
		handleCommand(action.command.name, ev);
	}

	function search(ev: InputEvent) {
		ev.preventDefault();

		searchText = (ev.target as HTMLInputElement).value;
	}
</script>

<div class="top-0 left-0 right-0 absolute p-2 bg-zinc-800 rounded border border-zinc-700 z-50">
	<div class="flex">
		<Input
			id="command-bar"
			class="p-1 w-full"
			placeholder="Busque páginas/comandos"
			oninput={search}
		/>
	</div>
	<div class="mt-4 flex gap-3 flex-col">
		{#if searchText === '' || searchText.length <= 0}
			{#each commands as action}
				{#if action.command.type !== 'app-action'}
					<button
						class="flex flex-col text-lg px-2 w-full border border-transparent hover:border-zinc-700 hover:bg-zinc-700/50 rounded"
						on:click={(ev) => handleEvent(ev, action)}
					>
						{action.action}
						<span class="text-sm text-zinc-500">{action.command.description}</span>
					</button>
				{/if}
			{/each}
		{/if}
	</div>
</div>
