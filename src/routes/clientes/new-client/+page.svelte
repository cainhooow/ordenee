<script>
	import ClientHeader from '../../../components/clientes/ClientHeader.svelte';
</script>

<ClientHeader justifyContent="space-between">
	<div slot="actions-additionals">
		<a href="/clientes">
			<i class="ri-arrow-left-s-line"></i> Voltar
		</a>
	</div>
</ClientHeader>

<main class="container pt-1.5 animation-showing" style="--delay: 1.5s">
	<h1>Controle de cliente</h1>
	<form action="POST" class="pt-1.5">
		<section class="fclient-control">
			<div class="floating-input">
				<input type="text" placeholder="nome" name="nome" id="nome" required />
				<label for="nome"> Nome </label>
			</div>
			<div class="floating-input">
				<input type="email" placeholder="email" name="email" id="email" required />
				<label for="email"> Email </label>
			</div>
		</section>
		<section class="fclient-control pt-1.5">
			<div class="floating-input">
				<input type="telephony" placeholder="telefone" name="telefone" id="telefone" required />
				<label for="telefone"> Telefone </label>
			</div>
		</section>
		<section class="fclient-control pt-1.5">
			<button class="btn" type="button">
				<i class="ri-user-add-line"></i>
				Adicionar
			</button>
			<button class="btn" type="reset">
				<i class="ri-delete-back-2-line"></i>
				 Limpar
			</button>
		</section>
	</form>
</main>

<style lang="scss">
	.fclient-control {
		display: flex;
		gap: 0.5rem;
	}

	.fclient-control .floating-input {
		width: 100%;
	}

	.fclient-control .floating-input input {
		width: 100%;
	}
</style>
