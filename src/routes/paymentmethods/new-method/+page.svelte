<script lang="ts">
	import ClientHeader from '../../../components/clients/ClientHeader.svelte';
	import { invoke } from '@tauri-apps/api';

	import Button from '../../../components/ui/Button/Button.svelte';
	import FloatingInput from '../../../components/ui/Input/FloatingInput.svelte';
	import FloatingInputContainer from '../../../components/ui/Input/FloatingInputContainer.svelte';
	import FloatingInputLabel from '../../../components/ui/Input/FloatingInputLabel.svelte';
	import ErrorPaymentAdd from '../../../components/paymentmethods/new-method/ErrorPaymentAdd.svelte';
	import TextHightLighter from '../../../components/ui/TextUtils/TextHightLighter.svelte';

	export let dialogVisible = false;

	async function formSubmit(ev: CustomEvent<any>) {
		ev.preventDefault();
		let form = document.getElementById('form-newpayment') as HTMLFormElement;
		let payment = form[0] as HTMLInputElement;

		if (payment.value === '' || payment.value.length <= 0) {
			dialogVisible = true;
			return;
		}

		await invoke('add_payment', {
			payment: payment.value
		})
			.then((res) => {
				console.log(':ORDENNE:formSubmit() invoked', res);

				window.location.href = '/paymentmethods';
			})
			.catch((err) => {
				console.error(err);
			});
	}

	function dialog(ev: CustomEvent) {
		dialogVisible = !dialogVisible;
	}
</script>

{#if dialogVisible}
	<ErrorPaymentAdd closeAct={dialog} />
{/if}

<ClientHeader justifyContent="end" title="">
	<div slot="actions-additionals">
		<a href="/paymentmethods">
			<i class="ri-arrow-left-s-line"></i> Voltar
		</a>
	</div>
</ClientHeader>

<div class="container mx-auto">
	<h1 class="text-2xl font-bold border-b border-b-zinc-500 pb-2">
		Adicionar nova forma de pagamento
	</h1>
	<p class="text-gray mt-[0.8rem]">
		Campos com bordas alaranjadas são obrigatórios e cinzas opcionais.
	</p>
	<p class="text-gray mt-[0.8rem]">
		Ao digitar a forma de pagamento <TextHightLighter class="px-1">PIX</TextHightLighter>, a chave
		ou QR code serão requisitados ao concluir uma nova ordem, sendo salvos para uma nova ordem.
	</p>
	<form class="pt-[1.5rem]" id="form-newpayment">
		<section class="flex w-full items-center">
			<FloatingInputContainer class="w-full">
				<FloatingInput class="border w-full" type="text" placeholder="Metodo" required />
				<FloatingInputLabel for="method">Método</FloatingInputLabel>
			</FloatingInputContainer>
		</section>
		<section class="flex pt-[1.5rem] gap-3">
			<Button type="button" onclick={formSubmit}>
				<i class="ri-user-add-line"></i>
				Adicionar
			</Button>
			<Button type="reset">
				<i class="ri-delete-back-2-line"></i>
				Limpar
			</Button>
		</section>
	</form>
</div>
