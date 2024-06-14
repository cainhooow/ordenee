<script lang="ts">
	export let title = '';
</script>

<section
	class="flex flex-row justify-between items-center p-5 px-10 py-5 mb-5 top-0 z-10"
>
	<slot name="actions-additionals" />
	<div class="rounded pr-3 pl-3">
		<h1 class="text-3xl bold">{title}</h1>
	</div>
	<slot name="buttons-actions" />
</section>
