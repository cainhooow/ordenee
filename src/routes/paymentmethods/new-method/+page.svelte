<script lang="ts">
	import ClientHeader from '../../../components/clients/ClientHeader.svelte';
	import { invoke } from '@tauri-apps/api';
	import { redirect } from '@sveltejs/kit';
	
</script>

<ClientHeader justifyContent="space-between">
	<div slot="actions-additionals">
		<a href="/paymentmethods">
			<i class="ri-arrow-left-s-line"></i> Voltar
		</a>
	</div>
</ClientHeader>

<main class="container pt-1.5 animation-showing" style="--delay: 1.5s">
	<h1>Adicionar nova forma de pagamento</h1>
	<p class="text-gray mt-0.5">Campos com "<span class="text-danger">*</span>" são obrigatorios</p>
	<p class="text-gray mt-0.5">
		Ao digitar a forma de pagamento <span>PIX</span>, a chave ou QR code serão requisitados ao
		concluir uma nova ordem, sendo salvos para uma nova ordem.
	</p>
	<form class="pt-1.5" id="form-newpayment">
		<section class="fclient-control">
			<div class="floating-input">
				<input type="text" placeholder="nome" name="nome" id="nome" required />
				<label for="nome"> Nome<span class="text-danger">*</span> </label>
			</div>
		</section>
		<section class="fclient-control pt-1.5">
			<button class="btn" type="button" on:click={formSubmit}>
				<i class="ri-user-add-line"></i>
				Adicionar
			</button>
			<button class="btn" type="reset">
				<i class="ri-delete-back-2-line"></i>
				Limpar
			</button>
		</section>
	</form>
</main>

<style lang="scss">
	.fclient-control {
		display: flex;
		gap: 0.5rem;

		.floating-input {
			width: 100%;
		}

		.floating-input input {
			width: 100%;
		}
	}
</style>

<script lang="ts" context="module">
    
    async function formSubmit(ev: MouseEvent) {
        ev.preventDefault();
        let form = document.getElementById("form-newpayment") as HTMLFormElement;
        let payment = form[0] as HTMLInputElement;

		await invoke('add_payment', { 
			payment: payment.value
		}).then((res) => {
			console.log(":ORDENNE:formSubmit() invoked", res)

			window.location.href = "/paymentmethods"
		}).catch((err) => {
			console.error(err);
		});
    }

</script>