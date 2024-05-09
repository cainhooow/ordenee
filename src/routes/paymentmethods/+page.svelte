<script lang="ts">
	import ClientHeader from "../../components/clients/ClientHeader.svelte";
	import List from "../../components/paymentmethods/List.svelte";
	
	export let data;
	const methods = data.methods ?? [];

</script>

<main>
	<ClientHeader title={'Formas de pagamento'}>
		<div slot="buttons-actions">
			<a href="/paymentmethods/new-method" class=":hover-def with-ic" type="button">
				<i class="ri-card"></i>
				Nova forma de pagamento
			</a>
		</div>
	</ClientHeader>
	<section class="container d-flex flex-col gap-0.5 p-1.5">
		{#each methods as method}
			<List method={method}/>
		{/each}
	</section>
</main>