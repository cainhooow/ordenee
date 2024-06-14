<script lang="ts">
	import type { Person } from '../../types/Person';

	import { invoke } from '@tauri-apps/api/core';
	import { crossfade } from 'svelte/transition';
	import { quintIn } from 'svelte/easing';
	import { flip } from 'svelte/animate';

	export let client = {} as Person;

	function isNow() {
		let day = 86400000;
		let diff = Math.abs(Date.parse(client.created_at) - Date.now());

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

	const [send, receive] = crossfade({
		fallback(node, params) {
			const style = getComputedStyle(node);
			const transform = style.transform === 'none' ? '' : style.transform;

			return {
				duration: 200,
				easing: quintIn,
				css: (t) => `transform: ${transform} scale(${t});
				opacity: ${t}
				`
			};
		}
	});
</script>

<section
	class="transition-all flex border border-zinc-700 rounded gap-5 hover:drop-shadow-[0_0_100px_rgb(255,0,255,0.5)]"
	in:receive={{ key: client.id }}
	out:send={{ key: client.id }}
>
	<div class="bg-zinc-400/10 p-2">
		<!-- <div
			class="flex h-[4rem] w-[4rem] {randomColor()} rounded-full text-center items-center justify-center shadow-lg shadow-indigo-500/10"
		>
			<span class="text-4xl drop-shadow-md">
				{client.name.slice()[0]}
			</span>
		</div> -->
	</div>
	<div class="flex flex-col w-full p-5">
		<h1 class="text-3xl drop-shadow-lg">{client.name}</h1>
		<div class="mb-3 mt-1">
			{#if isNow()}
				<span
					class="bg-indigo-800 rounded pl-3 pr-3 p-[0.09rem] text-center mt-2 shadow-lg shadow-indigo-500/10 text-indigo-200 font-semibold"
					>#Novo</span
				>
			{/if}
		</div>

		<span class="mb-2 text-zinc-300">
			<i class="ri-calendar-line"></i>
			{isNow() ? 'Recentemente' : new Date(client.created_at).toLocaleDateString('pt-br')}
			as {new Date(client.created_at).getHours()} horas e {new Date(client.created_at).getMinutes()}
			minutos
		</span>
		<span class="mb-2 text-zinc-300">
			<i class="ri-phone-line"></i>
			{client.tel_num || 'Não fornecido'}
		</span>
		<span class="text-zinc-300">
			<i class="ri-mail-line"></i>
			{client.email || 'Não fornecido'}
		</span>
	</div>
</section>
