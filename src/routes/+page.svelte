<script lang="ts">
	import Anchor from '../components/ui/Button/Anchor.svelte';
	import IconAnchor from '../components/ui/Button/IconAnchor.svelte';

	import { OrdeneeNer } from '../utils/ordenee/OrdeneeNer';
</script>

<section
	class="relative overflow-hidden m-3 rounded-md transition-all delay-75 shadow-[0_0px_300px_100px_rgb(0,0,0/0.1)] shadow-black/30 hover:shadow-[0_0px_500px_100px_rgb(0,0,0/0.1)] hover:shadow-green-600/10 z-0"
>
	<div
		class="relative flex items-end bg-gradient-to-tr from-black to-zinc-950/10 w-full top-0 bottom-0 p-5 z-0 rounded-md"
	>
		<section class="p-2">
			<h1 class="text-4xl w-2/3 mb-3">Olá, seja bem-vindo ao Ordenee!</h1>
			<p class="mb-3">
				Obrigado por utilizar nossos serviços. O Ordenee foi feito para gerenciar seus serviços,
				clientes e a sua empresa!
				<br /><br />
				Aqui está um guia de começo: <br /><br />
				- Optar primeiramente em configurar as formas de pagamento e equipamentos, clientes são adicionados
				quando uma nova ordem é criada;<br /><br />
				- Ordens: Todas as ordens podem ser encontradas na Tab "Ordens";<br /><br />
				- Empresa: O sistema pode ser modificado de acordo com a sua empresa, você pode adicionar sua
				logo e informações adicionais.
			</p>
		</section>
	</div>
</section>
<section
	class="transition-all relative px-5 py-5 mx-5 z-10 border border-zinc-700 bg-gradient-to-bl from-zinc-600/10 to-purple-400/10 rounded-md mt-10 drop-shadow-[0_0_170px_rgb(255,0,255,1)] mb-20"
>
	<section class="flex gap-3 text-3xl">
		<h1>Acesso rapido</h1>
	</section>
	<section class="flex flex-wrap basis-10 p-2 gap-3 pt-5 pb-5">
		<IconAnchor
			class="transition-all bg-purple-700/20 border-purple-800 text-purple-200 hover:border-purple-600 hover:text-purple-100"
			href="/clients"
		>
			<div class="bg-purple-200 text-purple-900 pr-1 pl-1 rounded" slot="icon">
				<i class="ri-team-fill"></i>
			</div>
			Clientes
		</IconAnchor>

		<IconAnchor
			class="transition-all bg-purple-700/20 border-purple-800 text-purple-200 hover:border-purple-600 hover:text-purple-100"
			href="/equipaments"
		>
			<div class="bg-purple-200 text-purple-900 pr-1 pl-1 rounded" slot="icon">
				<i class="ri-briefcase-2-line"></i>
			</div>
			Equipamentos
		</IconAnchor>

		<IconAnchor
			class="transition-all bg-purple-700/20 border-purple-800 text-purple-200 hover:border-purple-600 hover:text-purple-100"
			href="/paymentmethods"
		>
			<div class="bg-purple-200 text-purple-900 pr-1 pl-1 rounded" slot="icon">
				<i class="ri-bank-card-line"></i>
			</div>
			Formas de pagamento
		</IconAnchor>

		<IconAnchor
			class="transition-all bg-purple-700/20 border-purple-800 text-purple-200 hover:border-purple-600 hover:text-purple-100"
			href="/orders"
		>
			<div class="bg-purple-200 text-purple-900 pr-1 pl-1 rounded" slot="icon">
				<i class="ri-article-line"></i>
			</div>
			Ordens
		</IconAnchor>

		<IconAnchor
			class="transition-all bg-purple-700/20 border-purple-800 text-purple-200 hover:border-purple-600 hover:text-purple-100"
			href="/work"
		>
			<div class="bg-purple-200 text-purple-900 pr-1 pl-1 rounded" slot="icon">
				<i class="ri-shake-hands-line"></i>
			</div>
			Empresa
		</IconAnchor>

		<IconAnchor href="/equipaments/new-equipament" class="transition-all bg-purple-700/20 border-purple-800 text-purple-200 hover:border-purple-600 hover:text-purple-100">
			<div class="bg-purple-200 text-purple-900 pr-1 pl-1 rounded" slot="icon">
				<i class="ri-briefcase-2-line"></i>
			</div>
			Adicionar equipamento
		</IconAnchor>

		<IconAnchor href="/clients/new-client" class="transition-all bg-purple-700/20 border-purple-800 text-purple-200 hover:border-purple-600 hover:text-purple-100">
			<div class="bg-purple-200 text-purple-900 pr-1 pl-1 rounded" slot="icon">
				<i class="ri-team-fill"></i>
			</div>
			Adicionar cliente
		</IconAnchor>

		<IconAnchor href="/orders/new-order" class="transition-all bg-purple-700/20 border-purple-800 text-purple-200 hover:border-purple-600 hover:text-purple-100">
			<div class="bg-purple-200 text-purple-900 pr-1 pl-1 rounded" slot="icon">
				<i class="ri-article-line"></i>
			</div>
			Nova OS
		</IconAnchor>

		<!-- <Anchor href="/equipaments/new-equipament" type="button">
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
		</Anchor> -->
	</section>
</section>
