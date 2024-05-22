<script lang="ts">
	import type { Action } from '../../../types/Command/Action';
	import { commands, handleCommand } from '../../../utils/actions/Commands';
	import Input from '../../ui/Input/Input.svelte';
	import type { MouseEventHandler } from 'svelte/elements';

	let commandBarVisible: boolean;

	//NOTE - Actions for showing the command/search bar
	function closeByEsq(ev: KeyboardEvent) {
		ev.preventDefault();
		if (ev.code === 'Escape') {
			commandBarVisible = false;
		}
	}

	function openCommandBar(ev: MouseEvent) {
		ev.preventDefault();
		let target = ev.target as HTMLElement;

		commandBarVisible = !commandBarVisible;

		if (commandBarVisible) {
			setTimeout(() => {
				let input = document.getElementById('command-bar') as HTMLInputElement;
				input.focus();

				input.addEventListener('keydown', preventSpaceClose);
			}, 0);
		}
	}

	function preventSpaceClose(ev: KeyboardEvent) {
		if (ev.key === ' ') {
			ev.preventDefault();
			ev.stopPropagation();
			(ev.target as HTMLInputElement).value += ' ';
		}
	}

	window.onkeyup = (ev) => {
		closeByEsq(ev);
	};
	//NOTE - ENDING FOR ACTIONS;

	//NOTE - Handle actions for commands
	function handleEvent(ev: MouseEvent, action: Action) {
		ev.preventDefault();
		handleCommand(action.command.name, ev);
	}
</script>

<button
	class="p-1 bg-zinc-800/50 rounded border border-zinc-700 w-1/2 flex items-center justify-center hover:border-zinc-600 hover:bg-zinc-800/70 z-20"
	on:click={openCommandBar}
	tabindex="0"
>
	<div class="text-zinc-500">
		<i class="ri-search-2-line"></i>
		<span>Algo interessante</span>
	</div>
</button>
{#if commandBarVisible}
	<div class="top-0 left-0 right-0 absolute p-2 bg-zinc-800 rounded border border-zinc-700 z-50">
		<div class="flex">
			<Input id="command-bar" class="p-1 w-full" placeholder="Busque páginas/comandos" />
		</div>
		<div class="mt-4 flex gap-3 flex-col">
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
		</div>
	</div>
{/if}
