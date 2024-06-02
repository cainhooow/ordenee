<script lang="ts">
	import { page } from '$app/stores';
	import TextHighlighter from '../components/ui/TextUtils/TextHightLighter.svelte';
    import Sinalization from '../assets/ic/ic_sinalization.svg';
</script>

<section class="h-screen flex p-3">
	<div class="w-full flex items-center flex-col">
        <img class="drop-shadow-lg bg-cover object-cover w-[540px]" src={Sinalization} alt="404Sinalization"/>
		<h1 class="text-3xl text-center">
			Ops, parece que você <TextHighlighter>descobriu um caminho</TextHighlighter> secreto...
		</h1>
	</div>
</section>
