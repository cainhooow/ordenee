<script lang="ts">
	import ClientHeader from '../../components/clients/ClientHeader.svelte';
	import MethodsList from '../../components/paymentmethods/MethodsList.svelte';
	import Anchor from '../../components/ui/Button/Anchor.svelte';
	import IconAnchor from '../../components/ui/Button/IconAnchor.svelte';

	export let data;
	const methods = data.methods ?? [];
</script>

<ClientHeader title={'Formas de pagamento'}>
	<div slot="buttons-actions">
		<IconAnchor
			class="transition-all bg-purple-700/20 border-purple-800 text-purple-200 hover:border-purple-600 hover:text-purple-100"
			href="/paymentmethods/new-method"
		>
			<div class="bg-purple-200 text-purple-900 pr-1 pl-1 rounded" slot="icon">
				<i class="ri-bank-card-line"></i>
			</div>
			Novo método
		</IconAnchor>
	</div>
</ClientHeader>
<section class="mx-5 flex flex-col gap-[0.5rem]">
	{#each methods as method}
		<MethodsList {method} />
	{/each}
</section>
