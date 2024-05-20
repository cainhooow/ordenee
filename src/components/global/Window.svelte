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

<div
	class="relative flex items-center justify-between w-full top-0 p-2 border-b border-b-zinc-700"
	data-tauri-drag-region="true"
>
	<div class="flex gap-3 items-center">
		<a class="text-2xl border-r border-r-zinc-700 pr-2" href="/">Ordenne</a>
		<div class="flex gap-[0.1rem] items-center">
			<div
				class="relative px-2 ps-2 border border-transparent rounded group hover:bg-zinc-800/50 hover:border-zinc-700"
				role="button"
			>
				Acesso rápido
				<ul
					class="hidden absolute left-0 right-0 w-auto top-6 bg-zinc-800/90 backdrop-blur-md rounded border border-zinc-700 group-hover:inline group z-50"
				>
					<li class="flex">
						<a class="hover:bg-zinc-600/50 w-full p-[0.2rem] pl-2" href="/clients"> Clientes </a>
					</li>
					<li class="flex">
						<a class=" hover:bg-zinc-600/50 w-full p-[0.2rem] pl-2" href="/technicals">
							Técnicos
						</a>
					</li>
					<li class="flex">
						<a class=" hover:bg-zinc-600/50 w-full p-[0.2rem] pl-2" href="/equipaments">
							Equipamentos
						</a>
					</li>
					<li class="flex">
						<a class=" hover:bg-zinc-600/50 w-full p-[0.2rem] pl-2" href="/orders"> Ordens </a>
					</li>
					<li class="flex">
						<a class=" hover:bg-zinc-600/50 w-full p-[0.2rem] pl-2" href="/work"> Empresa </a>
					</li>
				</ul>
			</div>
		</div>
	</div>
	<div class="mx-1">
		<span> X </span>
	</div>
</div>
