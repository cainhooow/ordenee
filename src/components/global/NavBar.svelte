<script lang="ts">
	import { page } from '$app/stores';

	function currentRoute(route: String) {
		return route == $page.route.id;
	}

	// Matchers: [route1, route2]
	function routeMatch(routes: string[]) {
		console.log(routes);
		return routes.find((route) => route == $page.route.id);
	}
</script>

<nav class="navbar">
	<section class="app-title">
		<a href="/">
			<h1>Ordenne</h1>
		</a>
	</section>
	<section class="app-discover">
		<a class={routeMatch(['/clients', '/clients/new-client']) ? 'active' : ''} href="/clients">
			Clientes
		</a>
		<a class={routeMatch(['/technicals']) ? 'active' : ''} href="/technicals"> Tecnicos </a>
		<a
			class={routeMatch(['/equipaments', '/equipaments/new-equipament']) ? 'active' : ''}
			href="/equipaments"
		>
			Equipamentos
		</a>
		<a class={currentRoute('/orders') ? 'active' : ''} href="/orders"> Ordens </a>
		<a class={currentRoute('/work') ? 'active' : ''} href="/work"> Empresa </a>
	</section>
</nav>

<style lang="scss">
	.navbar {
		display: flex;
		align-items: center;
		justify-content: space-between;

		background-color: var(--secondary);
		padding: 1.5rem;

		.app-title {
			font-size: 1.6rem;
		}

		.app-discover {
			margin: 0.5rem;
			position: relative;

			a {
				margin: 0.5rem;
				position: relative;
				z-index: 1;
				padding: .5rem;
				background-color: var(--secondary);

				&.active {
					background-color: var(--white);
					border-radius: 0.3rem;
					color: var(--primary);
					box-shadow: 1px 6px 15px 0.1px var(--secondary);
					transition: all 50ms ease-in-out;

					&::before {
						content: '';
						position: absolute;
						width: 100%;
						height: 0.1rem;
						left: 0;
						bottom: -0.5rem;
						z-index: 0;
						transition: all 200ms ease-in-out;
						z-index: 0;
					}
				}

				&::before {
					content: '';
					position: absolute;
					background-color: var(--white);
					width: 0;
					height: 0.1rem;
					left: 0;
					bottom: -0.5rem;
					z-index: 0;
					transition: all 200ms ease-in-out;
					z-index: 0;
				}

				&:hover::before {
					width: 100%;
				}
			}
		}
	}
</style>
