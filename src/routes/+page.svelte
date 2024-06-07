<script lang="ts">
	import Anchor from '../components/ui/Button/Anchor.svelte';
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
	class="transition-all relative px-5 py-5 mx-5 z-10 border border-zinc-700 bg-gradient-to-bl from-zinc-600/10 to-purple-400/10 rounded-md mt-10 drop-shadow-[0_0_170px_rgb(255,0,255,1)]"
>
	<section class="flex gap-3 text-3xl">
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
