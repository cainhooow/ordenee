<script lang="ts">
	import CommandBar from './CommandBar.svelte';

	let commandBarVisible: boolean;

	//NOTE - Actions for showing the command/search bar
	function closeByEsq(ev: KeyboardEvent) {
		ev.preventDefault();
		if (ev.code === 'Escape') {
			commandBarVisible = false;
		}
	}

	function openCommandBar(ev: MouseEvent) {
		ev.preventDefault();
		let target = ev.target as HTMLElement;

		commandBarVisible = !commandBarVisible;

		if (commandBarVisible) {
			setTimeout(() => {
				let input = document.getElementById('command-bar') as HTMLInputElement;
				input.focus();

				input.addEventListener('keydown', preventSpaceClose);
			}, 0);
		}
	}

	function preventSpaceClose(ev: KeyboardEvent) {
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
		<span>Ordenee IA</span>
	</div>
</button>
{#if commandBarVisible}
	<CommandBar />
{/if}
