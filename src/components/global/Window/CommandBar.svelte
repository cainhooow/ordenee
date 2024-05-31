<script lang="ts">
	import { ordeneeIADialog } from '../../../store';
	import type { Action } from '../../../types/Command/Action';
	import { commands, getCommand, handleCommand, mapCommandsAction, mapCommandsName } from '../../../utils/actions/Commands';
	import OrdeneeSentencer from '../../../utils/ordenee/OrdeneeSentencer';
	import Input from '../../ui/Input/Input.svelte';

	let searchText = '';

	//NOTE - ENDING FOR ACTIONS;
	//NOTE - Handle actions for commands
	function handleEvent(ev: MouseEvent, action: Action) {
		ev.preventDefault();
		handleCommand(action.command.name, ev);
	}

	function searchHandled(ev: InputEvent) {
		ev.preventDefault();
		searchText = (ev.target as HTMLInputElement).value;
	}

	function search(ev: MouseEvent) {
		ev.preventDefault();
		const commander = new OrdeneeSentencer();

		ordeneeIADialog.set(true);
		
		commander.query(searchText, mapCommandsAction()).then(cmd => {
			console.log(cmd);
			getCommand(cmd)?.command.run();
		});
	}
</script>

<div class="top-0 left-0 right-0 absolute p-2 bg-zinc-800 rounded border border-zinc-700 z-50">
	<div class="flex gap-2">
		<Input
			id="command-bar"
			class="p-1 w-full"
			placeholder="Busque páginas/comandos"
			oninput={searchHandled}
		/>

		<button on:click={search} class="rounded pr-3 pl-3 bg-purple-500 hover:bg-purple-700"><i class="ri-corner-down-left-fill"></i></button>
	</div>
	<div class="mt-4 flex gap-3 flex-col">
		{#if searchText === '' || searchText.length <= 0}
			{#each commands as action}
				<!-- {#if action.command.type !== 'app-action'} -->
					<button
						class="flex flex-col text-lg px-2 w-full border border-transparent hover:border-zinc-700 hover:bg-zinc-700/50 rounded"
						on:click={(ev) => handleEvent(ev, action)}
					>
						{typeof action.action === 'object' ? action.action[0] : action.action}
						<span class="text-sm text-zinc-500">{action.command.description}</span>
					</button>
				<!-- {/if} -->
			{/each}
		{/if}
	</div>
</div>
