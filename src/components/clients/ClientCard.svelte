<script lang="ts">
	export let client: any = {};
</script>

<section class="client-card animation-showing" style="--delay: 1.5s">
	<div class="client-header">
		<div class="client-info">
			<h1 class="client-profile">{client.name.slice()[0]}</h1>
			<h1>{client.name}</h1>
		</div>
		<span class="pt-1.5">
			<i class="ri-calendar-line"></i> Criado em: {new Date(client.createdAt).toLocaleDateString(
				'pt-br'
			)}
			
			as {new Date(client.createdAt).toLocaleTimeString('pt-br')}
		</span>
		<span class="pt-1.5">
			<i class="ri-phone-line"></i>
			{client.telnum}
		</span>
		<span class="pt-0.5">
			<i class="ri-mail-line"></i>
			{client.email}
		</span>
	</div>
	<div class="client-actions pt-1.5">
		<button>
			<i class="ri-delete-bin-line"></i>
			Deletar
		</button>
		<button>
			<i class="ri-edit-circle-line"></i>
			Editar
		</button>
	</div>
</section>

<style lang="scss">
	.client-card {
		position: relative;
		padding: 1.5rem;
		background: var(--primary-opacity-2);
		border-radius: 0.5rem;
		transition: all 200ms ease-in-out;
		border: 0.5px solid #2d2d2d;
		z-index: 1;
		overflow: hidden;

		.client-header {
			display: flex;
			flex-direction: column;
			// padding: 1.5rem;
			h1 {
				font-size: 1.5rem;
			}

			span {
				color: darkgray;
			}
		}
	}

	.client-info {
		display: flex;
		align-items: center;
		gap: 1.5rem;
	}

	.client-profile {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 50px;
		height: 50px;
		background-color: rgb(255, 138, 95);
		border-radius: 50%;
	}
</style>
