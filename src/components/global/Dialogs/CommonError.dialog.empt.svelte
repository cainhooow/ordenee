<script lang="ts">
	import DialogOverflow from '../../ui/Dialog/DialogOverflow.svelte';
	import DialogContainer from '../../ui/Dialog/DialogContainer.svelte';
	import DialogHeader from '../../ui/Dialog/DialogHeader.svelte';
	import DialogTitle from '../../ui/Dialog/DialogTitle.svelte';
	import DialogDescription from '../../ui/Dialog/DialogDescription.svelte';
	import DialogActions from '../../ui/Dialog/DialogActions.svelte';
	import Button from '../../ui/Button/Button.svelte';
	import TextHightLighter from '../../ui/TextUtils/TextHightLighter.svelte';

	export let closeAct: any;
</script>

<DialogOverflow>
	<DialogContainer>
		<DialogHeader>
			<DialogTitle>Campos inválidos</DialogTitle>
			<DialogDescription>
				Por favor, preencha todos os campos marcados com uma borda alaranjada. <br />
				<TextHightLighter>
					Caso o problema persista, entre em contato com o suporte
				</TextHightLighter>
			</DialogDescription>
		</DialogHeader>
		<DialogActions>
			<Button onclick={closeAct}>Entendido!</Button>
		</DialogActions>
	</DialogContainer>
</DialogOverflow>
