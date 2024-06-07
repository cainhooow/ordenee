<script lang="ts">
	// $$restProps.class = [
	//     $$restProps.class,
	//     'border border-zinc-600 hover:border-violet-600 transition-all delay-[1.5ms] bg-zinc-800/10 hover:bg-zinc-900/30 p-2 px-5 rounded text-zinc-200 hover:text-zinc-50'
	// ]

	const props = $$restProps;
	props.class = [props.class, 'border border-zinc-600 pr-4 pl-4 py-2 rounded flex gap-2 items-center'].join(' ');
</script>

<button {...$$restProps}>
	<slot name="icon"></slot>
    <slot/>
</button>
