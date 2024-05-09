<script lang="ts">
	import type { Payment } from '../../types/Payment';

	export let method = {} as Payment;
</script>

<button class="card p-1.5 animation-showing" style="--delay: 1.5s" tabindex="{method.id}" on:click={(ev) => gotoPaymentInfo(ev, method.id)}>
	<h3>{method.name}</h3>
</button>

<style lang="scss">
	.card {
		border: 1px solid #2d2d2d;
		border-radius: 0.5rem;
		background: var(--primary-opacity-2);
		transition: all 200ms ease-in;
		cursor: pointer;

		&:hover {
			background-color: var(--thirdary);
		}

		&:focus {
			background-color: var(--primary);
		}

		h3 {
			text-align: start;
		}
	}
</style>

<script lang="ts" context="module">
	import type { MouseEventHandler } from 'svelte/elements';

	function gotoPaymentInfo(ev: MouseEvent, methodId: number) {
		ev.preventDefault();
		window.location.href = `/paymentmethods/methods/${methodId}`
	}
</script>