<script lang="ts">
	import { ordeneeIADialog, ordeneeIADialogResults } from '../../../store';
	import type { Action } from '../../../types/Command/Action';
	import {
		commands as commandList,
		getCommand,
		handleCommand,
		mapCommandsAction
	} from '../../../utils/actions/Commands';
	import OrdeneeSentencer from '../../../utils/ordenee/OrdeneeSentencer';
	import Input from '../../ui/Input/Input.svelte';

	let searchText = '';
	let commands = commandList;

	//NOTE - ENDING FOR ACTIONS;
	//NOTE - Handle actions for commands
	function handleEvent(ev: MouseEvent, action: Action) {
		ev.preventDefault();
		handleCommand(action.command.name, { fromClick: true });
	}

	function searchHandled(ev: InputEvent) {
		ev.preventDefault();
		searchText = (ev.target as HTMLInputElement).value;

		if (searchText.includes('/')) {
			let commandSearch = commandList.filter((act) => act.command.name.includes(searchText));
			commands = commandSearch;
		}
	}

	function search(ev: MouseEvent) {
		ev.preventDefault();
		const commander = new OrdeneeSentencer();

		ordeneeIADialog.set(true);
		ordeneeIADialogResults.update((vs) => [
			...vs,
			`Olá, eu recebi seu pedido! Agora irei analisar.... Aguarde....`
		]);

		commander
			.query(searchText, mapCommandsAction())
			.then((cmd) => {
				ordeneeIADialogResults.update((vs) => [
					...vs,
					`Pedido analisado, agora irei executar o comando requisitado: ${cmd}`
				]);
				getCommand(cmd)?.command.run({ searchText });
			})
			.catch((err) => {
				ordeneeIADialogResults.update((vs) => [
					...vs,
					`Ops, ocorreu um erro ao processar a sua busca: ${err}`
				]);
			});
	}
</script>

<div class="top-0 left-0 right-0 absolute p-2 bg-zinc-800 rounded border border-zinc-700 z-50">
	<div class="flex gap-2">
		<Input
			id="command-bar"
			class="p-1 w-full"
			placeholder="Busque comandos ou peça para a Ordenee AI"
			oninput={searchHandled}
		/>

		<button on:click={search} class="rounded pr-3 pl-3 bg-purple-500 hover:bg-purple-700"
			><i class="ri-corner-down-left-fill"></i></button
		>
	</div>
	<div class="mt-4 flex gap-3 flex-col">
		{#if searchText === '' || (searchText.length <= 0 && !searchText.includes('/'))}
			{#each commands as action}
				{#if action.filtable}
					<button
						class="flex flex-col text-lg px-2 w-full border border-transparent hover:border-zinc-700 hover:bg-zinc-700/50 rounded"
						on:click={(ev) => handleEvent(ev, action)}
					>
						{typeof action.action === 'object' ? action.action[0] : action.action}
						<span class="text-sm text-zinc-500">{action.command.description}</span>
					</button>
				{/if}
			{/each}
		{/if}
		{#if searchText.includes('/')}
			{#each commands as action}
				{#if action.filtable}
					<button
						class="flex flex-col text-lg px-2 w-full border border-transparent hover:border-zinc-700 hover:bg-zinc-700/50 rounded"
						on:click={(ev) => handleEvent(ev, action)}
					>
						{typeof action.action === 'object' ? action.action[0] : action.action}
						<span class="text-sm text-zinc-500">{action.command.name}</span>
					</button>
				{/if}
			{/each}
		{/if}

		<span class="text-sm text-zinc-300 font-semibold"
			>Você ira requisitar uma ação para Ordenee AI ao clicar em <i class="ri-corner-down-left-fill"
			></i></span
		>
	</div>
</div>
