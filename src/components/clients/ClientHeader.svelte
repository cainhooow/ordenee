<script lang="ts">
	export let title = '';
	export let justifyContent = 'space-evenly';
</script>

<section class="client-header p-1.5" style="--chd_justify-content: {justifyContent}">
	<slot name="actions-additionals" />
	<div>
		<h1>{title}</h1>
	</div>
	<slot name="buttons-actions" />
</section>

<style lang="scss">
	.client-header {
		display: flex;
		justify-content: var(--chd_justify-content);
		align-items: center;
		background: linear-gradient(190deg, var(--primary-opacity-1), var(--secondary));
	}
</style>