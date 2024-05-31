<script lang="ts">
	import ClientHeader from '../../components/clients/ClientHeader.svelte';
	import ClientCard from '../../components/clients/ClientCard.svelte';
	import Anchor from '../../components/ui/Button/Anchor.svelte';

	export let data;
	const clients = data.clients ?? [];
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
	{#each clients as client}
		<ClientCard client={client} />
	{/each}
</section>
