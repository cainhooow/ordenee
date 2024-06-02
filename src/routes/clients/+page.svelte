<script lang="ts">
	import ClientHeader from '../../components/clients/ClientHeader.svelte';
	import ClientCard from '../../components/clients/ClientCard.svelte';
	import Anchor from '../../components/ui/Button/Anchor.svelte';
	import type { Person } from '../../types/Person';

	import { invoke } from '@tauri-apps/api';
	import { crossfade } from 'svelte/transition';
	import { quintIn } from 'svelte/easing';
	import { flip } from 'svelte/animate';

	export let data;

	const clientsList = (data.clients as Person[]) ?? [];
	let clients = clientsList;

	const [send, receive] = crossfade({
		fallback(node, params) {
			const style = getComputedStyle(node);
			const transform = style.transform === 'none' ? '' : style.transform;

			return {
				duration: 160,
				easing: quintIn,
				css: (t) => `transform: ${transform} scale(${t});
				opacity: ${t}
				`
			};
		}
	});

	function search(ev: any) {
		const val = (ev.target as HTMLInputElement).value;

		clients = clientsList.filter(
			(client) =>
				client.name.includes(val) ||
				client.email?.includes(val) ||
				client.tel_num?.toString().includes(val) ||
				client.person_id?.includes(val)
		);
	}
</script>

<ClientHeader title={'Clientes'}>
	<div slot="buttons-actions">
		<Anchor href="/clients/new-client" type="button">
			<i class="ri-id-card-line"></i>
			Adicionar novo cliente
		</Anchor>
	</div>
</ClientHeader>
<section class="p-3 flex flex-col gap-2 mt-5 mb-10">
	<div class="mb-3 flex">
		<div class="px-5 py-3 bg-zinc-600/10 border-b border-t border-l border-zinc-700 rounded-l">
			<i class="ri-search-line"></i>
		</div>
		<input
			class="group w-full p-3 transition-all delay-100 bg-zinc-600/10 border-r border-t border-b border-zinc-700 rounded-r-md appearance-none outline-none"
			on:input={search}
			placeholder="Buscar cliente"
		/>
	</div>
	{#each clients as client (client.id)}
		<ClientCard {client}/>
	{/each}
</section>
