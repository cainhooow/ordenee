<script lang="ts">
	import NavBar from '../components/global/NavBar.svelte';
	import OrdeneeBanner from '../assets/OrdeneeFull.png';
	import Anchor from '../components/ui/Button/Anchor.svelte';
</script>

<section class="relative overflow-hidden">
	<img class="object-cover h-[550px]" src={OrdeneeBanner} alt="presentation" />
	<div
		class="absolute flex items-end bg-gradient-to-t from-zinc-950 to-zinc-950/70 w-full h-[550px] top-0 bottom-0 p-5 z-0"
	>
		<section class="">
			<h1 class="text-5xl w-2/3 drop-shadow-lg mb-3">Olá, seja bem-vindo ao Ordenne!</h1>
			<p class="mb-3">
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
<section class="container mx-auto mt-5 mb-10">
	<section class="flex gap-3 text-3xl border-b border-b-zinc-600 pb-3 mb-3">
		<i class="ri-menu-line"></i>
		<h1>Acesso rapido</h1>
	</section>
	<section class="flex flex-wrap basis-10 p-2 gap-3 pt-5 pb-5">
		<Anchor href="/clients" type="button">
			<i class="ri-team-fill"></i>
			Clientes
		</Anchor>
		
		<Anchor href="/equipaments/new-equipament" type="button">
			<i class="ri-briefcase-2-line"></i>
			Adicionar equipamento
		</Anchor>

		<Anchor href="/equipaments" type="button">
			<i class="ri-briefcase-2-line"></i>
			Equipamentos
		</Anchor>

		<Anchor href="/paymentmethods" type="button">
			<i class="ri-bank-card-line"></i>
			Formas de pagamento
		</Anchor>

		<Anchor href="/orders/new-order" type="button">
			<i class="ri-article-line"></i>
			Nova OS
		</Anchor>

		<Anchor href="/orders" type="button">
			<i class="ri-article-line"></i>
			Ordens
		</Anchor>

		<Anchor href="/work" type="button">
			<i class="ri-shake-hands-line"></i>
			Empresa
		</Anchor>
	</section>
</section>
