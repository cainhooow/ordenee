<script>
	import DialogContainer from '../../ui/Dialog/DialogContainer.svelte';
	import DialogDescription from '../../ui/Dialog/DialogDescription.svelte';
	import DialogHeader from '../../ui/Dialog/DialogHeader.svelte';

	import DialogOverflow from '../../ui/Dialog/DialogOverflow.svelte';
	import DialogTitle from '../../ui/Dialog/DialogTitle.svelte';
	import TextHightLighter from '../../ui/TextUtils/TextHightLighter.svelte';
</script>

<DialogOverflow>
	<DialogContainer>
		<DialogHeader>
			<DialogTitle>Ordenee</DialogTitle>
			<DialogDescription>
				Um comando foi executado e a <TextHightLighter>Ordenee IA</TextHightLighter> esta processando,
				aguarde...<br />
			</DialogDescription>
		</DialogHeader>
	</DialogContainer>
</DialogOverflow>
