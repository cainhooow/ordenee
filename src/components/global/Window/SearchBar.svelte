<script lang="ts">
	import Input from '../../ui/Input/Input.svelte';

	let commandBarVisible: boolean;

	function closeByEsq(ev: KeyboardEvent) {
		ev.preventDefault();
		if (ev.code === 'Escape') {
			commandBarVisible = false;
		}
	}

	function openCommandBar(ev: MouseEvent) {
		ev.preventDefault();
		let target = ev.target as HTMLElement;

		if (target.tagName === 'BUTTON') {
			commandBarVisible = !commandBarVisible;

			if (commandBarVisible) {
				setTimeout(() => {
					let input = document.getElementById('command-bar') as HTMLInputElement;
					input.focus();

					input.addEventListener('keydown', preventSpaceClose);
				}, 0);
			}
		}
	}

	function preventSpaceClose(ev: KeyboardEvent) {
		console.log('preventSpaceClose called', { ev });
		if (ev.key === ' ') {
			ev.preventDefault();
			ev.stopPropagation();
			(ev.target as HTMLInputElement).value += ' ';
		}
	}

	window.onkeyup = (ev) => {
		closeByEsq(ev);
	};
</script>

<button
	class="p-1 bg-zinc-800/50 rounded border border-zinc-700 w-1/2 flex items-center justify-center hover:border-zinc-600 hover:bg-zinc-800/70 z-20"
	on:click={openCommandBar}
	tabindex="0"
>
	<div class="text-zinc-500">
		<i class="ri-search-2-line"></i>
		<span>Algo interessante</span>
	</div>

	{#if commandBarVisible}
		<div class="top-0 left-0 right-0 absolute p-2 bg-zinc-800 rounded border border-zinc-700 z-50">
			<div class="flex">
				<Input id="command-bar" class="p-1 w-full" placeholder="Busque páginas/comandos" />
			</div>
			<div class=""></div>
		</div>
	{/if}
</button>
