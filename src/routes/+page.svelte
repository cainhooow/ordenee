<script lang="ts">
	import NavBar from '../components/global/NavBar.svelte';
	import OrdeneeBanner from '../assets/OrdeneeFull.png';
</script>

<main>
	<section class="app-presentation pb-1.5">
		<img src={OrdeneeBanner} alt="presentation" />
		<div class="pre-bg-metadata">
			<section class="pre-metadata">
				<h1 class="text-5xl">Olá, seja bem-vindo ao Ordenne!</h1>
				<p>
					Obrigado por utilizar nossos serviços. O Ordenne foi feito para gerenciar seus serviços,
					clientes e a sua empresa!
					<br /><br />
					Aqui está um guia de começo: <br /><br />
					- Optar primeiramente em configurar os tecnicos, formas de pagamento e equipamentos, clientes
					são adicionados quando uma nova ordem é criada;<br /><br />
					- Ordens: Todas as ordens podem ser encontradas na Tab "Ordens";<br /><br />
					- Empresa: O sistema pode ser modificado de acordo com a sua empresa, você pode adicionar sua
					logo e informações adicionais.
				</p>
			</section>
		</div>
	</section>
	<section class="container mx-auto app-fastaccess animation-showing" style="--delay: 1.5s">
		<section class="app-fastaccess-header">
			<i class="ri-menu-line"></i>
			<h1>Acesso rapido</h1>
		</section>
		<section class="app-fastaccess-items">
			<a href="/clients" type="button">
				<i class="ri-team-fill"></i>
				Clientes
			</a>

			<a href="/technicals" type="button">
				<i class="ri-team-fill"></i>
				Tecnicos
			</a>

			<a href="/equipaments/new-equipament" type="button">
				<i class="ri-briefcase-2-line"></i>
				Adicionar equipamento
			</a>

			<a href="/equipaments" type="button">
				<i class="ri-briefcase-2-line"></i>
				Equipamentos
			</a>

			<a href="/paymentmethods" type="button">
				<i class="ri-bank-card-line"></i>
				Formas de pagamento
			</a>

			<a href="/orders/new-order" type="button">
				<i class="ri-article-line"></i>
				Nova OS
			</a>

			<a href="/orders" type="button">
				<i class="ri-article-line"></i>
				Ordens
			</a>

			<a href="/work" type="button">
				<i class="ri-shake-hands-line"></i>
				Empresa
			</a>
		</section>
	</section>
</main>

<style lang="scss">
	.app-presentation {
		position: relative;
		//margin: 1.5rem .8rem;
		overflow: hidden;

		img {
			width: 100%;
			height: 450px;
			//border-radius: .5rem;
			object-fit: cover;
		}

		.pre-bg-metadata {
			position: absolute;
			display: flex;
			align-items: end;
			background: linear-gradient(10deg, #000, #00000054);
			//border-radius: .5rem;
			width: 100%;
			height: 450px;
			top: 0;
			bottom: 0;

			.pre-metadata {
				display: flex;
				width: 80%;
				flex-direction: column;
				padding: 1.2rem;
				color: var(--white);

				h1 {
					font-size: 1.5rem;
				}

				p {
					font-size: 1rem;
					margin: 0.5rem 0;
				}
			}
		}
	}

	.app-fastaccess-header {
		display: flex;
		align-items: center;
		padding: 0.5rem 1.5rem;
		gap: 0.5rem;
		background-color: var(--thirdary-opacity-1);

		h1,
		i {
			font-size: 1.5rem;
		}
	}

	.app-fastaccess {
		display: flex;
		width: 100%;
		flex-direction: column;
		padding: 0.5rem 0.9rem;

		.app-fastaccess-items {
			display: flex;
			width: 100%;
			flex-direction: row;
			flex-wrap: wrap;
			flex-basis: 100px;
			padding: 1rem;

			a[type='button'] {
				background: #fff;
				display: flex;
				gap: 0.5rem;
				align-items: center;
				color: var(--primary);
				padding: 0.5rem 1.5rem;
				margin: 0.5rem;
				border-radius: 0.5rem;
				cursor: pointer;
				border: none;
				transition: all 100ms ease-in;

				li {
					font-size: 1rem;
				}

				&:hover {
					background-color: var(--primary);
					color: var(--white);
				}
			}
		}
	}
</style>
