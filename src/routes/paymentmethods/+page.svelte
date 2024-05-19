<script lang="ts">
	import ClientHeader from '../../components/clients/ClientHeader.svelte';
	import List from '../../components/paymentmethods/List.svelte';
	import Anchor from '../../components/ui/Button/Anchor.svelte';

	export let data;
	const methods = data.methods ?? [];
</script>

<main>
	<ClientHeader title={'Formas de pagamento'}>
		<div slot="buttons-actions">
			<Anchor href="/paymentmethods/new-method" type="button">
				<i class="ri-card"></i>
				Nova forma de pagamento
			</Anchor>
		</div>
	</ClientHeader>
	<section class="container flex flex-col gap-[0.5rem] p-[1.5rem]">
		{#each methods as method}
			<List {method} />
		{/each}
	</section>
</main>
