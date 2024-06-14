<script lang="ts">
	import ClientHeader from '../../components/clients/ClientHeader.svelte';
	import ClientCard from '../../components/clients/ClientCard.svelte';
	import Anchor from '../../components/ui/Button/Anchor.svelte';
	import type { Person, ReturnablePerson } from '../../types/Person';
	import IconAnchor from '../../components/ui/Button/IconAnchor.svelte';

	export let data;
	const clientsList = (data.clients as ReturnablePerson[]) ?? [];
	let clients = clientsList;

	function search(ev: any) {
		const val = (ev.target as HTMLInputElement).value;

		clients = clientsList.filter(
			(client) =>
				client.person.name.includes(val) ||
				client.person.email?.includes(val) ||
				client.person.tel_num?.toString().includes(val) ||
				client.person.person_id?.includes(val)
		);
	}
</script>

<ClientHeader title={'Clientes'}>
	<div slot="buttons-actions">
		<IconAnchor class="transition-all bg-purple-700/20 border-purple-800 text-purple-200 hover:border-purple-600 hover:text-purple-100" href="/clients/new-client">
			<div class="bg-purple-200 text-purple-900 pr-1 pl-1 rounded" slot="icon">
				<i class="ri-id-card-line"></i>
			</div>
			Adicionar cliente
		</IconAnchor>
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
	{#each clients as client (client.person.id)}
		<ClientCard client={client.person} />
	{/each}
</section>
