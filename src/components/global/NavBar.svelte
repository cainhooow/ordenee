<script lang="ts">
	import { page } from '$app/stores';

	function currentRoute(route: String) {
		return route == $page.route.id;
	}

	// Matchers: [route1, route2]
	function routeMatch(routes: string[]) {
		return routes.find((route) => route == $page.route.id);
	}
</script>

<nav class="flex flex-col justify-around items-center border-b border-b-zinc-600 p-5 shadow-lg mt-[0.5rem]">
	<section class="container mx-auto flex flex-row gap-3 text-lg">
		<a
			class={routeMatch(['/clients', '/clients/new-client'])
				? 'text-white bg-violet-500 ps-4 px-4 rounded-lg transition-all ease-in-out delay-150'
				: 'text-zinc-300 ps-4 px-4 rounded-lg'}
			href="/clients"
		>
			Clientes
		</a>
		<a
			class={routeMatch(['/technicals'])
				? 'text-white bg-violet-500 ps-4 px-4 rounded-lg transition-all ease-in-out delay-150'
				: 'text-zinc-300 ps-4 px-4 rounded-lg'}
			href="/technicals"
		>
			Tecnicos
		</a>
		<a
			class={routeMatch(['/equipaments', '/equipaments/new-equipament'])
				? 'text-white bg-violet-500 ps-4 px-4 rounded-lg transition-all ease-in-out delay-150'
				: 'text-zinc-300 ps-4 px-4 rounded-lg'}
			href="/equipaments"
		>
			Equipamentos
		</a>
		<a
			class={currentRoute('/orders')
				? 'text-white bg-violet-500 ps-4 px-4 rounded-lg transition-all ease-in-out delay-150'
				: 'text-zinc-300 ps-4 px-4 rounded-lg'}
			href="/orders"
		>
			Ordens
		</a>
		<a
			class={currentRoute('/work')
				? 'text-white bg-violet-500 ps-4 px-4 rounded-lg transition-all ease-in-out delay-150'
				: 'text-zinc-300 ps-4 px-4 rounded-lg'}
			href="/work"
		>
			Empresa
		</a>
	</section>
</nav>
