<script lang="ts">
	import DialogContainer from '../../ui/Dialog/DialogContainer.svelte';
	import DialogDescription from '../../ui/Dialog/DialogDescription.svelte';
	import DialogHeader from '../../ui/Dialog/DialogHeader.svelte';

	import DialogOverflow from '../../ui/Dialog/DialogOverflow.svelte';
	import DialogTitle from '../../ui/Dialog/DialogTitle.svelte';
	import TextHightLighter from '../../ui/TextUtils/TextHightLighter.svelte';
	import Ordenee from '../../../assets/ic/icon.png';
	import { ordeneeIADialogResults } from '../../../store';

	let ordeneeResults = [] as string[];

	ordeneeIADialogResults.subscribe((vs) => (ordeneeResults = vs));
</script>

<DialogOverflow>
	<DialogContainer>
		<DialogHeader>
			<DialogTitle>
				<div class="flex gap-2 items-center">
					<img class="w-10" src={Ordenee} alt="Ordenee icon ORDENEE AI"> 
					Ordenee
				</div>
			</DialogTitle>
			<DialogDescription>
				Um comando foi executado e a <TextHightLighter>Ordenee AI</TextHightLighter> esta processando,
				aguarde...<br />

				<div class="mt-3">
					{#each ordeneeResults as result}
						<p class="mb-2">{result}</p>
					{/each}
				</div>
			</DialogDescription>
		</DialogHeader>
	</DialogContainer>
</DialogOverflow>
