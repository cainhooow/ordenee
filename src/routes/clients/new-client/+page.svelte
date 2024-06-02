<script lang="ts">
	import ClientHeader from '../../../components/clients/ClientHeader.svelte';
	import FloatingInputContainer from '../../../components/ui/Input/FloatingInputContainer.svelte';
	import FloatingInput from '../../../components/ui/Input/FloatingInput.svelte';
	import FloatingInputLabel from '../../../components/ui/Input/FloatingInputLabel.svelte';
	import Button from '../../../components/ui/Button/Button.svelte';
	import ErrorClientAdd from '../../../components/global/Dialogs/CommonError.dialog.empt.svelte';

	import type { Person } from '../../../types/Person';

	import { invoke } from '@tauri-apps/api';
	export let dialogVisible = false;
	export let addresses = [] as Array<{ formid: number }>;

	async function formSubmit(e: MouseEvent) {
		e.preventDefault();

		let allowedAddress = addresses.map((address: { formid: number }) => {
			const addressName = document.getElementById(`address-${address.formid}`) as HTMLInputElement;

			if (addressName.value === '' || addressName.name.length <= 0) {
				dialogVisible = true;
				return;
			}

			return {
				address: addressName.value,
				home_num:
					(document.getElementById(`home_num-${address.formid}`) as HTMLInputElement).value.length >
					0
						? parseInt(
								(document.getElementById(`home_num-${address.formid}`) as HTMLInputElement).value
							)
						: 0,
				street:
					(document.getElementById(`street-${address.formid}`) as HTMLInputElement).value.length > 0
						? (document.getElementById(`street-${address.formid}`) as HTMLInputElement).value
						: null,
				city:
					(document.getElementById(`city-${address.formid}`) as HTMLInputElement).value.length > 0
						? (document.getElementById(`city-${address.formid}`) as HTMLInputElement).value
						: null
			};
		});

		// console.log(allowedAddress);

		const form = {
			name: (document.getElementById('nome') as HTMLInputElement).value,
			email:
				(document.getElementById('email') as HTMLInputElement).value.length > 0
					? (document.getElementById('email') as HTMLInputElement).value
					: null,
			person_id:
				(document.getElementById('cpf') as HTMLInputElement).value.length > 0
					? (document.getElementById('cpf') as HTMLInputElement).value
					: null,
			tel_num:
				(document.getElementById('telefone') as HTMLInputElement).value.length > 0
					? (document.getElementById('telefone') as HTMLInputElement).value
					: null,
			is_technical: false
		};

		if (form.name === '' || form.name.length <= 0) {
			dialogVisible = true;
			return;
		}

		await invoke('add_client', {
			client: JSON.stringify(form)
		})
			.then(async (res: unknown | Person) => {
				const user = res as Person;

				if (allowedAddress.length >= 0) {
					allowedAddress.map(async (address) => {
						await invoke('create_address', {
							address: JSON.stringify({ person_id: user.id, ...address })
						}).then((d) => {
							window.location.href = "/clients"
						}).catch((err) => {
							console.error(err);
						});
					});

					return;
				}

				window.location.href = "/clients"
			})
			.catch((err) => {
				console.error(err);
			});
	}

	function dialog(ev: CustomEvent) {
		dialogVisible = !dialogVisible;
	}

	function addAddress(ev: MouseEvent) {
		ev.preventDefault();
		addresses.push({ formid: addresses.length + 1 }) as any;
		addresses = addresses;
	}

	function rmAddress(ev: MouseEvent) {
		ev.preventDefault();
		addresses.pop();
		addresses = addresses;
	}
</script>

{#if dialogVisible}
	<ErrorClientAdd closeAct={dialog} />
{/if}

<ClientHeader justifyContent="space-between">
	<div slot="actions-additionals">
		<a href="/clientes">
			<i class="ri-arrow-left-s-line"></i> Voltar
		</a>
	</div>
</ClientHeader>

<div class="container mx-auto mb-20">
	<section>
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
	</section>
	<section class="mt-10">
		{#each addresses as address}
			<form action="POST" class="mt-[1.5rem] mb-20">
				<h1 class="text-2xl">Linha de endereço {address.formid}</h1>
				<section class="flex mb-[1.5rem] gap-3 mt-[1.5rem]">
					<FloatingInputContainer class="w-full">
						<FloatingInput
							class="border w-full"
							type="text"
							placeholder="address"
							name="address"
							id="address-{address.formid}"
							required
						/>
						<FloatingInputLabel for="address">Endereço</FloatingInputLabel>
					</FloatingInputContainer>
					<FloatingInputContainer class="w-full">
						<FloatingInput
							class="border w-full"
							type="number"
							placeholder="home_num"
							name="home_num"
							id="home_num-{address.formid}"
						/>
						<FloatingInputLabel for="home_num">Número</FloatingInputLabel>
					</FloatingInputContainer>
				</section>
				<section class="flex mb-[1.5rem] gap-3">
					<FloatingInputContainer class="w-full">
						<FloatingInput
							class="border w-full"
							type="text"
							placeholder="street"
							name="street"
							id="street-{address.formid}"
						/>
						<FloatingInputLabel for="street">Rua</FloatingInputLabel>
					</FloatingInputContainer>

					<FloatingInputContainer class="w-full">
						<FloatingInput
							class="border w-full"
							type="text"
							placeholder="city"
							name="city"
							id="city-{address.formid}"
						/>
						<FloatingInputLabel for="method">Cidade</FloatingInputLabel>
					</FloatingInputContainer>
				</section>
			</form>
		{/each}
		<div class="flex justify-end mt-5 gap-3">
			{#if addresses.length > 0}
				<Button
					onclick={rmAddress}
					class="bg-red-500/75 p-2 pr-4 pl-4 rounded text-red-200 font-bold"
				>
					<i class="ri-delete-bin-2-line bg-red-200 text-red-900 p-1 rounded mr-1"></i>
					Remover ultimo
				</Button>
			{/if}
			<Button
				onclick={addAddress}
				class="bg-green-500/75 p-2 pr-4 pl-4 rounded text-green-100 font-bold"
			>
				<i class="ri-map-pin-add-line bg-green-200 text-green-900 p-1 rounded mr-1"></i>
				Adicionar endereço
			</Button>
		</div>
	</section>
</div>
