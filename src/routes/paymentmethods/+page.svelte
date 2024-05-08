<script>
	import ClientCard from "../../components/clients/ClientCard.svelte";


	import ClientHeader from "../../components/clients/ClientHeader.svelte";

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
	<section class="container clients p-1.5">
		
	</section>
</main>

<style lang="scss">
	.clients {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
</style>
