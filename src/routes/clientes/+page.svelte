<script lang="ts">
	import ClienteCard from '../../components/clientes/ClienteCard.svelte';
	import NavBar from '../../components/global/NavBar.svelte';

	let clientes = [
		{
			name: 'João Silva',
			createdAt: new Date('2023-06-04 22:40:01'),
			telnum: '+55(13)998877665',
			cpf: '123.456.789-01'
		},
		{
			name: 'Maria Oliveira',
			createdAt: new Date('2023-06-04 22:40:01'),
			telnum: '+55(13)997654321',
			cpf: '987.654.321-00'
		},
		{
			name: 'Carlos Souza',
			createdAt: new Date('2023-06-04 12:40:01'),
			telnum: '+55(13)996633445',
			cpf: '111.222.333-44'
		},
		{
			name: 'Ana Santos',
			createdAt: new Date('2023-03-04 22:40:01'),
			telnum: '+55(13)995566778',
			cpf: '555.444.333-22'
		},
		{
			name: 'Rafael Pereira',
			createdAt: new Date('2023-09-10 15:30:00'),
			telnum: '+55(13)994433221',
			cpf: '777.888.999-00'
		},
		// Adicionando novos clientes
		{
			name: 'Fernanda Lima',
			createdAt: new Date('2023-11-20 18:15:00'),
			telnum: '+55(13)993322110',
			cpf: '333.222.111-00'
		},
		{
			name: 'Luiz Oliveira',
			createdAt: new Date('2023-08-15 10:00:00'),
			telnum: '+55(13)992211334',
			cpf: '444.555.666-77'
		}
	];
	console.log(clientes);
</script>

<main class="container pt-1.5">
	<section class="client-header p-1.5">
		<div>
			<h1>Clientes</h1>
		</div>
		<div>
			<button class=":hover-def with-ic">
				<i class="ri-user-add-line"></i>
				Novo cliente
			</button>
		</div>
	</section>
	<section class="clients p-1.5">
		{#each clientes as cliente}
			<ClienteCard {cliente} />
		{/each}
	</section>
</main>

<style lang="scss">
	.client-header {
		display: flex;
		justify-content: space-evenly;
		background: linear-gradient(190deg, var(--primary-opacity-1), var(--secondary));
		border-radius: 0.5rem;
	}

	.clients {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
</style>
