<script lang="ts">
	import ClientHeader from '../../../components/clients/ClientHeader.svelte';
	import FloatingInputContainer from '../../../components/ui/Input/FloatingInputContainer.svelte';
	import FloatingInput from '../../../components/ui/Input/FloatingInput.svelte';
	import FloatingInputLabel from '../../../components/ui/Input/FloatingInputLabel.svelte';
	import Button from '../../../components/ui/Button/Button.svelte';

	import { invoke } from '@tauri-apps/api';

	async function formSubmit(e: MouseEvent) {
		e.preventDefault();

		const form = {
			name: (document.getElementById('nome') as HTMLInputElement).value,
			email: (document.getElementById('email') as HTMLInputElement).value,
			person_id: (document.getElementById('cpf') as HTMLInputElement).value, 
			tel_num: (document.getElementById('telefone') as HTMLInputElement).value
 		};

		if (form.name === '' || form.name.length <= 0) {
			return;
		}

		await invoke('add_client', {
			client: JSON.stringify(form)
		}).then((res) => {
			window.location.href = "/clients"
		})
	}
</script>

<ClientHeader justifyContent="space-between">
	<div slot="actions-additionals">
		<a href="/clientes">
			<i class="ri-arrow-left-s-line"></i> Voltar
		</a>
	</div>
</ClientHeader>
<div class="container mx-auto">
	<h1 class="text-3xl">Adicionar novo cliente</h1>
	<p class="text-gray">Campos com "<span class="text-danger">*</span>" são obrigatorios</p>
	<form action="POST" class="mt-[1.5rem]">
		<section class="flex mb-[1.5rem] gap-3">
			<FloatingInputContainer class="w-full">
				<FloatingInput
					class="border w-full"
					type="text"
					placeholder="nome"
					name="nome"
					id="nome"
					required
				/>
				<FloatingInputLabel for="method">Nome</FloatingInputLabel>
			</FloatingInputContainer>
			<FloatingInputContainer class="w-full">
				<FloatingInput
					class="border w-full"
					type="email"
					placeholder="email"
					name="email"
					id="email"
				/>
				<FloatingInputLabel for="method">Email</FloatingInputLabel>
			</FloatingInputContainer>
		</section>
		<section class="flex mb-[1.5rem] gap-3">
			<FloatingInputContainer class="w-full">
				<FloatingInput class="border w-full" type="text" placeholder="cpf" name="cpf" id="cpf" />
				<FloatingInputLabel for="method">CPF</FloatingInputLabel>
			</FloatingInputContainer>

			<FloatingInputContainer class="w-full">
				<FloatingInput
					class="border w-full"
					type="telephony"
					placeholder="telefone"
					name="telefone"
					id="telefone"
				/>
				<FloatingInputLabel for="method">Telefone</FloatingInputLabel>
			</FloatingInputContainer>
		</section>
		<section class="flex gap-3">
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
