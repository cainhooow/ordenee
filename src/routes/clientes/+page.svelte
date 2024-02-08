<script lang="ts">
	import ClientHeader from '../../components/clientes/ClientHeader.svelte';
	import ClienteCard from '../../components/clientes/ClienteCard.svelte';
	import NavBar from '../../components/global/NavBar.svelte';
	
	let clientes = [
		{
			name: 'João Silva',
			createdAt: new Date('2023-06-04 22:40:01'),
			telnum: '+55(13)998877665',
			email: 'agat@gmail.com',
			cpf: '123.456.789-01'
		},
		{
			name: 'Maria Oliveira',
			createdAt: new Date('2023-06-04 22:40:01'),
			telnum: '+55(13)997654321',
			email: 'agat@gmail.com',
			cpf: '987.654.321-00'
		},
		{
			name: 'Carlos Souza',
			createdAt: new Date('2023-06-04 12:40:01'),
			telnum: '+55(13)996633445',
			email: 'agat@gmail.com',
			cpf: '111.222.333-44'
		},
		{
			name: 'Ana Santos',
			createdAt: new Date('2023-03-04 22:40:01'),
			telnum: '+55(13)995566778',
			email: 'agat@gmail.com',
			cpf: '555.444.333-22'
		},
		{
			name: 'Rafael Pereira',
			createdAt: new Date('2023-09-10 15:30:00'),
			telnum: '+55(13)994433221',
			email: 'agat@gmail.com',
			cpf: '777.888.999-00'
		},
		// Adicionando novos clientes
		{
			name: 'Fernanda Lima',
			createdAt: new Date('2023-11-20 18:15:00'),
			telnum: '+55(13)993322110',
			email: 'agat@gmail.com',
			cpf: '333.222.111-00'
		},
		{
			name: 'Luiz Oliveira',
			createdAt: new Date('2023-08-15 10:00:00'),
			telnum: '+55(13)992211334',
			email: 'agat@gmail.com',
			cpf: '444.555.666-77'
		}
	];
</script>

<main>
	<ClientHeader title={'Clientes'}>
		<div slot="buttons-actions">
			<a href="/clientes/new-client" class=":hover-def with-ic" type="button">
				<i class="ri-user-add-line"></i>
				Novo cliente
			</a>
		</div>
	</ClientHeader>
	<section class="container clients p-1.5">
		{#each clientes as cliente}
			<ClienteCard {cliente} />
		{/each}
	</section>
</main>

<style lang="scss">
	.clients {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
</style>
