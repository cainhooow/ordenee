<script lang="ts">
	export let title = '';
</script>

<section
	class="flex flex-row justify-between items-center p-5 px-10 py-5 mb-5 bg-zinc-600/10 border border-zinc-700 rounded-md m-2"
>
	<slot name="actions-additionals" />
	<div>
		<h1 class="text-3xl bold">{title}</h1>
	</div>
	<slot name="buttons-actions" />
</section>
