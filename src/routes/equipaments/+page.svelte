<script lang="ts">
	import ClientHeader from "../../components/clients/ClientHeader.svelte";
	import IconAnchor from "../../components/ui/Button/IconAnchor.svelte";
	import NavBar from "../../components/global/NavBar.svelte";
</script>


<ClientHeader title={'Equipamentos'}>
	<div slot="buttons-actions">
		<IconAnchor class="transition-all bg-purple-700/20 border-purple-800 text-purple-200 hover:border-purple-600 hover:text-purple-100" href="/equipaments/new-equipament">
			<div class="bg-purple-200 text-purple-900 pr-1 pl-1 rounded" slot="icon">
				<i class="ri-id-card-line"></i>
			</div>
			Adicionar Equipamento
		</IconAnchor>
	</div>
</ClientHeader>