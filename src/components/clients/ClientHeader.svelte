<script lang="ts">
	export let title = '';
	export let justifyContent = 'space-evenly';
</script>

<section class="flex flex-row justify-around items-center p-3" style="--chd_justify-content: {justifyContent}">
	<slot name="actions-additionals" />
	<div>
		<h1 class="text-3xl bold">{title}</h1>
	</div>
	<slot name="buttons-actions" />
</section>