<script lang="ts">
	import { page } from '$app/stores';
	import TextHighlighter from '../components/ui/TextUtils/TextHightLighter.svelte';
    import Sinalization from '../assets/ic/ic_sinalization.svg';
	import Ordenee from '../assets/ic/icon.png';
</script>

<section class="h-screen flex p-3">
	<div class="w-full flex items-center flex-col">
        <img class="transition-all bg-cover object-cover w-[540px] drop-shadow-[0_0_50px_rgb(0,0,0,0.5)] hover:drop-shadow-[0_0_25px_rgb(255,255,55,0.5)]" src={Sinalization} alt="404Sinalization"/>
		<h1 class="text-3xl text-center">
			Ops, parece que você <TextHighlighter>descobriu um caminho</TextHighlighter> secreto...
		</h1>
	</div>
</section>
