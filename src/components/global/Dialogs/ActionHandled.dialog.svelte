<script lang="ts">
	import DialogContainer from '../../ui/Dialog/DialogContainer.svelte';
	import DialogDescription from '../../ui/Dialog/DialogDescription.svelte';
	import DialogHeader from '../../ui/Dialog/DialogHeader.svelte';

	import DialogOverflow from '../../ui/Dialog/DialogOverflow.svelte';
	import DialogTitle from '../../ui/Dialog/DialogTitle.svelte';
	import TextHightLighter from '../../ui/TextUtils/TextHightLighter.svelte';
	import Ordenee from '../../../assets/ic/icon.png';
	import { ordeneeIADialogResults } from '../../../store';
	import { crossfade } from 'svelte/transition';
	import { quintIn } from 'svelte/easing';

	let ordeneeResults = [] as string[];

	const [send, receive] = crossfade({
		fallback(node, params) {
			const style = getComputedStyle(node);
			const transform = style.transform === 'none' ? '' : style.transform;

			return {
				duration: 10,
				easing: quintIn,
				css: (t) => `transform: ${transform} scale(${t});
				opacity: ${t}
				`
			};
		}
	});

	ordeneeIADialogResults.subscribe((vs) => (ordeneeResults = vs));
</script>

<DialogOverflow>
	<DialogContainer>
		<DialogHeader>
			<DialogTitle>
				<div class="flex gap-2 items-center">
					<img class="w-10" src={Ordenee} alt="Ordenee icon ORDENEE AI" />
					Ordenee
				</div>
			</DialogTitle>
			<DialogDescription>
				<div class="mt-3">
					{#each ordeneeResults as result, i}
						<p class="mb-2" in:receive={{ key: i + 1 }} out:send={{ key: i + 1 }}>
							{result}
						</p>
					{/each}
				</div>
			</DialogDescription>
		</DialogHeader>
	</DialogContainer>
</DialogOverflow>
