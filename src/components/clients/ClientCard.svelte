<script lang="ts">
	import type { Person } from '../../types/Person';

	export let client = {} as Person;

	function isNow() {
		let day = 24 * 60 * 60 * 1000;
		let diff = Date.parse(client.created_at) - Date.now();

		return diff <= day;
	}

	function randomColor() {
		let colors = [
			'bg-indigo-500',
			'bg-orange-500',
			'bg-yellow-500',
			'bg-purple-500',
			'bg-blue-500',
			'bg-green-500'
		];

		let randomIndex = Math.floor(colors.length * Math.random());
		return colors[randomIndex];
	}
</script>

<section class="flex border border-zinc-700 p-5 rounded gap-5">
	<div>
		<div
			class="flex h-[4rem] w-[4rem] {randomColor()} rounded-full text-center items-center justify-center shadow-lg shadow-indigo-500/10"
		>
			<span class="text-4xl drop-shadow-md">
				{client.name.slice()[0]}
			</span>
		</div>
	</div>
	<div class="flex flex-col w-full">
		<h1 class="text-3xl drop-shadow-lg">{client.name}</h1>
		<div class="mb-3 mt-1">
			{#if isNow()}
				<span
					class="bg-indigo-800 rounded pl-3 pr-3 p-[0.09rem] text-center mt-2 shadow-lg shadow-indigo-500/10 text-indigo-300 font-semibold"
					>#Novo</span
				>
			{/if}
		</div>

		<span class="mb-2 text-zinc-300">
			<i class="ri-calendar-line"></i>
			{isNow() ? 'Hoje' : new Date(client.created_at).toLocaleDateString('pt-br')}
			as {new Date(client.created_at).getHours()} horas e {new Date(client.created_at).getMinutes()} minutos
		</span>
		<span class="mb-2 text-zinc-300">
			<i class="ri-phone-line"></i>
			{client.tel_num || 'Não fornecido'}
		</span>
		<span class="mb-2 text-zinc-300">
			<i class="ri-mail-line"></i>
			{client.email || 'Não fornecido'}
		</span>
	</div>
</section>
